use std::any;
use std::fmt;

/// Prints the type of the given item.
pub fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

pub fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq<U> + From<U>,
    U: fmt::Display + PartialEq<T>,
{
    if a == b {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b);
    }
}
