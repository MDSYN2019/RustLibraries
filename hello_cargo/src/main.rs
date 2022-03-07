/*

Last Updated: 07/03/2022
------------------------

*/

use std::path::PathBuf;
use structopt::StructOpt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


fn main() {

    let mut user1 = User {
	email: String::from("something@example.com"),
	username: String::from("someuser123"),
	active: true,
	sign_in_cout: 1, 
    };

    /*
    The variable s refers to a string literal, where the value of the string 
    is hardcoded into the text of our program. The variable is valid from the point 
    at which it's declared until the end of the current scope. 

    
     */
    
    let s = "hello";
    
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    

    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world"); // push_str() appends a literal to a string
    println!("{}", s); 
}

/* 

Ownership and Functions
-----------------------

The semantics for passing a value to a function are similar to those for assigning a value to a 
variable. Passing a variable 

 */

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

