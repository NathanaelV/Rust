fn main() {
    
    println!("");
    
    let numero = 8;

    if numero < 5 {  //Se não ouver um condição que permita o if fazer um
                     //Teste de verdadeiro ou falso. Dará erro 
                     //Teste como <, >, =, >=, ...
        println!("Nº é maior que 5.");
    } else {
        println!("Condição era Falsa.")
    }

    if numero != 0 {  // != siginica não igual
        println!("Número é diferente de zero")
    }
    
    let x = 11;
    
    if x %4 == 0 {
        println!("Número divisivel por 4.");
    } else if x % 3 == 0 {
        println!("Número divisivel por 3.");
    } else if x % 2 == 0 {
        println!("Número divisivel por 2.");
    } else {
        println!("Número não é divisivel por 4, 3 ou 2.");
    }
    
    let condicao = false;       //Os valores retornardos por if e else 
                                //Devem ser compativeis, ter o mesmo tipo primitivo
                                // todos i32 ou todos f64 ou todos Str                            
    let numero = if condicao {
        5
    } else {
        6
    };

    println!("O valor do númeor é: {}", numero);
    
    println!("");

}
