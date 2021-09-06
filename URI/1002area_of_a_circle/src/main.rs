use std::io;

fn main() {
    println!("Starting Calcualte: ");
    let mut raio = String::new();

    io::stdin().read_line(&mut raio).expect("Failed to read from StdIN");

    let raio: f64 = raio.trim().parse().expect("Falha Sua!!");

    let area = 3.14159 * raio * raio;

    println!("A={:.4}", area);
}
