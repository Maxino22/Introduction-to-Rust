
fn main() {
  let s =  String::from("Maxwell Muhanda");  

  let slice = &s[0..6];
  let slice2 = &s[7..];

  println!("{} and {}", slice, slice2);
 
 let _word = first_word(&s);

 //s.clear();

//  println!("the first word is: {}", word);

 let a = [1,2,3,4,5];
 let slice = &a[1..3];

 assert_eq!(slice, &[2,3]);

}



fn first_word(s: &String)-> &str{
  // Slice Type 
  let bytes = s.as_bytes();
   
  for (i , &item ) in bytes.iter().enumerate(){
    if item ==b' ' {
      return  &s[0..i];
    }
  }
 &s[..]

}