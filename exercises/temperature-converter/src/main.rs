fn main() {
    let temp: f64 = 300.0;
    let result = convert_celsius_to_fahrenheit(temp);
    println!("{temp}C is equal to {result}F")
}

fn convert_celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}
