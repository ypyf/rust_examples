// 计算自然数前100项的和
fn main() {
    let mut sum = 0;
    for x in 1..101 {
        sum += x;
    }
    println!("1+2+..+100 = {}", sum);
}
