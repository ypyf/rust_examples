// 打印命令行参数，不包括程序名
fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
    }
}
