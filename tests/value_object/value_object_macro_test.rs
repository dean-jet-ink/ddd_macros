use rust_ddd_macros::ValueObject;

mod value_objectのテスト {
    use super::*;

    #[derive(ValueObject)]
    struct TestValueObject {
        value: i32,
    }

    #[test]
    fn newメソッドが使用できる() {
        let value = 42;
        let obj = TestValueObject::new(value);
        assert_eq!(obj.value, value);
    }

    #[test]
    fn valueメソッドが使用できる() {
        let value = 42;
        let obj = TestValueObject::new(value);
        assert_eq!(obj.value(), value);
    }
}
