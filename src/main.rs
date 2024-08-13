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


// fn interproduct(a:i32, b:i32, c:i32)->i32{
//     return a*b*c;
// }
// fn main(){
//     println!("result: {}", interproduct(10, 2, 0));
// }

// fn takes_u32(x: u32){
//     println!("u32: {x}");
// }

// fn takes_i8(y: i8){
//     println!("i8: {y}");
// }

// fn main(){
//     let x = 10;
//     let y = 20;
//     takes_u32(x);
//     takes_i8(y);
// }