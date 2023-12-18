use serde::Deserialize;
use serde_deserializer_2::from_str;

fn main() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        key: String,
        array: Vec<u64>,
    }

    let j = r#"{"key":"value","array":[1,2,3]}"#;

    let expected = Test {
        key: "value".to_string(),
        array: vec![1, 2, 3],
    };
    assert_eq!(expected, from_str(j).unwrap());
}
