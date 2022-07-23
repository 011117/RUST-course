#[cfg(test)]
mod front_of_house{
   pub mod hosting{
        pub fn add_to_wailist(){

        }
        fn seat_at_table() {
            
        }

    }
    pub mod serving{
        fn take_order(){

        }
        fn serve_order(){

        }
        fn take_payment(){

        }
    }
}
mod back_of_house{
    pub struct Breakfast{
        pub toast : String,
        seasonal_fruit : String,
    }
    impl Breakfast {
          pub fn summer(toast : &str) -> Breakfast{
              Breakfast{
                   toast : String::from(toast),
                seasonal : String::from("peaches")
              }
          }
    }
}
pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer;
    meal.toast = String::from("Wheal");
    println!("i'd like {} toast please",meal.toast);
}
