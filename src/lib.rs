#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;

extern crate serde;

mod magnet;

pub use magnet::{Magnet};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() -> () {
        Magnet::from_file("screen.magnet").unwrap();
        ()
    }

    #[bench]
    fn bench_test1(b: &mut Bencher) {
        b.iter(|| {
            Magnet::from_file("screen.magnet").unwrap()
        });
    }
}
