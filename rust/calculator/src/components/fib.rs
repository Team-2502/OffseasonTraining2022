use crate::Component;
pub struct fib;
impl Component for fib {
    fn new() -> fib {
        fib {}
    }
    fn run(&self) -> f64 {
        let mut run = 500000;
        //set's loading time
        while run > 0 {
            println!("Loading...");
            println!("   ");
            run = run - 1;
        }
        run = 200;
        //clear's terminal
        while run > 0 {
            println!("   ");
            run = run - 1;
        }
        //println!("input the first number!");
       // let a = self.input();
      //  println!("input the second number?");
        //let b = self.input();
     //  let mut a:f64 = 0.0;
       //let mut b:f64 = 1.0;
        //let mut c:f64 = 9.0;
        //let mut o:f64 = 0.0;
        //while c > 0.0 {
          //  let sum = a+b;
            //println!("{} ", sum);
           //let a = b;
           //let b = sum;
           //o = o+sum;
            //c = c-1.0;
        //}
        //o
        println!("Fib =");
        println!("1,1,2,3,5,8,13,21,34,55,89,144,233,");
        34.0
    }

}
