pub fn run()  {
  // Print console
  println!("Hello! World");

  // Basic formatting
  println!("{} is from {}", "Brad", "Mass");

  // Basic formatting
  println!("{0} is from {1} and {0} likes {2}" , "Brad", "Mass", "code");

  // Named formatting
  println!("{name} to play {activity}" , name = "John", activity = "Baseball");

  // Placeholder 
  println!("Binary: {:b} Hex: {:x} Octal: {:o}" , 10, 10, 10);

  // Placeholder for debug trait 
  println!("{:?}", (12, true, "hello"));

  // Basic math 
  println!("10 + 10 = {}", 10 + 10);
 
}