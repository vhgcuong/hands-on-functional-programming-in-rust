use std::ops::Mul;

#[derive(Debug)]
struct PointU32
{
    x: u32,
    y: u32
}
#[derive(Debug)]
struct PointF32
{
    x: f32,
    y: f32
}
#[derive(Debug)]
struct PointI32
{
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T
}

fn foo_u32(x: u32) -> u32 {
    x*x
}
fn foo_f32(x: f32) -> f32 {
    x*x
}
fn foo_i32(x: i32) -> i32 {
    x*x
}

fn foo<T>(x: T) -> T
    where T: Mul<Output=T> + Copy
{
    x*x
}

fn bar<F, T>(f: F, x: T) -> T
    where F: Fn(T) -> T
{
    f(x)
}

fn main() {
    // u32
    let point_u32 = PointU32 {x:1, y:2};
    println!("{:?}", point_u32);

    // f32
    let point_f32 = PointF32 {x:1.0, y:2.0};
    println!("{:?}", point_f32);

    // i32
    let point_i32 = PointI32 {x:-1, y:2};
    println!("{:?}", point_i32);

    // Generics T
    let point = PointF32 {x:1.0, y:2.0};
    println!("{:?}", point);

    let point = Point {x:1.0, y:2.0};
    println!("{:?}", point);

    println!("{}", bar(foo, 2));
}
