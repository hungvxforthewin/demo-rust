fn main() {
    /*
        Create a NEW Vector
    */

    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    /*
        Updating a Vector
    */
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    
    /*
        Dropping a Vector Drops Its Elements 
    */
    {
        let v = vec![1, 2, 3, 4];
    }

    /*
        Reading Elements Of Vectors 
    */
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        let third2: i32 = v[2];
        println!("The third element is {}", third2);
        println!("{}", third2);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }
}
