impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut count = [0; 26];

        for byte in s.bytes() {
            let index = (byte - b'a') as usize;
            count[index] += 1;
        }

        for byte in t.bytes() {
            let index = (byte - b'a') as usize;
            count[index] -= 1;
        }

        count.iter().all(|&value| value == 0)
    }
}
