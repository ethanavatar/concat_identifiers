#[cfg(test)]
mod tests {
    use concat_identifiers::concat_identifiers;

    #[test]
    fn it_works() {
        fn what_function() -> &'static str {
            return "This function";
        }

        assert_eq!(concat_identifiers!(what, _function)(), "This function");
    }
}
