fn ask_password() -> String{
    println!("Establece una contraseña de 8 caracteres:\n");
    let mut password = String::new();
    let password = std::io::stdin().read_line(&mut password).expect("error");
    password.to_string()
}

fn contar_letras(password:String) -> String{
    let mut contador = 0;
    while password.len() != contador {
        contador+=1;
    }
    contador.to_string()
}

fn set_password() -> String{
    let mut preguntar = String::new();
    let _longitud = contar_letras(preguntar.to_string());
    let mut l = 0;
    while l !=8 {
        let longitud = contar_letras(preguntar.to_string());
        l = longitud.trim().parse::<i64>().unwrap();
        println!("{}", l );
        preguntar = ask_password();
    }
    preguntar.to_string()
    
}

fn main() {
    let password = set_password();
    println!("Se ha establecido la contraseña correctamente {}", password)
        
        
    }

