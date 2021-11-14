use determinate::determinate;

#[determinate]
fn inputs(i: i32, j: i32) -> (i32, i32) {
    (i, j)
}

#[determinate]
fn testing() -> i32 {
    42
}

fn world() {
    println!("World!");
}

#[determinate]
fn hello() {
    println!("Hello,");
    world();
}

fn main() {
    hello();
    println!("{}", testing());
    println!("{:?}", inputs(1, 2));
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
