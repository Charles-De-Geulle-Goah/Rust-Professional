pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut p_ = 1.0000;
    for i in 0..n {
        let term = (365-i) as f64/365.0;
        p_ *= term;
        // p_ = (p_ * 10000.0).round() / 10000.0;
    }
    1.0000 - p_ as f64
}
