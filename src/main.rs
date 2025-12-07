use rand;

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    let random_num: u32 = rand::random_range(1..=100);

    loop {
        let mut buff = String::new();

        io::stdin()
            .read_line(&mut buff)
            .expect("failed to read line");

        let guessed_num: u32 = buff.trim().parse().expect("expecting a number");

        match guessed_num.cmp(&random_num) {
            Ordering::Less => println!("it's less"),
            Ordering::Greater => println!("it's more"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
