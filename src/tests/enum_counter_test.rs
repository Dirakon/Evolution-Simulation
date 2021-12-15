#[cfg(test)]
#[derive(enum_variant_counter::EnumVariantCount)]
enum SampleEnum{
    A=0,
    B=1,
    C=2
}


#[test]
fn test_enum_counter() {
    assert!(LENGTH == 3);
}