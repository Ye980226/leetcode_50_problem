struct Solution();
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ").map(|x|{
            x.chars().rev().collect::<String>()+" "
        }).collect::<String>().trim().to_string()
    }
}
fn main()
{
    println!("{}",Solution::reverse_words(String::from("Let's take LeetCode contest")));

}