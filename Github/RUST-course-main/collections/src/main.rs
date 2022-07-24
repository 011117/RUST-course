
use std::collections::HashMap;


fn main() {
    //let v : Vec<i32> = Vec::new();
    let mut v = vec![1,2,3,4];
    v.push(7);
    match v.binary_search(&4){
        Ok(0..) => println!("Si esta"),
        _ => println!("no esta"),
    };
    //first way 
    let third: &i32 = &v[2];
    println!("{}",third);
    // second way
    match v.get(2) {
        Some(third) => println!("{}",third),
        None => println!("no tiene third"),
    }
    let v2 = vec![100,2,300];
    for i in &v2{
        println!("{}",i);
    }

    let mut s =  String::new();
    s.push_str("foo");//agrega un string
    s.push('l');//agrega un character
    let _s2 = &s;
    println!("{}",&s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

     //let s4 = s1 + "-" + &s2 + "-" + &s3; esto es lo mismo q format!
     let s4 = format!("{}-{}-{}",s1,s2,s3);
     println!("{}",s4);
     let teams = vec![String::from("azul"),String::from("amarillo")];
     let _scores = vec![12,59]; 
     
     let Scores : HashMap<_,_>  = teams.iter().zip(_scores.iter()).collect();
}
