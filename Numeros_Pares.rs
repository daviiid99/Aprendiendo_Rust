fn main(){
	for numero in 1..24+1{
	    // Si numero es par lo imprimirá
		if numero % 2 == 0{
			println!("Numero Par : {}",  numero);
		}
		else {
		    // Si no es par, dirá que es impar
		    println!("Numero Impar : {}", numero);
		}
	}
}