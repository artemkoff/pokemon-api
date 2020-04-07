mod models;
mod api;
mod error;

pub use error::Error;
pub use error::Result;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
