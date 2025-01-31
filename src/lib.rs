pub mod allocating;
pub mod nonallocating;
/// # Examples
/// ```
/// use xssan::prelude::*;
/// // string with html
/// let a = "<h1>Title</h1><p> paragraph.</p>";
/// // this function removes all html tags
/// assert_eq!("Title paragraph.", remove_html_tags(a.clone()));
/// // this function will convert the html tags to be displayed as html tags. 
/// // But it will not be executed as html.
/// assert_eq!("&lt;h1&rt;Title&lt;/h1&rt;&lt;p&rt; paragraph.&lt;/p&rt;", sanitize_string(a));
///
/// ```
pub mod prelude{
    pub use crate::allocating::*;
}
