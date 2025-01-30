//! # Allocating
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
    /// use xssan::allocating::*;
    /// let a = "<script>alert(0);</script>".to_string().sanitize();
    /// ```
    fn sanitize(&mut self);
}
impl AllocatingSanitizer for String {
    fn sanitize(&mut self) {
        *self = self.replace('<', "&lt;").replace('<', "&rt;");
    }
}


pub fn remove_html_tags<T:Into<String>>(input:T)->String{
    // we will search for the first < and than remove everything between it and >
    let mut start:Option<usize>=None;
    let mut it:usize = 0;
    let mut input:String =input.into();
    while it<input.len() {
        match input[it..(it+1)].as_ref() {
            "<"=>{
                if start==None {
                    start = Some(it);
                }
            }
            ">"=>{
                if let Some(loc) = start {
                    input.drain(loc..(it+1));
                    it=loc;
                    start = None;
                }
            }
           _=>{} 
        }
        it+=1;
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
        assert_eq!("hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_3() {
        let s = "<h1<p>>hi!</h1>".to_string();
        assert_eq!("hi!".to_string(), remove_html_tags(s));
    }
    #[test]
    fn remove_html_tags_4() {
        let s = "<h1 onclick=\"alert(0)\">hi!</h1>".to_string();
        assert_eq!("hi!".to_string(), remove_html_tags(s));
    }
}

