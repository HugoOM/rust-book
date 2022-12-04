use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a temperature in Fahrenheit  to convert to Celsius");

    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a string");
    let input: f32 = input.trim().parse().expect("Please enter a number");

    let input: f32 = (input - 32f32) * (5f32 / 9f32);

    println!("Temp in Celsius: {}", input);
}
