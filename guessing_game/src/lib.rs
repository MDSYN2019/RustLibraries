/*

Defining Modules to Control Scope and Privacy 
---------------------------------------------

We'll talk about modules and other parts of the module system, namely paths that 
allow you to name items; the use keyword that brings a path into scope.

--

en
*/

enum IpAddrKind {
    V4,
    V6, 
}
mod front_of_house {
    mod hosting {
	
	fn add_to_waitlist() {

	}

	fn seat_at_table() {

	}
    }


    mod serving {

	fn take_order() {}

	fn serve_order() {}

	fn take_payment() {}
	
    }
}