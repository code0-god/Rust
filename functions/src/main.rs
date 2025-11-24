fn main() {
    print_labeled_measurement(5, 'h');
    
    let y = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons
    };
    println!("The value of y: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/* the return value of the function is synonymous 
   with the value of the final expression in the block of the body of a function. */
fn five() -> i32 {
    5 // most functions return the last expression implicitly.
    // return 5; // also possible
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // compile error
}