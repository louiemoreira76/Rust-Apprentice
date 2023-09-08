fn main() {
    let (mut x, y) = (1, 2);    // Declaração das variáveis x e y usando a sintaxe de desestruturação
    x += 2;                     // Incremento do valor de x em 2

    assert_eq!(x, 3);           // Verifica se o valor de x é igual a 3 usando a macro assert_eq!
    assert_eq!(y, 2);           // Verifica se o valor de y é igual a 2 usando a macro assert_eq!

    println!("Success!");      // Imprime a mensagem "Success!"
}