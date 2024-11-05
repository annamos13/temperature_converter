pub fn temperature_converter(unit: String, value: i32) -> i32 {
    match unit.as_str() {
        "f" => (value - 32) * 5 / 9,
        "c" => value * 9 / 5 + 32,
        _ => {
            panic!("Invalid unit provided");
        }
    }
}
