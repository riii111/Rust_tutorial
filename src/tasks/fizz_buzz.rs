pub fn run() {
    let number: i32 = 30;

    for i in 1..=number {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} : FizzBuzz", i),
            (0, _) => println!("{} : Fizz", i),
            (_, 0) => println!("{} : Buzz", i),
            _ => println!("{}", i),
        }
    }
}
