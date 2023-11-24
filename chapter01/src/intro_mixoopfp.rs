struct MyObject
{
    a: u32,
    b: f32,
    c: String,
}

trait MyObjectTrait
{
    fn new(a: u32, b: f32, c: String) -> Self;
    fn get_a(&self) -> u32;
    fn get_b(&self) -> f32;
    fn get_c(&self) -> String;
}

impl MyObjectTrait for MyObject
{
    fn new(a: u32, b: f32, c: String) -> Self {
        MyObject { a, b, c}
    }

    fn get_a(&self) -> u32 {
        self.a
    }

    fn get_b(&self) -> f32 {
        self.b
    }

    fn get_c(&self) -> String {
        self.c.clone()
    }
}

trait MyObjectApply
{
    fn apply<F, R>(&self, f: F) -> R where F: Fn(u32, f32, String) -> R;
}

impl MyObjectApply for MyObject
{
    fn apply<F, R>(&self, f: F) -> R where F: Fn(u32, f32, String) -> R {
        f(self.a, self.b, self.c.clone())
    }
}

fn main() {
    let a = MyObject::new(1, 1.0, "Test".to_string());
    println!("a: {}, b: {}, c: {}", a.get_a(), a.get_b(), a.get_c());

    a.apply(|a, b, c| {
        println!("a: {}, b: {}, c: {}", a+1, b+2.0, c);
    });
}