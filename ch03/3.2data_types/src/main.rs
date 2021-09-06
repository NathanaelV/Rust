//3.2 Data Types

fn main() {
    println!("");
    
    
    let guess: u32 = "42".parse().expect("Não é um número!");
                     //parse() vai retornar o valor "42" no tipo
                     //Qua a variável guess está pedindo, u32
                     //parse() está vonvertendo uma String em u32
                     //Tradução de parse é 'Analisar Palavras'
    let x: u32 = 5;  //O u32 é padrão para numeros inteiros sem sinal

    println!("Guess = {} e x = {}", guess, x);

    let y: f64 = 1.5;  //Para números decimais, usa-se o f64 como padrão
                       //processa tão bem quanto o f32 e tem maior precisão

    println!("Y = {}", y);
    
    //Operações matemáticas
    //Só é permitido fazer operações com os tipos primitivos
    //u8 com u8, i32 com i32, f64 com f64 
    //Não é possível fazer i32 com u34 nem u32 com u64
    
    let soma = 4 + 5;           //Adição: +
    let diferenca = 9.6 - 14.3; //Subtração: -
    let produto = 4 * 5;        //Multiplicação: *
    let quociente = 43.2 / 5.3; //Divisão: /
    let resto = 43 % 5;         //Resto: %

    // Para as operações é permitido usar núros decimais sem usar o f64
    //Mas um número com virgula só fazer operação com outro número com virgula
    //Ex.: 4.0 + 3.2, (4 + 3.2 ñ funciona)
    println!("4 + 5 = {}", soma);
    println!("29.6 - 14.3 = {}", diferenca);
    println!("4 * 5 = {}", produto);
    println!("43.2 / 5.3 = {}", quociente);
    println!("Resto de 43 / 5: {}", resto);
    
    //Booleano 
     let t = true; //Forma compacta
     let f: bool = false; //Forma explícita
     println!("Verdadeiro: {} e Falso: {}", t, f);
    
    //char: letras
    let c = '😻';   //Para usar o char, deve-se usar aspas simples ''
    let a = 'Z';    //Pode ser usardo para emojis também
    println!("{} e {}", c, a);

       
    //Tipos Compostos
    //Tuplas / tuplaero
    //
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //Desestruturação: Quebra a tupla em 3 partes

    println!("O X = {}, Y = {}, Z = {}", x, y, z);

    let t: (i32, f64, u8) = (500, 6.4, 1);
    let quinhentos = t.0;   //0 indica a primeira posição da tupla
    let seis = t.1;         //1 indica a segunda posição da tupla
    let um = t.2;           //2 indica a segunda posição da tupla

    println!("Quinhentos = {}, Seis = {}, Um = {}", 
        quinhentos, seis, um);

    let j: (&str, u32, f64) = ("Hello", 46, 7.5);
    let (a, b, c) = j;
    
    println!("&str = {}, u32 = {}, f64 = {}", a, b, c);

 
        
    //Matriz
    //Na Matriz os valores devem ser do mesmo tipo
    //Tem tamanho fixo, não pode aumentar ou diminuir
    //Pode usado para saber os meses do ano, pois não terá auteração
    
    let matriz = [1, 2, 3, 4, 5];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio"];
    let primeiro = matriz[0];   //matriz começa a contar no 0
    let segundo = meses[1];     //Nomeda matriz[posição dentro]
    let indice = 3;
    let escolha = meses[indice];


    println!("1º número = {} e o 2º mês é {}", primeiro, segundo);
    println!("Escolha = {}", escolha);
    
     
    println!("");
}
