fn main() {
    let literal = "a string literal";
    println!("Hello, world! {}", literal);

    {
        let another_lit = "another one";
        println!("hi there {}", another_lit);
    } // another_lit goes out of scope


    if literal.is_empty() {
        let if_literal = "if literal is empty";
        println!("hi there {}", if_literal);
    } // if_literal goes out of scope

    println!("I can't do this {}", if_literal);

} // end of main literal goes out of scope and is no longer valid
