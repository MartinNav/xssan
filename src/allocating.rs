//! # Allocating sanitization functions
//! This module contains functions that may allocate more memmory and may increase the size of the
//! string containing the text being sanitized.

/// This funcion will replace all occurencies of `<` and `>` with `&lt;` and `&rt;` so they
/// will look when rendered by browser like regular versions of them self's but can not be used
/// to execute JS.
/// This function works on all types that implement the `Into<String>` trait.
/// # Examples
/// ```
/// use xssan::allocating::*;
/// let sanitized_string = sanitize_string("<p onclick=\"alert(0)\">hello there</p>");
/// ```
pub fn sanitize_string<T: Into<String>>(input: T) -> String {
    let a: String = input.into();
    a.replace('<', "&lt;").replace('>', "&rt;")
}
pub trait AllocatingSanitizer {
    /// This funcion will replace all occurencies of `<` and `>` with `&lt;` and `&rt;` so they
    /// will look when rendered by browser like regular versions of them self's but can not be used
    /// to execute JS
    /// # Examples
    /// ```
    /// //this string when appended to html will cause execution of malicious javascript
    /// let a = "<script>alert(0);</script>".to_string();
    /// // this one not
    /// use xssan::allocating::AllocatingSanitizer;
    /// let a = "<script>alert(0);</script>".to_string().sanitize();
    /// ```
    fn sanitize(&mut self);
}
impl AllocatingSanitizer for String {
    /// # Example
    /// ```
    /// use xssan::allocating::AllocatingSanitizer;
    /// let a = "<script>alert(0);</script>".to_string().sanitize();
    /// ```
    fn sanitize(&mut self) {
        *self = self.replace('<', "&lt;").replace('>', "&rt;");
    }
}

/// This function removes all html tags from the string.
/// As html tag is considered any substring starting with `<` and ending with `>`.
/// Everything in between these characters will be deleted.
/// # Examples
/// ```
/// use xssan::allocating::remove_html_tags;
/// // Removes the <b> tag from the input string.
/// let result = remove_html_tags("Hello <b>world!</b>");
/// assert_eq!(result, "Hello world!");
///
/// // Removes the <i> tag from the input string.
/// let result = remove_html_tags("This is an <i>bold text</i> example.");
/// assert_eq!(result, "This is an bold text example.");
/// ```
pub fn remove_html_tags<T: Into<String>>(input: T) -> String {
    // we will search for the first < and than remove everything between it and >
    let mut start: Option<usize> = None;
    let mut it: usize = 0;
    let mut input: String = input.into();
    while it < input.len() {
        match input[it..(it + 1)].as_ref() {
            "<" => {
                if start == None {
                    start = Some(it);
                }
            }
            ">" => {
                if let Some(loc) = start {
                    input.drain(loc..(it + 1));
                    it = loc;
                    start = None;
                    continue;
                }
            }
            _ => {}
        }
        it += 1;
    }
    input
}

#[cfg(test)]
mod tests_allocating {
    use super::*;

    #[test]
    fn sanitize_string_0() {
        let s = "<h1>hi!</h1>".to_string();
        assert_eq!("&lt;h1&rt;hi!&lt;/h1&rt;".to_string(), sanitize_string(s));
    }

    // Test case for an empty string.
    #[test]
    fn sanitize_string_empty_input() {
        let s = "".to_string();
        assert_eq!("".to_string(), sanitize_string(s));
    }

    // Test case for a string with no HTML tags.
    #[test]
    fn sanitize_string_no_tags() {
        let s = "hello world".to_string();
        assert_eq!("hello world".to_string(), sanitize_string(s));
    }

