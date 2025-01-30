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
        *self = self.replace('<', "&lt;").replace('<', "&rt;");
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
}
