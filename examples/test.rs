use determinate::determinate;
use futures::Future;

#[inline]
fn wrapper<T>(f: fn() -> T) -> impl Future<Output = T> {
    async move { f() }
}

#[determinate]
fn testing() -> i32 {
    42
}

#[determinate]
fn test123() {
    println!("Hello, World!")
}

fn main() {
    test123();
    println!("{}", testing());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = testing();
        assert_eq!(result, 42);
    }
}
