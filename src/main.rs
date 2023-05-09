use std::io;

fn main() {
    println!("Enter a temperature in F to convert to C");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let input = input.trim().parse::<f32>().unwrap();
    let converted_input: f32 = (input - 32.0) * 5.0/9.0;
    println!("{input} degrees fahrenheit is {converted_input} in degrees celsius");
    
}