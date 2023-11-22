struct MyStrct
{
    a: u32,
    b: f32,
    c: String,
}

#[derive(Debug)]
enum Term
{
    TermVal { value: String },
    TermVar { symbol: String },
    TermApp { f: Box<Term>, x: Box<Term> },
    TermAbs { arg: String, body: Box<Term> },
}
fn main() {
    let x = {
        fn f(x: u32) -> u32 {
            x * x
        }
        let y = f(5);
        y * 3
    };

    let y;
    if true {
        y = 1;
    } else {
        y = 2;
    }

    let z = if true { 1 } else { 2 };

    MyStrct {
        a: 1,
        b: 1.0,
        c: "".to_string()
    };

    (1, 1.0, "".to_string());

    let mut t = Term::TermVar {
        symbol: "".to_string()
    };

    println!("{:?}", t);

    let result = match t {
        Term::TermVal { value: v1} => v1,
        Term::TermVar { symbol: v1 } => v1,
        Term::TermApp { f: ref v1, x: ref v2} => "TermApp(?,?)".to_string(),
        Term::TermAbs { arg: ref mut v1, body: ref mut v2} => "TermAbs(?,?)".to_string()
    };

    println!("{:?}", result);
}