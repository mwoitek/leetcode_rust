use std::cmp;

impl Solution {
    pub fn intervals_overlap(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
        cmp::min(x2, y2) > cmp::max(x1, y1)
    }

    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let w1 = rec1[2] - rec1[0];
        let h1 = rec1[3] - rec1[1];

        let w2 = rec2[2] - rec2[0];
        let h2 = rec2[3] - rec2[1];

        Self::intervals_overlap(rec1[0], rec1[0] + w1, rec2[0], rec2[0] + w2)
            && Self::intervals_overlap(rec1[1], rec1[1] + h1, rec2[1], rec2[1] + h2)
    }
}
