
//! ----------------------
//! Author: Sang Young Noh
//! ----------------------
//! 
//! ------------------------
//! Last Updated: 11/04/2022
//! ------------------------
//! 
//! Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing 
//! their absence. 
//! 
//! Rosetta Stone code - http://rosettacode.org/wiki/Rosetta_Code
//! 
//! - Rosetta stone tasks not implemented in rust - http://rosettacode.org/wiki/Reports:Tasks_not_implemented_in_Rust
//! 
//! Useful Links:
//! ------------
//! 
//! -> https://fasterthanli.me/series/advent-of-code-2020/part-1 - advent of code 
//! -> https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
//! -> https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters
//! -> https://doc.rust-lang.org/std/macro.include_str.html
//! -> https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters
//! -> https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html  - object orientation in Rust 
//! -> https://os.phil-opp.com/ - building an OS in rust 
//! -> https://doc.rust-lang.org/book/ch09-00-error-handling.html - error handling in rust 
//! -> https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/ - blockchain app in rust 
//! -> https://robert.kra.hn/posts/2021-02-07_rust-with-emacs/
//!

#![allow(unused_variables)]

//extern crate itertools;
//extern crate itertools_num;

// Using linspace 
use itertools_num::linspace;
 
use std::fs; // filesystems? 
use std::io; 
use std::env;
use std::fs::File;
use std::io::ErrorKind; 
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::time::{Duration, Instant};
use std::error::Error;

use rand; 
use rand::prelude::*;
use rand::{random, Rng};
use num::complex::Complex;
use core::mem::swap;
use ndarray::array; // 

// importing back_of_house module::AveragedCollection
use SangMD::HartreeFock::HFDataset; // Hartree fock 
use SangMD::misc::User; // User 
use SangMD::MandelBrot::*; // Random MandelBrot class 


/*
Define types
*/


fn printLoop(value: &Vec<i32>) {
    let valueClone = value.clone(); // Why are we cloning the values here????
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

/*

'a represents the generic type 
*/

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

// -----

//type FILETYPE = String;

/*
What methods are and described how to make use of them in Rust 
*/

// Read computational chemistry type work??
static mut ERROR : isize = 0;

//#[derive(Debug)]
//fn one_in(n: u32) -> bool {
//    rand::thread_rng().gen_weighted_bool(n)
//}


fn parse_log(line: &'static str) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect(); // collect consumes an iterator and returns Vec<T>
    if parts.len() == 1 {
	return (Event::Unknown, String::from(line))
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
	"UPDATE" | "update" => (Event::Update, rest),
	"DELETE" | "delete" => (Event::Delete, rest),
	_ => (Event::Unknown, String::from(line)),
    }
}


impl FILETYPE {
    /*
    Description 
    -----------
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
    fn read(f: &mut FILETYPE, save_to: &mut Vec<u8>) -> Result<usize, String> { // Result<T, E> -> T is an integer type usize, and E is string. Using Sting
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

/*
Two points were raised discussing dissatissfaction with being unable to 
properly signify errors: 

- There was no attempt at implementing read() 

- open() and close() returns bools 

*/

fn one_in(denominator: u32) -> bool {      // <2>                                                                                                                                                         
  thread_rng().gen_ratio(1, denominator)   // <3>                                                                                                                                                        
}                                                                                                                                                                                                          

fn open(f:  FILETYPE) -> Result<FILETYPE, String> {
    if one_in(10_000) {
	let err_msg = String::from("Permission denied");
	return Err(err_msg);
    }
    Ok(f)	
}

fn close(f: FILETYPE) -> Result<FILETYPE, String> {
    if one_in(100_000) { // Once in 10000 executions, return an error 
	let err_msg = String::from("Interrupted by signal!");
	return Err(err_msg);
    }
    Ok(f)
}    



// Arrange all enums here

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),    
}

enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts, 
}

enum MessageNew {
    // Enum message can only take and get assigned these values.
    // Still not entirely certain what advantage this brings
    Quit,
    MoveInt {x: i64, y: i64},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)] // Forces the enum to be allowed to be printed in println! 
enum StatusMessage {
	Ok,
}

#[derive(Debug)]  // Allows the structure (whether it be enum/struct or whatever to be allowed to be printed out
enum Event {
    Update,
    Delete,
    Unknown,
}

