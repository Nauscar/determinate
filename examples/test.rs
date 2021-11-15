use determinate::{determinate, indeterminate};

#[determinate]
fn inputs(i: i32, j: i32) -> (i32, i32) {
    (i, j)
}

#[indeterminate]
fn random() -> i32 {
    rand::random()
}

#[determinate]
fn testing() -> i32 {
    random()
}

#[determinate]
fn hello() {
    println!("Hello, World!");
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
        let (i, j) = (1, 2);
        assert_eq!(inputs(i, j), (i, j));
    }
}
