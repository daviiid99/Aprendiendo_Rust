fn main() {

    // Declaramos variable 
    let suma =  4;
    
    // Imprimimos la variable
    println!("2 + 2 = {}", suma);

    let multiplicacion = 2 * 2;

    // Se debe cumplir que 2 variables sean iguales, sino crasheo
    assert_eq!(suma, multiplicacion );
}