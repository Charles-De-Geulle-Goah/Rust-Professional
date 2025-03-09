use std::cmp::min;

pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    // todo!()
    let mut dp = vec![0;amount as usize + 1];
    // dp[0] = 0;
    for i in 1..=amount{
        let mut res = 32767;
        if i >= 1 {
            res = min(res,dp[i as usize-1]);
        }
        if i >= 2 {
            res = min(res,dp[i as usize-2]);
        }
        if i >= 5 {
            res = min(res,dp[i as usize-5]);
        }
        if i >= 10 {
            res = min(res,dp[i as usize-10]);
        }
        if i >= 20 {
            res = min(res,dp[i as usize-20]);
        }
        if i >= 30 {
            res = min(res,dp[i as usize-30]);
        }
        if i >= 50 {
            res = min(res,dp[i as usize-50]);
        }
        if i >= 100 {
            res = min(res,dp[i as usize-100]);
        }
        dp[i as usize] = res + 1;
    }
    dp[amount as usize]
}
