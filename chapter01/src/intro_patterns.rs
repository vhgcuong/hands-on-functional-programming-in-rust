// Pure functions
fn pure_function1(x: u32) -> u32 {
    x * x
}

fn impure_function(x: u32) -> u32 {
    println!("x = {}", x);
    x * x
}

// Higher-order functions
// fn filter<P>(self, predicate: P) -> filter<Self, P> where P: FnMut(&Self::Item) -> bool {
//     !todo!()
// }


// Monads
// trait Monad<A> {
//     fn return_(t: A) -> Self;
//     //:: A -> Monad<A>
//     fn bind<MB,B>(m: Self, f: Fn(A) -> MB) -> MB
//         where MB: Monad<B>;
// //:: Monad<A> -> (A -> Monad<B>) -> Monad<B>
// }


// Function currying
fn not_curried(p1: u32, p2: u32) -> u32 {
    p1 + p2
}

fn curried(p1: u32) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |p2: u32| {
        p1 + p2
    })
}

// Lazy evaluation
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;
cached! {
    FIB;
    fn fib(n: u64) -> u64 = {
        if n==0 || n==1 {
            return n
        }

        fib(n-1) + fib(n-2)
    }
}

fn main() {
    // Functional composition
    let fsin = |x: f64| x.sin();


    // Functors
    let mut c = 0;
    for _ in vec!['a', 'b', 'c'].into_iter()
        .map(|letter| {
            c += 1;
            (letter, c)
        }) {};

    // Monads


    // Currying
    // and calling it
    not_curried(1, 2);

    // and calling it
    curried(1)(2);
    // or
    let add_one = curried(1);
    add_one(2);  // curried(1)(2)


    println!("{}", fib(60));
}