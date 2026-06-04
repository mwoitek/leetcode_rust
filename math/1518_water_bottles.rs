impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut cnt: i32 = 0;
        let mut bottles = num_bottles;
        let mut empty: i32 = 0;
        while bottles > 0 {
            cnt += bottles;
            empty += bottles;
            bottles = empty / num_exchange;
            empty %= num_exchange;
        }
        cnt
    }
}
