fn main() {
    let n: u32 = 10;
    let term = fibonacci(n);
    let position = get_position(n);
    println!("The {position} fibonacci number is {term}")
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn get_position(n: u32) -> String {
    if n % 10 == 1 {
        n.to_string() + "st"
    } else if n % 10 == 2 {
        n.to_string() + "nd"
    } else if n % 10 == 3 {
        n.to_string() + "rd"
    } else {
        n.to_string() + "th"
    }
}
