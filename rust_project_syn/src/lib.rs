/*

Last Updated: 11/04/2022
-------------------------

Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement the trait on their own types.

*/

//extern crate itertools;
//#[macro_use(c)] // cute is a macro for python-esque and dictionary

// Write unit tests here
//#[cfg(test)]
//mod tests {
//    use super::*;
//    #[test]
//}

pub mod molecular_structures {

    #[derive(Debug)]
    pub struct FILETYPE {
        name: String,
        data: Vec<u8>,
    }

    impl FILETYPE {
        /*
        A file interface able to read in a generic molecular mechanics or a force field file
        for crunching numbers with this rust program.
         */

        fn new(name: &str) -> FILETYPE {
            // Generate new FILETYPE with name, but empty vector
            FILETYPE {
                name: String::from(name),
                data: Vec::new(),
            }
        }

        fn new_with_data(name: &str, data: &Vec<u8>) -> FILETYPE {
            let mut f = FILETYPE::new(name);
            f.data = data.clone();
            f
        }

        #[allow(dead_code)]
        fn read(f: &mut FILETYPE, save_to: &mut Vec<u8>) -> Result<usize, String> {
            // Result<T, E> -> T is an integer type usize, and E is string. Using Sting
            let mut tmp = f.data.clone();
            let read_length = tmp.len();
            save_to.reserve(read_length); // Ensures that there is sufficient space to fit the incoming data
            save_to.append(&mut tmp);
            Ok(read_length) // return read length. Otherwise, return a string as we are returning a Result
        }

        #[allow(unused_variables)]
        fn readErr(f: &FILETYPE, save_to: &mut Vec<u8>) -> usize {
            if random() && random() && random() {
                unsafe {
                    ERROR = 1;
                }
            }
            0 // if no error, return normal non-error status
        }
    }

    // functions within the mods

    fn one_in(denominator: u32) -> bool {
        thread_rng().gen_ratio(1, denominator) // <3>
    }

    fn open(f: FILETYPE) -> Result<FILETYPE, String> {
        if one_in(10_000) {
            let err_msg = String::from("Permission denied");
            return Err(err_msg);
        }
        Ok(f)
    }

    fn close(f: FILETYPE) -> Result<FILETYPE, String> {
        if one_in(100_000) {
            // Once in 10000 executions, return an error
            let err_msg = String::from("Interrupted by signal!");
            return Err(err_msg);
        }
        Ok(f)
    }

    // specific coordinate operation

    pub struct Coordinates {
        pub X: f32, // x coordinates
        pub Y: f32, // y coordinates
    }

    /*
    Methods are functions that are
     */

    impl generateVec for Coordinates {
        fn vectorize(&self) -> Vec<Vec<f32>> {
            let mut vector_of_vectors: Vec<Vec<f32>> = Vec::new(); // initialize new vector of vectors
            let mut inner_vector: Vec<f32> = Vec::new(); // initialize new vector

            inner_vector.push(self.X);
            inner_vector.push(self.Y);
            vector_of_vectors.push(v);
            vector_of_vectors
        }
    }
}

pub mod CubSat {
    /*
    Create a modular implementation of the cubestats in queston
     */
    #[derive(Debug)]
    enum StatusMessage {
        Ok,
    }
}

pub mod misc {
    // Struct definitions
    pub struct User {
        pub active: bool,
        pub username: String,
        pub email: String,
        pub sign_in_count: u64,
    }

    impl User {
        pub fn printOutline(&self) {
            println!("{} {}", self.username, self.email)
        }
    }
}

pub mod MandelBrot {

    use core::mem::swap;
    use num::complex::Complex;
    use rand;
    use rand::prelude::*;
    use rand::{random, Rng};
    use std::cmp::Ordering;
    use std::io;
    use std::io::prelude::*;
    use std::io::ErrorKind;
    use std::thread;
    use std::time::Duration;

    // Libaries necessary to run all functions within
    // this module

    pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    // We require an intensity number from the user, which is specified when they request
    // a workout to indicate whether they want a low-intensity workout or high intensity workout

