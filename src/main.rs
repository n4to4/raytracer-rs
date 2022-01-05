fn main() {
    let mut counter = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{}", counter);
        counter+=1;
    }
}
