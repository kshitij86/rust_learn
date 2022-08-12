fn main() {
    // each value in rust has an owner
    // there can be only one owner at a time
    // when the owner goes out of scope, the value is deleted
    // "String" is allocated on the heap
    let mut x: String = String::from("scoped variable");
    // x is available here
    x.push_str(", can be modified as well");
    println!("{x}");

    // string literals are different from String
    // the value of string literal is hardcoded into the final executable
    // so its size cannot be modified (it cannot be modified)


    // when a variable goes out of scope, Rust will call "drop" on it
    // to return its memory to the free memory pool

    // this simply creates two variables with the same value on the stack
    let x = 5;
    let y = x;

    // data that is allocated on the heap works differently
    // String for example has the following on the stack
    // {ptr: 0x5A45BCD, len: 5, capacity: 5}
    // len is in bytes
    // and the following on the heap, starting at 0x5A45BCD:
    // ['h', 'e', 'l', 'l', 'o']
    let my_str: String = String::from("hello");
    // this will copy the stack info to new_str and 
    // nothing is created on the heap
    // the string created on the heap is still referred to by both
    // for variables on the heap, the data is not copied, but a new reference 
    // is created on the stack
    // my_str is moved to new_str
    let new_str: String = my_str; // my_str is no longer valid
    // this creates a problem; when the closing } is reached
    // both my_str and new_str try to free the memory on the heap
    // this will lead to a "double free error" and is a memory safety bug
    // to avoid this, after "new_str = my_str", the variable my_str goes out of scope
    // below line in erroneous as my_str is no longer valid
    // println!("{}", my_str);
  
    // this is called creating a "shallow copy"
    // if we want to deep cop heap data, 
    // that is create a new copy on the heap as well, use "clone"
    let s1: String = String::from("will be deep copied");
    let s2: String = s1.clone();

    // here, s1 does not go out of scope
    println!("original: {}, copy: {}", s1, s2);

    // function ownership rules
    let some_str: String = String::from("I will be given to function");
    takes_ownership(some_str);
    // some_str unusable here

    // but stack values can be passed and will not go out of scope
    let some_int: i32 = 4567;
    creates_copy(some_int);
    // some_int still in scope here
    println!("{}", some_int);

    // the ownership can also be given for variables declared in a function
    let str_main: String = gives_ownership();
    // str_fn is now moved to str_main
    println!("{}", str_main);


} // end of scope of x
// drop is called here automatically

fn takes_ownership(some_str: String) {
    println!("{}", some_str);
}  // some_str goes out of scope here and is not available even in main


fn creates_copy(some_int: i32){
    println!("{}", some_int);
} // some_int is still out of scope here and this some_int is not the same as in main

fn gives_ownership() -> String{
    let str_fn: String = String::from("created in function, outside main");
    str_fn // return string
} // str_fn is out of scope, but if function value is stored in a varaible, that is still available on heap