fn main(){
	// Inicializamos un contador a 0
	// Por defecto es inmutable, por tanto, para cambiar su valor debemos añadirle el atributo 'mut'
	let mut suma = 0;

	// Sumanos todos los numeros pares del 1 al 10
	for numero in 1..10+1{
		if numero % 2 == 0 {
			suma +=numero;
		}
		}

		// Imprimimos el resultado de la suma
		println!("El resultado de la suma es {}", suma );
	}