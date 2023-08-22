// @leetup=info id=273 lang=rust slug=integer-to-english-words

// @leetup=code
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            "Zero".to_string()
        } else {
            Solution::helper(num as usize).into_iter().flat_map(|s| if s.is_empty() { None } else {
                Some(s)
            }).collect::<Vec<_>>().join(" ")
        }
    }

    fn helper(n: usize) -> Vec<String> {
        let ones = [
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        let tens = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        match n {
            1_000_000_000.. => {
                let mut a = Solution::helper(n / 1_000_000_000);
                a.append(&mut vec!["Billion".to_owned()]);
                a.append(&mut Solution::helper(n % 1_000_000_000));
                a
            }
            1_000_000.. => {
                let mut a = Solution::helper(n / 1_000_000);
                a.append(&mut vec!["Million".to_owned()]);
                a.append(&mut Solution::helper(n % 1_000_000));
                a
            }
            1_000.. => {
                let mut a = Solution::helper(n / 1_000);
                a.append(&mut vec!["Thousand".to_owned()]);
                a.append(&mut Solution::helper(n % 1_000));
                a
            }
            100.. => {
                let mut a = Solution::helper(n / 100);
                a.append(&mut vec!["Hundred".to_owned()]);
                a.append(&mut Solution::helper(n % 100));
                a
            }
            20.. => {
                let mut a = vec![tens[n / 10].to_string()];
                a.append(&mut Solution::helper(n % 10));
                a
            }
            _ => {
                vec![ones[n].to_string()]
            }
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::number_to_words;
}
