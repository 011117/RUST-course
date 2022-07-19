
#[derive(Debug)]
enum IpKindAddr{
    V4,
    V6,
}
#[derive(Debug)]
struct  IpAddr{
    kind:IpKindAddr,
    Address:String
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
   let address1 = IpAddr{
       kind : IpKindAddr::V4,
       Address : String::from("127.0.01")
   };
   let address2 = IpAddr{
       kind : IpKindAddr::V6,
       Address : String::from("::1")
   };
   println!("{:#?} , {:#?}",address1,address2);
}
fn value_in_cents(coin:Coin)-> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel =>5,
        Coin::Dime =>10,
        Coin::Quarter => 25,
    }
}