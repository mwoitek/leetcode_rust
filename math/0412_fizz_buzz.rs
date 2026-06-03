impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::<String>::with_capacity(n.try_into().unwrap());
        for i in 1..=n {
            let s = if i % 3 == 0 && i % 5 == 0 {
                String::from("FizzBuzz")
            } else if i % 3 == 0 {
                String::from("Fizz")
            } else if i % 5 == 0 {
                String::from("Buzz")
            } else {
                i.to_string()
            };
            answer.push(s);
        }
        answer
    }
}
