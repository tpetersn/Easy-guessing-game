fn main() {
    let mut count = 0;

    let result = loop {

        count +=1;

        if count == 10 {
            break count * 2;
        }
    };

    print!("the result is: {result}");
}