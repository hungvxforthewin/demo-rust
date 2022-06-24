#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8); // This is Option<T>
    let config_max_2 = Some(3);
    match config_max_2 {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // instead
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // example 2
    let coin = Coin::Penny;
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) =>  println!("State quarter from {:?}!", state),
    //     _ => {
    //         println!("Not in here");
    //         count += 1
    //     } 
    // }
    println!("{}", count);
    // instead
    let coin_2 = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin_2 {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not in here");
        count += 1;
    }
}
