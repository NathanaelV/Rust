// 3.5 Control Flow Loop

//Exercício de loop usando if else com condição
//O indicado é usar o While
fn main() {
    println!("");

    // Usando if e else

    let mut x = 1;

    loop {
        if x < 6 {
            x = x + 1;
            println!("X = {}", x);
            println!("Novamente!");
    } else {
        println!("Acabou o Loop.");
        break;}
                   
    }
    let atribui = if x > 5 {  //é possível fazer uma atribuição usando o if
        "9"                   //mas devem ser do mesmo tipo
    } else {
        "X não é maior que 7"
    };

    println!("{}", atribui);

    // Usando while 
    println!("");
    println!("Usando o While:");

    let mut numero = 4;

    while numero != 0 {
        println!("{}!", numero);

        numero = numero - 1;
    }
    
    println!("LIFTOFF!!!");

    //While usado em uma Matriz
    let a = [10, 20, 30, 40, 50, 60, 120];  //A matriz começa com indice 0
    let mut i = 0;

    while i <= 5 {
        println!("O valor é: {}.", a[i]);

        i = i + 1;
    }

    //Outros jeito de execultar o código anterior, com mais velocidade
    //Sem o risco do indice ser maior que o número da matriz
    println!("");
    println!("Usando o for:");
    let a = [10, 20, 30, 40, 50];

    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }

    //Outro jeito de fazer a contagem regressiva usando for
    for numero in (1..4).rev() {
        println!("{}!", numero);
    }
    println!("LIFTOFF!!!");

    println!("");
    
}
