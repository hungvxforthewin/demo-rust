fn main() {
    
    /*
        variable and constant
    */ 

    // let x = 5;
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    /*
        Shadowing (scope)
    */
    
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

   /*
        -  Data Types
            + scalar 
            + compound
   */
  let x1: u32 = 10;
  println!("The value of x1 is: {}", x1);  
  let x2: i32 = -10;
  println!("The value of x2 is: {}", x2); 

  let x3 = 2.0;
  let x4: f32 = 3.0;
  println!("The value of x3 is: {}, x4 is: {}", x3, x4); 
  println!("The value of x3 + x4 is: {}, x3 - x4 is: {}", x3 + x4, x3 - x4); 
  println!("The value of x3 * x4 is: {}, x3 : x4 is: {}", x3 * x4, x3 / x4); 
  println!("The value of x3 % x4 is: {}", x3 % x4); 

  let is_active = true;
  let is_log: bool = false;
  println!("The value of isActive is: {}, value of logs is: {}", is_active, is_log);

  let cz = "cat";
  println!("The value of cat is: {}", cz);

  /*
        Tuple type
  */
  let tup: (i32, f64, u16) = (-100, 7.9, 300);
  let (get_tup1, _get_tup2, _get_tup3) = tup;
  println!("The value of tup1 is: {}", get_tup1);
  println!("The first value of tup is: {}", tup.0);

   /*
        array
   */
  
  let months = ["January", "February", "March", "April", "May", "June", "July",
  "August", "September", "October", "November", "December"];
  //println!("Array is {}", months);
  let array_1: [i32; 6] = [1, 2, 3, 4, 5, 6];
  let array_2: [i32; 3] = [3,3,3];
  let array_3 = [3;5]; // 5 element have value is 3
  println!("Value element 1 is: {}", array_1[0]);

}