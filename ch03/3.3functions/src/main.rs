// 3.3 Functions

fn mais_uma_f() {  //A função pode ser construida antes da função main
                   //Para o Rust não faz diferença onde foi definida 

    println!("Essa função vem antes da main");

}

fn main() { //A chave indica onde começa e termina o corpo da função
    
    println!("Hello, world!");

    outra_funcao(-5, 7);  //É possivel criar uma função fora da fn main 
                          //e chamá-la Depois
                    
    mais_uma_f();   //A ordem é de acordo com a posição dentro
                    //da fn main
    
    let (z, a, b) = (8, 2, 4);        //É uma DECLARAÇÃO, execulta uma ação
    println!("{}, {}, {}", z, a, b);  //E não retorna valor
    
    //let x = (let y = 6); não se atribui um let a outro
    
    let x = 5;
    let y = {
        let x = 3;  //Por que se eu tirar o x + 1 o código da erro?
        x + 1       //Por não ter um ponto e virgula(;), é uma Expressão
                    //Caso fosse com ponto e virgula, daria erro

    };
    
    println!("Y = {}", y);
    println!("X = {}", x);
    println!("Z = {}", z);
    
}

fn outra_funcao(x: i32, y: u32) { //Essa função tem um paramentro X

    println!("Outra função. X = {} e Y = {}.", x, y);
    
}