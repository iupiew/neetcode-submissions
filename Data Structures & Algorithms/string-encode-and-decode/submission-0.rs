impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(&s);
        }
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut i = 0;
        while i < s.len() {
            let hash = i + s[i..].find('#').unwrap();
            let len: usize = s[i..hash].parse().unwrap();
            let start = hash + 1;
            let end = start + len;
            result.push(s[start..end].to_string());
            i = end;
        }
        result
    }
}
