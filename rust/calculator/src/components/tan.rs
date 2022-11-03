use crate::Component;
pub struct tan;
impl Component for tan {
    fn new() -> tan {
        tan {}
    }
    fn run(&self) -> f64 {
        println!("input the first angle:");
        let a = self.input();
        println!("input the second angle:");
        let b = self.input();
        a.to_degrees().tan()
    }
}