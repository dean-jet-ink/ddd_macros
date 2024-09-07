use ddd_macros::ValueObject;

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

    #[derive(ValueObject)]
    #[Id]
    struct TestIdValueObject {
        value: Option<i32>,
    }

    #[test]
    fn noneメソッドが使用できる() {
        let obj = TestIdValueObject::none();
        assert!(obj.is_none());
    }

    #[test]
    fn is_noneメソッドが使用できる() {
        let obj = TestIdValueObject::none();
        assert!(obj.is_none());

        let obj_with_value = TestIdValueObject::new(Some(42));
        assert!(!obj_with_value.is_none());
    }
}
