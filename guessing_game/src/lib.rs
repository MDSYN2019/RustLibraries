

/*

Last Updated: 21/03/2022
-------------------------

Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement the trait on their own types.

*/

pub mod MandelBrot {
    
    use num::complex::Complex;
    
    pub fn MatchFunction(){

    }

    // Working through an example of a much more complex function than usual 
    pub fn calculate_mandelbrot(
	max_iters: usize,
	x_min: f64,
	x_max: f64,
	y_min: f64,
	y_max: f64,
	width: usize,
	height: usize,
    ) -> Vec<Vec<usize>> {

	let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width); // Create container 

	for img_y in 0..height { // Loop between 0 and height value 

	    let mut row : Vec<usize> = Vec::with_capacity(height); // Define new
	    // Vector with every looed height 

	    for img_x in 0..width {
		let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
		let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
		let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
		row.push(escaped_at);
	    }

	    all_rows.push(row);
	}

	return all_rows 
    }

    fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
	let mut z = Complex {re: 0.0, im: 0.0}; // initialize a complex number at the origin
	let c = Complex::new(cx, cy);

	for i in 0..=max_iters {
	    if z.norm() > 2.0 { // norm computes the absolute value of the complex number 
		return i;
	    }
	    z = z * z + c;
	}
	
	return max_iters;
    }

    fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
	for row in escape_vals {
	    let mut line = String::with_capacity(row.len());
	    for column in row {

		let val = match column { 
		    0..=2 => ' ',
		    2..=5 => '.',
		    5..=10 => '.',
		    11..=30 => '*',
		    30..=100 => '+',
		    100..=200 => 'x',
		    200..=400 => '$',
		    400..=700 => '#',
		    _ => '%', 
		};

		line.push(val);
	    }
	    
	    println!("{}", line);    
	}
    }
}


pub mod Alias {
    
    // The struct itself is marked pub so that other code may use it,
    // but the fields within the struct remain private.

    // We want to ensure that whenever a value is added or removed from the list,
    // the average is also updated

    pub struct AveragedCollection {
	pub list: Vec<i32>,
	pub average: f32,
	pub name: String, 
	//pub filename: String, 
    }
    
    // https://doc.rust-lang.org/book/ch10-01-syntax.html
    impl AveragedCollection {

	pub fn largest_i32(list: &[i32]) -> i32 {
	
	    let mut largest = list[0]; // Take the first item in the mutable list 
	
	    for &item in list { // loop over the list 
		if item > largest {
		    largest = item; // if the item is larger than the previously allocated largest value, then we allocate that value as the largest value 
		}
	    }
	    return largest // return largest 
	}

	
	pub fn largest_char(list: &[char]) -> char {
	    let mut largest = list[0];
	    
	    for &item in list {
		if item > largest {
		    largest = item;
		}
	    }
	    return largest
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
	//super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = Alias::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
