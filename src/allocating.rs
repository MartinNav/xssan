pub fn sanitize_string<T:Into<String>>(input:T)->String{
    let a:String = input.into();
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
impl AllocatingSanitizer for String{
    fn sanitize(&mut self){
        *self=self.replace('<', "&lt;").replace('<', "&rt;");
    }
}
#[cfg(test)]
mod tests_allocating {
    use super::*;

    #[test]
    fn sanitize_string_allocating() {
        let s = "<h1>hi!</h1>".to_string();
        assert_eq!("&lt;h1&rt;hi!&lt;/h1&rt;".to_string(),sanitize_string(s));
    }
}
