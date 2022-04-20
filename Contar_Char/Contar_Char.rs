fn count_chars(test: &str) -> i32 {
    let mut result = 0;
    let mut last_was_space = false;
    for c in test.chars() {
        // Only count whitespace chars that are not preceded by another whitespace.
        if c.is_whitespace() {
            if last_was_space == false {
                result += 1;
            }
            last_was_space = true;
        } else {
            result += 1;
            last_was_space = false;
        }
    }
    return result;
}

fn main(){
    println!("Tu contraseña tiene {} letras", count_chars("hola99") );
}