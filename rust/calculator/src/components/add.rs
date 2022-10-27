use crate::Component;
pub struct add;
impl Component for add {
    fn new() -> add {
        add {}
    }
    fn run(&self) -> f64 {
        println!("input the first number:");
        let a = self.input();
        println!("input the second number:");
        let b = self.input();
        a+b
    }
}
