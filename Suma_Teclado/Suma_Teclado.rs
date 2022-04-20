
fn main(){
	let mut user = String::new();

	println!("Introduce un numero: ");
	let numero =  std::io::stdin().read_line(&mut user).unwrap();
	let num = from_str::<int>(&*numero);



	println!("Tu numero es el {}", num);
}