// Arrange all structs here

#[derive(Debug)]
struct FILETYPE {
    name: String,
    data: Vec<u8>,
}

struct IntegerAdder {
    kind: SpreadsheetCell,
    address: String,
    numlist: Vec<i32>, 
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


pub struct LinesWithEndings<'a> {
    input: &'a str,
}
// Implement method 
impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings {
            input: input,
        }
    }
}

// Implement method 
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

// Cubsat part 
//fn check_status(sat_id: u64) -> StatusMessage {
//    // Check the status of the satellite in question 
//    StatusMessage::Ok
//}

type Message = String;

struct GroundStation;

#[derive(Debug)]
struct CubeSat { // Defining a separate Cubesat type 
    id: u64, // identifier for the specific Cubsat
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    //StatusMessage::Ok // Return ok 
    println!("{:?} {:?}", sat_id, StatusMessage::Ok);
    sat_id // The ownership goes back to the original variable instead of being passed to sat_id
	   // as we are returning the value 
}


fn main() {

    /*
    
    Implementing a Mock CubeSat Ground Station
    ------------------------------------------
    Type Safety 

     */
    
    // Indexing the Cubsat satellites, I think  
    //let sat_a = 0;
    //let sat_b = 1;
    //let sat_c = 2;

    let sat_a = CubeSat { id: 0, mailbox: Mailbox{ messages : vec![] } };
    let sat_b = CubeSat { id: 1, mailbox: Mailbox{ messages: vec![] } };
    let sat_c = CubeSat { id: 2, mailbox: Mailbox{ messages: vec![] } };
  
    // Created the CubeSat structs with differing identifiers, then checking the status 
    // Check the status of the satellites - At this current stage, we only print that the status is ok 

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

   // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // waiting 
    // Will produce an error with moved values
    
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    
    // Running a sample slow calculation
    let SampleSlowCalculation = simulated_expensive_calculation(3);
    let StructureFilename = String::from("/home/sang/Desktop/GIT/ProgrammingProjects/Project#01/input/acetaldehyde.dat");
    let mut f = BufReader::new(File::open("/home/sang/Desktop/GIT/ProgrammingProjects/Project#01/input/acetaldehyde.dat").unwrap());
    // Read the first line and extract the number from it
    
    let coordinateInformation = fs::read_to_string(StructureFilename);

    let xy = match coordinateInformation {
	// ???? 
	Ok(content) => content,
        Err(error) => {panic!("Could not open or find file: {}", error);}
    };
    
    let xyzpairs: Vec<&str> = xy.trim().split("\n").collect();

    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    let mut z: Vec<f32> = Vec::new();
    
    println!("{:?}", xyzpairs);
    
    // We are making from the defined types in the enum -- hance can only take certain values
    
    let NewEntryFromMessage = MessageNew::MoveInt {x: 10, y: 10};
    //NewEntryFromMessage::Write = "sd";
    
    
    let mut LinspaceData = linspace::<f32>(0., 1., 5); 
    for i in LinspaceData {
	println!("{}", i);
    }
    
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42,
		    132, 429, 1430, 4862];

    for item in &haystack {
	let result = match item {
	    42 | 132 => "hit",
	    _ => "miss", 
	};
	println!("{}, {}", item, result);
    }
    
    // panic!("crash and burn");
    // Using structs
    // Hashmaps    
    let user1 = User {
	email: String::from("sangyoung123@googlemail.com"),
	username: String::from("something"),
	active: true,
	sign_in_count: 1, 
    };

    user1.printOutline();
    
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
    
    let AveragedCollection1 = HFDataset {
	list: v.clone(),
	average: 0.0,
	name: "Sang".to_string(),
    };

    let user1 = User {
	email: String::from("sangyoung123@googlemail.com"),
	username: String::from("something"),
	active: true,
	sign_in_count: 1, 
    };


    MatchFunction();
    
    // Working on the advent of code
    // Our task is to find the two entries that sum to 2020
    // read filename    

    let filename = String::from("/home/sang/Desktop/GIT/RustLibraries/SangMD/src/input.txt");
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
    let contents = include_str!("/home/sang/Desktop/GIT/RustLibraries/SangMD/src/input.txt")
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



