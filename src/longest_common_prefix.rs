/*
Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string "".

Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

Constraints:

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters.
*/
pub fn solution(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs.get(0).unwrap().clone();
    }
    let mut lcp = "".to_string();
    let mut shortest_string = 1000;
    for i in strs.clone() {
        if i.len() < shortest_string {
            shortest_string = i.len();
        }
    }
    for i in 0..shortest_string {
        let cs: Vec<char> = strs
            .iter()
            .map(|x| x.chars().nth(i).unwrap_or('_'))
            .collect();
        let p = cs[0];
        for x in cs {
            if x != p {
                return lcp;
            }
        }
        lcp += p.to_string().as_str();
    }
    lcp
}
