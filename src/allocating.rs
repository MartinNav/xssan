pub fn sanitize_string<T:Into<String>>(input:T)->String{
    let a:String = input.into();
    a.replace('<', "&lt").replace('>', "&rt")
}
pub trait AllocatingSanitizer {
    fn sanitize(&mut self);
}
impl AllocatingSanitizer for String{
    fn sanitize(&mut self){
        *self=self.replace('<', "&lt").replace('<', "&rt");
    }
}
