use std::fs;
use std::path::Path;
use neon::prelude::*;

// Importing the entropy module
mod entropy;

fn check_password(mut cx: FunctionContext) -> JsResult<JsString> {
    let password = cx.argument::<JsString>(0)?.value(&mut cx) as String;
    let dictionary_path = Path::new("data/dictionary.txt");
    let common_passwords_path = Path::new("data/common-passwords.txt");
    if is_password_in_dictionary(&password, dictionary_path)
        || is_password_common(&password, common_passwords_path)
    {
        let message = "Password is weak: It matches a common password or dictionary word.".to_string();
        return Ok(cx.string(message))
    } else {
        // Use the imported calculate_entropy function
        let entropy = entropy::calculate_entropy(&password);
        let message = format!("Password strength: {}", entropy::get_strength(entropy)); 
        return Ok(cx.string(message))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()>{
    cx.export_function("check_password", check_password)?;
    Ok(())

}

fn is_password_in_dictionary(password: &str, dictionary_path: &Path) -> bool {
    let dictionary = fs::read_to_string(dictionary_path).unwrap();
    dictionary.lines().any(|line| line == password)
}

fn is_password_common(password: &str, common_passwords_path: &Path) -> bool {
    let common_passwords = fs::read_to_string(common_passwords_path).unwrap();
    common_passwords.lines().any(|line| line == password)
}
