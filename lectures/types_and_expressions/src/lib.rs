#![allow(dead_code, unused_variables)]

// declare function like so.
fn machine_types() {
    let n: i8 = 0;
    let m: i16 = 0;
    let o: i32 = 0;

    // Machine dependent types
    let n: isize = 40;
    // Indexer!
    let i: usize = 40;

    // Introduce array.
    let array: [i32; 3] = [1, 2, 3];

    // Implicit sizes are not good enough for systems programming, where the size
    // in bytes matters.

    // Index array using usize:

    // let i: i32 = 1;
    array[i /*as usize*/]; // Use 'as' operator to convert between numerical types.

    // Chars and bools
    let b: bool = true;
    let c: char = 'c';
}

// On size and allocations.

// http://static.duartes.org/img/blogPosts/linuxClassicAddressSpaceLayout.png

// What lives in the stack: local variables and functions.
// Talk about compiler, and statically compiled information.
// Enough room is made for variables in the stack. This size must be known at compile time.
// Dynamic allocation, when the size is only known at runtime.

// Arrays and vectors.
fn array() {
    // Size is hardcoded for arrays.
    // Allocated in the stack
    let b: [i32; 3] = [1, 2, 3];
    let a = [1, 2, 3];


    // Instead, Vectors are dynamically allocated.
    // Talk about heap and stack, and "pointers".
    let mut v: Vec<i32> = vec![1, 2, 3]; // talk about mut!
    v.push(3);
}

fn references() {
    // A reference lives in the stack of the current function, but
    // may point to data in stacks "above" or somewhere in the address space.
    let x = 3;
    let r = &x;
    let y = &3;

    // More on when and why we use references later...
}

// Arrays, vectors, and slices.
fn slices() {
    let a = [1, 2, 3];
    let v = vec![1, 2, 3];

    // Take a reference to the address of the vector.
    // We call this a _slice_, notice the type has changed to
    // resemble an array.

    // Explain references!
    let av: &[i32] = &a;
    let sv1: &Vec<i32> = &v;
    let sv2: &[i32] = &v;

    // Arrays and Vector references kinda have the same memory layout...

    // So we can take references to both! (There is some automatic conversion from
    // &Vec<i32> to &[i32])...

    // How big are references?
    // Probably the size a pointer.

    // Accept a vector:
    // Write functions that can take either.
    fn print(s: &[i32]){
        unimplemented!()
    }

    // Same function prints both arrays and vector references.
    print(&v);
    print(&a);
}

fn strings() {
    // Where does this reference live
    // .data
    let s: &str = "rust";
    // ^ inmutable.

    // Heap allocated, string.
    // growable, modifyiable
    let s: String = String::from("rust");

    // Example in book
    let noodles: String = "noodles".to_string();

    // Take a string lice to a smaller part of a string.
    let oodles: &str = &noodles[1..];

    // On unsized data
    // let a: [i32] = [1, 2, 3];
    // let s: str = "rust"; // This is why we need to add a reference,
    // to give it a specific size!

    // Unsized types.
    // String are laid out in pretty much the same way as arrays.
    // Notice the analogies between strings and &str:
    // String ~~ Vec<T>
    // &str ~~ &[T]
    // str ~~ [T]
}

#[allow(dead_code, unused_variables)]
fn structured_types() {
    // Tuple
    // How are tuples different than arrays?
    // - Heterogenous Data
    // Number of elements is inmutable.
    let t = (5, "cat");
    // Tuple Accessor
    let num = t.0;
    let str1 = t.1;
}

// Heap allocations.
fn boxes() {
    // Rust puts structures in the stack. No dereference needed!
    let t = (12, "eggs");

    // This incurs a pointer dereference to access all data.
    // Other languages are slower for this, but at the same time,
    // users don't have to worry about these details! Everything
    // is a tradeoff...

    // When `b` goes of scope, memory is freed (unless moved)
    let b = Box::new(t);
}

fn unit_type() -> () {
    //  What is the return type of the print function?
    let r: () = println!("hello");

    // Type of this function?
    return ();
}

// What is the difference between an expression and a statement?

// ifs are expressions, not statements.
// What is the difference between an expression and statement?
fn if_expr(b : bool) {
    // The types of each sub-expression must match.
    let x = if b { 5 } else { 10 };
}


fn while_loop(b: bool) {
    while b {
        unimplemented!()
    }

    loop {
        // Do things.
    }
}

fn for_loop() {
    // Using ranges, exclusive
    for i in 0..20 {
        println!("{}", i);
    }

    // Inclusive
    for i in 0..=20 {
        println!("{}", i);
    }

    let v: Vec<&str> = vec!["cis", "198", "rust", "programming"];

    // Do not iterate over i with a index. Instead use iterators!
    for e in v {
        let e2: &str = e;
        // Do stuff with v
    }
}


// Type inference:
// Rust is able to infer the types of many of our expressions, making
// static types a lot easier to write!
fn build_vector_rough() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::new();
    v.push(10);
    v.push(20);
    v
}

// Question: Can we omit the type of the function? No

// Omiting types:
#[allow(dead_code)]
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

// Closures
fn make_closure() {
    let f = |x| x + 1;

    assert_eq!(f(3), 4);

}
