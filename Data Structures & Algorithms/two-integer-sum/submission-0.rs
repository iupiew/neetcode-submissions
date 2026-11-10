impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (j, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&i) = seen.get(&complement){
                return vec![i as i32, j as i32];
            }

            seen.insert(num, j);
        }

        vec![]
    }
}