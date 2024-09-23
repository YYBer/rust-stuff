// fn fizzbuzz() {
//     for i in 0..=301 {
//         if i % 15 == 0 {
//             println!("fizz buzz");
//         } else if i % 3 == 0 {
//             println!("fizz");
//         } else if i % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }
// fn main() {
//     println!("Hello, world!");
//     fizzbuzz();
// }


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

// fn main() {
//     let optional_word = Some(String::from("rustlings"));
//     // TODO: Make this an if let statement whose value is "Some" type
//     if let Some(word) = optional_word {
//         println!("The word is: {}", word);
//     } else {
//         println!("The optional word doesn't contain anything");
//     }

//     let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
//     for x in 1..10 {
//         optional_integers_vec.push(Some(x));
//     }

//     // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
//     // You can stack `Option<T>`'s into while let and if let
//     while let Some(integer) = optional_integers_vec.pop() {
//         println!("current value: {}", integer.unwrap());
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("abec");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the result is {}", result);
}   

