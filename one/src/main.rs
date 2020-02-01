fn main() {
    // literal comes into scope at this line -- cant use it before that line
    let literal = "a string literal";
    println!("Hello, world! {}", literal);

    // Here, we're creating a scope/block with brackets, just like functions,
    // branch arms (match), and ifs
    {
        // Can't use another_lit before this line
        let another_lit = "another one";
        println!("hi there {}", another_lit);
    } // another_lit goes out of scope
    // Cant use another_lit after the closing curly

    if literal.is_empty() {
        let if_literal = "if literal is empty";
        println!("hi there {}", if_literal);
    } // if_literal goes out of scope

    println!("I can't do this {}", if_literal); // oops tried to use if_literal

} // end of main literal goes out of scope and is no longer valid
