fn main() {
    let arr: [u32; 4] = [11, 22, 33, 44];
    let s: &[u32] = &arr[1..3];
    println!("{:#?}", s);
}
