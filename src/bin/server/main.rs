use urlkurztacean::domain::url_shortener::models::original_url;

fn main() {
    let url = original_url::OriginalUrl::new("https://google.com").unwrap();
    println!("{}", url);
}
