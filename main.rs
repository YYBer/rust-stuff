fn fizzbuzz() {
    for i in 0..=301 {
        if i % 15 == 0 {
            println!("fizz buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}
fn main() {
    println!("Hello, world!");
    fizzbuzz();
}

