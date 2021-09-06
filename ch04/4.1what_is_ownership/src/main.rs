fn main() {
    let mut s = String::from("Hello"); 

    s.push_str(", World!");  // push_str() adiciona um letaral à String s
                             // A variavel deve ser mutavel para funcionar
    
    println!("\n{}\n", s);  // Será exibido 'Hello, World!'

    let s1 = String::from("texto");  // Com esse comando não posso atribuir 
                                     // s1 a uma outra variavel e depois pedir
                                     // para exibir s1, ele será movido
    let s2 = s1;  // Valor foi movido de s1 para s2
                  // Para essa copia ser valida precisa usar uma função 
                  // s1.clone()

    println!("{}\n", s2);  // s1 não pode ser exibido aqui.

    let x = "Hello";
    let y = x; 

    println!("X = {}, Y = {}\n", x, y);

}  // Agora terminou o escopo, e 's' não é mais válida
