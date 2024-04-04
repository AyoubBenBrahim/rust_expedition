// Vectors - Resizable arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
  
  // Loop through vector values
  println!("============");

  for x in numbers.iter() {
    println!("nbr: {}", x);
  }

  println!("============");


  println!("Numbers Vec: {:?}", numbers);
}
