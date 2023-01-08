//! ----------------------
//! Author: Sang Young Noh
//! ----------------------
//!
//! ------------------------
//! Last Updated: 01/01/2023
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

/*
Rust's big features


- Performance
- Concurrency
- Memrory Efficiency

- Rust does not rely on a garbage collector to provide
its memory safety

The Rust community prefers to have a bigger language with a
compiler that does more, rather than a simpler language where the
compiler does less. The Rust compiler aggressively optimizes
both the size and speed of your program.

Cache-friendly data structures are provided by default. Arrays usually
hold data within rust programs, rather than deeply nested tree structures
that are created by pointers. This is referred to as data-orientated programming.

Haing a modern package manager avaliable makes it very easy to benefit from
the world's smartest programmers.

Methods are always dispatched statically, unless dispatch is explicitly requested.
This enables the compiler to heavilt optimize code, ometimes to to the point
of eliminating the cost of the function call entirely.

(dispatching means the act of sending something somewhere. There are two forms
of dispatch, static and dynamic. The former means that a call to a method
is resolved at compile time, and the latter means that the the dispatch is resolved
at run time.

When you have a variable of a base class that points to an instance of a derived class,
and you call a method that the child overrides, the call will be dispatvhed to the child.

Dynamic dispatch is the mechanism that allows polymorphic operations. Together with the this
pointer, these are the tools that were built on top of the structured languages to create the
object orientated languages.
)

Fearless concurrency - Concurrent and parallel programming has always been seen as difficult.
Rust frees you from whole classes of errors that have plagued its peer languages.


*/

#![allow(unused_variables)]

//extern crate itertools;
//extern crate itertools_num;

// Using linspace
use itertools_num::linspace;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs; // filesystems?
use std::fs::File;
use std::io;
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

/*
Define types

when you want to re-use container later in the program, use a reference. For reaosn that
are explained, when a reference is omitted, Rust will assume that container is no longer needed.
*/

fn print_loop(value: &Vec<i32>) {
    /*

    The ampersand around Vec ensures that we are working
    with a reference to a vector.

    */

    let value_clone = value.clone();
    for index in &value_clone {
        println!("{} \n", index)
    }
}

fn printLoop(value: &Vec<i32>) {
    let valueClone = value.clone(); // Why are we cloning the values here????
    for i in &valueClone {
        println!("{} \n", i)
    }
}

fn print_loop(value: &Vec<i32>) {
    let value_clone = value.clone();

    for i in &value_clone {
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
    fn vectorize(&self) -> Vec<Vec<f32>>;
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
static mut ERROR: isize = 0;

//#[derive(Debug)]
//fn one_in(n: u32) -> bool {
//    rand::thread_rng().gen_weighted_bool(n)
//}

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

/*
Two points were raised discussing dissatissfaction with being unable to
properly signify errors:

- There was no attempt at implementing read()

- open() and close() returns bools

*/

// Arrange all enums here

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

// Arrange all structs here

struct IntegerAdder {
    kind: SpreadsheetCell,
    address: String,
    numlist: Vec<i32>,
}

pub struct LinesWithEndings<'a> {
    input: &'a str,
}
// Implement method
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

/*
- Discovering what the term 'lifetime' means in the context of rust programming
- Working with the borrow checker rather than against it
- Multiple tactics for dealing with issues when they crop up
- understanding what the responsibilities of an 'owner' are
- learning how to 'borrow' values that are owned elsewhere

The borrow checker - checks that all access to data is 'legal'.

Learning to work with the borrow checker allows you to build larger
software systems with confidence. It underpins the term 'fearless concurrency'.

Example of simulating a satellite constellation. - ..?


Cubesats are minature artificial satellites that have increasingly increased the
acessibility of space research compared to conventiaonal satellite

A ground station is an intermediary between the operator and the satellites themselves. It
is listening on the radio, checking on the status of every satellite in the constellation
and transmitting messages to and fro. When introduced in our code, it will act as the gateway
between the user and satellites.



*/

type Message = String;
struct GroundStation;

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

    /*

    Every value in Rust is owned. In the listings, sat_a, sat_b and sat_c
    own the data that they refer to. When calls to check_status() are made,
    ownership of the data moves from variables in the scope of main()
    to the sat_id variable within the function. The significant difference is
    that the second examples places that integer within a cubesat struct. This type
    change alters the semantics of how the program behaves.

    ---

    Special behaviour of primitive types

    Before carrying on, it may be wise to explain why the first code even compiled in
    the first place. Indeed, the only change that we make was to wrap out satellite
    variable in a custom type. As it happens, primitive types in Rust have special
    behaviour.

    Types implementing Copy are duplicated at times that would otherwise be illegal.
    This provides some convenience day-to-day, at the expensve of adding a trap to newcomers.
    As you grow out from toy programming using integers, your code suddenly breaks.

    ! formally, primitive types are said to possess copy semantics.

    --

    In the world of rust, the notion of ownership is rather limited; An owner cleans up
    when its value's lifetime ends.

    When values go out of scope or their lifetimes end for some other reason,
    their desctructors are called. A desctructor is a function that remoes traces
    of the value from the program by deleting references and freeing memory. You won't find
    a call to any destructors in most rust code; the compiler injects that code itself as apart of the process of tracking every value's lifetime .


    To provide a custom destructor for a type, implement Drop. This will typically be needed
    in cases where you have used unsafe blocks to allocate memory. Drop has one method, Drop.

    ! An implication of this system is that values may not outlive their owner. This kind
    of situation can make data structures built with references such as trees and graphs, feel lsightly buearoucratic. If the root node of a tree is the owner of the whole tree, it can't be removed without taking ownership into account.



     */
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

    let sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_b = CubeSat {
        id: 1,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_c = CubeSat {
        id: 2,
        mailbox: Mailbox { messages: vec![] },
    };

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
    let StructureFilename = String::from(
        "/home/sang/Desktop/GIT/ProgrammingProjects/Project#01/input/acetaldehyde.dat",
    );
    let mut f = BufReader::new(
        File::open("/home/sang/Desktop/GIT/ProgrammingProjects/Project#01/input/acetaldehyde.dat")
            .unwrap(),
    );
    // Read the first line and extract the number from it

    let coordinateInformation = fs::read_to_string(StructureFilename);

    let xy = match coordinateInformation {
        // ????
        Ok(content) => content,
        Err(error) => {
            panic!("Could not open or find file: {}", error);
        }
    };

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

    let CoordinatesXY = Coordinates { X: 30.0, Y: 20.0 };

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

    fn read_file(file_name: &str) -> File {
        let filename = String::from(file_name);
        let f = File::open(filename);
    }
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
