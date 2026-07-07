use postgres::{Client, NoTls};
use std::time::Instant;


fn main() {
	
	// 1. Capture the start time
    let start = Instant::now();
	
	//method call
	//call_postresql();	
	insert3();
	
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

fn insert() -> Result<(), postgres::Error>  {
	
	let mut client = Client::connect(
        "postgresql://postgres:2rHPwHr0pu2k@localhost:5432/postgres", 
        NoTls
    )?;
	    for i in 1..=1000000 {
        // 2. Define the record variables
        let first_name = format!("User {}", i);
		let email = format!("ricardostev{}@gmail.com", i);
		let last_name = "Mendez";
		
		match client.execute(
			"INSERT INTO users (email, first_name, last_name) VALUES ($1, $2, $3)",
			&[&email, &first_name, &last_name],
		){
			Ok(rows_affected) => println!("Inserted {} row(s).", rows_affected),
			Err(e) => {
				// Print the high-level error message
				eprintln!("Execution error: {}", e);
				
				// Print the underlying database details (if available)
				if let Some(db_error) = e.as_db_error() {
					eprintln!("DB Message: {}", db_error.message());
					eprintln!("DB Code: {:?}", db_error.code());
					}
				}
			}
    }
    
    Ok(())

	
}


fn insert2() -> Result<(), postgres::Error>  {
	
	let mut client = Client::connect(
        "postgresql://postgres:2rHPwHr0pu2k@localhost:5432/postgres", 
        NoTls
    )?;
	    for i in 1000001..=2000000 {
        // 2. Define the record variables
        let first_name = format!("User {}", i);
		let email = format!("ricardostev{}@gmail.com", i);
		let last_name = "Mendez";
		
		client.execute(
			"INSERT INTO users (email, first_name, last_name) VALUES ($1, $2, $3)",
			&[&email, &first_name, &last_name],
		)?;
    }
    
    println!("Record inserted successfully!");
    Ok(())

	
}

fn insert3() -> Result<(), postgres::Error>  {
	
	let mut client = Client::connect(
        "postgresql://postgres:2rHPwHr0pu2k@localhost:5432/postgres", 
        NoTls
    )?;
    
    // 3. Start a transaction for synchronous/batched insertion
    let mut transaction = client.transaction()?;
    
	    for i in 2000001..=2100000 {
        // 2. Define the record variables
        let first_name = format!("User {}", i);
		let email = format!("ricardostev{}@gmail.com", i);
		let last_name = "Mendez";
		
		transaction.execute(
			"INSERT INTO users (email, first_name, last_name) VALUES ($1, $2, $3)",
			&[&email, &first_name, &last_name],
		)?;
    }
    
    // 4. Commit the transaction
    transaction.commit()?;
    
    println!("Record inserted successfully Transaction!");
    Ok(())

	
}
