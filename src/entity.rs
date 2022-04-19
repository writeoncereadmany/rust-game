use core::any::*;
use std::collections::HashMap;

struct Entity {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {

    pub fn new() -> Self {
        Entity { data: HashMap::new() }
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn get_2<T1: Any, T2: Any>(&self) -> Option<(&T1, &T2)> {
        Some((self.get()?, self.get()?))
    }

    pub fn get_3<T1: Any, T2: Any, T3: Any>(&self) -> Option<(&T1, &T2, &T3)> {
        Some((self.get()?, self.get()?, self.get()?))
    }

    pub fn get_4<T1: Any, T2: Any, T3: Any, T4: Any>(&self) -> Option<(&T1, &T2, &T3, &T4)> {
        Some((self.get()?, self.get()?, self.get()?, self.get()?))
    }

    pub fn with<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn without<T: Any>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
    }

    pub fn apply<T: Any, R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        Some(f(self.get()?))
    }

    pub fn apply_2<T1: Any, T2: Any, R>(&self, f: impl FnOnce((&T1, &T2)) -> R) -> Option<R> {
        Some(f((self.get()?, self.get()?)))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq, Eq)] struct Count(u32);
    #[derive(Debug, PartialEq, Eq)] struct Score(u32);
    #[derive(Debug, PartialEq, Eq)] struct Name(&'static str);

    #[test]
    pub fn fetches_value_by_type() {
        let mut entity = Entity::new();
        entity.with(Count(123));
        entity.with(Name("Hello"));

        assert_eq!(Some(&Count(123)), entity.get::<Count>());
        assert_eq!(Some(&Name("Hello")), entity.get());
    }

    #[test]
    pub fn returns_empty_when_no_value_provided() {
        let mut entity = Entity::new();
        entity.with(Score(123));
        let count : Option<&Count> = entity.get();
        assert_eq!(None, count)
    }

    #[test]
    pub fn applies_updater_to_entity() {
        let mut entity = Entity::new();
        entity.with(Count(123));
        entity.with(Score(456));
        let count : Option<u32> = entity.apply(|Count(x)| { *x });
        let sum : Option<u32> = entity.apply_2(|(Count(x), Score(y))| {
            x + y
        });
        assert_eq!(Some(123), count);
        assert_eq!(Some(579), sum);
    }

    #[test]
    pub fn can_remove_values_from_entity() {
        let mut entity = Entity::new();
        entity.with(Count(123));
        assert_eq!(Some(&Count(123)), entity.get::<Count>());
        entity.without::<Count>();
        assert_eq!(None, entity.get::<Count>());
    }

    #[test]
    pub fn can_store_generic_types() {
        let mut entity = Entity::new();
        entity.with(vec![Count(123), Count(456)]);
        entity.with(vec![Score(1), Score(2)]);
        entity.with(Count(789));
        assert_eq!(Some(&vec!(Count(123), Count(456))), entity.get());
        assert_eq!(Some(&vec!(Score(1), Score(2))), entity.get());
        assert_eq!(Some(&Count(789)), entity.get());
        assert_eq!(None, entity.get::<Vec<Name>>());
    }


}