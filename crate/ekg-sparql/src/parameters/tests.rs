#![cfg(all(test, not(target_family = "wasm")))]

#[test]
fn test_set_param() {
    let mut params = crate::Parameters::empty().unwrap();
    params.set_string("key1", "value1").unwrap();
    let value = params.get_string("key1", "whatever").unwrap();
    assert_eq!(value, "value1");
}