    // Test case for a string with only HTML tags.
    #[test]
    fn sanitize_string_only_html_tags() {
        let s = "<h1><h2><p></p></h2></h1>".to_string();
        assert_eq!("&lt;h1&rt;&lt;h2&rt;&lt;p&rt;&lt;/p&rt;&lt;/h2&rt;&lt;/h1&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with nested HTML tags.
    #[test]
    fn sanitize_string_nested_html_tags() {
        let s = "<h1><p>hello</p></h1>".to_string();
        assert_eq!("&lt;h1&rt;&lt;p&rt;hello&lt;/p&rt;&lt;/h1&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with incomplete tags.
    #[test]
    fn sanitize_string_incomplete_tags() {
        let s = "<p>hello".to_string();
        assert_eq!("&lt;p&rt;hello".to_string(), sanitize_string(s));
    }

    // Test case for a string with tags and special characters.
    #[test]
    fn sanitize_string_tags_and_special_chars() {
        let s = "<p>&amp;hello</p>".to_string();
        assert_eq!("&lt;p&rt;&amp;hello&lt;/p&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with multiple tags.
    #[test]
    fn sanitize_string_multiple_tags() {
        let s = "<p>hello</p><h1>world</h1>".to_string();
        assert_eq!("&lt;p&rt;hello&lt;/p&rt;&lt;h1&rt;world&lt;/h1&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with tags at the beginning and end.
    #[test]
    fn sanitize_string_tags_at_beginning_and_end() {
        let s = "<h1>hello</h1><p>world</p>".to_string();
        assert_eq!("&lt;h1&rt;hello&lt;/h1&rt;&lt;p&rt;world&lt;/p&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with mixed case tags.
    #[test]
    fn sanitize_string_mixed_case_tags() {
        let s = "<P>Hello</P>".to_string();
        assert_eq!("&lt;P&rt;Hello&lt;/P&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with numbers and symbols within tags.
    #[test]
    fn sanitize_string_numbers_and_symbols_in_tags() {
        let s = "<h1_!@#$>hello</h1_!@#$>".to_string();
        assert_eq!("&lt;h1_!@#$&rt;hello&lt;/h1_!@#$&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string that is already sanitized.
    // This test verifies that the sanitize_string function correctly handles strings that are already sanitized.
    // It ensures that existing HTML entities are not double-encoded.
    #[test]
    fn sanitize_string_already_sanitized() {
        let s = "&lt;p&rt;hello&lt;/p&rt;".to_string();
        assert_eq!("&lt;p&rt;hello&lt;/p&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with only opening tags.
    #[test]
    fn sanitize_string_only_opening_tags() {
        let s = "<p><h1><div>".to_string();
        assert_eq!("&lt;p&rt;&lt;h1&rt;&lt;div&rt;".to_string(), sanitize_string(s));
    }

    // Test case for a string with only closing tags.
    #[test]
    fn sanitize_string_only_closing_tags() {
        let s = "</p></h1></div>".to_string();
        assert_eq!("&lt;/p&rt;&lt;/h1&rt;&lt;/div&rt;".to_string(), sanitize_string(s));
    }

    #[test]
    fn remove_html_tags_0() {
        let s = "<h1>hi!</h1>".to_string();
        assert_eq!("hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_1() {
        let s = "<h1>>hi!</h1>".to_string();
        assert_eq!(">hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_2() {
        let s = "<h1<>>hi!</h1>".to_string();
        assert_eq!(">hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_3() {
        let s = "<h1<p>>hi!</h1>".to_string();
        assert_eq!(">hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_4() {
        let s = "<h1 onclick=\"alert(0)\">hi!</h1>".to_string();
        assert_eq!("hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_5() {
        let s = "<h1 onclick=\"alert(0)\"><p>hi!</p></h1>".to_string();
        assert_eq!("hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_6() {
        let s = "<h1<<<<<<<>>>>>>>hi!</h1>".to_string();
        assert_eq!(">>>>>>hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_7() {
        let s = "<<<<<<<hi!".to_string();
        assert_eq!("<<<<<<<hi!".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with an empty string.
    #[test]
    fn remove_html_tags_empty_string() {
        let s = "".to_string();
        assert_eq!("".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with a string with no HTML tags.
    #[test]
    fn remove_html_tags_no_tags() {
        let s = "hello world".to_string();
        assert_eq!("hello world".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with a string with only HTML tags.
    #[test]
    fn remove_html_tags_only_html_tags() {
        let s = "<h1><p></p></h1>".to_string();
        assert_eq!("".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with HTML tags and no content.
    #[test]
    fn remove_html_tags_tags_no_content() {
        let s = "<a></a>".to_string();
        assert_eq!("".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with incomplete tags at the end.
    #[test]
    fn remove_html_tags_incomplete_tag_at_end() {
        let s = "hello<world".to_string();
        assert_eq!("hello<world".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with incomplete tags at the beginning.
    #[test]
    fn remove_html_tags_incomplete_tag_at_beginning() {
        let s = "world>hello".to_string();
        assert_eq!("world>hello".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with only an opening tag.
    #[test]
    fn remove_html_tags_only_opening_tag() {
        let s = "<h1>".to_string();
        assert_eq!("".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with only a closing tag.
    #[test]
    fn remove_html_tags_only_closing_tag() {
        let s = "</h1>".to_string();
        assert_eq!("".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with self-closing tags.
    #[test]
    fn remove_html_tags_self_closing_tags() {
        let s = "<img src='test.png'/>".to_string();
        assert_eq!("".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with tags containing attributes with special characters.
    #[test]
    fn remove_html_tags_attributes_with_special_chars() {
        let s = "<a title='<hello>'>link</a>".to_string();
        assert_eq!("link".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with multiple nested and sibling tags.
    #[test]
    fn remove_html_tags_multiple_nested_sibling_tags() {
        let s = "<div><p><span>text1</span></p><b>text2</b></div>".to_string();
        assert_eq!("text1text2".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with tags and surrounding whitespace.
    #[test]
    fn remove_html_tags_tags_with_surrounding_whitespace() {
        let s = "  <p>hello</p>  ".to_string();
        assert_eq!("  hello  ".to_string(), remove_html_tags(s));
    }

    // Test case for remove_html_tags with mixed valid and invalid tags.
    #[test]
    fn remove_html_tags_mixed_valid_invalid_tags() {
        let s = "<p>valid</p><invalid>invalid content<p>more valid</p>".to_string();
        assert_eq!("validinvalid contentmore valid".to_string(), remove_html_tags(s));
    }

    // Test case for AllocatingSanitizer::sanitize with an empty string.
    #[test]
    fn allocating_sanitizer_sanitize_empty_string() {
        let mut s = "".to_string();
        s.sanitize();
        assert_eq!("".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with no HTML tags.
    #[test]
    fn allocating_sanitizer_sanitize_no_tags() {
        let mut s = "hello world".to_string();
        s.sanitize();
        assert_eq!("hello world".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with only HTML tags.
    #[test]
    fn allocating_sanitizer_sanitize_only_html_tags() {
        let mut s = "<h1><h2><p></p></h2></h1>".to_string();
        s.sanitize();
        assert_eq!("&lt;h1&rt;&lt;h2&rt;&lt;p&rt;&lt;/p&rt;&lt;/h2&rt;&lt;/h1&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with nested HTML tags.
    #[test]
    fn allocating_sanitizer_sanitize_nested_html_tags() {
        let mut s = "<h1><p>hello</p></h1>".to_string();
        s.sanitize();
        assert_eq!("&lt;h1&rt;&lt;p&rt;hello&lt;/p&rt;&lt;/h1&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with incomplete tags.
    #[test]
    fn allocating_sanitizer_sanitize_incomplete_tags() {
        let mut s = "<p>hello".to_string();
        s.sanitize();
        assert_eq!("&lt;p&rt;hello".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with tags and special characters.
    #[test]
    fn allocating_sanitizer_sanitize_tags_and_special_chars() {
        let mut s = "<p>&amp;hello</p>".to_string();
        s.sanitize();
        assert_eq!("&lt;p&rt;&amp;hello&lt;/p&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with multiple tags.
    #[test]
    fn allocating_sanitizer_sanitize_multiple_tags() {
        let mut s = "<p>hello</p><h1>world</h1>".to_string();
        s.sanitize();
        assert_eq!("&lt;p&rt;hello&lt;/p&rt;&lt;h1&rt;world&lt;/h1&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with tags at the beginning and end.
    #[test]
    fn allocating_sanitizer_sanitize_tags_at_beginning_and_end() {
        let mut s = "<h1>hello</h1><p>world</p>".to_string();
        s.sanitize();
        assert_eq!("&lt;h1&rt;hello&lt;/h1&rt;&lt;p&rt;world&lt;/p&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with mixed case tags.
    #[test]
    fn allocating_sanitizer_sanitize_mixed_case_tags() {
        let mut s = "<P>Hello</P>".to_string();
        s.sanitize();
        assert_eq!("&lt;P&rt;Hello&lt;/P&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with numbers and symbols within tags.
    #[test]
    fn allocating_sanitizer_sanitize_numbers_and_symbols_in_tags() {
        let mut s = "<h1_!@#$>hello</h1_!@#$>".to_string();
        s.sanitize();
        assert_eq!("&lt;h1_!@#$&rt;hello&lt;/h1_!@#$&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string that is already sanitized.
    // This test verifies that the sanitize method correctly handles strings that are already sanitized.
    // It ensures that existing HTML entities are not double-encoded.
    #[test]
    fn allocating_sanitizer_sanitize_already_sanitized() {
        let mut s = "&lt;p&rt;hello&lt;/p&rt;".to_string();
        s.sanitize();
        assert_eq!("&lt;p&rt;hello&lt;/p&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with only opening tags.
    #[test]
    fn allocating_sanitizer_sanitize_only_opening_tags() {
        let mut s = "<p><h1><div>".to_string();
        s.sanitize();
        assert_eq!("&lt;p&rt;&lt;h1&rt;&lt;div&rt;".to_string(), s);
    }

    // Test case for AllocatingSanitizer::sanitize with a string with only closing tags.
    #[test]
    fn allocating_sanitizer_sanitize_only_closing_tags() {
        let mut s = "</p></h1></div>".to_string();
        s.sanitize();
        assert_eq!("&lt;/p&rt;&lt;/h1&rt;&lt;/div&rt;".to_string(), s);
    }
}
