# Generics: when behavior is shared across types

We've already seen that types can have *generic parameters* that describe how
the type could contain any different type. Among the examples we've seen:

```rust
Box<T>
Vec<T>
Option<T>
Result<T, E>
...
```

Generic parameters on data types mean that we only have to describe the *shape*
of something once, because we can *parameterize* it by the things within that
shape. Imagine if you had to make an entirely separate `Vec` library for every
possible type that could be contained in a `Vec` -- and too bad if you come up
with one that the library authors didn't anticipate!

However, generic parameters on data types are only one half the story. In order
for us to take full advantage of generic data types, we need generic operations.
For instance, let's think about how we'd write down a `reverse` function for
`Vec`:

```rust
fn reverse(v: Vec<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    for e in v {
        output.push(e);
    }
    return output
}
```

But what about vectors of `String`? And vectors of `Option<char>`? And...?

Let's write it once and for all:

```rust
fn reverse<T>(v: Vec<T>) -> Vec<T> {
    let mut output = Vec::new();
    for e in v {
        output.push(e);
    }
    return output
}
```

Note that all we did was add a `<T>` to `reverse`, making it `reverse<T>`, and
then used that `T` in the rest of the signature (where `i64` used to be).

Okay, let's try some other things: how about `sort`? There are lots of things
you can sort, after all...

```rust
fn bubblesort<T>(slice: &mut [T]) {
    for i in 0 .. slice.len() {
        for j in i + 1 .. slice.len() {
            if slice[j] < slice[i] {
                slice.swap(i, j)
            }
        }
    }
}
```

Let's talk about slices again [review]

Seems legit...

```
error[E0369]: binary operation `<` cannot be applied to type `T`
 --> scratch.rs:4:25
  |
4 |             if slice[j] < slice[i] {
  |                -------- ^ -------- T
  |                |
  |                T
  |
  = note: `T` might need a bound for `std::cmp::PartialOrd`
```

Oh, right! Not *every* thing can be compared -- this function, although it could
apply to *many* types, can't apply to *all* of them.

This is where *traits* come in.

A trait is, in general, just a way to say "here is a collection of operations
that some number of types have in common." In this instance, the trait
`PartialOrd` represents the comparison operations necessary for the `<` operator
to *mean something*.

```rust
fn bubblesort<T>(slice: &mut [T]) where T: PartialOrd {
    for i in 0 .. slice.len() {
        for j in i + 1 .. slice.len() {
            if slice[j] < slice[i] {
                slice.swap(i, j)
            }
        }
    }
}
```

But what does `where T: PartialOrd` mean? It means "this function works for all
`T` so long as that type `T` has an ordering defined on it (that is, the `<`
operation works)."

Now that we fixed `bubblesort`, we can use it for all sorts of types:

```
let a: &mut [i64]  = &mut [-1, 200, -3, 400];
let b: &mut [u8]   = &mut [2, 0, 1, 9];
let c: &mut [bool] = &mut [false, true, false, true];
let d: &mut [char] = &mut ['r', 'u', 's', 't', 'b', 'r', 'i', 'd', 'g', 'e'];
let e: &mut [
bubblesort(a);
bubblesort(b);
bubblesort(c);
bubblesort(d);
println!("{:?}\n{:?}\n{:?}\n{:?}", a, b, c, d)
```

What about our own custom types?

```rust
enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}
```

```rust
use Season::*;
let times: &mut [(u64, Season)] = &mut [(2019, Autumn), (1993, Spring)];
bubblesort(times);
```

```
error[E0277]: can't compare `Season` with `Season`
  --> scratch.rs:21:5
   |
21 |     bubblesort(times);
   |     ^^^^^^^^^^ no implementation for `Season < Season` and `Season > Season`
   |
   = help: the trait `std::cmp::PartialOrd` is not implemented for `Season`
   = note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `(u6
4, Season)`
note: required by `bubblesort`
  --> scratch.rs:1:1
   |
1  | fn bubblesort<T>(slice: &mut [T]) where T: PartialOrd {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

But it's obvious how to compare them, they're an enum! But we need to tell Rust
that, because we *could* have wanted a different ordering.

```rust
#[derive(PartialEq, PartialOrd)]
enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}
```

However, since we want "the obvious thing" we can just `derive` it. You saw
earlier that we can `derive` Clone and Copy. Clone and Copy are traits just like
PartialOrd is. Some other traits we can derive:

- Comparison traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`.
- `Clone`, to create `T` from `&T` via a copy.
- `Copy`, to give a type 'copy semantics' instead of 'move semantics'.
- `Hash`, to compute a hash from `&T`.
- `Default`, to create an empty instance of a data type.
- `Debug`, to format a value using the `{:?}` formatter.

But let's talk traits in general, cause there are a lot more out there than just
the ones we can derive.

You can define your own traits:

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

trait Colorful {
    fn color(&self) -> Color;
}

impl Colorful for Color {
    fn color(&self) -> Color {
        self.clone()
    }
}

impl Colorful for bool {
    fn color(&self) -> Color {
        match self {
            true  => Color::Green,
            false => Color::Red,
        }
    }
}
```

```rust
trait Sortable {
    fn sort(&mut self);
}
```

```rust
impl Sortable for &mut [i64] {
    fn sort(&mut self) {
        bubblesort(self);
    }
}
```

Wait a second... this should work *for all T: PartialOrd*.

```rust
impl<T> Sortable for &mut [T] where T: PartialOrd {
    fn sort(&mut self) {
        bubblesort(self);
    }
}
```

That signature is a mouthful, but let's think about it for a second, because if
we *really understand* it, we'll have understood the fundamentals of:

- generics
- traits
- references
- mutability
- slices

Notice that we can only **ever** have one trait impl for a given type at the
same time -- it is never ever allowed to have multiple. This means you always
know exactly what `impl` you are going to be using, no matter where the trait
methods are called.
