pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut a = 0;
    let mut b = 1;
    let mut res = 0;
    while a < threshold {
        if a % 2 == 1 {
            res += a;
        }
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    res
}
