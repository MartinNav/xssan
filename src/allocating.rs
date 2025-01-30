pub fn sanitize_string<T:Into<String>>(input:T)->String{
    let a:String = input.into();
    a.replace('<', "&lt;").replace('>', "&rt;")
}
pub trait AllocatingSanitizer {
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
