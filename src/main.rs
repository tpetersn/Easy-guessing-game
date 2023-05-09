fn main() {
    for number in (1..4).rev() {
        //(1..4) is called 'Range" and is provided by the standard library
        //range generates all numbers in the sequence from one number and ending before another number
        // .rev() reverses the range, counting from 3->2->1
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}