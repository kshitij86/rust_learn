fn main() {
    println!("in main");
    outer_fn();
    takes_arg(10);

    // statements in rust do not return a value
    let mut y = 6;
    // so these cannot be assigned to another variable
    // let z = (let u = 56); // gives error

    // an expression will return a value
    // a variable in a block is also an expression
    // Expressions do not include ending semicolons
    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    let d = {
            let g = 4;
            g+1
    };
    println!("value returned from expression: {d}");

    println!("value returned from addition: {}", sum_i32(4,5));
}

// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
fn outer_fn(){
    println!("in outer fn");
}

fn takes_arg(x: i32){
    println!("value of x: {x}");
}

fn sum_i32(a: i32, b: i32) -> i32{
    // expression so no semi-colon needed
    a + b
}