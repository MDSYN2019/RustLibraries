
/*

Last Updated: 13/03/2022
-------------------------

Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement the trait on their own types.

*/

pub mod back_of_house {
    
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
	//super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
