use std::str::FromStr;

extern crate num;
use num::Complex;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

// 屏幕像素坐标转换到复平面坐标
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: (f64, f64),
                  lower_right: (f64, f64))
                  -> (f64, f64) {
    let (width, height) = (lower_right.0 - upper_left.0, upper_left.1 - lower_right.1);
    (upper_left.0 + pixel.0 as f64 * width / bounds.0 as f64,
     upper_left.1 - pixel.1 as f64 * height / bounds.1 as f64)
}

//逃逸时间算法
fn escapes(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex{re:0.0, im:0.0};
    // 迭代[0, limit)
    for i in 0..limit{
        z = z*z + c;
        // 发生逃逸
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    return None;
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75), (-1.0, 1.0), (1.0, -1.0)),
               (-0.5, -0.5));
}

// 引用类型
fn main() {
    let mut b = 1i32;
    {
        let b1 = b;
        drop(b1);
        let c1 = &b1;
        println!("{}", b);
    }
    let bb = &b;
    let _ = "ss";
    let (a, b) = (1, 2);
    let mut a = Vec::new();
    a.push("aaa");
    a.push("bbb");
    for s in &a[1..] {
        println!("{}", s);
    }
}
