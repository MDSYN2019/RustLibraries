//! ----------------------
//! Author: Sang Young Noh
//! ----------------------
//!
//! ------------------------
//! Last Updated: 11/02/2023
//! ------------------------
//!

/*
The Hartree-fock SCF method is the standard first-principles
approach for computing approximate quantum mechanical
eigenstates of interacting fermion systems.

Such systems include electons in atoms, molecules, and condensed matter.
Protons and neutrons in nuclei, and nuclear matter. The HF SCF method is
often a useful approximation by itself.

In more difficult situations, it serves as a foundation for more accurate
quantum many-body systems.


*/

#![allow(unused_variables)]

//extern crate itertools;
//extern crate itertools_num;

// file related imports

use itertools_num::linspace;
use std::fs::File;
use std::path::Path;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs; // filesystems?
use std::io;
use std::io::{self, BufRead};

use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use core::mem::swap;
use ndarray::array;

// using complex number types
use num::complex::Complex;
use rand::prelude::*;
use rand::{random, Rng}; //

// importing back_of_house module::AveragedCollection
use SangMD::misc::User; // User
use SangMD::HartreeFock::HFDataset; // Hartree fock
use SangMD::MandelBrot::*; // Random MandelBrot class

// All enum variables defined here

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug)] // Allows the structure (whether it be enum/struct or whatever to be allowed to be printed out
enum Event {
    Update,
    Delete,
    Unknown,
}

fn print_loop(value: &Vec<i32>) {
    /*

    */
    let value_clone = value.clone(); // get the cloned value
    for index in &value_clone {
        println!("{} \n", index) // for each value referenced in the index, print out the value index
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

//
pub trait generateVec {
    fn vectorize(&self) -> Vec<Vec<f32>>;
}

// trait with default return value implemented
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

static mut ERROR: isize = 0;

fn parse_log(line: &'static str) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect(); // collect consumes an iterator and returns Vec<T>
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

// Arrange all structs here

struct IntegerAdder {
    kind: SpreadsheetCell,
    address: String,
    numlist: Vec<i32>,
}

pub struct LinesWithEndings<'a> {
    input: &'a str,
}

// Implement method onto the struct - same as implementing a method onto a class (at least that is how I see it)
impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings { input: input }
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
        let split = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or(self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}

type Message = String; // new typedef
struct GroundStation; // make a struct describing the groundstation

/*
The derive attribute automatically creates the implementation
required to make this struct printable with fmt::debug
*/

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

struct Mailbox {
    messages: Vec<Message>,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok // Return ok
}

fn satellite_function() {
    // create a variable for each of them.

    let sat_a = CubeSat { id: 0 }; // ownership of the id 0 is within the cubesat object
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    //let a_status = check_status(a);
    //let b_status = check_status(b);
    //let c_status = check_statuc(c);

    let a_status = check_status(sat_a); // ownership changes from cubesat to check_status?
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    let a_status = check_status(sat_a); // ownership has already moved from cubsat, so this cubesat object can no longer claim access to the new variable
}

// Main executable function

fn open_xyx(geom: str) {
    /*
    docstring like explanation for this functionality need to go in here
    */
    let StructureFilename = String::from(geom);
    let coordinateInformation = fs::read_to_string(StructureFilename);

    let xy = match coordinateInformation {
        // this needs to be figured out now
        Ok(content) => content,
        Err(error) => {
            panic!("Could not open or find file: {}", error);
        }
    };
}

fn main() {
    let geomfile = "geom.xyz";

    // Running a sample slow calculation
    let SampleSlowCalculation = simulated_expensive_calculation(3);
    let xyzpairs: Vec<&str> = xy.trim().split("\n").collect();

    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    let mut z: Vec<f32> = Vec::new();

    println!("{:?}", xyzpairs);

    // We are making from the defined types in the enum -- hance can only take certain values

    let NewEntryFromMessage = MessageNew::MoveInt { x: 10, y: 10 };
    //NewEntryFromMessage::Write = "sd";

    let mut LinspaceData = linspace::<f32>(0., 1., 5);
    for i in LinspaceData {
        println!("{}", i);
    }

    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit",
            _ => "miss",
        };
        println!("{}, {}", item, result);
    }

    let user1 = User {
        email: String::from("sangyoung123@googlemail.com"),
        username: String::from("something"),
        active: true,
        sign_in_count: 1,
    };

    user1.printOutline();

    let CoordinatesXY = Coordinates { X: 30.0, Y: 20.0 };

    for coordinate in &CoordinatesXY.vectorize() {
        println!("{} {}", coordinate[0], coordinate[1]);
    }

    // let mut scores = HashMap::new();
    let mut scores: HashMap<String, i32> = HashMap::new(); // make a hashmap of a string and an integer
    HashMapRelatedFunction(&mut scores); // execute function known as Hashmaprelatedfunction to the
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Simple pushing of values onto a vector
    let mut v: Vec<i32> = Vec::new();
    // Adding elements to the vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

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

    // for the operation of opening the file, if the file opens correctly,
    // then we simply return the file. Otherwise, we define the error we
    // wish the compiler to spit out.

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Depending on the type of error, return different things
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    let contents =
        include_str!("/home/sang/Desktop/GIT/RustLibraries/SangMD/src/input.txt").split("\n");

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
