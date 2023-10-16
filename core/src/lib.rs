#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub mod inventory;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
