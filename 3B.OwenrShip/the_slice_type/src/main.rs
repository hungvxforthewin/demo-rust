fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("{}", word);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!



    /*
        Slices String
    */
    // let s1 = String::from("hello world");

    // let hello = &s1[0..5];
    // let world = &s1[6..11];
    // println!("{}", hello);
    // println!("{}", world);

    let s2 = String::from("hello world");
    let len = s2.len();
    let first_s = &s2[0..5];
    let first_s_1 = &s2[..5];
    let last_s = &s2[6..len];
    let last_s_1 = &s2[6..];

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for element in bytes {
        println!("the value is: {}", element);
    }
    for (i, &item) in bytes.iter().enumerate() {
        println!("the value is: {}: {}", i, &item);
    }
    println!("{}", b' ');
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    println!("{}", s);
    s.len()
}
