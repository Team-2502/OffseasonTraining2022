use std::error::Error;
use std::io;
mod components;

use crate::components::add;
use crate::components::fib;

pub fn run() -> Result<(), Box<dyn Error>> {

    let selectedMode: mode = getMode().unwrap();
    println!("{:?}",selectedMode);

    match selectedMode {
        ADD => {
            println!("{}",add::add(1,2));
        },
        FIB => {
            println!("{}",fib::fib(1));
        },
        _ => {
            println!("Unimplemented");
        }
        
    }
    

    Ok(())
}

#[derive(Debug)]
enum mode {
    FIB,
    ADD,
    SUB,
    FIZZ,
    SQRT,
    UNKNOWN,
}

fn getMode() -> Result<mode, Box<dyn Error>> {
    println!("What mode do you want?");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;


    let id: u16 = buffer.trim().parse()?;
    //let id = buffer.trim.parse::<i32>().unwrap();


    match id {
        1 => Ok(mode::FIB),
        2 => Ok(mode::ADD),
        3 => Ok(mode::SUB),
        _ => Ok(mode::UNKNOWN),
    }

}
