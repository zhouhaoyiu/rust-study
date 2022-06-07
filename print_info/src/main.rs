/// ### zhouhaoyiu
/// #### 2022-06-07
///
/// ```
/// let res = test();
/// ```
///
fn test() {
    println!("test print");
}
fn main() {
    test();
    let my_name: &str = "zhy";
    let my_age: u8 = 21;
    let _birth: (u16, u8, u8) = (2000, 6, 24);
    println!("my name is {0},my age is {1}", my_name, my_age);

    let s = String::from("zhouhaoyu");

    let part1 = &s[0..4];
    let part2 = &s[4..7];
    let part3 = &s[7..9];

    println!("{}={}+{}+{}", s, part1, part2, part3);
}
