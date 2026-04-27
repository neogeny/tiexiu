// SPDX-License-Identifier: MIT OR Apache-2.0

use indexmap::{IndexMap, IndexSet};

pub type Ref<T> = Box<T>;
pub type Str = Ref<str>;
pub type StrSet = IndexSet<Str>;
pub type FlagMap = IndexMap<Str, bool>;
pub type Define = (Str, bool);
pub type DefineSet = IndexSet<Define>;

// pub struct Str(pub Rc<str>);
//
// impl Deref for Str {
//     type Target = str;
//     fn deref(&self) -> &str {
//         &self.0
//     }
// }
//
// // Now this works perfectly:
// fn print_len(s: &str) {
//     println!("{}", s.len());
// }
//
// let my_str = Str(Rc::from("TieXiu"));
// print_len(&my_str); // Coerces to &str
