extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("");
    println!("ADIVINHE O NÚMERO!");

    let nsecreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", nsecreto);
    
    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada. Deveria ser um número");
    
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&nsecreto) {
            Ordering::Less => println!("Muito Baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        
        }
        
    }

}
