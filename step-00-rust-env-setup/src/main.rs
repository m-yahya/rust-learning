use std::io;

fn main() {
    println!("Enter the conversion type (C or F):");
    let mut conversion_type = String::new();
    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Failed to read input");

    println!("Enter the temperature you want to convert:");
    let mut temperature_input = String::new();
    io::stdin()
        .read_line(&mut temperature_input)
        .expect("Failed to read input");
    let temperature: f32 = temperature_input
        .trim()
        .parse()
        .expect("Invalid temperature");

    if conversion_type == "c" {
        let celsius = fahrenheit_to_celsius(temperature);
        println!(
            "{} degrees Fahrenheit is equal to {} degrees Celsius",
            temperature, celsius
        );
    } else {
        let fahrenheit = celsius_to_fahrenheit(temperature);
        println!(
            "{} degrees Celsius is equal to {} degrees Fahrenheit",
            temperature, fahrenheit
        );
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
