#[test]
fn test() {
    let example_com = safe_url_macro::safe_url!("https://example.com");
    println!("example_com is {:?}", &example_com);
}
