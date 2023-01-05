// Input output library
use std::io;

fn main() {
    println!("Temperature Converter");
    println!(
        "Please enter 1 or 2 to Convert \n
        1. Fahrenheit to Celsius, \n
        2. Celsius to Fahrenheit"
    );

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    if input == 1 {
        println!("Enter Fahrenheit");
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number!");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
    } else if input == 2 {
        println!("Enter Celsius");
        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        let celsius: f32 = celsius.trim().parse().expect("Please type a number!");
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);
    } else {
        println!("Please enter 1 or 2");
    }
}
