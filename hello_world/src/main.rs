use regex::Regex;

fn main() {
	//const BIRTHYEAR: i32 = 1980;
	//const MINUTES_PER_HOUR: i32 = 60;
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
	
	let time = 20;
	let greeting = if time < 18 {
	  "Good day."
	} else {
	  "Good evening."
	};
	println!("{}", greeting);
	
	let day = 4;

	match day {
		1 => println!("Monday"),
		2 => println!("Tuesday"),
		3 => println!("Wednesday"),
		4 => println!("Thursday"),
		5 => println!("Friday"),
		6 => println!("Saturday"),
		7 => println!("Sunday"),
		_ => println!("Invalid day."),
	}
  
	let day2 = 6;

	match day2 {
		1 | 2 | 3 | 4 | 5 => println!("Weekday"),
		6 | 7 => println!("Weekend"),
		_ => println!("Invalid day"),
	}
    
    let day3 = 4;

	let result3 = match day3 {
		1 => "Monday",
		2 => "Tuesday",
		3 => "Wednesday",
		4 => "Thursday",
		5 => "Friday",
		6 => "Saturday",
		7 => "Sunday",
		_ => "Invalid day.",
	};

	println!("{}", result3);
  
	let mut count = 1;

	loop {
	  println!("Hello World!");

	  if count == 3 {
		break;
	  }

	  count += 1;
	}
	
	let mut count2 = 1;

	let resultcount2 = loop {
	  println!("Hello!");

	  if count2 == 3 {
		break count2; // Stop the loop and return the number 3
	  }

	  count2 += 1;
	};

	println!("The loop stopped at: {}", resultcount2);
	
	let mut countwhile = 1;

	while countwhile <= 5 {
	  println!("countwhile: {}", countwhile);
	  countwhile += 1;
	}
	
	for i in 1..6 {
	  println!("i1 is: {}", i);
	}
	
	let cont_for_loop = 10;
	for i in 1..cont_for_loop {
	  println!("i2 is: {}", i);
	}
	
	for i in 1..=6 {
	  println!("i3 is: {}", i);
	}
	
	for i in 1..=10 {
	  if i == 3 {
		continue; // skip 3
	  }
	  if i == 5 {
		break; // stop before printing 5
	  }
	  println!("i4 is: {}", i);
	  
	  let cont_loop = 2;
	  loop {
		  println! ("Loop");
		  if cont_loop == 2 {
			break;
		  }
		}
	}
	function_steven();
	my_function();
	//println!("{}", message); // Error - you cannot access the message variable outside of the function myFunction
	let x = 5;
	{
	  let x = 10;
	  println!("Inside block: {}", x);
	}
	println!("Outside block: {}", x);
	let x = 15;
	println!("SAME block SHADOWING: {}", x);
	
	// STRINGS
	let greeting: &str = "Hello";
	println!("{}", greeting);
	let text1 = "Hello World".to_string();
	let text2 = String::from("Hello World");
	println!("{}...{}", text1, text2); // 
	
	let mut greeting = String::from("Hello");
	greeting.push_str(" World");
	println!("{}", greeting); // Hello World
	
	let mut word = String::from("Hi");
	word.push('!');// Use push() to add one character:
	println!("{}", word); // Hi!
	
	let s1 = String::from("Hello");
	let s2 = String::from("World!");
	let s3 = String::from("What a beautiful day!");
	let result = format!("{} {} {}", s1, s2, s3);
	println!("{}", result);
	
	//Note: You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here.
	let s1 = String::from("Hello");
	let s2 = String::from("World!");
	let s3 = String::from("What a beautiful day!");
	let result = s1 + " " + &s2 + " " + &s3;
	println!("{}", result);
	println!("Length: {}", result.len()); 
	
	// Basic Ownership Example
	let a = String::from("Hello");
	let b = a;

	// println!("{}", a); Error: a no longer owns the value
	println!("{}", b); // Ok: b now owns the value
	// When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.
	//println!("{}", a);  WILL PRODOCE ERROR 
	/*
	 * error[E0382]: borrow of moved value: `a`
	 * --> src/main.rs:186:17
     * |
	 *	181 |     let a = String::from("Hello");
	 *		|         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
	 *	182 |     let b = a;
	 *		|             - value moved here
	 *	...
	 *	186 |     println!("{}", a); 
	 *		|                    ^ value borrowed here after move
	 *		|
	 *	help: consider cloning the value if the performance cost is acceptable
	 *		|
	 *	182 |     let b = a.clone();
	 *		|              ++++++++
	 */ 
	// But simple types like numbers, characters and booleans are copied, not moved.
	// This means you can still use the original variable after assigning it to another:
	let a = 5;
	let b = a;
	println!("a = {}", a);  // Works
	println!("b = {}", b);  // Works
	
	//Clone
	//For other types, like String, if you really want to keep the original value and also assign it to another variable, 
	//you can use the .clone() method, which makes a copy of the data:
	let a = String::from("Hello");
	let b = a.clone(); // Now both have the same value

	println!("a = {}", a);  // Works
	println!("b = {}", b);  // Works
}

fn function_steven() {
	let cont_loop = 2;
	loop {
		  println! ("Loop - Steven Function");
		  if cont_loop == 2 {
			greet("STEVEN CALLING ANOTHER FUNCTION");
			break;
		  }
	}
}

fn greet(name: &str) {
	println!("Hello, {}!", name);
	let sum = add(3, 4);
	println!("Sum is: {}", sum);
	let sum2 = add2(3, 4);
	println!("In Rust, you can omit the return keyword Sum2 is: {}", sum2);
}

fn add(a: i32, b: i32) -> i32 {
	return a + b;
}

/**
 * In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon:
 */
fn add2(a: i32, b: i32) -> i32 {
  a + b
}

fn my_function() {
  let message = "Hello!";
  println!("{}", message);  // You can access the message variable here
}


