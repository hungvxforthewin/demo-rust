use crate::garden::vegetables::Asparagus;
use crate::garden::hungvx::BachelorDegree;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    let man = BachelorDegree {};
    println!("I'm growing {:?}!", plant);
    println!("I'm growing {:?}!", man);
}
