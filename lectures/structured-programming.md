# Structured Programming in Rust

So far we've seen Rust's various primitive types and its syntax for loops,
conditionals, etc. Most programming languages, though, have higher-level
constructs that let us build complex data out of simpler data by grouping
smaller pieces of data together.

## Representation vs. Implementation

In object-oriented languages like Java or Python, we build complex data by
making *classes*. A *class* combines two things in one declaration:

- the *representation* of an object
- the *things we can do* with that object

A `Point` in Java:

```java
class Point {

    // Private fields
    int x;
    int y;

    // Constructor
    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    // Method
    public Point add(Point other) {
        return new Point(this.x + other.x, this.y + other.y);
    }

}
```

A `Point` in Python:

```python
class Point(object):

    # Constructor
    def __init__(self, x, y):
        self.x = x
        self.y = y

    # Method
    def add(self, other):
        return Point(self.x + other.x, self.y + other.y)
```

In Rust, we separate the *representation* of some data from the *things we can
do* with it.

```rust
// Type definition (and implicitly, how to construct it)
pub struct Point {
    x: i32,
    y: i32,
}

// Implementation of one or more methods
impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

Notice that although `add` takes one argument (the other point) when called, we
*define* it by giving it two arguments: the special argument `&self` that refers
to the struct on which the method is called, and the other struct `other` which
is given as a syntactic argument to the method. We'll see other uses of `self`
later.

## Publicity!

You might notice the keyword `pub` above, which we haven't discussed yet. When
something is marked `pub`, that means that thing is available outside the module
it was defined in. Choosing *not* to mark something `pub` is a way you can keep
your implementation of a type abstract -- so that other people who use your
definition can't mess with its implementation details in ways that would break
it. So, notice:

- We declare `Point` as `pub struct Point`, which means "public structure". If
  we had just said `struct Point`, the type definition would be private to the
  module we are in, and no other module would be allowed to use that type.

- We declare the `add` method as `pub fn add`, which similarly means that the
  `add` method can be used from other modules.

- We keep the fields `x` and `y` private, which means in another module nobody
  can say `p.x`. If they do, they'll get an error like:

  ```rust
  field `x` of struct `point::Point` is private
  ```

  This lets me make an important point (no pun intended!): Just because a
  `struct` is `pub` does not mean that its fields are -- or vice-versa.

### Quick Exercise: `struct` and `impl`

[TODO]

## Either/Or: Enums

Sometimes, data can be shaped in multiple different ways, and we want to
represent precisely the values the data can take on, and no extra values.
Suppose we are building a calendar and we want to represent the day of the week.
One way we could do this is to use a `String` for each day. In this example, we
want to determine the day that follows a given input day:

```rust
fn day_after(today: String) -> String {
    if today == "Monday" {
        "Tuesday"
    } else if today == "Tuesday" {
        "Wednesday"
    } else if today == "Wednesday" {
        "Thursday"
    } else if today == "Thursday" {
        "Friday"
    } else if today == "Friday" {
        "Saturday"
    } else if today == "Saturday" {
        "Sunday"
    } else if today == "Sunday" {
        "Monday"
    } else {
        unimplemented!("What should go here?")
    }.to_string()
}
```

In the above, we ran into a problem: because we were representing days of the
week as strings, there were inputs to our `day_after` function that did not
correspond to any valid day of the week. For those extra `String`s, we didn't
have a good way to finish writing the function, because there isn't a meaningful
`String` to represent the day following, e.g., `"8929h83hgfnsdznjhkjhnswgdflh"`.

Instead, a more idiomatic way to represent days of the week would be by
*pattern-matching* on an *enum type*:

```rust
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn day_after2(today: &Day) -> Day {
    match today {
        Day::Monday    => Day::Tuesday,
        Day::Tuesday   => Day::Wednesday,
        Day::Wednesday => Day::Thursday,
        Day::Thursday  => Day::Friday,
        Day::Friday    => Day::Saturday,
        Day::Saturday  => Day::Sunday,
        Day::Sunday    => Day::Monday,
    }
}
```

### Aside: Namespaces and `enum`s

Often, we will want to use the different options of an enum without
prefixing them with the enum they come from. We can import them at the top of
the file and simplify our definition:

```rust
use self::Day::*;

enum Day { ... }

