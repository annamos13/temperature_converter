use std::io;
mod converter;

fn main() {
    println!("Input the temperature value to convert");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: i32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Wrong value");
            return;
        }
    };

    println!("Input the unit F for Fahrenheit, C for Celsius");
    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit = unit.trim().to_lowercase();

    let result = converter::temperature_converter(unit, value);
    println!("The converted result is: {result}");
}
