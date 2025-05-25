/// Returns a greeting message for the given name.
///
/// # Examples
///
/// ```
/// use hello_rust::greet;
///
/// let message = greet("World");
/// assert_eq!(message, "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub trait Foo {}
pub trait Bar {}

impl<T: Foo> Bar for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
        assert_eq!(greet("World"), "Hello, World!");
    }
}
