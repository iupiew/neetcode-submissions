impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in &nums {
            if set.contains(n) {
                return true;
            }
            set.insert(n);
        }
        return false;
    }
}
