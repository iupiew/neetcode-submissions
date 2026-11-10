impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = heights.len() - 1;
        let mut best = 0;
        while left < right {
            let mut width = (right - left) as i32;
            let mut area = width * min(heights[left], heights[right]);
            best = max(best, area);
            
            if heights[left] < heights[right] {
                left += 1;
            } else {
                right -=1;
            }        
        }
        best
    }
}
