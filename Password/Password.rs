fn ask_password() ->  String {
    println!("Establece una contraseña de 8 caracteres:\n");
    let mut password = String::new();
    let password = std::io::stdin().read_line(&mut password).expect("error");
    return password.to_string()
}

fn count_password(password: &str) -> i32{
    let mut contador = 0;
    for letra in password.chars(){
        if letra.is_whitespace(){
            contador = contador;
        } else {
            contador+=1;
        }
    }
    return contador;
}

fn main() {
    let password = ask_password();
    println!("Se ha establecido la contraseña correctamente, tiene {} letras", count_password(&password));
        
        
    }

