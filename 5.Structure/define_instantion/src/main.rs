fn main() {
    /*
        Defining and Instantiating Structs
    */
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //user1.email = String::from("anotheremail@example.com"); --> let mut user1
    println!("{}", user1.email);

    //let user2 = build_user("hungvx@gmail.com".to_string(), "hungvx".to_string());
    let user2 = build_user(String::from("hungvx@gmail.com"), String::from("hungvx"));
    println!("{}", user2.email);


    /*
        Using the Field Init Shorthand
     */

    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         email,
    //         username,
    //         active: true,
    //         sign_in_count: 1,
    //     }
    // }


    /*
        Creating Instances From Other Instances With Struct Update Syntax
    */

    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // --> comment using user1 [value used here after move, value moved here]
    // TÁT cả các property của user1 (except: email). Trước, sau như nhau, check compiler


    /*
    
        Using Tuple Structs without Named Fields to Create Different Types
    
    */

    let black = Color(0, 0, 0);

    /*
        Unit-Like Structs Without Any Fields
    */

    let subject = AlwaysEqual;


    /*
        Ownership of Struct Data
        using lifetime
    */


}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct User_Ownership {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64
}

struct AlwaysEqual;

struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}