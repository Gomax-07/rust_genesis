// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "John";
  let mut age = 24;
  println!("My name is {} and I am {}", name, age);
  age = 28;
  println!("My name is {} and I am {}", name, age);


  //Define constant
  const ID: i32 = 001;
  println!("ID:{}", ID);



  //Assign multiple variables
  let( my_name, my_age) =  ("John", 24);
  println!("{} is {}", my_name, my_age);

}
