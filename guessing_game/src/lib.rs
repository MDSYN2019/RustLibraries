
mod front_of_house;

fn serve_order() {}

/*
Error Handling
--------------


Rust's commitment to reliability to error handling. Errors are a fact of life 
in software, so Rust has a number of features for handling situations in which 
sometimes goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error 
and take some action before your code will compile.

This requirement makes your program more robust by ensuring that you'll 
discover errors and handle them appriately before you've deployed your code to production!

---

Rust groups errors into two major categories: recoverable and unrecoverable errors. For a 
recoverable error, such as file not found, its reasonable to report the problem 
to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, 
like trying to access a location beyond the end of an array 

Most languages dont distinguish between these two kinds of errors and handle both 
in the same way, using mechanisms such as exceptions 


--- 

Most languages don't distinguish between these two kind of errors and handle both in the same 
way, using mechanisms such as exceptions. 

 */


pub mod back_of_house {
    
    // Private enum inside the module 

    /*
    The T and E are generic type parameters: we'll discuss generics in more detail. What you need to know
    is that T represents the type of the value that will be returned in a success case within the Ok variant,
    and E represents the type of the error that will be returned in a failure case within the Err variant 
    */
    
    pub struct AveragedCollection {
	pub list: Vec<i32>,
	pub average: f32,
	//pub filename: String, 
    }

 
    // The struct itself is marked pub so that other code may use it,
    // but the fields within the struct remain private.

    // We want to ensure that whenever a value is added or removed from the list,
    // the average is also updated

    impl AveragedCollection {
	// Add - call upon the mutable self, and push value into the list

	//pub fn read(&self) {
	//    self.File::open(self.filename);
	//}

	pub fn printVec(&self) {
	    
	}

	
	pub fn add(&mut self, value: i32) {
	    self.list.push(value);
	    self.update_average();
	}

	pub fn average(&self) -> f32 {
	    self.average
	}
	
	fn update_average(&mut self) {
	    let total: i32 = self.list.iter().sum();
	    self.average = total as f32 / self.list.len() as f32;
	}
    
    }
    
    pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String, 
    }

    // impl on breakfast 

    impl Breakfast {
	pub fn summer(toast: &str) -> Breakfast {
	    Breakfast {
		toast: String::from(toast),
		seasonal_fruit: String::from("peaches"),
	    } // return breakfast 
	}
    }
    
    struct Example {
	number: i32, 
    }
    
    impl Example {

	fn boo() {
	    println!("boo! Example:boo() was called!");
	}

	fn answer(&mut self) {
	    self.number += 42; 
	}

	fn get_number(&self) -> i32 {
	    self.number // return number
	}
    }
    
    fn fix_incorrect_order() {
	cook_order();
	super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
