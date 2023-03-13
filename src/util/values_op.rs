#[cfg(test)]
mod tests {
    // use super::*;

    #[test] //交换两个数
    fn value_exchange_test() {
        let mut a = 20;
        let mut b = 30;
        // a = a ^ a; //0
        // b = b ^ b; //0

        a = a ^ b; // a=a^b
        b = b ^ a; // b=b^a^b=a
        a = a ^ b; // a=a^b^a=b

        println!("a:{},b:{}", a, b);
    }
    #[test] //移除最后一个 1
    fn value_move_last_test() {
        let a = 0b1011;
        let b = (a - 1) & a;
        println!("a:{:#b},b:{:#b},c{:#b}", a, b, (b - 1) & b);
    }
    #[test] //获取最后一个 1
    fn value_get_last_test() {
        let a = 0b1010;
        let b = ((a - 1) & a) ^ a;
        println!("a:{},b:{}", a, b);
    }

    #[test] //获取最后一个 1
    fn sigle_number3_test() {
        let vals = [1, 1, 2, 2, 3, 3, 4, 4, 5, 6];
        let mut res = vals[0];
        for i in 1..vals.len() {
            res = res ^ vals[i];
        }
        //res= x^y //未知两个值的异或

        let mut tmp: i32 = res;
        for i in 0..vals.len() {
            tmp = tmp ^ vals[i];

            if tmp == 0 {
                println!("x:{},y:{},i:{}", vals[i], res ^ vals[i], i);
                break;
            }
        }
    }
}
