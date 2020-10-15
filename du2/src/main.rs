use std::io;

fn ask_user() -> String {
    let reader = io::stdin();
    let mut buffer: String = String::new();

    reader.read_line(&mut buffer)
        .ok()
        .expect("ERRMSG");

    println!("Janko Ukradol {}!", buffer);
    return buffer;
}

fn main() {
    
    let stolen_apples = ask_user();
    println!("Pocet Ukradnutych jablk: {}", stolen_apples);
   
    
}
