use std::collections::BTreeSet;

fn main() {
    let mut v = Vec::new();
    v.push(11);
    v.push(22);

    let mut s = BTreeSet::new();
    s.insert(100);
    s.insert(200);

    let it = v.iter().chain(s.iter());
    for n in it {
        println!("{}", n);
    }
}
