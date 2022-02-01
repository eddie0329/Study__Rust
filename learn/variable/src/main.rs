fn main() {
  // let practice
  let mut x = 10;
  println!("The value x is: {}", x);
  x = 20;
  println!("The value x is: {}", x);

  const TEMP: i32 = 1000;
  println!("The value temp is: {}", TEMP);

  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("Inner: {}", x);
  }
  println!("Outer: {}", x);

  let mut spaces = "   ";
  spaces = spaces.len(); // error
  println!("Spaces: {}", spaces); // 3

}
