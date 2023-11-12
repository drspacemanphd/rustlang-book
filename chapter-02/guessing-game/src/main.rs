use std::io as input_output;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess now:");

    let val = rand::thread_rng().gen_range(1..=100);

    println!("The number is: {val}");

    loop {
      let mut guess = String::new();

      input_output::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
  
      println!("You guessed: {guess}");
  
      let guess: u16 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Please input an integer");
          continue;
        },
      };
  
      match guess.cmp(&val) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
    }
}
