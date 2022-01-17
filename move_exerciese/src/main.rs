fn main() {
    let v = vec!["hello ".to_string(), "world".to_string(), "!".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    struct Person { name: Option<String>, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: Some("zxq".to_string()), birth: 1243});

    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("zxq".to_string()));
    assert_eq!(composers[0].name, None);
}
