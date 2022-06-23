fn main() {

    /*
        An Example Program Using Structs
    */
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    /*
        Refactoring with Tuples
    */
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );
    /*
        Refactoring with Structs: Adding More Meaning
    */
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(&rect2)
    );
    //println!("{}", rect2.width);
    //println!("{}", rect2.width);
    

    // let rect3 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_4(rect3)
    // );
    // println!("{}", rect3.width); --> Ownership move data



    /*
        Adding Useful Functionality with Derived Traits
    */

    
    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect4 is {:?}", rect4);
    println!("rect4 is {:#?}", rect4);

}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_4(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Why use & reference