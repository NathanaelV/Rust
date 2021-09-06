//Chapter 3 Challenges

use std::io;

fn resgate () -> String {
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("Falha!!!");  //le o valor para dentro do e
    e 
}

fn main() {
    
    loop {
        println!("");
        println!("Qual operação deseja realizar?");
        println!("  1 - Conver ºC em ºF");
        println!("  2 - Ver um n-ésimo número de Fibonacci");
        println!("  3 - The Twelve Days of Christmas");
        println!("  4 - SAIR!");
        println!("Digite um númeor de 1 a 4:");
        println!("");

        //Copia do jodo da adivinhação.
        //Alocar/separar um espaço para a String
        
        let captura = resgate();
        
        let e: u32 = match captura.trim().parse() {
            Ok(num) => num,
            Err(_) => {                                     //Como imprimir uma mensagem aqui??
                println!("Por favor digite um número.");
                0  //retorna um inteiro para a variavel e
            },  
        };
        
        if e == 1 {
            celsius();  //c * 1.8 + 32 = f
        } else if e == 2 {
            fibonacci();  //1, 1, 2, 3, 5, 8 ... Fn = Fn-1 + Fn-2
        } else if e == 3 {
            the_twelve_days_of_christmas();
        } else if e == 4 {  //SAIR DO MENU
            {println!("Você saiu!");break;}
        } else {
            println!("ESCOLHA INVALIDA.");
            println!("Escolha a opção 1, 2, 3 ou 4!");
        }
        
    }
    
    println!("");

}
    
    //Calculo de Graus Celsius para Graus Fahrenheit
fn celsius() {
    
    loop {
        println!("Digite a temperatura em ºC:");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Falha!!!");
        let c: f64 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let f:f64 = c * 1.8 + 32.0; //Quando trabalhar com float, todos os números 
                                    //Devem ser float

        println!("{} ºC é igual a {} ºF. ", c, f);
        println!("");
        println!("Deseja fazer outra conversão de ºC para ºF?");
        println!("  1 - Sim");
        println!("  2 - Não");
                    
        let mut vf = String::new();
        io::stdin().read_line(&mut vf).expect("Falha!!!");
        let vf: u32 = match vf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if vf != 1 {
            {break;}
        } else {}    
        
    }
        
}
    
    //N-ésimo número de Fibonacci
fn fibonacci (){
    loop {
        println!("");
        println!("Fibonacci.");
        
        let mut i = 3;
        let mut f0: u128 = 2;  //Permite um n <= 186
        let mut f1 = 1;
        let mut f2 = 1;
        
        println!("Digite o n-ésimo Nº: ");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Falha!!!");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n > 186 {
            println!("Computador não tem memoria para esse calculo.");
            println!("Por favor digite um nº menor que 187.");
            //let f0: String = f0.trim().parse().expect("Ruim!");
        } else if n > 2 {while i <= n {
            
    /*          
            let mut f0: u128 = 2;  //Permite um n <= 186
            let mut f1 = 1;
            let mut f2 = 1;
    */
            f0 = f1 + f2;
            f2 = f1;
            f1 = f0;
            i = i + 1;
            
        } } else if n > 0 {
            println!("  F0 = 1");
        } else if n == 0 {
            println!("  F0 = 0");
        } else {
            println!("Valor não existe");
        }

        println!("  F0 = {}.", f0);

        println!("");
        println!("Deseja saber outro número da sequencia de Fibonacci?");
        println!("  1 - Sim");
        println!("  2 - Não");
        
        let mut vf = String::new();
        io::stdin().read_line(&mut vf).expect("Falha!!!");
        let vf: u32 = vf.trim().parse().expect("Falha!");

            if vf != 1 {
                {break;}
            } else {}   
        }
     

}
    
    //The Twelve Days of Christmas
fn the_twelve_days_of_christmas() {
    let mut a = 1;

    //let cancao = [10];

    while a < 13 {
        println!("On the {}º day of Christmas,", a);
        println!("My true love sent to me");
        println!("");

        a = a + 1;

    }
    
    println!("Fim da Canção!");
}
