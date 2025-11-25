fn main() {
    let number = 3;

    // Unlike languages such as Ruby and JavaScript,
    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if is an expression
    // let number = if condition { 5 } else { "six" }; // type mismatch 
    println!("The value of number is: {number}");
}
