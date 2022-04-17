fn preguntar_dni() -> String {
	let mut _numero = String::new(); 
	std::io::stdin().read_line(&mut _numero).expect("Failed");
	_numero
}

fn main(){
	let mut i = 0;
	let mut _dni = 0;
	let mut _user = String::new();

	while i != 99 {
		println!("Introduce los 8 digitos de tu DNI: \nEscribe '99' para salir:\n");
		let n = preguntar_dni().trim().parse::<i64>().unwrap(); 
		i = n;
		let resto_dni = i % 23;

		if resto_dni < 0 | resto_dni && 0 | resto_dni > 22{
			println!("Tu DNI no es válido, inténtalo de nuevo");
		}

		if resto_dni == 0 {
			println!("Tu dni es {}T", n);
		}

		if resto_dni == 1 {
			println!("Tu dni es {}R", n);
		}

		if resto_dni == 2 {
			println!("Tu dni es {}W", n);
		}

		if resto_dni == 3 {
			println!("Tu dni es {}A", n);
		}

		if resto_dni == 4 {
			println!("Tu dni es {}G", n);
		}

		if resto_dni == 5 {
			println!("Tu dni es {}M", n);
		}

		if resto_dni == 6 {
			println!("Tu dni es {}Y", n);
		}

		if resto_dni == 7 {
			println!("Tu dni es {}F", n);
		}

		if resto_dni == 8 {
			println!("Tu dni es {}P", n);
		}

		if resto_dni == 9 {
			println!("Tu dni es {}D", n);
		}

		if resto_dni == 10 {
			println!("Tu dni es {}X", n);
		}

		if resto_dni == 11 {
			println!("Tu dni es {}B", n);
		}

		if resto_dni == 12 {
			println!("Tu dni es {}N", n);
		}

		if resto_dni == 13 {
			println!("Tu dni es {}J", n);
		}

		if resto_dni == 14 {
			println!("Tu dni es {}Z", n);
		}

		if resto_dni == 15 {
			println!("Tu dni es {}S", n);
		}

		if resto_dni == 16 {
			println!("Tu dni es {}Q", n);
		}

		if resto_dni == 17 {
			println!("Tu dni es {}V", n);
		}

		if resto_dni == 18 {
			println!("Tu dni es {}H", n);
		}

		if resto_dni == 19 {
			println!("Tu dni es {}L", n);
		}

		if resto_dni == 20 {
			println!("Tu dni es {}C", n);
		}

		if resto_dni == 21 {
			println!("Tu dni es {}K", n);
		}

		if resto_dni == 22 {
			println!("Tu dni es {}E", n);
		}


}

}