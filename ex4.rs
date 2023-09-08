fn main() {
    define_x();
}

fn define_x() {
    let x: &str = "hello";// Declaração da variável x como uma referência a uma string literal "hello"
    
    println!("{}, world", x);// Imprime a string concatenada com ", world"
}