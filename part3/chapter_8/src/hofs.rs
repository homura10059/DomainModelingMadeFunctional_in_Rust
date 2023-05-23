fn plus3(x: i32) -> i32 {
    x + 3
}

fn times2(x: i32) -> i32 {
    x * 2
}

fn square(x: i32) -> i32 {
    x * x
}

fn eval_with_5_than_add_2(f: impl Fn(i32) -> i32) -> i32 {
    f(5) + 2
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn add_generator(number_to_add: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| -> i32 { add(x, number_to_add) }
}

fn say_greeting(greeting: &str, name: &str) {
    println!("{} {}", greeting, name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_with_5_than_add_2_plus3() {
        let result = eval_with_5_than_add_2(plus3);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_eval_with_5_than_add_2_square() {
        let result = eval_with_5_than_add_2(square);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_add_generator() {
        let add_10 = add_generator(10);
        let add_10_result = add_10(5);
        assert_eq!(add_10_result, 15);
    }

    #[test]
    fn test_say_greeting() {
        let say_hello = |name: &str| say_greeting("Hello", name);
        let say_goodbye = |name: &str| say_greeting("Goodbye", name);
        say_hello("homura");
        say_goodbye("homura");
    }
}
