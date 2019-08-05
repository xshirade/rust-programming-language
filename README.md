# Rust Programming Language

Learning ["Rust Programming Language"](https://doc.rust-lang.org/book/)

## Work Summary

| Chapter                                                            |    Date    | Comment |
| ------------------------------------------------------------------ | :--------: | ------- |
| CH01 - Getting Started                                             | 2019/08/02 |         |
| CH02 - Programming a Guessing Game                                 | 2019/08/02 |         |
| CH03 - Common Programming Language                                 | 2019/08/03 |         |
| CH04 - Understanding Ownership                                     |     -      |         |
| CH04-01 - What is Ownership?                                       | 2019/08/03 |         |
| CH04-02 - References and Borrowing                                 | 2019/08/03 |         |
| CH04-03 - The Slice Type                                           | 2019/08/03 |         |
| CH05 - Using Structs to Structure Related Data                     | 2019/08/03 |         |
| CH06 - Enums and Pattern Matching                                  | 2019/08/04 |         |
| CH07 - Managing Growing Projects with Packages, Crates and Modules | 2019/08/04 |         |
| CH08 - Common Collections                                          | 2019/08/05 |         |
| CH09 - Error Handling                                              | 2019/08/dd |         |
| CH10 - Generic Types, Traits, and Lifetimes                        | 2019/08/dd |         |
| CH11 - Writing Automated Tests                                     | 2019/08/dd |         |
| CH12 - An I/O Projects: Building a Command Line Program            | 2019/08/dd |         |
| CH13 - Functional Language Features: Iterators and Closures        | 2019/08/dd |         |
| CH14 - More abour Cargo and Crates.io                              | 2019/08/dd |         |
| CH15 - Smart Pointers                                              | 2019/08/dd |         |
| CH16 - Fearless Concurrency                                        | 2019/08/dd |         |
| CH17 - Object Oriented Programming Features of Rust                | 2019/08/dd |         |
| CH18 - Patterns and Matching                                       | 2019/08/dd |         |
| CH19 - Advanced Features                                           | 2019/08/dd |         |
| CH20 - Final Project: Building a Multithreaded Web Server          | 2019/08/dd |         |
| CH21 - Appendix                                                    |     -      |         |

## Notes

The following pictures are from ["Rust Programming Language"](https://doc.rust-lang.org/book/).

### CH04-02: References

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

![references](./img/f_4_5.png)

### CH04-03: Slices

```rust
fn main() {
    let s = String::from("hello wolrd");
    let hello = &s[..5];
    let world = &s[6..];
}
```

![slices](./img/f_4_6.png)
