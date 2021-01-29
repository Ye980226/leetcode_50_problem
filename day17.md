## 344.反转字符串

编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 char[] 的形式给出。

不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外空间解决这一问题。

你可以假设数组中的所有字符都是 ASCII 码表中的可打印字符。

```rust
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>){
        if s.len()<=1{
            return ;
        }
        let n = s.len();
        let mut idx;
        for i in 0..n/2{
            idx=n-i-1;
            s.swap(i,idx);
        }
        // let mut p=0;
        // let mut q=s.len()-1;
        // loop{
        //     if p>=q{
        //         break;
        //     }else{
        //         s.swap(p,q);
        //         q-=1;
        //         p+=1;
        //     }
        // }
        // s.clone()
    }
}
```

## 557.反转字符串中的单词III

给定一个字符串，你需要反转字符串中每个单词的字符顺序，同时仍保留空格和单词的初始顺序。

```rust
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ").map(|x|{
            x.chars().rev().collect::<String>()+" "
        }).collect::<String>().trim().to_string()
    }
}
```



