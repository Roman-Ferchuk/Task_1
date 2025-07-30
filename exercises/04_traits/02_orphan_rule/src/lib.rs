// error[E0117]: only traits defined in the current crate can be implemented for primitive types
//  --> src\lib.rs:7:1
//   |
// 7 | impl PartialEq for u32 {
//   | ^^^^^---------^^^^^---
//   |      |             |
//   |      |             `u32` is not defined in the current crate
//   |      `u32` is not defined in the current crate
//   |
//   = note: impl doesn't have any local type before any uncovered type parameters
//   = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
//   = note: define and implement a trait or new type instead