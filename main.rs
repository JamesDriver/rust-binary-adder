use std::cmp;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        //compute difference in size between a and b
        let l1 = if (a.len() > b.len()) { a.len() - b.len() } else { 0 };
        let l2 = if (b.len() > a.len()) { b.len() - a.len() } else { 0 };
        //pad short with 0's, then reverse both for easier indexing
        let mut short: Vec<char> = (format!("{}{}","0".repeat(l2+1),a)).chars().into_iter().rev().collect();
        let mut long:  Vec<char> = (format!("{}{}","0".repeat(l1+1),b)).chars().into_iter().rev().collect();
        
        let mut a = false;
        let mut b = false;
        let mut c = false;
        let mut fin: Vec<char> = vec![];
        //binary adder
        for i in 0..(short.len()) {
            a = if short[i] == '0' { false } else { true };
            b = if long[i]  == '0' { false } else { true };
            //inverse xor due to carry bit
            if c {
                if a ^ b {
                    fin.push('0');
                } else {
                    fin.push('1');
                    //carry bit
                    if !(a || b) { c = false; }
                }
            //xor
            } else {
                if a ^ b {
                    fin.push('1');
                } else {
                    fin.push('0');
                    //carry bit
                    if a && b { c = true; }                    
                }
            }
        }
        if (fin[fin.len()-1]=='0') {
            fin.pop();
        }
        return fin.into_iter().rev().collect();
    }
}
