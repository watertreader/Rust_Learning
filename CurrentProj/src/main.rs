#![allow(non_snake_case)]
fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed already!");
}

fn celsius_to_fahrenheit(degree : f64) -> f64
{
    let factor  = 1.8;
    let bias    = 32.0;

    let faren = degree * factor + bias;

    return faren;
}