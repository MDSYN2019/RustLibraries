/*

Last Updated: 07/03/2022
------------------------

Memory and Allocation

We can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time 
and whose size might change while running the program 





*/

//use std::path::PathBuf;
//use structopt::StructOpt;

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
	sign_in_count: 1, 
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

    let s1 = String::from("hello");
    let s2 = s1.clone(); // now we have a deep copy of the initial variable
    
    println!("{}", s1);

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

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn do_something(process_id: u64) -> Option<()> {

}
