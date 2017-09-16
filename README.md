# WHAT (May 18th, 2015)

An update of Lindsey Kuper's excellent [Rust FizzBuzz](http://composition.al/blog/2013/03/02/fizzbuzz-revisited/) using Rust 1.x, as a way of taking some tentative first steps towards learning the language. 

The [`by_reference`](https://github.com/urschrei/rust_fizzbuzz/tree/by_reference) branch contains a working copy of the code which passes the results of the division operations to `in_to_rem` by reference, where they're destructured.

This is by no means the most efficient (or fastest) way to do it (see [Chris Morgan's explorations](http://chrismorgan.info/blog/rust-fizzbuzz.html) for more), but I find the implementation particularly intuitive, and I do what I feel like.
