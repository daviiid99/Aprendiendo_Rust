fn main(){

	let mut i = 0;

	while i != -1{

		println!("Introduce un numero: ");
		fn get_input() -> String {
		 let mut _numero = String::new();
		 std::io::stdin().read_line(&mut _numero).expect("Failed");
		 _numero
		}

		let n = get_input().trim().parse::<i64>().unwrap();
		i = n;
		println!("El numero es {}", n );

	}
 }