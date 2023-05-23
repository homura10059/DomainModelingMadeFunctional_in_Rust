fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn add1(x: i32) -> i32 {
    x + 1
}

fn square(x: i32) -> i32 {
    x * x
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn print_bool(x: bool) {
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_1() {
        let add1_then_square = compose::<i32, i32, i32>(add1, square);
        let result = add1_then_square(5);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_compose_2() {
        let is_even_then_print_bool = compose::<i32, bool, ()>(is_even, print_bool);
        is_even_then_print_bool(5);
    }
}
