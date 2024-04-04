// Tuples group together values of different types
// Max 12 elements

fn print_person(person: (&str, &str, i32)) {
  println!("Name: {} {}, Age: {}", person.0, person.1, person.2);
}

fn get_point() -> (i32, i32) {
  (3, 5)
}

pub fn run() {
  let person: (&str, &str, i8) = ("Brad", "Poo", 37);
  
  println!("{} is from {} and is {}", person.0, person.1, person.2);
  
  let person = ("John", "Doe", 30);
  print_person(person);
  
  // Destructuring
  let (name, age) = ("Alice", 25);
  println!("Name: {}, Age: {}", name, age);

  // return multiple values
  let (x, y) = get_point();
  println!("x = {}, y = {}", x, y);
  
}




