# WHAT (May 18th, 2015)

An update of Lindsey Kuper's excellent [Rust FizzBuzz](http://composition.al/blog/2013/03/02/fizzbuzz-revisited/) using Rust 1.x, as a way of taking some tentative first steps towards learning the language. 

The [`by_reference`](https://github.com/urschrei/rust_fizzbuzz/tree/by_reference) branch contains a variant which passes the results of the division operations to `int_to_rem` by reference, where they're destructured. Note that this isn't necessarily more efficient or flexible, however, since `i32` is a primitive type, and thus implements the `Copy` trait.

This approach to solving `FizzBuzz` using Rust is by no means the fastest way to do it (see [Chris Morgan's explorations](http://chrismorgan.info/blog/rust-fizzbuzz.html) for more), but I find this implementation particularly intuitive, and well-suited to the language.
