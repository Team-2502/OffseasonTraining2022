use crate::Component;
pub struct fizz;
impl Component for fizz {
    fn new() -> fizz {
        fizz {}
    }
    fn run(&self) -> f64 {
        println!("input the number");
        let n = self.input();
        let mut s = "".to_owned();

        for i in 1..n as i64 + 1 {
            if (i % 3 == 0) && (i % 5 == 0){
                s.push_str("FizzBuzz ");
            } else if (i % 3 == 0) {
                s.push_str("Buzz ");
            } else if (i % 5 == 0) {
                s.push_str("Fizz ");
            } else {
                s.push_str(&i.to_string());
                s.push_str(" ");
            }
        }

        println!("{}", s);
        0.0

    }
}
