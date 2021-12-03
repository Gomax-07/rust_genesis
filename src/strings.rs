//  Primitive str = Immutable fixed-length string somewhere in memory
//  String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run  () {
  let hello1 = "Hello people";
  let mut hello = String::from("Hello ");

//  Get length
  println!("length: {}", hello1.len());
  println!("length: {}", hello.len());

  //  Push char
  hello.push('W');

  //  Push string
  hello.push_str("orld!");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  //  Check if empty
  println!{"is_Empty: {}", hello.is_empty()};

  //  Contains
  println!("contains 'World' {}", hello.contains("World"));

  // Replace
  println!("Replace: {}", hello.replace("World", "There"));

  //  Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word); 
  }

  println!("{}", hello1);
  println!("{}", hello);

}