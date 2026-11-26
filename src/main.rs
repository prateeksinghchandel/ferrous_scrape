mod crawler;

fn main() {
    println!("Programm Running");

    use crawler::fetcher;
    fetcher::fetch("https://example.com");
}
