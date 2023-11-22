fn scoped() {
    vec![1, 2, 3];
}

fn scoped2() -> Vec<u32> {
    vec![1, 2, 3]
}

fn scoped3() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
}

fn scoped4()
{
    vec![1, 2, 3].clone();
    "".to_string().clone();
}

fn main() {}