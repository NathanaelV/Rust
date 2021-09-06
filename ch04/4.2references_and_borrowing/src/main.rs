fn main() {
    let s1 = String::from("Texto");

    let tamanho = calcula_tamanho(&s1);
    // & é referência: permite usar um valor sem tomar posse, podereri usar 
    // esse valor novamente

    println!("\nO tamanho de '{}' é {}.", s1, tamanho);


    // Referências Mutáveis
    let mut s = String::from("texto 2");

    modifica(&mut s);  // Notação para modificar uma variável

    println!("\nReferência mutável: \n{}\n", s);

}


fn calcula_tamanho(s: &String) -> usize { // s é uma referência para uma String
    s.len()
} // Atui, s sai de escopo. Mas como ela não possui o valor a que se refere,
  // nada acontece.


fn modifica(uma_string: &mut String) {
    uma_string.push_str(" longo");
}
