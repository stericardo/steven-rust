use postgres::{Client, NoTls};
use std::time::Instant;


fn main() {
	
	// 1. Capture the start time
    let start = Instant::now();
	
	//method call
	call_postresql();	
	
	// 3. Calculate the elapsed duration
    let duration = start.elapsed();
    
    // 4. Print the result in your preferred unit
    println!("Time elapsed in expensive_method() is: {:?}", duration);
    println!("Time in milliseconds: {}ms", duration.as_millis());

}


fn call_postresql() -> Result<(), postgres::Error> {
    let mut client = Client::connect(
        "postgresql://postgres:2rHPwHr0pu2k@localhost:5432/postgres", 
        NoTls
    )?;

    // Execute a query
    for row in client.query("SELECT id, email, first_name, last_name FROM users", &[])? {
        let id: i32 = row.get(0);
        let email: &str = row.get(1);
        let first_name: &str = row.get(2);
        let last_name: &str = row.get(3);
        println!("Found user: {} with id: {}, email:{}, last_name:{}", first_name, id, email, last_name);
    }

    Ok(())
}


