use core::any::*;
use std::collections::HashMap;

trait GetComponent<'a, T> {
    fn get_component(entity: &'a Entity) -> Option<T>;
}

trait SetComponent {
    fn set_on_entity(self, entity: &mut Entity);
}

impl <'a, A, B> GetComponent<'a, (A, B)> for (A, B) where 
    A: GetComponent<'a, A>, 
    B: GetComponent<'a, B>,
{
    fn get_component(entity: &'a Entity) -> Option<(A, B)> {
        Some((A::get_component(entity)?, B::get_component(entity)?))
    }
}

impl <'a, A> GetComponent<'a, Option<A>> for Option<A> where 
    A: GetComponent<'a, A> {
    fn get_component(entity: &'a Entity) -> Option<Option<A>> {
        Some(A::get_component(entity))
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

    pub fn get_atom<T: Any>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn get<'a, T: GetComponent<'a, T>>(&'a self) -> Option<T> {
        T::get_component(self)
    }

    pub fn set_atom<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn remove_atom<T: Any>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
    }

    pub fn update<T, R: Any>(&mut self, f: impl Fn(&T) -> R) 
        where for <'a> &'a T: GetComponent<'a, &'a T>
    {
        let maybe : Option<R> = self.get().map(f);
        if let Some(val) = maybe {
            self.set_atom(val);
        }
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

    pub fn collect<'a, T: 'a>(&'a self) -> Vec<&'a T>
        where &'a T: GetComponent<'a, &'a T>
    {
        self.entities.iter().map(|e| { e.get() }).filter(|e| { e.is_some() }).map(|e| { e.unwrap() }).collect()
    }

    pub fn fold<'a, T: 'a>(&'a self, initial: T, f: impl Fn(&T, &'a T) -> T) -> T 
        where &'a T: GetComponent<'a, &'a T>
    {
        let mut accumulated = initial;
        for entity in &self.entities {
            if let Some(next) = entity.get() {
                accumulated = f(&accumulated, next);
            }
        }
        accumulated
    }

    pub fn apply<I, O: Any>(&mut self, f: impl Fn(&I) -> O) 
        where for <'a> &'a I: GetComponent<'a, &'a I>
    {
        for entity in self.entities.iter_mut() {
            entity.update(&f);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq, Eq)] struct Count(u32);
    #[derive(Debug, PartialEq, Eq)] struct Score(u32);
    #[derive(Debug, PartialEq, Eq)] struct Name(&'static str);

    impl <'a> GetComponent<'a, &'a Count> for &'a Count {
        fn get_component(entity: &'a Entity) -> Option<&'a Count> {
            entity.get_atom()
        }
    }

    impl <'a> GetComponent<'a, &'a Score> for &'a Score {
        fn get_component(entity: &'a Entity) -> Option<&'a Score> {
            entity.get_atom()
        }
    }

    impl <'a> GetComponent<'a, &'a Name> for &'a Name {
        fn get_component(entity: &'a Entity) -> Option<&'a Name> {
            entity.get_atom()
        }
    }

    #[test]
    pub fn fetches_value_by_type() {
        let mut entity = Entity::new(1);
        entity.set_atom(Count(123));
        entity.set_atom(Name("Hello"));

        assert_eq!(Some(&Count(123)), entity.get::<&Count>());
        assert_eq!(Some(&Name("Hello")), entity.get());
    }

    #[test]
    pub fn returns_empty_when_no_value_provided() {
        let mut entity = Entity::new(1);
        entity.set_atom(Score(123));
        let count : Option<&Count> = entity.get();
        assert_eq!(None, count)
    }

    #[test]
    pub fn can_remove_values_from_entity() {
        let mut entity = Entity::new(1);
        entity.set_atom(Count(123));
        assert_eq!(Some(&Count(123)), entity.get::<&Count>());
        entity.remove_atom::<Count>();
        assert_eq!(None, entity.get::<&Count>());
    }

    #[test]
    pub fn can_get_tuples() {
        let mut entity = Entity::new(1);
        entity.set_atom(Count(123));
        entity.set_atom(Score(456));

        assert_eq!(Some(&Count(123)), entity.get());
        assert_eq!(Some(&Score(456)), entity.get());

        assert_eq!(Some(&Count(123)), entity.get::<&Count>());
        assert_eq!(None, entity.get::<&Name>());

        assert_eq!(Some((&Count(123), &Score(456))), entity.get());
        assert_eq!(Some((&Count(123), &Score(456))), entity.get::<(&Count, &Score)>());
        assert_eq!(None, entity.get::<(&Count, &Name)>());
    }

    #[test]
    pub fn handles_option_in_tuples() {
        let mut entity = Entity::new(1);
        entity.set_atom(Count(123));
        entity.set_atom(Score(456));

        assert_eq!(Some((&Count(123), Some(&Score(456)))), entity.get());
        assert_eq!(Some((&Count(123), None)), entity.get::<(&Count, Option<&Name>)>());
    }

    #[test]
    pub fn can_spawn_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set_atom(Count(123)); });
        entities.spawn(|e| { e.set_atom(Count(456)); e.set_atom(Score(123)); });
        entities.spawn(|e| { e.set_atom(Score(456)); });

        assert_eq!(vec![&Count(123), &Count(456)], entities.collect());
        assert_eq!(vec![&Score(123), &Score(456)], entities.collect());
    }

    #[test]
    pub fn can_fold_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set_atom(Count(123)); });
        entities.spawn(|e| { e.set_atom(Count(456)); e.set_atom(Score(123)); });
        entities.spawn(|e| { e.set_atom(Score(456)); });

        assert_eq!(Score(579), entities.fold(Score(0), |Score(a), Score(b)| Score(a + b)));
    }


    #[test]
    pub fn can_modify_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set_atom(Count(123)); });
        entities.spawn(|e| { e.set_atom(Count(456)); e.set_atom(Score(123)); });
        entities.spawn(|e| { e.set_atom(Score(456)); });

        fn f(Count(c): &Count) -> Count {
            Count(c + 1)
        }

        entities.apply(f);

        assert_eq!(vec![&Count(124), &Count(457)], entities.collect());
    }


    // #[test]
    // pub fn can_modify_entities_based_on_tuples() {
    //     let mut entities = Entities::new();

    //     entities.spawn(|e| { e.set_atom(Count(123)); });
    //     entities.spawn(|e| { e.set_atom(Count(456)); e.set_atom(Score(123)); });
    //     entities.spawn(|e| { e.set_atom(Score(456)); });

    //     entities.apply(f);

    //     assert_eq!(vec![&Count(123), &Count(579)], entities.collect());
    // }
}