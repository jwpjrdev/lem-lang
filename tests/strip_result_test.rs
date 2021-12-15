#[test]
fn test_strip_token_vec() {
    let lexed = lem_lang::initial_scan(&"nameof".to_string());
    let stripped = lem_lang::strip_token_vec(&lexed);
    assert_eq!(lexed.get(0).unwrap().0, *stripped.get(0).unwrap());
}
