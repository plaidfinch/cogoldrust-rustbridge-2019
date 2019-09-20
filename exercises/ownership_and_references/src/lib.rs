// Problem 1
// 1 star
// Write function pick_longest which picks the longests of two string-ish
// objects. Please make the function as general as possible (i.e do not
// take "String" as a parameter).
//
// From simplicity return a new String, later we will learn how to return
// references. Write additional tests.
//

#[test]
fn pick_longest_tests() {
    assert_eq!(pick_longest(& "cat".to_string(), & "dog".to_string()), "cat");
}



// Problem 2
// 1 star
// Why does the following implementation not work as expected?
// Fix by changing the type signature of add1 and the way it's called on add1_test().
// do NOT change the return type.

#[test]
fn add1_test() {
    let mut x = 1;
    add1(x);
    assert_eq!(x, 2);
}

fn add1(mut x : i32) -> () {
    x += 1;
}

// Problem 3
// 1 star
// Error says: cannot assign to immutable borrowed content `*str1`
// But we declared it mutable? Fix by changing only the line below.
fn mut2() {
    let hello = String::from("hello");

    // CHANGE ONLY THIS LINE:
    // let mut str1: & String = & String::from("str1");

    // *str1 = hello;
}

// Problem 4
// 1 star
// What went wrong? Copy strings properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1;
    assert_eq!(str1, str2);
}

// Problem 5
// 2 star
// Now we know how to implement this type of function. Implement it and write a tests.
// fn pick_longest2(s1: & str, s2: & str) -> & str{
    // unimplemented!()
// }


// Bonus Problem.
// Problem 6.
// Create functions split_ref  such that
// all the following tests will pass. Feel free to use Rust's split method
// https://doc.rust-lang.org/std/primitive.str.html#method.split
// as needed.

// split_ref must have the return type Vec<&str>

#[test]
fn split_ref_tests(){
    let string = "Hello World!".to_string();
    assert_eq!(split_ref(& string), ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), & ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), vec!["Hello", "World!"]);
}
