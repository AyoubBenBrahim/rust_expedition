fn modify_value(s: &mut String) {
    // s is a mutable reference to a String
    s.push_str(", world!");
}

fn main() {
    let mut my_string = String::from("Hello");
    modify_value(&mut my_string); // Pass a mutable reference to my_string
    println!("{}", my_string); // Ownership of my_string is still retained
}

