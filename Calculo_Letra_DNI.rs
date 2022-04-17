fn preguntar_dni() -> String {

	let mut _numero = String::new();  // Aqui es donde vamos a almacenar los numeros del DNI
	println!("Introduce los 8 digitos de tu DNI: \nEscribe '99' para salir:\n");
	std::io::stdin().read_line(&mut _numero).expect("Failed"); // El usuario introduce el DNI
	_numero

}

fn main(){

	let mut i = 0;

	while i != 99 {
		let n = preguntar_dni().trim().parse::<i64>().unwrap(); // Llamamos a la funcion que pregunte el DNI
		i = n;
		let resto_dni = i % 23; // El calculo de la letra del DNI se determina por el resto de dividir el numero entre 23

		if i == 99{
			println!("Operacion abortada por el usuario");
		}

		if resto_dni == 0 {
			println!("Tu dni es {}T\n", n);
		}

		if resto_dni == 1 {
			println!("Tu dni es {}R\n", n);
		}

		if resto_dni == 2 {
			println!("Tu dni es {}W\n", n);
		}

		if resto_dni == 3 {
			println!("Tu dni es {}A\n", n);
		}

		if resto_dni == 4 {
			println!("Tu dni es {}G\n", n);
		}

		if resto_dni == 5 {
			println!("Tu dni es {}M\n", n);
		}

		if resto_dni == 6 {
			println!("Tu dni es {}Y\n", n);
		}

		if resto_dni == 7 && i != 99 {
			println!("Tu dni es {}F\n", n);
		}

		if resto_dni == 8 {
			println!("Tu dni es {}P\n", n);
		}

		if resto_dni == 9 {
			println!("Tu dni es {}D\n", n);
		}

		if resto_dni == 10 {
			println!("Tu dni es {}X\n", n);
		}

		if resto_dni == 11 {
			println!("Tu dni es {}B\n", n);
		}

		if resto_dni == 12 {
			println!("Tu dni es {}N\n", n);
		}

		if resto_dni == 13 {
			println!("Tu dni es {}J\n", n);
		}

		if resto_dni == 14 {
			println!("Tu dni es {}Z\n", n);
		}

		if resto_dni == 15 {
			println!("Tu dni es {}S\n", n);
		}

		if resto_dni == 16 {
			println!("Tu dni es {}Q\n", n);
		}

		if resto_dni == 17 {
			println!("Tu dni es {}V\n", n);
		}

		if resto_dni == 18 {
			println!("Tu dni es {}H\n", n);
		}

		if resto_dni == 19 {
			println!("Tu dni es {}L\n", n);
		}

		if resto_dni == 20 {
			println!("Tu dni es {}C\n", n);
		}

		if resto_dni == 21 {
			println!("Tu dni es {}K\n", n);
		}

		if resto_dni == 22 {
			println!("Tu dni es {}E\n", n);
		}


}

}