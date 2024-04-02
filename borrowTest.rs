fn borrow_value(s: &String) {
    // s is a reference to a String
    println!("Borrowed string: {}", s);
}

fn main() {
    let my_string = String::from("Hello");
    borrow_value(&my_string); // Pass a reference to my_string without transferring ownership
    println!("{}", my_string); // Ownership of my_string is still retained
}
