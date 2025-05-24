/// This function will remove all left an right brackets.
/// Therefore the html tags will no longer be interpreted as html
/// ## Performance
/// This function is one of the fastest. But because it requires to copy the entire string it is
/// not adviced to be used on large strings.(Large string is above 100k characters.)
/// This function is O(n).
/// # Examples
/// ```
/// use xssan::fast::*;
/// assert_eq!("h1hello/h1", remove_brackets("<h1>hello</h1>"));
///
/// ```
pub fn remove_brackets<T: Into<String>>(input: T) -> String {
    let input: String = input.into();
    input
        .chars()
        .filter(|x| !(*x == '<' || *x == '>'))
        .collect::<String>()
}
#[cfg(test)]
mod test_fast {
    use super::*;

    #[test]
    fn sanitize_string_0() {
        assert_eq!("h1hello/h1", remove_brackets("<h1>hello</h1>"));
    }
    #[test]
    fn sanitize_string_1() {
        assert_eq!("h1hello/h1", remove_brackets("<<<<<h1>>>>>hello</h1>"));
    }

    // Test case for remove_brackets with an empty string.
    #[test]
    fn remove_brackets_empty_string() {
        assert_eq!("", remove_brackets(""));
    }

    // Test case for remove_brackets with a string with no brackets.
    #[test]
    fn remove_brackets_no_brackets() {
        assert_eq!("hello world", remove_brackets("hello world"));
    }

    // Test case for remove_brackets with a string with only brackets.
    #[test]
    fn remove_brackets_only_brackets() {
        assert_eq!("", remove_brackets("<<<>>>"));
    }

    // Test case for remove_brackets with alternating brackets and characters.
    #[test]
    fn remove_brackets_alternating_brackets_chars() {
        assert_eq!("abcd", remove_brackets("<a<b>c<d>>"));
    }

    // Test case for remove_brackets with consecutive brackets at the beginning.
    #[test]
    fn remove_brackets_consecutive_at_beginning() {
        assert_eq!("abc", remove_brackets("<<<abc"));
    }

    // Test case for remove_brackets with consecutive brackets at the end.
    #[test]
    fn remove_brackets_consecutive_at_end() {
        assert_eq!("abc", remove_brackets("abc>>>"));
    }

    // Test case for remove_brackets with consecutive brackets in the middle.
    #[test]
    fn remove_brackets_consecutive_in_middle() {
        assert_eq!("abcd", remove_brackets("ab<<<>>>cd"));
    }

    // Test case for remove_brackets with brackets and special characters.
    // Note: '&', '!', etc., are not brackets and should remain.
    #[test]
    fn remove_brackets_with_special_chars() {
        assert_eq!("&hello!", remove_brackets("<&hello!>"));
    }

    // Test case for remove_brackets with numbers and symbols mixed with brackets.
    #[test]
    fn remove_brackets_numbers_symbols_mixed() {
        assert_eq!("123!@#456", remove_brackets("<123!@#><456>"));
    }

    // Test case for remove_brackets with a string that contains only opening brackets.
    #[test]
    fn remove_brackets_only_opening_brackets() {
        assert_eq!("hello", remove_brackets("<<<hello"));
    }

    // Test case for remove_brackets with a string that contains only closing brackets.
    #[test]
    fn remove_brackets_only_closing_brackets() {
        assert_eq!("hello", remove_brackets("hello>>>"));
    }

    // Test case for remove_brackets with Unicode characters and brackets.
    #[test]
    fn remove_brackets_unicode_chars() {
        assert_eq!("你好世界", remove_brackets("<你好><世界>"));
    }
}
