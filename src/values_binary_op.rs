//给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
pub fn single_number1(arrs: &[i32]) -> i32 {
    let mut sum = 0;
    for val in arrs.iter() {
        sum = sum ^ val;
    }
    return sum;
}

//给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现了三次。找出那个只出现了一次的元素。
pub fn single_number2(arrs: &[i32]) -> i32 {
    let mut res = 0;

    for i in 0..32 {
        let mut sum = 0; //二进制位 1的个数
        for val in arrs.iter() {
            sum += (val >> i) & 0x01;
        }
        res += (sum % 3) << i;
    }

    return res;
}

//给定一个整数数组  nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。
//0001      //0001
//0001      //0001
//0100      //0100
//0010      //0010
// ^  >0110 // ^  >0110  //获取二进制异或和  最后一个位置1的值 可有特性
pub fn single_number3(arrs: &[i32]) -> (i32, i32) {
    let mut sum = 0;
    for val in arrs.iter() {
        sum ^= val;
    }

    let sum_last = (sum - 1) ^ sum;

    let mut res = (sum, sum); //(x^y,x^y)
    for val in arrs.iter() {
        if sum_last & val == 0 {
            //这个位的值 可以把  值区分成两组 其中一组有且只有一个
            res.0 ^= val;
        } else {
            res.1 ^= val;
        }
    }

    return res;
}

//编写一个函数，输入是一个无符号整数，返回其二进制表达式中数字位数为 ‘1’  的个数（也被称为汉明重量）。
pub fn hamming_weight(mut num: u32) -> u32 {
    let mut res = 0;
    while num != 0 {
        num &= num - 1;
        res += 1;
    }
    return res;
}
use std::mem;
//给定一个非负整数  num。对于  0 ≤ i ≤ num  范围中的每个数字  i ，计算其二进制数中的 1 的数目并将它们作为数组返回。
////直接解
pub fn count_bits(num: u32) -> Vec<u32> {
    let mut arr: Vec<u32> = Vec::new();
    for i in 0..num + 1 {
        arr.push(hamming_weight(i))
    }
    println!("array occupies {} bytes", mem::size_of_val(&arr));

    return arr;
}
////动态规划？？？

//颠倒给定的 32 位无符号整数的二进制位。
pub fn reverse_bits(num: u32) -> u32 {
    let mut res: u32 = 0;
    for i in 0..32 {
        res = res << 1;
        res += (num >> i) & 0x01;
        // println!("{:#b},{}", res, i);
    }
    return res;
}
//给定范围 [m, n]，其中 0 <= m <= n <= 2147483647，返回此范围内所有数字的按位与（包含 m, n 两端点）。
//5: 0101
//6: 0110
//7: 0111

pub fn range_bitwise_and(mut m: u32, mut n: u32) -> u32 {
    let mut move_bit = 0;
    while m != n {
        m >>= 1;
        n >>= 1;
        move_bit = move_bit + 1;
    }

    return m << move_bit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_test() {
        let arrs = [1, 1, 2, 2, 3, 3, 4, 4, 5];
        assert_eq!(5, single_number1(&arrs));
    }

    #[test]
    fn single_number2_test() {
        let arrs = [1, 1, 1, 2, 2, 2, 5, 5, 5, 4, 7, 7, 7, 8999, 8999, 8999];
        assert_eq!(4, single_number2(&arrs));
    }

    #[test]
    fn single_number3_test() {
        let arrs = [1, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8];
        assert_eq!((1, 2), single_number3(&arrs));
    }
    #[test]
    fn hamming_weight_test() {
        let num = 0b111001;
        assert_eq!(4, hamming_weight(num));
    }
    #[test]
    fn count_bits_test() {
        let arr = count_bits(3);
        println!("{:?}", arr);
    }
    #[test]
    fn reverse_bits_test() {
        assert_eq!(0x80000000, reverse_bits(0b0000000000000001));
    }

    #[test]
    fn range_bitwise_and_test() {
        assert_eq!(0, range_bitwise_and(1, 2147483647));
    }
}
