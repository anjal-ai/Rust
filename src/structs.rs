struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main(){
    let user1 = User {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };

    println!("First Name: {}", user1.first_name);
    println!("Last Name: {}", user1.last_name);
    println!("Age: {}", user1.age);
}