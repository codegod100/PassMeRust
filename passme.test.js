import {test,expect} from "bun:test"
const {check_password} = require("./index.node")
test("test password",()=>{
    expect(check_password("hello universe")).toBe("Password strength: Very Strong")
    expect(check_password("yolo")).toBe("Password strength: Weak")
    expect(check_password("password")).toBe("Password is weak: It matches a common password or dictionary word.")
})
