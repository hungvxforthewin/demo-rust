fn main() {
    // owner ship: stack, heap
    /*
        Variable Scope
     */

    // let mut s = "hello";
    // {
    //     let s = "hello <3";
    //     println!("The value of s is: {}", s);
    // }
    // println!("The value of s is: {}", s);


    /*
        The String Type
     */
    // let mut s = String::from("hello");
    
    // s.push_str(", world!"); // push_str() appends a literal to a String

    // println!("{}", s); // This will print `hello, world!`
    
     /*
        Memory and Allocation
     */
    // {
    //     let mut s1 = String::from("hello");
    
    //     s1.push_str(", world!"); // push_str() appends a literal to a String

    //     println!("{}", s1); // This will print `hello, world!`
    // }

    /*
        Ways Variables and Data Interact: Move
     */
    // let x = 5;
    // let y = x;
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}", s1);

    /*
        Ways Variables and Data Interact: Clone
    */
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    /*
        Stack-Only Data: Copy
     */

    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);


    /*
        Ownership and Functions
    */
        
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("{}", s);                                
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
 
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.