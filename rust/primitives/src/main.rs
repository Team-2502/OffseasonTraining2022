fn main() {
    let logical: bool = true;

    let a_float: f32 = 1.0;
    let an_integer: i64   = 5;

    let default_float   = 3.0;
    let default_integer = 7;
    
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    
    let mut mutable = 12;
    mutable = 21;
    
    //mutable = true;
    
	let context_result: bool = {
		let mutable = true;
		mutable
	};

	println!("{}",context_result);

	let mut context_result: i32 = if context_result {
		println!("mutable was true!");
		7
	} else {
		println!("mutable was false!");
		4
	};

	context_result = 222;

	println!("{}", context_result);

	match context_result {
		1 =>  println!("one"),
		7 | 4 | 3 | 2 => println!("good number"),
		s => {
			println!("nonstandard number");
			if (s % 2 == 0) {
				println!("even number");
			}
		}
	}

}