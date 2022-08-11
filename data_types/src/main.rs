fn main() {
    // signed 32 bit integer
    // stored in 2s complement
    let x : i32 = 6;
    // gives error as x is immutable
    // x = 7;
    // mtable signed integer
    let mut y: i32 = 45;
    y = 32;
    // 64 bit floating point number
    // stored using IEEE-754 standard representation
    let mut g: f64 = 65.878770;
    println!("{}", x+1);
    println!("{y}");
    println!("{g}");

    // basic character type
    let z: char = 'c';
    println!("{z}");

    // compound types
    // tuple will not grow or shrink after bound to a value
    let tuple_same: (i32, i32, i32) = (2, 4, 8);
    // but tuple value types need not be same
    let tuple_mix: (i32, i64, f32, f64, char) = (45, 23232423, 56.90, 4567.8723181238, 'x');

    // "destructuring" a tuple
    let (v1, v2, v3) = tuple_same;
    println!("the second value of tuple_same: {v2}");

    // can also access tuple values using "." notation
    println!("third value of tuple_mix: {}", tuple_mix.2);

    // arrays - all values of same type and do not grow or shrink at runtime
    // let var_name: [type_name; size] = [val1, val2, ...];
    // allocated on the stack
    let a: [i32; 5] = [1,2,3,4,5];

    // initialize an array with the following initial value
    let b = [0; 5]; // an array of 5 i32's initialized to zero = [0,0,0,0,0]

}
