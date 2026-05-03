fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {MAX_POINTS}");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y is {y}");
}
