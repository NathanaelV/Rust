// 3.1 Variables

fn main() {
    let y = 8;      //Variaveis imutáveis não podem ser auteradas
    let mut x = 5;  //Variaveis mutáveis podem ser auteradas ao longo do código

    println!("The value of x is: {}", x);

    x = 6; //Isso só pode acontecer com uma variavel mutável

    println!("The value of x = {} and y = {}", x, y);

    const MAX_POINTS: u32 = 100000; //Constantes nunva vão sofrer alteração

    println!("O Valor máximo é: {}", MAX_POINTS);

    let y = y + 3;  //Shadowing: Sombrear uma variavel imutável com outro valor
    let y = y * 3;  //A troca de valor acontece por causa do comando let
                    //Usanodo o comando Shadowing é possível trocar
                    //O tipo da variavel, de String para numeral
    
    println!("Novo valor de Y é {}", y);

    

}
