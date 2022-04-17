fn main(){

	
	let mut i = 0; // Declaramos variable mutable

	
	while i != -1{ // Condicion para seguir pidiendo por teclado

		println!("Introduce un numero: ");
		fn get_input() -> String { // Metodo que recoge el String 
		 let mut _numero = String::new(); // Variable donde almacenamos el String 
		 std::io::stdin().read_line(&mut _numero).expect("Failed"); // Pedimos por teclado un numero 
		 _numero
		}

		let n = get_input().trim().parse::<i64>().unwrap(); // Tranformamos el String pedido por teclado en un numero
		i = n; // Igualamos i al numero introducido por teclado // Si es -1 acaba el bucle
		println!("El numero es {}", n ); // Imprime el numero intoroducido, ahora como integer

	}
 }