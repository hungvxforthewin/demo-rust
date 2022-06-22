fn main() {
    /*
        References and Borrowing
     */
    let s1 = String::from("Hello");
    //println!("{}", s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    /*
        References immutable 
     */
    //change(&s1);

    /*
        Mutable References
     */

     let mut s1 = String::from("hello");
     let mut s2 = String::from("hello");
     change_mut(&mut s1);
     //change_mut(&mut s2) --> you can have only one mutable reference to a particular piece of data at a time
     {
        change_mut(&mut s2);
     }

     //
     let mut s3 = String::from("hello 1");
     let r1 = &s3; // no problem
     let r2 = &s3; // no problem
     //let r3 = &mut s3; // BIG PROBLEMC
     println!("{}, {}", r1, r2);
     println!("{}, {}", r1, r2);
     // variables r1 and r2 will not be used after this point
     let r3 = &mut s3; // NO PROBLEMC
     println!("{}", r3);
     // println!("{}, {}, and {}", r1, r2, r3); // immutable borrow later used here --> ERROR



     /*
        Dangling References
     */
     let s4 = no_dangle();
     println!("{}", s4);

}

fn calculate_length(s: &String) -> usize {
    println!("{}", s);
    s.len()
}

// fn change(some_string: &String){
//     some_string.push_str(", world");s
// }

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
} 

// fn dangle() -> &String {
//     let s = String::from("Hello Dangling References");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("Hello Dangling References");

    s
}