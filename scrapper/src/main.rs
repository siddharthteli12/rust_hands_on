fn main() {
    let url = "https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    println!("Content {:?}", document);
}
