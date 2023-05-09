fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        print!("number is divisible by 3");
    } else if number % 2 == 0 {
        print!("number is divisible by 2");
    } else {
        print!("number is not divisible by 4, 3 or 2");
    }
}