/*

Last Updated: 08/03/2022
------------------------

Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing 
their absence 

Useful Links:
------------

-> https://fasterthanli.me/series/advent-of-code-2020/part-1 - advent of code 

-> https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

-> https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters

-> https://doc.rust-lang.org/std/macro.include_str.html

-> https://stackoverflow.com/questions/40455997/iterate-over-lines-in-a-string-including-the-newline-characters

-> https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html  - object orientation in Rust 


-> https://os.phil-opp.com/ - building an OS in rust 

--

As a project grows, you can organize code by splitting it into multiple modules and then 
multiple files. A package can contain multiple binary crates and optionally one library crate. 
As a package grows, you can extract parts into separate crates that become external 
dependencies. 


In addition to grouping functionality, encapsulating implementation details lets you 
reuse code at a higher level; once you've implemented an operation, other code can 
call that code via the code's public interface without knowing how the implementation works

A related concept is scope; the nested context in which code is written has a set of names 
that are defined as 'in scope'. When reading, writing and compiling code, programmers and compilers
need to know whether a particular name at a particular spot refers to:

- variable
- struct 
- enum 
- module 
- constant 


Rust has a number of features that allow you to manage your code's organization, including which
details are exposed, which details are private, and what names are in each scope in your programs. 
These features, sometimes collectively referred to as the module system, include:

- packages
- crates
- Modules
- Paths 


*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


use rand::Rng;
use std::cmp::Ordering;
use std::io;
//use std::env;
use std::fs;

fn main() {

    let user1 = User {
	email: String::from("sangyoung123@googlemail.com"),
	username: String::from("something"),
	active: true,
	sign_in_count: 1, 
    };

    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    // Result's variants are Ok or Err. The ok variant indicates the operation was successful,
    // and inside ok is the successfully generated value. The Err variant means the operation
    // failed, and Err contains information about how or why the operation failed

    // Generating a secret number

    /*

    Next, we need to generate a secret number that the user will try to guess. The secret
    number should be different every time so the game is fun to play more than once.

    We'll use a random number between 1 and 100 so the game isn't too difficult. Rust doesn't yet
    include a random number functionality in its standard library. However, Rust does provide
    a rand crate with said functionality.

    ---

    Cargo's coordination of external crates is where Cargo really shines. Before we can write
    code that uses rand, we need to modify the Cargo.toml file to include the rand crate
    as a dependency. Open the file now and add the following line to the bottom
    beneath the [dependencies] section header that Cargo created for you.

    ---

    Cargo has a mechanism that ensures that you can build the same artifact every time you
    or anyone else builds your code: Cargo will use only the versions of the dependencies
    you specified until you indicate otherwise.

    When you build a project for the first time, Cargo figures out all the versions of the
    dependencies that fit the criteria and then writes


     */

    /*

    Comparing the Guess to the Secret number
    ---------------------------------------

    Now that we have user input and a random number, we can compare them. That step is shown
    below

     */

    // matching the guess

    // shadowing lets us reuse the guess variable name rather than forcing us to create
    // two unique variables, such as guess_str and guess for example .

    // We bind this new variable to the expression guess.trim().parse(). The guess
    // in the expression refers to the original variable that contained the input
    // as the string

    // The trim method on a String instance will eliminate any whitespace at the beginning
    // and end, which we must do to be able to compare the string to the u32, which can only
    // contain numerical data.

    // The parse method will only work on characters that can logically be converted
    // into numbers so can easily cause errors.

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
    let filename2 = filename.clone();
    //    let contents = fs::read_to_string(filename)
    //.expect("Something went wrong reading the file");
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

}


/*

How to write tests
------------------

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. 
The bodies of test functions typically perform these three actions 

1. Set up any needed data or state.

2. Run the code you want to test. 

3. Assert the results are what you expect. 



 */


/*
/// Iterator yielding every line in a string. The line includes newline character(s).
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
 */

//fn parse(input: &str) {
//}
