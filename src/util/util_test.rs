pub fn util_test_println(a: i32, b: i32) -> i32 {
    let x = a * b;
    println!("hello util_test_println!! result{}", x);
    return x;
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] //忽略运行
    fn it_works() {
        println!("it_works!!!");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn util_test_println_work() {
        println!("util_test_println_work!!! start==================");
        assert_eq!(600, util_test_println(20, 30));
        println!("util_test_println_work!!! end==================");
    }
}
