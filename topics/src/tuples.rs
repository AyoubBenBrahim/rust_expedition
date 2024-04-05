// Tuples group together values of different types
// Max 12 elements

// allow use of camelCase
#![allow(non_snake_case)]

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
  



  // String tuple
  let _sTuple = ("rice", "crab", "fish", "salad");
  // Integer Tuple
  let _iTuple = (1,2,3,4,5);
  // Mixed Tuple
  let mixed_Tuple = ("Smith", "Laura", true, 5.2, 100, 'x');
  println!("The Second name is {}, and the integer is {}, while the Boolean value is {} ", mixed_Tuple.1, mixed_Tuple.4, mixed_Tuple.2);

  // Nested Tuple
  let mixedTuple = ("Smith", "Laura", true, 5.2, 100, 'x', (10,20,30));
  println!("first value of nested tuple is {}, the second is {}, and the third is {}", (mixedTuple.6).0, (mixedTuple.6).1, (mixedTuple.6).2 );

}




