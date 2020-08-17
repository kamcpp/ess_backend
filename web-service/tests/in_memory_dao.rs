use web_service::traits::{Identifiable, Appliable, Dao};
use web_service::daos::InMemoryDao;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Person {
        id: Option<i32>,
        name: Option<String>,
        family: Option<String>,
        age: Option<i32>,
    }

    impl Identifiable for Person {
        fn id(&self) -> Option<i32> {
            self.id
        }

        fn set_id(&mut self, id: i32) {
            self.id = Some(id);
        }
    }

    impl Appliable<Person> for Person {
        fn apply(&mut self, other: &Person) {
            if other.name.is_some() {
                self.name = Some(other.name.as_ref().unwrap().clone());
            }
            if other.family.is_some() {
                self.family = Some(other.family.as_ref().unwrap().clone());
            }
            if other.age.is_some() {
                self.age = Some(*other.age.as_ref().unwrap());
            }
        }
    }

    #[test]
    fn empty_dao_test() {
        let dao = InMemoryDao::<Person>::new();
        assert_eq!(dao.count(), 0);
    }

    #[test]
    fn insert_into_with_already_set_id_test() {
        let mut dao = InMemoryDao::<Person>::new();
        match dao.insert_into(Person {
            id: Some(100),
            name: Some("Foo".to_string()),
            family: Some("Bar".to_string()),
            age: Some(33),
        }) {
            Ok(_) => {
                panic!("You must not be able to insert an entity with an already set id!");
            },
            Err(err) => {
                assert_eq!(err.get_code(), 2000);
            }
        }
    }

    #[test]
    fn insert_into_test() {
        let mut dao = InMemoryDao::<Person>::new();
        assert!(
            dao.insert_into(Person {
                id: None,
                name: Some("Foo".to_string()),
                family: Some("Bar".to_string()),
                age: Some(33),
            }).is_ok()
        );
        assert!(
            dao.insert_into(Person {
                id: None,
                name: Some("Foo2".to_string()),
                family: Some("Bar2".to_string()),
                age: Some(40),
            }).is_ok()
        );
        assert_eq!(dao.count(), 2);
    }
}
