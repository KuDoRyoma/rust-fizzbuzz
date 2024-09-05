fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    for x in 1..101 {
        // 1から100まで繰り返す
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
