fn main(){
    let s = String::from("Anjla");
    let first_a = find_first_a(s);
    match first_a {
        Some(index) => println!("The first 'a' is at index {}", index),
        None => println!("There is no 'a' in the string"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}