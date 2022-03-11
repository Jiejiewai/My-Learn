#[derive(Debug)]
struct Rectangle {
  width: u32,
  length: u32,
}

impl Rectangle{
  fn area(&self) -> u32 {
    self.width * self.length
  }
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      length: size,
    }
  }
}

fn main() {
  let rect = Rectangle{
    width: 30,
    length: 50,
  };
  let cube = Rectangle::square(32);
  println!("{}", rect.area());
  println!("{:#?}", cube);
}