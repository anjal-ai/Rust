use std::fs::read_to_string;

fn main(){
    let result = read_to_string("./08_result.rs");

    match result {
        Ok(data) => println!("File data: {}", data),
        Err(error) => println!("Error: {}", error),
    } 
}
