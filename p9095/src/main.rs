use std::io::{self, BufRead};

fn main() {

    fn getInt(v: Option<Result<String, std::io::Error>>) -> i32 {
        v.unwrap().unwrap().trim().parse::<i32>().unwrap()
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();      
    let n = getInt(lines.next());    

    fn solve(v: usize, dp: &mut Vec<usize>) -> usize {
        if v == 1 {
            return 1
        } else if v == 2 {
            return 2
        } else if v == 3 {
            return 4
        }
        let g = dp[v];
        if g > 0 { 
            return g
        }
        dp[v] = solve(v-1, dp) + solve(v-2, dp) + solve(v-3, dp);
        dp[v]
    }        

    for _ in 0..n {
        let v = getInt(lines.next()) as usize;
        let mut dp = vec![0 as usize; v+1];        
        let res = solve(v, &mut dp);
        println!("{}", res);
    }    
}
