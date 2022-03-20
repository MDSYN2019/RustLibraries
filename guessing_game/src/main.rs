/*
----------------------
Author: Sang Young Noh
----------------------

------------------------
Last Updated: 20/03/2022
------------------------

Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing 
their absence. 

Rosetta Stone code - http://rosettacode.org/wiki/Rosetta_Code

- Rosetta stone tasks not implemented in rust - http://rosettacode.org/wiki/Reports:Tasks_not_implemented_in_Rust


Useful Links:
------------

-> https://fasterthanli.me/series/advent-of-code-2020/part-1 - advent of code 

-> https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

-> https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters

-> https://doc.rust-lang.org/std/macro.include_str.html

-> https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters

-> https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html  - object orientation in Rust 

-> https://os.phil-opp.com/ - building an OS in rust 

-> https://doc.rust-lang.org/book/ch09-00-error-handling.html - error handling in rust 

-> https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/ - blockchain app in rust 

*/

/*

The code will panic! no matter why File::open failed. What we want to do instead is take different actions 
for different actions for different failure reasons: 

if File::open failed because the file doesn't exist, we want to create the file 
and return the handle to the new file. 

If File::open failed for any other reason - for example, because we didn't have permission to open the file 
- we still want to code to panic! in the same way as it did. 

*/


#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::io::ErrorKind; 
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io;
use std::time::{Duration, Instant};

use rand; 
use rand::Rng;


use num::complex::Complex;


// importing back_of_house module::AveragedCollection
use guessing_game::back_of_house::AveragedCollection;

// Struct definitions
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32, 
}

// enum definitions

/*
Enums are a way of defining custom data types in a different way than you do with structs. Let's look 
at a situation we might want to express in code. 

Any ip address can be either a version four or version six address, but not both at the same time. That property
of IP addresses makes the enum data structure appropriate, because an enum value can only be one of its variants. Both version four and 
version six addresses are still fundamentally IP addresses, so they should be treated as the same type 
when the code  
*/


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    
}

struct IntegerAdder {
    kind: SpreadsheetCell,
    address: String,
    numlist: Vec<i32>, 
}

fn printLoop(value: &Vec<i32>) {
    let valueClone = value.clone();
    for i in &valueClone {
	println!("{} \n", i)
    }   
}


fn HashMapRelatedFunction(value: &mut HashMap<String, i32>) {
    /*

    Description
    -----------
    From the mutable hashmap, insert the <string, int> entry into the 
    hashmap with the function. 

     */

    value.insert(String::from("Blue"), 10);
    value.insert(String::from("Yellow"), 50);
    
    // what other transformations do I wish to do with the hashmap?
}


// https://doc.rust-lang.org/book/ch10-01-syntax.html

fn largest_i32(list: &[i32]) -> i32 {

    let mut largest = list[0]; // Take the first item in the mutable list 

    for &item in list { // loop over the list 
        if item > largest {
            largest = item; // if the item is larger than the previously allocated largest value, then we allocate that value as the largest value 
        }
    }
    return largest // return largest 
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest
}

/*

'a represents the generic type 
*/

pub struct LinesWithEndings<'a> {
    input: &'a str,
}

impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings {
            input: input,
        }
    }
}

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.input.is_empty() {
            return None;
        }
        let split = self.input.find('\n').map(|i| i + 1).unwrap_or(self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}

// 
pub trait generateVec {
    fn vectorize(&self) ->  Vec<Vec<f32>>; 
}

// trait with default return value implemented 
pub trait Summary {
    fn summarize(&self) -> String {
	String::from("(Read more...)")
    }
}

pub struct Coordinates {
    pub X: f32,
    pub Y: f32, 
}
    
impl generateVec for Coordinates {
    fn vectorize(&self) ->  Vec<Vec<f32>> {
	let mut  vv : Vec<Vec<f32>> = Vec::new(); 
	let mut v: Vec<f32> = Vec::new();
	v.push(self.X);
	v.push(self.Y);
	vv.push(v);
	vv
    }    
}

fn main() {

    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42,
		    132, 429, 1430, 4862];

    for item in &haystack {
	let result = match item {
	    42 | 132 => "hit",
	    _ => "miss", 
	};	
    }
    
    //panic!("crash and burn");
    // Using structs
    // Hashmaps    
    let user1 = User {
	email: String::from("sangyoung123@googlemail.com"),
	username: String::from("something"),
	active: true,
	sign_in_count: 1, 
    };

    let CoordinatesXY = Coordinates {
	X: 30.0,
	Y: 20.0, 
    };
   
    for i in &CoordinatesXY.vectorize() {
	println!("{} {}", i[0], i[1]);
    }
    
    //println!("{}", CoordinatesXY.vectorize());
    /*
    You can create an empty hash map with new and add elements with insert.
    */
    
    // let mut scores = HashMap::new();
    let mut scores: HashMap<String, i32> = HashMap::new();
    HashMapRelatedFunction(&mut scores); // Adding in values into the hashmaps
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let mut v: Vec<i32> = Vec::new();
    // Adding elements to the vector 
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    printLoop(&v);
    
    let AveragedCollection1 = AveragedCollection {
	list: v.clone(),
	average: 0.0 
    };

    let user1 = User {
	email: String::from("sangyoung123@googlemail.com"),
	username: String::from("something"),
	active: true,
	sign_in_count: 1, 
    };

    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    
    /*

    Handling Invalid Input
    ---------------------

    To further refine the game's behavior, rather than crashing the program when
    the user inputs a non-number, let's make the game ignore a non-number so the user
    can continue guessing.

    */

    loop {

	println!("Please input your guess");
        let mut guess = String::new(); // Rust has a strong, static type system.

	/*
        However, it also has type inference. When we write

        let mut guess = String::new(), Rust was able to infer that guess should be a string
        and didn't make us write the type. The secret_number, on the other hand, is a number a type.

        A few of Rust's number types can have a value between 1 and 100:
        - i32, a 32-bit number
        - u32, an unsigned 32-bit number
        - i64, a 64-bit number

        */
	
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // handling potential failure with the result type

        println!("You guessed: {}", guess);
	
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

    // Working on the advent of code
    // Our task is to find the two entries that sum to 2020
    // read filename    

    let filename = String::from("/home/sang/Desktop/GIT/RustLibraries/guessing_game/src/input.txt");
    let filename2 = filename.clone(); // 
    let f = File::open(filename2);    
    let f = match f {
	Ok(file) => file,
	Err(error) => match error.kind() { // Depending on the type of error, return different things
	    ErrorKind::NotFound => match File::create(filename) { 
		Ok(fc) => fc,
		Err(e) => panic!("Problem creating the file {:?}", e),
	    },
	    other_error => {
		panic!("Problem opening the file: {:?}", other_error) 
	    }
	},

    };
    let contents = include_str!("/home/sang/Desktop/GIT/RustLibraries/guessing_game/src/input.txt")
	.split("\n");
    
    let contents2 = contents.clone();
    //println!("With text {}", contents);
    let vec = contents2.collect::<Vec<&str>>(); // convert the split string to a vector
    for elements in vec.iter() {
	println!("{}", elements);
    }    
    // https://doc.rust-lang.org/rust-by-example/flow_control/for.html
    for entry in contents {
	println!("{}", entry);
    }

    let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Text(String::from("blue")),
	SpreadsheetCell::Float(10.12),
    ];
    
}    



