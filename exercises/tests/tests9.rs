
extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"] 
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo {

    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {

        unsafe {
            let result = my_demo_function(123);
            assert_eq!(result, 124);

            let result_alias = my_demo_function_alias(456);
            assert_eq!(result_alias, 457);
        }
    }
}
