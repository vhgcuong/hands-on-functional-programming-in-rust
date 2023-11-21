fn main() {
    //  combines two iterators into tuple pairs, iterating until the end of the shortest iterator
    let zip = (0..10).zip(10..20);
    println!("Zip (0..10) + (10..20): ( (0,10), (1,11),..., (9,19) )");
    zip.for_each(|item| {
        println!("{:?}", item);
    });

    //  Chain concatenates two iterators
    let chain = (0..10).chain(10..20);
    println!("Chain (0..10) + (10..20): (0, 1, 2,..., 19)");
    chain.for_each(|item| {
        println!("{:?}", item);
    });

    //
    println!("Enumerate: ");
    for item in (8..16).enumerate() {
        println!("{:?}", item);
    }

    // The inspect function applies a function to all values in the iterator during iteration
    println!("Inspect: ");
    (0..10).inspect(|x| {
        println!("value {}", *x);
    }).for_each(|_| {});

    // The map function applies a function to each element, returning the result in place
    println!("Map: ");
    let _ = (0..10).map(|x| x*x).inspect(|x| {
        println!("{}", x);
    }).collect::<Vec<_>>();

    // the filter function restricts elements to those satisfying a predicate
    (2..18).filter(|x| *x > 3).inspect(|x| {
        println!("{x}");
    }).for_each(|_| {});

    // The fold function accumulates all values into a single result
    let data = (0..10).fold(0, |x, y| x+y);
    println!("{}", data);

    // When you want to apply the iterator, you can use a for loop or call collect
    for i in (0..10) {
        println!("{i}");
    }

    (0..10).collect::<Vec<u64>>();
}
