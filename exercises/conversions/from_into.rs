// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

/// 定义一个 `Person` 结构体，包含姓名和年龄两个字段
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
/// 为 `Person` 结构体实现 `Default` 特性，用于在字符串无法转换为 `Person` 对象时提供默认值
impl Default for Person {
    /// 返回一个默认的 `Person` 对象，姓名为 "John"，年龄为 30
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }
        let name = parts[0].trim();
        if name.is_empty() {
            return Person::default();
        }
        let age_str = parts[1].trim();
        let age = match age_str.parse::<usize>() {
            Ok(age) => age,
            Err(_) => return Person::default(),
        };
        Person {
            name: name.to_string(),
            age,
        }
    }
}

/// 程序入口点，演示如何使用 `From` 和 `Into` 特性
fn main() {
    // Use the `from` function
    // 使用 `from` 方法从字符串创建 `Person` 对象
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    // 由于 `Person` 实现了 `From` 特性，因此可以使用 `Into` 特性
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

/// 测试模块，包含多个测试用例，用于验证 `Person` 结构体的功能
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试默认的 `Person` 对象是否为 30 岁的 John
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    /// 测试当提供空字符串时，是否返回默认的 `Person` 对象
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当提供格式正确的字符串时，是否能正确创建 `Person` 对象
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    /// 测试当年龄无法解析时，是否返回默认的 `Person` 对象
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串中缺少逗号和年龄时，是否返回默认的 `Person` 对象
    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串中缺少年龄时，是否返回默认的 `Person` 对象
    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串中缺少姓名时，是否返回默认的 `Person` 对象
    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串中缺少姓名和年龄时，是否返回默认的 `Person` 对象
    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串中缺少姓名且年龄无法解析时，是否返回默认的 `Person` 对象
    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串末尾有逗号时，是否返回默认的 `Person` 对象
    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    /// 测试当字符串末尾有逗号和其他字符串时，是否返回默认的 `Person` 对象
    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
