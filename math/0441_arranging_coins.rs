impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let mut i: i64 = 0;
        let mut t: i64 = 0;
        while t <= n {
            i += 1;
            t = (i * (i + 1)) / 2;
        }
        (i - 1) as i32
    }
}
