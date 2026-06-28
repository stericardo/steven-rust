use regex::Regex;

fn main() {
	let name = "STEVEN";
	let age = 40;
	
    println!("Hello, world {}! >> has {} years", name, age);
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
    
    let mut x = 5;
	println!("Before: {}", x);
	x = 10;
	println!("After: {}", x);
	
	let _my_num = 5;         // integer
	let _my_double = 5.99;   // float
	let _my_letter = 'D';    // character
	let _my_bool = true;     // boolean
	let _my_text = "Hello";  // string
	
	// or sayind the type
	
	let _my_num2: i32 = 5;          // integer
	let _my_double2: f64 = 5.99;    // float
	let _my_letter2: char = 'D';    // character
	let _my_bool2: bool = true;     // boolean
	let _my_text2: &str = "Hello";  // string
}
