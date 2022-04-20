fn preguntar_dni() -> String {

	let mut _numero = String::new();  // Aqui es donde vamos a almacenar los numeros del DNI
	println!("Introduce los 8 digitos de tu DNI: \nEscribe '99' para salir:\n");
	std::io::stdin().read_line(&mut _numero).expect("Failed"); // El usuario introduce el DNI
	_numero

}

fn resto_dni(dni:i64)-> String{
	let resto = dni % 23;
	resto.to_string()

}

fn determinar_letra_dni(resto_dni:i64, i:i64, n:i64){
	let terminacion = ["T","R","W","A","G","M","Y","F","P","D","X","B","N","J","Z","S","Q","V","H","L","C","K","E"];
	let posicion = terminacion[resto_dni];

		if resto_dni == 0 {
			println!("Tu dni es {}T\n", n);
		}

		else if resto_dni == 1 {
			println!("Tu dni es {}R\n", n);
		}

		else if resto_dni == 2 {
			println!("Tu dni es {}W\n", n);
		}

		else if resto_dni == 3 {
			println!("Tu dni es {}A\n", n);
		}

		else if resto_dni == 4 {
			println!("Tu dni es {}G\n", n);
		}

		else if resto_dni == 5 {
			println!("Tu dni es {}M\n", n);
		}

		else if resto_dni == 6 {
			println!("Tu dni es {}Y\n", n);
		}

		else if resto_dni == 7 && i != 99 {
			println!("Tu dni es {}F\n", n);
		}

		else if resto_dni == 8 {
			println!("Tu dni es {}P\n", n);
		}

		else if resto_dni == 9 {
			println!("Tu dni es {}D\n", n);
		}

		else if resto_dni == 10 {
			println!("Tu dni es {}X\n", n);
		}

		else if resto_dni == 11 {
			println!("Tu dni es {}B\n", n);
		}

		else if resto_dni == 12 {
			println!("Tu dni es {}N\n", n);
		}

		else if resto_dni == 13 {
			println!("Tu dni es {}J\n", n);
		}

		else if resto_dni == 14 {
			println!("Tu dni es {}Z\n", n);
		}

		else if resto_dni == 15 {
			println!("Tu dni es {}S\n", n);
		}

		else if resto_dni == 16 {
			println!("Tu dni es {}{}\n", n, posicion);
		}

		else if resto_dni == 17 {
			println!("Tu dni es {}V\n", n);
		}

		else if resto_dni == 18 {
			println!("Tu dni es {}H\n", n);
		}

		else if resto_dni == 19 {
			println!("Tu dni es {}L\n", n);
		}

		else if resto_dni == 20 {
			println!("Tu dni es {}C\n", n);
		}

		else if resto_dni == 21 {
			println!("Tu dni es {}K\n", n);
		}

		else if resto_dni == 22 {
			println!("Tu dni es {}E\n", n);
		}


}

fn main(){

	let mut i = 0;

	while i != 99 {
		let preguntar_dni = preguntar_dni().trim().parse::<i64>().unwrap(); // Llamamos a la funcion que pregunte el DNI
		i = preguntar_dni;
		let resto_dni = resto_dni(i).trim().parse::<i64>().unwrap() ; // El calculo de la letra del DNI se determina por el resto de dividir el numero entre 23
		let _letra_dni = determinar_letra_dni(resto_dni, i, preguntar_dni); // Aqui determinamos la letra de nuestro DNI y la imprimimos

		if i == 99{
			println!("Operacion abortada por el usuario"); // Imprimir en caso de que el usuario detenga la ejecucion
		}

	}

}