fn main() {
    another_function(5);
    let x = five();
    println!("{}",x);
    println!("{}",plus_one(5));
}
fn another_function(x:i32){
    println!("The value is {}",x);
}
fn five() -> i32 {
    5
}
fn plus_one(x:i32) -> i32 {
    x+1
}