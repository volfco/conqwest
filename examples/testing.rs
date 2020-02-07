use conqwest;

fn main() {
    let c = conqwest::http::get("nomad", "/endpoint", Some("http"));
    println!("hello. {:?}", c);
}