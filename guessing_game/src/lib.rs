
mod front_of_house;
/*

Defining Modules to Control Scope and Privacy 
---------------------------------------------

We'll talk about modules and other parts of the module system, namely paths that 
allow you to name items; the use keyword that brings a path into scope.

Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control
the privacy of items, which is whether an item can be used by outside code (public) or is an internal implmentation 
detail 

--

We define a module by starting with the mod keyword and then specify the name of the module 
and place curly brackets around the body of the module. Inside modules, we can have other modules, 
as in this case with the modules hosting and serving 

By using modules, we can group related definitions together and name why they're related. 
Programmers using this code would have an easier time finding definitons they wanted to use 
because they could navigate the code based on the groups rather than having to read through 
all the definitions.


*/


// Paths for referring to an item in the module tree - to show rust where to find an item
// in a module tree, we use a path in the same way we use a path when navigating a filesystem.

// If we want to call a function, we need to know its path.

/*

Choosing whether to use a relative or absolute path is a decision you'll make based on your
project. The decision should depend on whether you're more likely to move item defintiion 
code separately from or together with the code that uses the item. 

In the absolute path, we start with crate, the root of our crate's module tree. Then the 
front_of_house module is defined in the crate root. The front_of_house module isn't public

 */


/*
https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html

Separating Modules into Different files
---------------------------------------

So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to 
move their definitions to a separate file to make the code easier to navigate.



*/
enum IpAddrKind {
    V4,
    V6, 
}

/*

Starting relative paths with super 
----------------------------------

We can also construct relative paths that begin in the parent module 
by using super at the start of the path. 

*/

// impl - https://doc.rust-lang.org/std/keyword.impl.html

// The impl keyword is primarily used to define implemntation on types. Inherent
// implementations are standalone, while trait implmentations are used to implmentation trait
// for types, or other trait 


fn serve_order() {}

mod back_of_house {

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
	super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
