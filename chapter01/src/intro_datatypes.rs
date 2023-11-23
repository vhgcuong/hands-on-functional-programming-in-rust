use std::ops::Mul;
use std::collections::HashMap;

// alias
type Name = String;

// new type
struct NewName(String);

struct Data1 {
    a: i32,
    b: f64,
    c: String,
}

struct Data2 {
    a: u32,
    b: String,
    c: f64,
}

// alias to tuples
type Tuple1 = (i32, f64, String);
type Tuple2 = (u32, String, f64);

// named tuples
struct New1(i32, f64, String);

struct New2(u32, String, f64);


struct Point
{
    x: i32,
    y: i32,
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

type CustomeHasMap = HashMap<i32, u32>;

enum BTree<T>
{
    Branch { val: T, left: Box<BTree<T>>, right: Box<BTree<T>> },
    Leaf { val: T },
}

enum Term
{
    TermVal { value: String },
    TermVar { symbol: String },
    TermApp { f: Box<Term>, x: Box<Term> },
    TermAbs { arg: String, body: Box<Term> },
}

trait Data1Trait
{
    //constructors
    fn new(a: i32, b: f64, c: String) -> Self;
    //methods
    fn get_a(&self) -> i32;
    fn get_b(&self) -> f64;
    fn get_c(&self) -> String;
}

trait BehaviorOfShow
{
    fn show(&self) -> String;
}

fn main()
{}