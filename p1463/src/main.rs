use std::cmp;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();
    
    let mut dp = vec![0; 1000001];
    
    dp[2] = 1;
    dp[3] = 1;

    for i in 4..=n {
        let mut v1 = dp[(i-1) as usize] + 1;        
        if i % 3 == 0 {
            v1 = cmp::min(v1, dp[(i / 3) as usize] + 1);
        }      
        if i % 2 == 0 {
            v1 = cmp::min(v1, cmp::min(v1, dp[(i/2) as usize] + 1));            
        }       
        dp[i as usize] = v1;
    }

    println!("{}", dp[n as usize]);
}