    pub fn generate_workout(intensity: u32, random_number: u32) {
        /*


        */
        if intensity < 25 {
            println!("{} exercises", simulated_expensive_calculation(intensity));
        } else {
            if random_number == 3 {
                println!("take a break today! Remember to stay hydrated");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }

    pub fn MatchFunction() {
        /*

        Packing up the previous functions into here

        from the guess, see if the number fits

         */
        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("The secret number is: {}", secret_number);

        loop {
            println!("Please input your guess");
            let mut guess = String::new(); // Rust has a strong, static type system.
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line"); // handling potential failure with the result type

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too Small"),
                Ordering::Greater => println!("Too big"),
                //Ordering::Equal => println!("You win!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
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

        for img_y in 0..height {
            // Loop between 0 and height value

            let mut row: Vec<usize> = Vec::with_capacity(height); // Define new
                                                                  // Vector with every looed height

            for img_x in 0..width {
                let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
                let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
                let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
                row.push(escaped_at);
            }

            all_rows.push(row);
        }

        return all_rows;
    }

    fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
        let mut z = Complex { re: 0.0, im: 0.0 }; // initialize a complex number at the origin
        let c = Complex::new(cx, cy);

        for i in 0..=max_iters {
            if z.norm() > 2.0 {
                // norm computes the absolute value of the complex number
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

// https://medium.com/analytics-vidhya/practical-introduction-to-hartree-fock-448fc64c107b

pub mod HartreeFock {
    /*


    https://nznano.blogspot.com/2018/03/simple-quantum-chemistry-hartree-fock.html

     */

    use core::mem::swap;
    use core::mem::take;
    use cute::c; // https://crates.io/crates/cute
    use num::complex::Complex;
    use std::fs; // filesystems?

    //use iter_num_tools::lin_space;
    use itertools_num::linspace;

    //extern crate cute;

    // The struct itself is marked pub so that other code may use it,
    // but the fields within the struct remain private.

    pub struct HFDataset {
        pub list: Vec<i32>, // this data is private now
        pub average: f32,
        pub name: String,
        //pub CoordinatesX: Vec<f32>, // Store X coordinates
        //pub CoordinatesY: Vec<f32>, // Store Y coordinates
        //pub CoordinatesZ: Vec<f32>, // Store Z Coordinates
        //pub filename: String,
    }

    // https://doc.rust-lang.org/book/ch10-01-syntax.html
    impl HFDataset {
        pub fn OpenStructureFile(filename: &str) {
            // We wish to be able to read the coordinates of the coordinates
            //
            let coordinateInformation = fs::read_to_string(filename);
        }

        // https://nznano.blogspot.com/2018/03/simple-quantum-chemistry-hartree-fock.html
        pub fn psi_STO(minimum: f32, maximum: f32, num: i32) {
            // https://stackoverflow.com/questions/45282970/does-rust-have-an-equivalent-to-pythons-list-comprehension-syntax
            //let mut LinspaceData = c![x.abs(), for x in minimum..maximum];
            let mut LinspaceData = linspace::<f32>(minimum, maximum, num.try_into().unwrap());
            //let mut LinspaceData = (minimum..maximum).filter(|x| x.abs()).collect::<Vec<u32>>();
            let zeta: f64 = 1.0;
            let PI: f64 = 3.14159265358979323846264338327950288;
            let r: f64 = 0.0;
            // Need to convert the values from signed to absolute values in the linspace

            // Rust list comphension equivalents - https://stackoverflow.com/questions/45282970/does-rust-have-an-equivalent-to-pythons-list-comprehension-syntax

            let v1 = (0u32..9)
                .filter(|x| x % 2 == 0)
                .map(|x| x.pow(2))
                .collect::<Vec<_>>();
            let v2 = (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>();

            let psi_STO = (zeta.powf(3.0) / PI).powf(0.5) * (-1.0 * zeta.powf(r));
            // https://doc.rust-lang.org/std/primitive.f64.html#method.exp
        }

        pub fn matchevenodd(&mut self) -> Vec<String> {
            let mut vecString: Vec<String> = Vec::new();
            let buf = self.list.clone();

            for item in buf {
                // Depending on whether the value
                // is even or odd, we will push a different categorical
                // string

                if (item % 2 == 0) {
                    vecString.push("Even".to_string());
                } else {
                    vecString.push("Odd".to_string());
                }
            }

            return vecString;
        }

        /*
        Find the largest element within the vector that is in the struct
        */

        pub fn largest_i32(&mut self) -> i32 {
            // https://stackoverflow.com/questions/63353762/cannot-move-out-of-which-is-behind-a-mutable-reference
            let buf = self.list.clone();
            //swap(&mut buf, &self.list);
            let mut largest = buf[0]; // Take the first item in the mutable list
            for item in buf {
                // loop over the list
                if item > largest {
                    largest = item; // if the item is larger than the previously allocated largest value, then we allocate that value as the largest value
                }
            }
            return largest; // return largest
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
    let mut meal = HartreeFock::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
