fn main() {
    println!("Hello, world!");

    another_function(5);

    // Statements and Expressions
    let y = {
        let x = 3;
        // Expressions do not include ending semicolons
        x + 1
    };
    println!("The value of y is: {y}");

    let a = one();
    println!("The value of a is: {a}");
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

// If you don't specify a return type, () i.e. the unit type is returned by default
fn one() -> i32 {
    // Implicit return
    1
}
