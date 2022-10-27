use crate::Component;
pub struct sqrt;
impl Component for sqrt {
    fn new() -> sqrt {
        sqrt {}
    }
    fn run(&self) -> f64 {
        println!("Which number");
        let a = self.input();
        a.sqrt()
    }
}