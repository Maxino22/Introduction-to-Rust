fn main(){
  // references and Borrowing
  let s1 = String::from("Hello");
  let len = calculate_len(&s1);
  print!("The lenght of '{}' is {}", s1, len);

  // mutable references
  let mut boy = String::from("Adam");
  let b2 = &boy;
  let b3 = &boy;
  
  println!("{}, {}", b2, b3);
  
  let b5 = &mut boy;
  
  b5.push_str("Owen");

  println!("{}", b5);

 //Dangling References
 let reference_to_nothing = dangle();


}

fn calculate_len(s: &String)-> usize {
  s.len()


  
}



fn dangle() -> String{
  let s = String::from("Maxino");

  s
}

