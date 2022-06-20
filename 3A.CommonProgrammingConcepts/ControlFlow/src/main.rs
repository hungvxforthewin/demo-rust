fn main() {
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }


    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }


    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // //  let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {}", number);


    // loop {
    //     println!("again!");
    // }


    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    /*
        Returning Values from Loops
     */

    //  let mut counter = 0;
    //  let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    //  };

    // println!("The result is {}", result);

    /*
        Conditional Loops with while
    */

    // let mut number: i32 = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    let my_array = [7, 8, 3, 9, 9];
    let mut index: usize = 0; // why i32
    // When developing in Rust, numeric types like i32 cannot be used as indexes to a slice or vector. This causes an error for new Rust developers.

    while index < 5 {
        println!("the value is: {}", my_array[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
