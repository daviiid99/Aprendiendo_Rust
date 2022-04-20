fn main(){
	// Declaracion de una array
	let array = [2, 4 ,6 ,8 ,10];

	// Almacenamos el primer valor de la array en una variable y lo imprimimos
	let primera_pos = array[0];

	println!("Primera posicion de array : {} " , primera_pos);

	// Vamos a imprimir cada una de las posiciones del array anterior
	for posicion in 0..5{
		println!("Estamos en la posicion : [{}] = {}", posicion, array[posicion]);
	}

}