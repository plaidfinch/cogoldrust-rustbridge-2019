#![allow(dead_code, unused_variables, unused_mut)]

// If variables can "own" data, then variables should also be able
// to borrow data right?

// Example from last lecture:
fn print_vec(vec: Vec<i32>) {
    for v in vec {
        println!("{}", v);
    }
}

fn use_print_vec() {
    let v = vec![1, 2, 3];
    print_vec(v);
    // dbg!(v);
}

// Use referenes to pass data around without consuming it.

fn reference_variables() {
    // A reference is a type just like any type.
    // We can make variables of type reference:
    let v = vec![1, 2, 3];
    // v_ref has _borrowed_ v.
    let v_ref: &Vec<i32> = &v;
}

// References are like "pointers" to data owned by someone else.
fn references_explained() {
    let n = 10;
    let r = &n; // types can be inferred by compiler

    // We use the * to access the data a reference points to.
    // this operation is known as a "dereference"
    let z = *r + 10;
}

fn push_vec() {
    // you're probably familiar with methods from other languages.
    let mut v = vec![1, 2, 3];
    v.push(4);
}

// Lets try making this into a function.
fn push_vec_ref(v: &mut Vec<i32>) {
    // (*v).push(4);
    (*v).push(4); // Rust automatically does dereference on methods.
}

// References are inmutable by default! We must make them mutable
// in order to change the data they point to.

fn using_mut_ref() {
    let mut v = vec![1, 2, 3];
    let vr: &mut Vec<i32> = &mut v; // Rust can guess type here as well.
    push_vec_ref(vr);
    // push_vec_ref(&mut v);
}

// We can even have multiple references to the same data!
fn multiple_refs() {
    let mut v = vec![1, 2, 3];
    let vr1 = &v;
    let vr2 = &v;

    // but we cannot have readonly and mutable references at the same
    // time!
    // let vr3 = &mut v;

    // rust is too smart, need to use it for rust to complain.
    // *vr3 = vec![1, 2, 3 , 4];
    // dbg!(vr1);
}
// How long to borrows live?
// Borrows live until the v_ref goes out of scope.

// This is where things get interesting, how do references interact with
// borrows.

// Main idea: You can move data around. But you cannot move something that
// someone else is borrowing!
fn borrows_and_moves() {
    let v = vec![1, 2, 3];

    // let v_ref: &Vec<i32> = &v;

    let v2 = v; // data moved to v2.
}

// lets try to trick Rust into having a hanging reference.
fn hanging_reference() {
    let v_ref: &Vec<i32>;
    {
        let v = vec![1, 2, 3];
        v_ref = &v;
    } // remeber that v goes out of scope here!

    // dbg!(v_ref);
}

// Can we make thi work.
// fn interesting_function() -> &i32 {
//     let n = 3;
//     return &n;
// }

// This concept is fundamental to Rust! It is known as _lifetimes_.
// Lifetimes dictate how long the data a reference, refers to, lives.

// Returning references from functions.
// Rust is able to tell that the returned reference should live as long
// as the passed in reference.
fn ret_str(s1: &str) -> &str {
    s1
}

// We can also make it explicit!
// This is should be read as s1 is a reference to str with lifetime a
fn find_greater_string_explicit<'a>(s1: &'a str) -> &'a str {
    s1
}

// When can't Rust figure it out?
// fn find_greater_str(s1: &str, s2: &str) -> &str {
//     s1
// }

// In this case it might be obvious to us that the lifetime is always s1.
// But in more complicated examples it might not be.
// So Rust needs our help to figure it out. Notice Rust doesn't just blindly
// trust us. It checks what we say!

// A lifetime defines how long a reference lives for. Which is based on how
// long the data lives for. Some data, lives for the lifetime of the entire
// program.

fn static_lifetime(){
    let s: &'static str = "hello"; // notice we can usually omit this.
}
