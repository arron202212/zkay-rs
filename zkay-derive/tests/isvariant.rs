// #![cfg(test)]
// // This lets us ensure that the generated methods get doc comments.
// #![deny(missing_docs)]
// #![deny(unreachable_patterns)]

// /// Tests for `#[derive(is_enum_variant)]`.

// #[macro_use]
// extern crate zkay_derive;
// // extern crate diff;

// // use zkay_derive::IsVariant;
// struct TestStruct;
// #[derive(IsVariant)]
// enum TestEnum {
//     A,
//     B(),
//     C(i32, i32),
//     D { _name: String, _age: i32 },
//     VariantTest,
//     TestStruct(TestStruct),
// }

// #[test]
// fn test_enum() {
//     let x = TestEnum::C(1, 2);
//     assert!(x.is_c());

//     let x = TestEnum::A;
//     assert!(x.is_a());

//     let x = TestEnum::B();
//     assert!(x.is_b());

//     let x = TestEnum::D {_name: "Jane Doe".into(), _age: 30 };
//     assert!(x.is_d());

//     let x = TestEnum::VariantTest;
//     assert!(x.is_variant_test());
//     let x = TestEnum::TestStruct(TestStruct);
//     assert!(x.is_test_struct());
// }
