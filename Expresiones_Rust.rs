fn main(){
	for numero in 1..10+1{
		let es_par = if numero % 2 == 0 {"Es Par"} else {"Es Impar"};
		println!("{} {}", es_par, numero );
	}
}