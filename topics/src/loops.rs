// Loops - Used to iterate until a condition is met

pub fn run() {
  let mut count = 0;

  // Infinite Loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }

  // While Loop (FizzBuzz)
  // while count <= 7 {
    // println!("Number: {}", count);
  //   }

  //   // Inc
  //   count += 1;
  // }

  // For Range
  // for x in 0..15 {
  //   println!("Number: {}", x);
  // }

  let fruits = vec!["orange", "apple", "mango"];
    
    // Enumerate: An iterator that yields the current count and the element during iteration.
    for (index, i) in fruits.iter().enumerate(){
        println!("Fruit number {} is {}", index, i);
    }

}
