// bring in the io library
use std::io;
// bring random number library
// the Rng trait must be in scope to use random generator
use rand::Rng;
// bring in a Ordering enum
// it has types "Less", "Greater", "Equal"
use std::cmp::Ordering;

fn main() {
    // print using macro
    println!("guess the number!!!");
    println!("type quit to stop playing");

        // generate a random number to guess
        // "thread_rng()" will generate the random number
        // "start..=end" is a range expression, inclusive of start and end
        // "secret_number" is immutable by default

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // this creates an infinite loop
    loop { 
        println!("enter your guess: ");

        // rust variables are immutable by default
        // and have to be declared mutable explicitly "mut"
        // variable to read a line
        // String::new() returns a new empty instance of a string
        // which is growable, UTF-8 encoded
        let mut guess = String::new();

        // read_line() will append to the string and not overwrite it
        // & -> a reference and modifies the actual variable and not a copy of it
        // &guess would mean passing an immutable reference of guess
        // to be able to modify it, pass a mutable reference - "&mut guess"

        // read_line will modify the string but also return and Enum calles Result
        // this Enum can be like = "Ok" or "Err"
        // the ".expect()" will handle the Result in case of Err
        // all Results must be handled, unhandled ones give warning
        io::stdin()
            .read_line(&mut guess)
            .expect("couldn't read the line");

        
        println!("your guess: {guess}");

        // guess is a string right now, but we want it to be a i32
        // the random number is also an i32 and this will cause mismatch
        // so typecast guess to an integer
        // also this declaration will shadow the previous guess variable
        // from this point on, guess is an integer (unsigned 32 bit)
        // "trim()" will remove whitespaces
        // "parse()" will convert it to u32 using type annotation on left side
        // the match will try to process the Enum returned by parse
        // if the string was successfully coverted to a number, that u32 is returned
        // if there is an error, the loop just continues and will not execute further code
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not really a number, but ok...");
                continue;
            },
        };

        // rust packages are called crates
        // they should be defined in the "Cargo.toml" under dependencies
        // "Cargo.lock" file will store all the versions for crates for reproducible builds

        // secret_number is a reference
        // match returns a reference of "Ordering" type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }
   }

}
