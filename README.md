![Rust](https://github.com/LiquidityC/rs-complete/workflows/Rust/badge.svg)
![Clippy check](https://github.com/LiquidityC/rs-complete/workflows/Clippy%20check/badge.svg)
![Security audit](https://github.com/LiquidityC/rs-complete/workflows/Security%20audit/badge.svg)
![Docs](https://docs.rs/rs-complete/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/LiquidityC/rs-complete/badge.svg?branch=master)](https://coveralls.io/github/LiquidityC/rs-complete?branch=master)

# rs-complete

rs-completion is a library to use when you want to implement tab-completion (or similar)
in your project.

rs-completion is mainly built for memory efficiency. Completions are stored in binary trees
where each node holds a number of characters. The characters in turn link to new nodes. Similar
words will thusly share memory.

## Visual example

```rust
//                          'c' - 'a' - 'v' - 'e'
//                         /
//    root - 'b' - 'a' - 't' - 'm' - 'a' - 'n'
//                               \
//                                'o' - 'b' - 'i' - 'l' - 'e'
```


This means that a worst case scenario you could have 25^25 nodes in memory where 25 is the size
of your alphabet. But this would mean that you are holding every thinkable combination of
characters in memory with no regards for consonant or verb rules. If this is what you need then
you don't need a library for it.

I can't argue if this solution is fast or efficient. It has worked to solve the problem 
I intended to solve when I created the library. If you have ideas for extensions or
improvements I'm happy to see them.

## Example
```rust
extern crate rs_completion;
use rs_completion::CompletionTree;

let mut completions = CompletionTree::default();

completions.insert("large bunch of words that bungalow we want to be bundesliga able to complete");
assert_eq!(
    completions.complete("bun"),
    Some(vec!["bunch", "bundesliga", "bungalow"].iter().map(|s| s.to_string()).collect()));
```
