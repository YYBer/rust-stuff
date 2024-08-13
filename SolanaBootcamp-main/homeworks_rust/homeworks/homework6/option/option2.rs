// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// Have problem!!!!

fn main() {
    let optional_word = Some(String::from("rustlings"));
    let word = Some(String::from("I like rustings"));
    if word == optional_word {
        println!("The word is: {}", word.unwrap());//did not work
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(integer)) = optional_integers_vec.pop(){
    println!("current value: {}", integer);
    }
}
