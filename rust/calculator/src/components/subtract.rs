use crate::Component;
pub struct subtract;
impl Component for subtract {
    fn new() -> subtract {
        subtract{}
    }
    fn run(&self) -> f64 {
        println!("input the first number:");
        let a = self.input();
        println!("input the second number");
        let b = self.input();
        a-b    
    }
}
