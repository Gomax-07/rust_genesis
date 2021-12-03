pub fn run() {
  // print to console
  println!("Hello from the print.rs file");

  // Basic Formatting
  println!("{} is from {}","Brad", "Mars");

  //Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mars", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name="John", activity="football" );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10,10);

  // Placeholder for debug trait
  println!("{:?}", (12,true,"hello"));

  // Basic math
  println!("10+10 = {}", 10+10);
  println!("{}", 10*3);

  
}