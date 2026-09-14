use std::io;

fn tabuada(numero: i32){
    for i in 1..11{
        println!("{} x {} = {}", numero, i, numero*i);
    }
}

fn main() {
    let mut input = String::new();

    println!("Você quer a tabuada de que número?");

    io::stdin()
        .read_line(&mut input)
        .unwrap();
        
    let numero: i32 = input
        .trim()
        .parse()
        .unwrap();
    
    tabuada(numero);
}
