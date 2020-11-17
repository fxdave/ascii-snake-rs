/// Initializes a linked list with the given arguments
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate linked_list_macro;
/// let list = linked_list![1,2,3,4,];
///
/// let mut iter = list.iter();
/// assert_eq!(iter.next(), Some(&1));
/// assert_eq!(iter.next(), Some(&2));
/// assert_eq!(iter.next(), Some(&3));
/// assert_eq!(iter.next(), Some(&4));
/// ```
#[macro_export]
macro_rules! linked_list {
    ($($expression:expr,)*) => {
        {
            use std::collections::LinkedList;
            let mut body = LinkedList::new();
            $(
                body.push_back($expression);
            )*
            body
        }
    };
}
