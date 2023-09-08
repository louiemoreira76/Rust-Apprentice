fn main() {
    let x: i32 = 5;      // Declaração da variável x e inicialização com o valor 5

    {
        let x = 12;      // Declaração de uma nova variável x no escopo interno e inicialização com o valor 12
        assert_eq!(x, 12);  // Verifica se o valor de x é igual a 12

    }

    assert_eq!(x, 5);    // Verifica se o valor de x no escopo externo é igual a 5

    let x = 42;         // Declaração de uma nova variável x no escopo externo e inicialização com o valor 42
    println!("{}", x);  // Imprime o valor de x, que será 42
}