use rust_ddd_macros::Entity;

mod entityのテスト {
    use super::*;

    #[derive(Entity)]
    struct TestEntity {
        id: i32,
        name: String,
    }

    #[test]
    fn newメソッドが使用できる() {
        let id = 1;
        let name = String::from("Test Name");
        let entity = TestEntity::new(id, name.clone());
        assert_eq!(entity.id, id);
        assert_eq!(entity.name, name);
    }

    #[test]
    fn idメソッドが使用できる() {
        let id = 1;
        let name = String::from("Test Name");
        let entity = TestEntity::new(id, name);
        assert_eq!(entity.id(), id);
    }

    #[test]
    fn nameメソッドが使用できる() {
        let id = 1;
        let name = String::from("Test Name");
        let entity = TestEntity::new(id, name.clone());
        assert_eq!(entity.name(), &name);
    }
}
