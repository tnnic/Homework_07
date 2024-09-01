pub mod macros_declarative;
pub use my_crate_macros::my_macro;
#[cfg(test)]
mod tests {
    use super::*;

    fn fo() -> i32 {
        10
    }

    #[allow(dead_code)]
    fn foo() -> i32 {
        20
    }

    fn fooo() -> i32 {
        30
    }

    #[test]
    fn test_call_even_length_functions() {
        let (fo_result, fooo_result) = my_macro!("fo", "foo", "fooo");
        assert_eq!(fo_result, 10);
        assert_eq!(fooo_result, 30);
    }
    #[test]
    fn test_my_macro_with_all_even_length_names() {
        let fo_result = my_macro!("fo");
        assert_eq!(fo_result, 10);
    }

    #[test]
    fn test_my_macro_with_no_even_length_names() {
        let result = my_macro!("foo");
        // Если ни одна функция не была вызвана, возвращаемый кортеж должен быть пустым
        assert_eq!(result, ());
    }

    #[test]
    fn test_my_macro_with_mixed_names() {
        let fo_result = my_macro!("fo", "foo");
        assert_eq!(fo_result, 10);
    }
}
#[cfg(test)]
mod tests2 {
    use super::*;

    pub fn foo() -> i32 {
        1
    }

    pub fn bar() -> i32 {
        2
    }

    pub fn baz() -> i32 {
        3
    }

    #[test]
    fn test_call_functions() {
        let (foo_result, bar_result, baz_result) = call_functions!(foo, bar, baz);
        assert_eq!(foo_result, 1);
        assert_eq!(bar_result, 2);
        assert_eq!(baz_result, 3);
    }
    #[test]
    fn test_my_macro_with_two_functions() {
        let (foo_result, bar_result) = call_functions!(foo, bar);
        assert_eq!(foo_result, 1);
        assert_eq!(bar_result, 2);
    }

    #[test]
    fn test_my_macro_with_one_function() {
        let (foo_result,) = call_functions!(foo);
        assert_eq!(foo_result, 1);
    }
}
