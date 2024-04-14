fn take_ownership(s: String) {
    // s is owned by this function
    println!("Received string: {}", s);
}

fn main()
{
let my_string = String::from("Hello");
take_ownership(my_string);//.clone()); // Ownership of my_string transferred to take_ownership
//  println!("{}", my_string); // Error! Ownership of my_string has been transferred
}