fn main(){
	for numero in 1..10+1{
		let numero_par = if numero % 2 == 0 {"Es Par"} else {"Es Impar"};
		println!("{} {}", numero_par, numero );
	}
}