fn day_after(today: &Day) -> Day {
    match today {
        Monday    => Tuesday,
        Tuesday   => Wednesday,
        Wednesday => Thursday,
        Thursday  => Friday,
        Friday    => Saturday,
        Saturday  => Sunday,
        Sunday    => Monday,
    }
}
```

### Quick Exercise: `enum` and `match`

Define a function which determines the color you get when you mix two primary
colors of paint together:

```rust
enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

enum SecondaryColor {
    Orange,
    Green,
    Purple,
}

enum AnyColor {
    Primary(PrimaryColor),
    Secondary(SecondaryColor),
}

fn mix_primary_colors(color_1: PrimaryColor, color_2: PrimaryColor) -> AnyColor {
    // Fill in here!
}
```

If you get frustrated with how much you're typing, hint: you can pattern-match
on tuples of values, like:

```rust
match (x, y) {
    (..., ...) => ...,
    (..., ...) => ...,
    ...
}
```

## Error Handling & `enum`s: `Option` and `Result`

Unlike in many other programming languages, in Rust, `enum`s can *contain
values*. For what seems to be a small alteration to the language, this turns out
to be a remarkably powerful feature!

While many languages let functions fail by throwing an exception, Rust chooses
instead to return special values from functions that could fail, indicating in
the type of those functions the possibility of failure.

The `Option` type represents the result of a function that could fail to return
any result. Since Rust does not have any `null` values, the only way for a
function to return no result is for it to use an `Option`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

In the above, notice that we used the new notation `<T>`. This is called a
*generic parameter*, and means that `Option`s can be made from any type. When we
use an `Option` type, we'll fill in the type `T`, like this:

```rust
fn divide_or_none(a: i64, b: i64) -> Option<i64> {
    if b == 0 {                      // T = i64
        None
    } else {
        Some(a / b)
    }
}
```

Unlike in languages with `null` in Rust we *have to* handle errors when they
could happen. So, if we use `divide_or_none`, we have to account for what
happens when the number couldn't be divided. The only way to find out what
happened is to use pattern matching!

```rust
fn print_quotient(a: i64, b: i64) {
    match divide_or_none(a, b) {
        None => println!("Couldn't divide!"),
        Some(r) => println!("{}", r),
    }
}
```

Sometimes it's nice to return an explanation of why something went wrong, rather
than just "sorry, nope." For those cases, we use `Result`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

This `enum` declaration tells us that `Result` has *two* of these generic type
parameters, `T` and `E`, where `T` is the type we want to return when all is
well, and `E` is the type of errors that could happen.

### When one case is the interesting one: `if let`

Frequently, we end up caring about one particular piece of an enum and not
caring about all the other cases it could be. If we always had to handle enums
with `match`, this would quickly become tedious because we'd still have to have
one match arm for each enum constructor. Luckily, we can use `if let` to express
this. Contrast:

```rust
match some_expression {
    None => { },
    Some(x) => println!("{}", x),
}

if let Some(x) = some_expression {
    println!("{}", x)
}
```

`Option` has only two constructors, but if there were more we would see even
more advantage.

### When Error-Handling Gets Clunky: `?`

Often, we end up in a situation where we want to do a bunch of things, each of
which could throw an error, and we want to continue only if none of the errors
happen. For instance:

```rust
fn write_hello(filename: &str) -> Result<(), io::Error> {
    match File::create(filename) {
        Err(e) => Err(e),
        Ok(mut file) => {
            match file.write_all("Hello, world!".as_bytes()) {
                Err(e) => Err(e),
                Ok(()) => {
                    println!("Wrote to file!");
                    Ok(())
                },
            }
        }
    }
}
```

Because you always have to handle every error, the more errors you have, the
more `match` statements you might have to write. But notice a pattern: every
match statement here is just returning whatever error it found! Because this
pattern is very common, Rust provides special syntax for dealing with it: the
`?` operator. Here's how we can rephrase the above using `?`:

```rust
fn write_hello(filename: &str) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all("Hello, world!".as_bytes())?;
    println!("Wrote to file!");
    Ok(())
}
```

When you write `?`, you're implicitly saying: "pattern match on this `Result`
type, returning the error if it was `Err(_)`, and continuing with the
non-erroneous value if it was `Ok(_)`. Anything you can write in terms of `?` is
just "syntactic sugar" for something you could write in terms of `match`--just
simpler and cleaner to read and write.

Another note: You can use `?` with `Option` as well, and it works much the same
way! Just keep in mind that you can't use `?` with both `Option` and `Result` in
the same function at the same time, since the types don't match.

### Longer Exercise: Stack Calculator

See the rust source for details.
