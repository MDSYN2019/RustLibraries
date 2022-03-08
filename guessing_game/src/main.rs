/*

Last Updated: 08/03/2022
------------------------

The io library comes from the standard library

*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
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
}
