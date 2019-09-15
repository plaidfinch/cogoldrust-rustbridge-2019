#![allow(dead_code, unused_variables)]
// Ownership

fn memory() {
    // Talk about stack allocation and memory if it hasn't been mentioned yet.
    {
        let v = vec![1, 2, 3];  // Memory allocated here.
    } // Memory is dropped here.

    fn f(){
        let my_box = Box::new(5); // Explicit heap memory
    } // dropped here

    // dbg!(v) // vec is not in scope here.
}

fn ownership() {
    // n owns the memory for "5"
    let n = 5;
    // v owns the memory for the vector
    let v = vec![1, 2, 3];

    // So what happens when the data is assigned to a different variable?
    let v2 = v; // move!
}

fn move_semantics() {
    let v = vec![1, 2, 3];
    // Here the data is moved. It is no longer owned by v,
    // but instead by v2.
    let v2 = v;

    // What happens if we try to use after it has been moved?
    // dbg!(v);
    // Using moved values in an error.
}

fn making_copies_of_data() {
    // How do you make copies of data then?
    let v = vec![1, 2, 3];
    let v2 = v.clone(); // Make a copy by cloning the data.
    dbg!(v);
}

// As we saw assingment, moves values. So do function calls:
fn function_call() {
    fn print_elements(vec: Vec<i32>) {
        for e in vec {
            dbg!(e);
        }
    }

    let v = vec![1, 2, 3];
    print_elements(v);
    // dbg!(v);
    // We must use a reference here...
}

// Looping over data moves values.
fn looping_over_data_move() {
  let v = vec![1, 2, 3];
  for element in v {
      // do something with element...
  }
  // dbg!(v);
}

fn move_on_function_return() {
    fn make_vec() -> Vec<i32> {
        let v = vec![1, 2, 3];
        return v;
    }
    // data of v moved into v2
    let v2 = make_vec();
}

// Let's look at the same example but with integers:
fn move_with_ints() {
    let x: i32 = 5;
    let y = x;
    dbg!(x);

    // Does this example compile!?

    // It actually does... why?
    // Shouldn't /x/ be moved after being assigned to y?

    // Some types are special, they are marked as /Copy/.

    // It would be very tedious/annoying to have to /clone()/ things like
    // integers.

    // So Rust does automatic copying for several types.
}

// Copy Trait
// Traits define in a generic way what operations are allowed with trait.
// Clone (which we saw above is a trait), Copy is also a trait.
// Yes, you can define your own traits on types.


// How does Rust decide which types can be copied automatically?

// The idea of copying is that copying should be a very
// fast, lightweight operation. So things like bool, integers, can be
// copied, but things like String, and Vec<_> cannot be copied.

// Instead to make a copy of a String or Vec<_> you must explicitly
// call /clone()/. A vector or string could gigabytes big! So cloning
// is potentially a very expensive operation.


// Values moved into function, values moved out of function.
fn move_around(v: Vec<i32>) -> Vec<i32> {
    v
}

fn use_move_around() {
    let mut vec = vec![1, 2, 3];
    vec = move_around(vec);
    dbg!(vec);
}

// Some languages like C++ will implicitly copy values when needed.
// This can be very expensive and not obvious!

// Why Don't Other Languages Worry about Moves?

// Languages like Java and Python don't have a strict ownership models,
// instead multiple variables may have a reference to data.
// The language keeps track of how many people have a reference to the
// data, when no variable points to the data, the runtime system will
// clean up the data via the garbage collector.

// Why doesn't Rust just do what Java and Python do?
// There is a performance penalty to keeping track of references to
// data.
// The program must be paused to clean up data (garbage collect)

// Instead, by tracking moves Rust can always _statically_ determine the
// last use of some data, no garbage collection needed! No manual memory
// freeing needed!
