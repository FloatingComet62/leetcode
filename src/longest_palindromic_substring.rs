/*
Given a string s, return the longest
palindromic substring in s.

Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.

Example 2:

Input: s = "cbbd"
Output: "bb"

Constraints:

1 <= s.length <= 1000
s consist of only digits and English letters.
*/
use crate::Tests;
use std::ops::{Bound, RangeBounds};

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        };
        if start > end {
            return "";
        }
        self.substring(start, end - start)
    }
}

fn expand_from_center(s: &str, init_left: usize, init_right: usize) -> &str {
    let mut left = init_left as isize;
    let mut right = init_right;
    let chars: Vec<char> = s.chars().collect();
    while left >= 0 && right < s.len() && chars[left as usize] == chars[right] {
        left -= 1;
        right += 1;
    }
    return s.slice(((left + 1) as usize)..right);
}

pub fn solution(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let mut max_str = s.substring(0, 1);
    for i in 0..(s.len() - 1) {
        let odd = expand_from_center(s.as_str(), i, i);
        let even = expand_from_center(s.as_str(), i, i + 1);

        if odd.len() > max_str.len() {
            max_str = odd;
        }
        if even.len() > max_str.len() {
            max_str = even;
        }
    }
    max_str.to_string()
}

#[allow(dead_code)]
fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let s_len = s.len();
    for i in 0..(s_len / 2) {
        if chars[i] == chars[s_len - 1 - i] {
            continue;
        }
        return false;
    }
    true
}

// Time Limit Exceeded
#[allow(dead_code)]
fn solution1(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let mut biggest_palindrome = String::new();
    for i in 0..s.len() {
        for j in (i + biggest_palindrome.len() + 1)..(s.len() + 1) {
            let slice = s.slice(i..j);
            if !is_palindrome(slice) {
                continue;
            }
            if slice.len() > biggest_palindrome.len() {
                biggest_palindrome = slice.to_string();
            }
        }
    }
    biggest_palindrome
}

pub fn test(test: &mut Tests) {
    test.add_test(solution("babad".to_string()) == "bab");
    test.add_test(solution("cbbd".to_string()) == "bb");
    test.add_test(solution("a".to_string()) == "a");
    test.add_test(solution("bb".to_string()) == "bb");
    test.add_test(solution(
        "lejyqjcpluiggwlmnumqpxljlcwdsirzwlygexejhvojztcztectzrepsvwssiixfmpbzshpilmojehqyqpzdylxptsbvkgatzdlzphohntysrbrcdgeaiypmaaqilthipjbckkfbxtkreohabrjpmelxidlwdajmkndsdbbaypcemrwlhwbwaljacijjmsaqembgtdcskejplifnuztlmvasbqcyzmvczpkimpbbwxdtviptzaenkbddaauyvqppagvqfpednnckooxzcpuudckakutqyknuqrxjgfdtsxsoztjkqvfvelrklforpjnrbvyyvxigjhkjmxcphjzzilvbjbvwiwnnkbmboiqamgoimujtswdqesighoxsprhnsceshotakvmoxqkqjvbpqucvafiuqwmrlfjpjijbctfupywkbawquchbclgvhxbanybret"
    .to_string()) == "vbjbv");
    test.add_test(solution(
        "kztakrekvefgchersuoiuatzlmwynzjhdqqftjcqmntoyckqfawikkdrnfgbwtdpbkymvwoumurjdzygyzsbmwzpcxcdmmpwzmeibligwiiqbecxwyxigikoewwrczkanwwqukszsbjukzumzladrvjefpegyicsgctdvldetuegxwihdtitqrdmygdrsweahfrepdcudvyvrggbkthztxwicyzazjyeztytwiyybqdsczozvtegodacdokczfmwqfmyuixbeeqluqcqwxpyrkpfcdosttzooykpvdykfxulttvvwnzftndvhsvpgrgdzsvfxdtzztdiswgwxzvbpsjlizlfrlgvlnwbjwbujafjaedivvgnbgwcdbzbdbprqrflfhahsvlcekeyqueyxjfetkxpapbeejoxwxlgepmxzowldsmqllpzeymakcshfzkvyykwljeltutdmrhxcbzizihzinywggzjctzasvefcxmhnusdvlderconvaisaetcdldeveeemhugipfzbhrwidcjpfrumshbdofchpgcsbkvaexfmenpsuodatxjavoszcitjewflejjmsuvyuyrkumednsfkbgvbqxfphfqeqozcnabmtedffvzwbgbzbfydiyaevoqtfmzxaujdydtjftapkpdhnbmrylcibzuqqynvnsihmyxdcrfftkuoymzoxpnashaderlosnkxbhamkkxfhwjsyehkmblhppbyspmcwuoguptliashefdklokjpggfiixozsrlwmeksmzdcvipgkwxwynzsvxnqtchgwwadqybkguscfyrbyxudzrxacoplmcqcsmkraimfwbauvytkxdnglwfuvehpxd"
    .to_string()) == "dtzztd");
}
