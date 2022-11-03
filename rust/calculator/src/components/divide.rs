use crate::Component;
pub struct div;
impl Component for div {
    fn new() -> div {
        div {}
    }
    fn run(&self) -> f64 {
        println!("input the first number:");
        let a = self.input();
        println!("input the second number:");
        let b = self.input();
       a/b
    }
}