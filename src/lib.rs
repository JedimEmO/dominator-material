#![feature(async_closure)]

#[macro_use]
extern crate futures_signals;

pub mod components;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
