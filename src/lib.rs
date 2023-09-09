use neon::prelude::*;

// Importing the entropy module
mod entropy;

const common_passwords: &str = include_str!("../data/common-passwords.txt");
const dictionary: &str = include_str!("../data/dictionary.txt");

fn check_password(mut cx: FunctionContext) -> JsResult<JsString> {
    let password = cx.argument::<JsString>(0)?.value(&mut cx) as String;

    if is_password_in_dictionary(&password) || is_password_common(&password) {
        let message =
            "Password is weak: It matches a common password or dictionary word.".to_string();
        return Ok(cx.string(message));
    } else {
        // Use the imported calculate_entropy function
        let entropy = entropy::calculate_entropy(&password);
        let message = format!("Password strength: {}", entropy::get_strength(entropy));
        return Ok(cx.string(message));
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("check_password", check_password)?;
    Ok(())
}

fn is_password_in_dictionary(password: &str) -> bool {
    dictionary.lines().any(|line| line == password)
}

fn is_password_common(password: &str) -> bool {
    common_passwords.lines().any(|line| line == password)
}
