fn main() {
    // Rust scalar types
    // - Integers
    // - Floating-point numbers
    // - Booleans
    // - Characters

    // Integer types in Rust
    // Signed integers: i8, i16, i32, i64, i128, isize
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    // Default integer type in Rust is i32
    let x: i32 = 10;
    let y: u32 = 20;
    println!("x = {}", x);
    println!("y = {}", y);

    // Floating-point numbers
    // f32, f64
    // Default floating-point type in Rust is f64
    // f64 is roughly the same speed as f32, but it's capable of more precision
    let a: f32 = 10.0;
    let b: f64 = 20.0;
    println!("a = {}", a);
    println!("b = {}", b);

    // Boolean types in Rust
    let c: bool = true;
    let d: bool = false;
    println!("c = {}", c);
    println!("d = {}", d);

    // Character types in Rust
    let e: char = 'a';
    let f: char = 'b';
    println!("e = {}", e);
    println!("f = {}", f);

    // Rust primitive compound types
    // - Tuples
    // - Arrays

    // Tuple types in Rust
    // Access to elements is by destructuring or by index with dot notation
    let g: (i32, f64, bool, char) = (10, 20.0, true, 'a');
    println!("g = {:?}", g);

    // Array types in Rust
    // Access to elements is by index with square bracket notation
    let h: [i32; 3] = [10, 20, 30];
    println!("h = {:?}", h);
}
