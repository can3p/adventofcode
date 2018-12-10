# Notes on Advent of Code 2018

Or, actually

# Notes on using Rust for implementation

* Fantastic vim integration
* Fantastic toolchain (`cargo` was always enough, `cargo test` just works,
  compiler messages are just amazing. `rustup` is magical, can manage rust
  version and install `rustfmt` or `clippy` in one command.
* Strings are done a proper way with regards to unicode. Wanna get list
  of characters in the string? Use `chars` iterator.
* Iterators are everywhere which makes a nice separation of converns, making
  it possible to map/filter on any datastructure that can give an iterator
* Any getter returns an `Option` making it explicit if you get the result
* The whole ownership model brings headaches sometimes, but in most of the
  cases fixes were quite staightforward
* Separation of concerns everywhere. E.g. any value that means an index of
  array or any other addressable datastructure has type `usize`, not `i32`
* Day 9 - fantastic difference in speed with and without using Vec::insert.
  It's even not 10x, more like 500x. Lesson - memory allocation matters.
