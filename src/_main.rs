fn main() {
    //-----Ownership rules-----
    // 1. Each value in Rust has a variable called owner has an owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of dcope the value will be dropped

    // maxino not valid
    {
        let _maxino = String::from("Muhanda");
        // do stuff with me

    }
    // maxino not valid

    {
        let age = 5;
        let my_age = age + 20;
        println!("My son is {age} and i am {my_age}");
    }

    //illustrate move 

    let girl_friend = String::from("Single");
    let _ex = girl_friend.clone();
    println!("{}", girl_friend);

    // Ownership and funtions
   
    let cake1 = eat_cake();
    println!("{}", cake1);

    // references
    //references do not take ownership
    let s1 = String::from("Maxino");
    let len = calculate_len(&s1);
    println!("The lenght of {} is {}.", s1, len);
  

  // mutable refernce
  let mut first_name = String::from("Maxwell");
 last_name(&mut first_name);
  

    

}



fn eat_cake() -> String{
    let cake = String::from("Blueberry");
    cake
} // after this is executed cake is dropped

fn calculate_len(s: &String)-> usize{
  let length = s.len();
  length
}

fn last_name(last_name: &mut String){
    last_name.push_str(" Muhanda");
}