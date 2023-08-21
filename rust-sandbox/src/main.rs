use rand::Rng;
use std::cmp;
use std::io;
mod functions;
fn main() {
    // println!("Guess the number!");

    // let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number is {secret_number}");
    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("Your guess: {guess}");

    //     match guess.cmp(&secret_number) {
    //         cmp::Ordering::Less => println!("Too Less"),
    //         cmp::Ordering::Greater => println!("Too Big"),
    //         cmp::Ordering::Equal => {
    //             println!("You Win!");
    //             break;
    //         }
    //     }
    // }
    // let tuple_Sample: (f64, i32, &str) = (4.90, 5, "sample");
    // let x: f64 = tuple_Sample.0;
    // let y: i32 = tuple_Sample.1;
    // let z: &str = tuple_Sample.2;

    // println!("value of tuple: {x}, {y}, {z}");

    // let x = [3, 2, 7, 0];
    // let mut index: String = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("please input number");

    // let index: usize = match index.trim().parse() {
    //     Ok(idx) => idx,
    //     Err(_) => 0,
    // };

    // let result = x[index];
    // println!("value at index {index}: {result}");

    let string_sample = String::from("sample");
    functions::another_function(5, &string_sample);
}
