impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut numbers = nums;
        numbers.sort();

        let n = numbers.len();
        let mut triplets: Vec<Vec<i32>> = Vec::new();

        if n < 3 {
            return triplets;
        }

        for i in 0..n - 2 {
            // skip a repeated first value so we don't emit the same triplet twice
            if i > 0 && numbers[i] == numbers[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = numbers[i] + numbers[left] + numbers[right];

                if sum == 0 {
                    triplets.push(vec![numbers[i], numbers[left], numbers[right]]);
                    left += 1;
                    right -= 1;

                    // skip duplicate values on both sides
                    while left < right && numbers[left] == numbers[left - 1] {
                        left += 1;
                    }
                    while left < right && numbers[right] == numbers[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        triplets
    }
}
