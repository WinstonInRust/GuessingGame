use rand::Rng;
use std::cmp::Ordering;
use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Welkom bij: Raad Het Getal Spel!!");

    loop {
        println!("Wat is het geheime getal? Voor het hier in...");
        let mut gok = String::new();
        stdin().read_line(&mut gok)?;

        let gok: u32 = match gok.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Verkeerd getal!");
                continue
            }
        };

        match gok.cmp(&secret_number) {
            Ordering::Less => println!("Te Laag!"),
            Ordering::Greater => println!("Te Hoog!"),
            Ordering::Equal => {
                println!("Je hebt het getal geraden! Je wint het spel...");
                break
            }
        }
    }
    Ok(())
}
