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
pub fn remove_brackets<T:Into<String>>(input:T)->String{
    let input:String = input.into();
    input.chars().filter(|x| !(*x=='<'||*x=='>')).collect::<String>()
}
#[cfg(test)]
mod test_fast{
    use super::*;

    #[test]
    fn sanitize_string_0() {
        assert_eq!("h1hello/h1", remove_brackets("<h1>hello</h1>"));
    }
    #[test]
    fn sanitize_string_1() {
        assert_eq!("h1hello/h1", remove_brackets("<<<<<h1>>>>>hello</h1>"));
    }
}
