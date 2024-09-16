use std::collections::BTreeMap;

fn main() {
    let mut m = BTreeMap::new();
    m.insert(1, "foo");
    m.insert(2, "bar");
    m.insert(3, "baz");

    if let Some(x) = m.remove(&2) {
        println!("{}", x);
    }
    if let Some(x) = m.get(&3) {
        println!("{}", x);
    }
}
