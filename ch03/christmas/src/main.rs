fn main() {
    
    println!("\nTHE TWELVE DAYS OF CHRISTMAS\n");

    let td = [ "", "", "", "", "", "", "", "", "", "", "", 
        "\nAnd a partridge in a pear tree", "\nTwo turtles doves", 
        "\nThree French hens", "\nFour calling birds", 
        "\nFive golden rings","\nSix geese a laying",
        "\nSeven swans a swimming","\nEight maids a milking", 
        "\nNine ladies dancing", "\nTen lords a leaping",
        "\nEleven pipers piping", "\nTwelve drummers drumming",];

    let mut i = 0;

    if i == 0 {

        println!("In my {}º day of Christmas", (i+1));
        println!("My true love send to me");
        println!("A partridge in a pear tree\n");
        i = i + 1

    } else {}

        while i < 12 {
            println!("In my {}º day of Christmas", (i+1));
            print!("My true love send to me");
            print!("{}", td[i+11]);
            print!("{}", td[i+10]);
            print!("{}", td[i+9]);
            print!("{}", td[i+8]);
            print!("{}", td[i+7]);
            print!("{}", td[i+6]);
            print!("{}", td[i+5]);
            print!("{}", td[i+4]);
            print!("{}", td[i+3]);
            print!("{}", td[i+2]);
            print!("{}", td[i+1]);
            println!("{}", td[i]);
            println!("");
            i = i + 1

        }
    
}
