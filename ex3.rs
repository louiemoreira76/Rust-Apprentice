fn main() {
    let x: i32 = 10;                   // Declaração da variável x e inicialização com o valor 10
    let y: i32 = 5;                    // Declaração da variável y e inicialização com o valor 5

    {
        let soma: i32 = x + y;         // Declaração da variável soma e atribuição da soma de x e y
        println!("The value of x is {} and value of y is {}", x, y);  // Imprime os valores de x e y
        println!("{soma}");            // Imprime o valor da variável soma
    }

    println!("The value of x is {} and value of y is {}", x, y);  // Imprime novamente os valores de x e y
	}
//x + y só é valido dentro do bloco (scope) onde esta sendo executado
//por isso se quiser que resultado final seja esse tire do alcance do scope