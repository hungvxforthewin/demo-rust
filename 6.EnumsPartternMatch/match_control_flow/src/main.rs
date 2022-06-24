enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[derive(Debug)]
enum Coin_2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum GenderCategory {
   Name(String),
   Usr_ID(i32)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn is_even(no:i32)->Option<bool> {
    if no%2 == 0 {
       Some(true)
    } else {
       None
    }
 }

 fn value_in_cents_2(coin: Coin_2) -> u8 {
    match coin {
        Coin_2::Penny => 1,
        Coin_2::Nickel => 5,
        Coin_2::Dime => 10,
        Coin_2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    /*
        Match Statement and Enum
    */
    let result = value_in_cents(Coin::Penny);
    println!("{}", result);

    /*
        Match Statement with option
    */
    match is_even(10) {
        Some(data) => {
           if data==true {
              println!("Even no");
           }
        },
        None => {
           println!("not even");
        }
     }
     /*
        Match & Enum with Data Type
     */
    let p1 = GenderCategory::Name(String::from("Mohtashim"));
    let p2 = GenderCategory::Usr_ID(100);
    println!("{:#?}",p1); //:?
    println!("{:#?}",p2);

    match p1 {
        GenderCategory::Name(val) => {
           println!("{}",val);
        }
        GenderCategory::Usr_ID(val) => {
           println!("{}",val);
        }
     }

     /*
        Patterns that Bind to Values
     */
    value_in_cents_2(Coin_2::Quarter(UsState::Alaska));
}
