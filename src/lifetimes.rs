#[derive(Debug)]
struct Foo<'a> {
    x: &'a i32,
}

#[allow(dead_code)]
fn foo1() {
    let mut a = Vec::new();
    a.push("hello");
    // 所有权移动到b
    let b = a;
    // 所有权移动回到a
    a = b;
    // 使用a
    println!("{:?}", a);
}

fn main() {
    let y = &5;
    let f = Foo { x: y };
//    let z = &7;
//    f.x = z;
    println!("{:?}", f);

    // 基本类型的 copy 语义
    let f:i32 = 1985;
    let p = f;
    println!("f={} p={}", f, p); // it just works!
}
