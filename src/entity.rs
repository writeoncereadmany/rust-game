use core::any::*;
use std::collections::HashMap;

trait Foo<'a, T> {
    fn bar(entity: &'a Entity) -> Option<T>;
}

impl <'a, A, B> Foo<'a, (A, B)> for (A, B) where 
    A: Foo<'a, A>, 
    B: Foo<'a, B>,
{
    fn bar(entity: &'a Entity) -> Option<(A, B)> {
        Some((A::bar(entity)?, B::bar(entity)?))
    }
}

struct Entity {
    pub id: u64,
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {

    pub fn new(id: u64) -> Self {
        Entity { id, data: HashMap::new() }
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn getto<'a, T: Foo<'a, T>>(&'a self) -> Option<T> {
        T::bar(self)
    }

    pub fn get_2<T1: Any, T2: Any>(&self) -> Option<(&T1, &T2)> {
        Some((self.get()?, self.get()?))
    }

    pub fn with<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn without<T: Any>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
    }

    pub fn apply<T: Any, R>(&self, f: impl Fn(&T) -> R) -> Option<R> {
        Some(f(self.get()?))
    }

    pub fn apply_2<T1: Any, T2: Any, R>(&self, f: impl Fn((&T1, &T2)) -> R) -> Option<R> {
        Some(f(self.get_2()?))
    }
}

struct Entities {
    next_id: u64,
    entities: Vec<Entity>
}

impl Entities {
    pub fn new() -> Self {
        Entities{ next_id: 0, entities: Vec::new() }
    }

    pub fn spawn(&mut self, initialise: impl Fn(&mut Entity) -> ()) {
        let mut entity = Entity::new(self.next_id);
        self.next_id += 1;

        initialise(&mut entity);

        self.entities.push(entity);
    }

    pub fn collect<T: Any>(&self) -> Vec<&T> {
        self.entities.iter().map(|e| { e.get() }).filter(|e| { e.is_some() }).map(|e| { e.unwrap() }).collect()
    }

    pub fn fold<T: Any>(&self, initial: T, f: impl Fn(&T, &T) -> T) -> T {
        let mut accumulated = initial;
        for entity in &self.entities {
            if let Some(next) = entity.get() {
                accumulated = f(&accumulated, next);
            }
        }
        accumulated
    }

    pub fn apply<I: Any, O: Any>(&mut self, f: impl Fn(&I) -> O) {
        for entity in &mut self.entities.iter_mut() {
            entity.get()
                .map(&f)
                .map(|v| entity.with(v));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq, Eq)] struct Count(u32);
    #[derive(Debug, PartialEq, Eq)] struct Score(u32);
    #[derive(Debug, PartialEq, Eq)] struct Name(&'static str);

    impl <'a> Foo<'a, &'a Count> for &'a Count {
        fn bar(entity: &'a Entity) -> Option<&'a Count> {
            entity.get()
        }
    }

    impl <'a> Foo<'a, &'a Score> for &'a Score {
        fn bar(entity: &'a Entity) -> Option<&'a Score> {
            entity.get()
        }
    }

    impl <'a> Foo<'a, &'a Name> for &'a Name {
        fn bar(entity: &'a Entity) -> Option<&'a Name> {
            entity.get()
        }
    }

    #[test]
    pub fn fetches_value_by_type() {
        let mut entity = Entity::new(1);
        entity.with(Count(123));
        entity.with(Name("Hello"));

        assert_eq!(Some(&Count(123)), entity.get::<Count>());
        assert_eq!(Some(&Name("Hello")), entity.get());
    }

    #[test]
    pub fn returns_empty_when_no_value_provided() {
        let mut entity = Entity::new(1);
        entity.with(Score(123));
        let count : Option<&Count> = entity.get();
        assert_eq!(None, count)
    }

    #[test]
    pub fn applies_updater_to_entity() {
        let mut entity = Entity::new(1);
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
        let mut entity = Entity::new(1);
        entity.with(Count(123));
        assert_eq!(Some(&Count(123)), entity.get::<Count>());
        entity.without::<Count>();
        assert_eq!(None, entity.get::<Count>());
    }

    #[test]
    pub fn can_store_generic_types() {
        let mut entity = Entity::new(1);
        entity.with(vec![Count(123), Count(456)]);
        entity.with(vec![Score(1), Score(2)]);
        entity.with(Count(789));
        assert_eq!(Some(&vec!(Count(123), Count(456))), entity.get());
        assert_eq!(Some(&vec!(Score(1), Score(2))), entity.get());
        assert_eq!(Some(&Count(789)), entity.get());
        assert_eq!(None, entity.get::<Vec<Name>>());
    }

    #[test]
    pub fn can_get_tuples() {
        let mut entity = Entity::new(1);
        entity.with(Count(123));
        entity.with(Score(456));

        assert_eq!(Some(&Count(123)), entity.getto());
        assert_eq!(Some(&Score(456)), entity.getto());

        assert_eq!(Some(&Count(123)), entity.getto::<&Count>());
        assert_eq!(None, entity.getto::<&Name>());

        assert_eq!(Some((&Count(123), &Score(456))), entity.getto());
        assert_eq!(Some((&Count(123), &Score(456))), entity.getto::<(&Count, &Score)>());
        assert_eq!(None, entity.getto::<(&Count, &Name)>());
    }

    #[test]
    pub fn handles_option_in_tuples() {
        let mut entity = Entity::new(1);
        entity.with(Count(123));
        entity.with(Score(456));

        assert_eq!(Some(&Count(123)), entity.getto());
        assert_eq!(Some(&Score(456)), entity.getto());

        assert_eq!(Some(&Count(123)), entity.getto::<&Count>());
        assert_eq!(None, entity.getto::<&Name>());

        assert_eq!(Some((&Count(123), &Score(456))), entity.getto());
        assert_eq!(Some((&Count(123), &Score(456))), entity.getto::<(&Count, &Score)>());
    }

    #[test]
    pub fn can_spawn_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.with(Count(123)); });
        entities.spawn(|e| { e.with(Count(456)); e.with(Score(123)); });
        entities.spawn(|e| { e.with(Score(456)); });

        assert_eq!(vec![&Count(123), &Count(456)], entities.collect());
        assert_eq!(vec![&Score(123), &Score(456)], entities.collect());
    }

    #[test]
    pub fn can_fold_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.with(Count(123)); });
        entities.spawn(|e| { e.with(Count(456)); e.with(Score(123)); });
        entities.spawn(|e| { e.with(Score(456)); });

        assert_eq!(Score(579), entities.fold(Score(0), |Score(a), Score(b)| Score(a + b)));
    }


    #[test]
    pub fn can_modify_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.with(Count(123)); });
        entities.spawn(|e| { e.with(Count(456)); e.with(Score(123)); });
        entities.spawn(|e| { e.with(Score(456)); });

        entities.apply(|Count(c)| Count(c + 1));

        assert_eq!(vec![&Count(124), &Count(457)], entities.collect());
    }
}