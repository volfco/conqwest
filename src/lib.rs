//! # conqwest
//! A consul proxy library that handles service discovery and requests

extern crate log;

// mod tcp;
pub mod http;
pub mod consul;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
