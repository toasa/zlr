fn main() {
    let (tx, rx) = std::sync::mpsc::sync_channel(64);
    let handler = std::thread::spawn(move || match rx.recv() {
        Ok((x, y)) => println!("({}, {})", x, y),
        Err(e) => eprintln!("{e}"),
    });

    if let Err(e) = tx.send((11, 22)) {
        eprintln!("{e}");
    }

    if let Err(e) = handler.join() {
        eprintln!("{:?}", e);
    }
}
