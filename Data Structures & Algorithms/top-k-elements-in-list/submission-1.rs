use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new(); // Добавили mut, так как карта будет меняться
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }
        
        let mut frequent_vec: Vec<(i32, i32)> = map.into_iter().collect();       
        frequent_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        
        frequent_vec
            .into_iter()
            .take(k as usize)
            .map(|(num, _count)| num)
            .collect()
    }
}
