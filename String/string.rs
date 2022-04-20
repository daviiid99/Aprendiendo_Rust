fn main() {

    let char = "23";
    let char : i32 = char.trim().parse().unwrap();
    println!("{}", char + 1);

}