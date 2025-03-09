pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    //将输入字符串num_str拆分为数字和原始进制两部分
    let parts:Vec<&str> = num_str.split('(').collect(); //以左括号为界拆分，注意split返回的是迭代器
    if parts.len() > 2 {
        panic!("Invalid input!");
    }
    //提取数字和原始进制
    let number = parts[0];
    //获取原始进制时候要去掉结尾的右括号,然后再解析为整数
    let base = parts[1].trim_end_matches(')').parse::<u32>().expect("Invalid base");

    //将数字从原始进制转换为十进制
    let decimal = u32::from_str_radix(number,base).expect("Invalid number for given base");

    //进行下一步的操作之前，先3判断第二个输入参数to_base是否合法
    if to_base < 2 || to_base > 36 {   //36进制：01234567890+26个字母
        panic!("Invalid param to_base");
    } 

    //将demical转换成最终要求的结果
    let result = match to_base {
        //如果是2，8，10，16进制，直接使用系统内置的函数转换
        2=>format!("{:b}", decimal),
        8=>format!("{:o}", decimal),
        10=>format!("{}", decimal),
        16=>format!("{:x}", decimal),
        _=>{
            let digits = "0123456789abcdefghijklmnopqrstuvwxyz";
            let mut result = String::new();
            let mut num = decimal;
            while num > 0 {
                let remainder = num % to_base;
                result.push(digits.chars().nth(remainder as usize).unwrap());  //nth获取第n个字符,注意调用nth返回的是Option
                num /= to_base;
            }
            if result.is_empty() {  //对应输入值为0的情况
                result.push('0');
            }
            
            result.chars().rev().collect()  //最后要反转一下
        }
    };
    result
}
