// 声明外部函数
extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "foo::my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

// 定义 foo 模块
mod foo {
    // 使用 #[no_mangle] 确保函数名不被修改
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a + 1
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 由于是外部导入的函数，使用 unsafe 块调用
        unsafe {
            let result = my_demo_function_alias(1);
            assert_eq!(result, 2);

            // 调用 my_demo_function
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
