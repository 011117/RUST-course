





struct User {
    username: String,
    email:String,
    active: bool,
    sign_in_count: u64
}
#[derive(Debug)]
struct Rectangle {
    width:i32,
    height:i32
}
impl Rectangle {
    fn area(&self) -> i32{
        self.height * self.width
    }
    fn can_hold(&self ,other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn square(size : i32) -> Rectangle{
          let square = Rectangle{width:size , height:size};
          square
    }
}

fn main() {
  let user1 = User{
      username:String::from("Abdel"),
      email:String::from("abdelfregel@gmail.com"),
      sign_in_count:1,
      active:true
  };
  let user2 = User{
      username:String::from("Fregel"),
      email:String::from("fregel@gmail.com"),
      sign_in_count:user1.sign_in_count,
      active:user1.active
  };
  let user3 = User{
      username:String::from("Arian"),
      email:String::from("abdel@gmail.com"),
      ..user1
  };
  let rect = Rectangle{
     width:30 ,
     height:50
    };
  let rect2 = Rectangle{
    width:10 ,
    height:30
    };
    let square = Rectangle::square(3);
  println!("the area is {}",area(&rect));
  println!("the area is {}",rect.area());
  println!("rect1 can hold rect2? {}",rect.can_hold(&rect2));
  println!("{}",user1.username);
  println!("rect is {:?}",rect);
}
fn area(rect1 : &Rectangle) -> i32{
    rect1.height*rect1.width
}
