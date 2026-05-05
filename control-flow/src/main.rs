fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    // Taking advantage of the shadowing feature here
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    loop_n_times_loop(5);
    loop_n_times_while(5);
    loop_n_times_for(5);
}

fn loop_n_times_loop(iterations: i32) {
    let mut counter = 0;

    // You can return values from "loop"
    let result = loop {
        counter += 1;
        println!("Counter: {counter}");
        if counter == iterations {
            break counter;
        }
    };

    println!("Successfully counted {result} times using 'loop'");
}

fn loop_n_times_while(iterations: i32) {
    let mut counter = 0;

    while counter < iterations {
        counter += 1;
        println!("Counter: {counter}");
    }

    println!("Successfully counted {counter} times using 'while'");
}

fn loop_n_times_for(iterations: i32) {
    for i in 1..=iterations {
        println!("Counter: {i}");
    }

    println!("Successfully counted {iterations} times using 'for'");
}
