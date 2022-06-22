fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    let word2 = first_word_2(&s); // word will get the value 5
    //s.clear(); // error because
    println!("{}", word);
    println!("{}", word2);

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

    let all_s = &s2[0..len];
    let all_s_1 = &s2[..];

    /*
        String Literals Are Slices
    */
    let s5 = "Hello, World!";
    let se = &s5;
    let see =  &s5[..];
    println!("{}, {}, {}", s5, se, see);

    /*
        String Slices as Parameters
    */

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word1 = first_word_3(&my_string[0..6]);
    let word2 = first_word_3(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    //  to whole slices of `String`s
    let word3 = first_word_3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word4 = first_word_3(&my_string_literal[0..6]);
    let word5 = first_word_3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word6 = first_word_3(my_string_literal);

    /*
        Other slices
     */

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
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

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
