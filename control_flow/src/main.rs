fn main() {
    let num: i32 = 17;

    if num < 6 {
        println!("num is lesser than 6");
    } else if num > 6 && num < 10 {
        println!("num is at least as much as 6, but less than ten");
    } else {
        println!("num is greater than 10");
    }

    // rust will not automatically convert non-boolean to boolean
    // if num {
    //     println!("this should not print");
    // }

    // if is an expression and can be used to assign values
    let cond: bool = true;
    let s = if cond { 5 } else { 6 };
    println!("the value of s: {s}");

    // both the arms of a conditional should have the same type
    // this gives an error as there is type mismatch
    // each expression in if-else should have the same type

    // let cond2: bool = true;
    // let g = if cond2 { 5 } else { "six" };
    // println!("the value of g: {g}");

    // "loop" is a type of infinite loop
    loop {
        println!("infinite");
        break; // remove to see infinite behaviour
    }

    // returning values from a loop
    let mut counter = 0;
    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{res}"); // prints 20

    // loop labels to distinguish loops
    // loop label has a single quote on it
    let mut f = 0;
    'outer_loop: loop {
        'inner_loop: loop {
            f += 100;
            if f == 100 {
                break 'outer_loop;
            }
        }
    }

    // conditional loops using while
    let mut a = 0;
    while a < 5 {
        println!("{a}");
        a = a + 1;
    }

    // while can be used with Collections
    let mut index = 0;
    let arr: [i32; 5] = [1,2,3,4,5];
    while index < 5 {
        println!("{} element of array is: {}", index+1, arr[index]);
        index += 1;
    }

    // for loop is also used with Collections
    for element in arr {
        println!("{element}");
    }

    // it can be used with Range types also
    // the range is exclusive of the last value
    // i = [1,3]
    for i in 1..4 {
        println!("value of i: {i}");
    }

    println!();

    // a reverse range is also possible
    println!("count down from 6");
    for i in (1..7).rev() {
        println!("{i}");
    }
}
