// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// PONDER: what is `#[macro_use] extern crate` and `#[macro_export]`?
// READ: https://veykril.github.io/tlborm/
// READ: https://doc.rust-lang.org/1.29.0/book/first-edition/macros.html?highlight=macro_use#scoping-and-macro-importexport
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
