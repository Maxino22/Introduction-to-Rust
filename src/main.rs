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
    let ex = girl_friend.clone();
    println!("{}", girl_friend);
}
