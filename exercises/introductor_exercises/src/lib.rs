// Problem 1
// 1-star
// Implement the sum function on slices. Do not use the predefined sum function.
fn sum(slice: &[i32]) -> i32 {
    unimplemented!()
}

#[test]
fn sum_test_1(){
    assert_eq!(sum(&[1, 2, 3, 4]), 10);
}

// Write your own tests for sum! Run `cargo test` to run your tests.


// Problem 2.
// 2-star
// Make unique. Create a new vector which contains each item in the vector
// only once! Much like a set would.
// Please implement this using a for loop.
fn unique(vs: &Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

#[test]
fn unique_test_1(){
    // assert_eq!(unique(&vec![1, 1, 2], &vec![1, 2]));
}

// Problem 3
// 2-star
// Given starting fibonacci numbers n1 and n2, compute a vector
// where v[i] is the ith fibonacci number.
fn fibonacci(n1: i32, n2: i32, how_many: usize) -> Vec<i32> {
    unimplemented!()
}

#[test]
fn fibonaci_1(){
    // assert_eq!(fibonacci(1, 1, 5), vec![1, 1, 2, 3, 5]);
}
// Problem 4
// 2-star
// return a new vector containing only elements that satisfy `pred`.
fn filter(vs: & Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    unimplemented!()
}

#[test]
fn filter_tests(){
    // assert_eq!(filter(& vec![1, 2, 3, 4, 5, 6], & |n| n % 2 == 0),
              // vec![2, 4, 6]);
}
