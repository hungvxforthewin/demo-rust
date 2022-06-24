#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn display(&self) -> String {
        String::from("This is method two")
    }
    fn with(&self) -> String {
        // name same property of struct
        String::from("This is method two, same name")
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*
        Method syntax
            + Methods are similar to functions: we declare them with the fn keyword and a name, 
              they can have parameters and a return value, 
              and they contain some code that’s run when the method is called from somewhere else.
            + Unlike functions, methods are defined within the context of a struct (or an enum or a trait object) 
              and their first parameter is always self, which represents the instance of the struct the method is being called on.
    */

    /*
        Define method
    */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.display()
    );

    /*
        Methods with More Parameters
    */
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /*
        Associated Functions
        Multiple impl Blocks
    */
    let sq = Rectangle::square(3);
}
