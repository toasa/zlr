#[derive(Debug)]
enum List<T> {
    Node { val: T, next: Box<List<T>> },
    Nil,
}

#[derive(Debug)]
struct Buffer<const S: usize> {
    buf: [u8; S],
}

fn main() {
    let n1 = List::<u32>::Nil;
    let n2 = List::<u32>::Node {
        val: 10,
        next: Box::<List<u32>>::new(n1),
    };
    let n3 = List::Node {
        val: 20,
        next: Box::new(n2),
    };

    println!("{:?}", n3);

    println!("{:?}", make_pair::<u8, bool>(40, false));

    let buf = Buffer::<128> { buf: [0; 128] };
    println!("{:?}", buf);
}

fn make_pair<T1, T2>(a: T1, b: T2) -> (T1, T2) {
    (a, b)
}
