
pub fn remove_brackets<T:Into<String>>(input:T)->String{
    let input:String = input.into();
    input.replace('<', "").replace('>', "")
}
