extern crate rand;
 
use std::io; // Acesso a biblioteca de Entrada e Saída. io (In/Out)
use std::cmp::Ordering;
use rand::Rng;  // Geração de um número aleatorio

fn main() {

    println!("");   
    println!("ADVINHE O NÚMERO!!!");
    println!("Você deve chutar um número de 1 a 100");

    let nsecreto = rand::thread_rng().gen_range(1,101);

    println!("O número secreto é: {}", nsecreto);

loop {

    println!("\nDigite o seu palpite: ");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    let palpite: u32 = match palpite.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Você disse: {}", palpite);

    match palpite.cmp(&nsecreto) { 
        Ordering::Less => println!("Mutito BAIXO!"),
        Ordering::Greater => println!("Mutito ALTO!"),
        Ordering::Equal => { println!("Você ACERTOU!"); break;}
       }

    }   

     println!("");

}
