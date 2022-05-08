/// my rust study
/// # 云香绕雪
/// # 雪甜环云
/// 
/// ```
/// let res = test();
/// ```
/// 
fn test () {
    println!("test print");
}
fn main() {
    test();
    let my_name: &str = "zhy";
    let my_age: u8 = 21;
    let _birth: (u16,u8,u8) = (2000,6,24);
    println!("myname is {0},myage is {1}", my_name, my_age);
}
