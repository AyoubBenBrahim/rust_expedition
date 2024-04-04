// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [0, 1, 2, 3, 4];

  // Re-assign value
  numbers[4] = 40;

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..4]; // 0..N is exclusive of the last index (0 to N-1)
  println!("Slice: {:?}", slice);

  println!("======rev======");
  for x in numbers.iter().rev() {
    println!("Number: {}", x);
  }

  println!("=====mut=======");
  for x in numbers.iter_mut() {
    *x *= 2;
  }

   for x in numbers.iter() {
    println!("Number: {}", x);
  }

  println!("============");

  // iter from 2 to 4
  for x in numbers[1..4].iter() {
    println!("Number: {}", x);
  }

}
