struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

#[derive(Debug)]
struct Rectangle{
  width: i32,
  height: i32
}

impl  Rectangle
 {
    fn square(size: i32) -> Self{
      Self { width: size, height: size}
    }
}

impl Rectangle {
    fn area(&self)-> i32 {
       self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle)-> bool{
      self.width  > other.width && self.height > other.height
    }
}

fn main(){
 let rect = Rectangle{
  width: 23,
  height: 10
 };

 let rect1 = Rectangle{
  width: 10,
  height: 8
 };

 let rect2 = Rectangle{
  width: 32,
  height: 20
 };

 let rect3 = Rectangle::square(30);

 println!("rect: {:#?}", rect);
 println!("area of rect: {}", rect.area());
 println!("rect can hold rect 1: {}", rect.can_hold(&rect1));
 println!("rect can hold rect 2: {}", rect.can_hold(&rect2));
 println!("This is a square: {:#?}", rect3);
}

