use std::io;

fn main() {
    println!("Enter 1 to convert from F to C. Enter 2 to convert from C to F");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read input");
    let user_input: u32 = user_input.trim().parse().expect("enter a number");





    if user_input == 1{
        println!("Enter a temperature in F to convert to C");
    
        let mut inputf = String::new();
        io::stdin().read_line(&mut inputf).expect("failed to read line");

        let inputf = inputf.trim().parse::<f32>().unwrap();
        let converted_inputf: f32 = (inputf - 32.0) * 5.0/9.0;
        println!("{inputf} degrees fahrenheit is {converted_inputf} in degrees celsius");
    } else {
        println!("Enter a temperature in C to convert to F");
    
        let mut inputc = String::new();
        io::stdin().read_line(&mut inputc).expect("failed to read line");

        let inputc = inputc.trim().parse::<f32>().unwrap();
        let converted_inputc: f32 = (inputc * 1.8) + 32.0;
        println!("{inputc} degrees celsius is {converted_inputc} in degrees fahrenheit");


    }
}