use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..101);

  println!("Guess the number!");

  loop {
    println!("Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // permite a entrada de dados

    let guess: u32 = match guess.trim().parse() { // continua o programa caso o user digite um texto
      Ok(num) => num,
      Err(_) => continue, // só igonra aquela entrada caso seja invalida
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { // compara os dois inteiros
      Ordering::Less => println!("Too small!\n"),
      Ordering::Greater => println!("Too big!\n"),
      Ordering::Equal => {
        println!("HAHA JONATHAN YOU WON\n");
        break;
      },
    }
  }
}
