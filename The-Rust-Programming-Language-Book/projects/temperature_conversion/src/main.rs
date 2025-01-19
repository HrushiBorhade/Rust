// write a program that first takes user input to determine whether they want to convert from Fahrenheit to Celsius or vice versa
// and then asks for the temperature value to convert.

use std::io;

fn main() {
    let mut conversion_standard = String::new();
    println!("Welcome to the temperature conversion program!");
    loop {
        println!("Please enter 0 to convert from Fahrenheit to Celsius or 1 to convert from Celsius to Fahrenheit: ");
        io::stdin()
            .read_line(&mut conversion_standard)
            .expect("Failed to read line");

        let conversion_standard: i8 = conversion_standard
            .trim()
            .parse()
            .expect("Please enter a number");

        if conversion_standard == 0 || conversion_standard == 1 {
            let mut temperature = String::new();
            println!("Please enter the temperature value to convert: ");
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f64 = temperature.trim().parse().expect("Please enter a number");

            if conversion_standard == 0 {
                let celsius = (temperature - 32.0) * 5.0 / 9.0;
                println!(
                    "(Your input) The temperature in Fahrenheit is: {}",
                    temperature
                );
                println!("The temperature in Celsius is: {}", celsius);
            } else {
                let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
                println!(
                    "(Your input) The temperature in Celsius is: {}",
                    temperature
                );
                println!("The temperature in Fahrenheit is: {}", fahrenheit);
            }
            break;
        } else {
            println!("Please enter either 0 or 1");
        }
    }

    println!("Thank you for using the temperature conversion program!");
}
