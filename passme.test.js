import {test,expect} from "bun:test"
import {check_password} from "./passme.js"
test("test password",()=>{
    expect(check_password("hello universe")).toBe("Password strength: Very Strong")
})
