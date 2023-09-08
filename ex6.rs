fn main() {
    let mut x: i32 = 1;  // Declaração de uma variável mutável x e inicialização com o valor 1
    x = 7;               // Reatribuição do valor de x para 7 (graças a o mut)

    // Shadowing and re-binding
    let mut x = x;       // Shadowing: Declaração de uma nova variável mutável x que sombreia a variável anterior
    x += 3;              // Incremento do valor de x em 3

    let y: i32 = 4;      // Declaração de uma variável y imutável(pq não tem o mut) e inicialização com o valor 4

    // Shadowing
    let y: &str = "I can also be bound to text!";  // Shadowing: Declaração de uma nova variável y que sombreia a variável anterior

    println!("Success!");
}