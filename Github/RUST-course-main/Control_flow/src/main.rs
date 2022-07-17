fn main() {
    let x =3;
    if x < 5 {
      println!("Yes");
    } else {
        println!("No");
    }
    //loop
    let mut counter = 0;
    let  result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; 
        }
    };
    println!("{}",result);
    //while
    let mut count = 3;
    while count != 0 {
        println!("{}",count);
        count -= 1;
    }
    println!("Ya");
    //for
    let arr = [10,20,30,40,50];
    for elem in arr.iter() {
        println!("{}",&elem);
    }
}
