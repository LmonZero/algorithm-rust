//给定一个经过编码的字符串，返回它解码后的字符串。 s = "3[a]2[bc]", 返回 "aaabcbc". s = "3[a2[c]]", 返回 "accaccacc". s = "2[abc]3[cd]ef", 返回 "abcabccdcdcdef".
pub fn decode_string(s: String) -> String {
    let (mut p, mut q, mut ans, mut t) = (Vec::new(), Vec::new(), "".to_string(), 0);

    for c in s.chars() {
        match c {
            '0'..='9' => {
                t = c as usize - 48;
            }
            '[' => {
                p.push(ans);
                q.push(t);
                ans = "".to_string();
                t = 0;
            }
            ']' => {
                ans = p.pop().unwrap() + &ans.repeat(q.pop().unwrap());
            }
            _ => {
                ans.push(c);
            }
        }
    }
    ans
}

//波兰表达式计算 > 输入: ["2", "1", "+", "3", "*"] > 输出: 9  ((2 + 1) * 3) = 9
pub fn eval_prn(arr: &[char]) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for c in arr.iter() {
        let mut val1 = 0 as f64;
        let mut val2 = 0 as f64;
        if c == &'+' || c == &'-' || c == &'*' || c == &'/' {
            val1 = stack.pop().unwrap();
            val2 = stack.pop().unwrap();
        }

        match c {
            '+' => {
                stack.push((val1 + val2) as f64);
            }
            '-' => {
                stack.push((val1 - val2) as f64);
            }
            '*' => {
                stack.push((val1 * val2) as f64);
            }
            '/' => {
                stack.push((val1 / val2) as f64);
            }
            _ => stack.push((c.clone() as i16 - 48) as f64),
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_string_test() {
        assert_eq!(
            "accaccacc",
            decode_string(String::from("3[a2[c]]")).to_string()
        );
    }

    #[test]
    fn eval_prn_test() {
        let arr = ['2', '1', '+', '3', '*'];
        // println!("{}", eval_prn(&arr));
        assert_eq!(9 as f64, eval_prn(&arr));
    }
}
