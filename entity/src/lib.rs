use core::any::*;
use std::collections::HashMap;

pub trait Component: Any { }
pub trait Variable: Component { }

pub struct Id(u64);

impl Component for Id { }

pub struct EntityBuilder {
    data: HashMap<TypeId, Box<dyn Any>>
}

pub fn entity() -> EntityBuilder {
    EntityBuilder { data: HashMap::new() }
}

impl EntityBuilder {
    pub fn with<T: Component>(mut self, value: T) -> Self {
        self.data.insert(TypeId::of::<T>(), Box::new(value));        
        self
    }
}

pub struct Entity {
    pub id: u64,
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Entity { id, data: HashMap::new() }
    }

    fn get<T: Component>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    fn set<T: Variable>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    fn remove<T: Variable>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
    }
}

pub struct Entities {
    next_id: u64,
    entities: Vec<Entity>
}

impl Entities {
    pub fn new() -> Self {
        Entities{ next_id: 0, entities: Vec::new() }
    }

    pub fn spawn(&mut self, builder: EntityBuilder) {
        let mut entity = Entity { id: self.next_id, data: builder.data };
        self.next_id += 1;

        self.entities.push(entity);
    }

    pub fn delete(&mut self, id: u64) {
        self.entities.retain(|entity| entity.id != id)
    }

    pub fn collect<T: Component>(&self) -> Vec<&T> {
        self.entities.iter().flat_map(|e| e.get() ).collect()
    }

    pub fn map<T>(&self, f: impl Fn(&Entity) -> Option<T>) -> Vec<T> {
        self.entities.iter().flat_map(f).collect()
    }

    pub fn fold<T: Component, R>(&self, initial: R, f: impl Fn(&R, &T) -> R) -> R 
    {
        let mut accumulated = initial;
        for entity in &self.entities {
            if let Some(next) = entity.get() {
                accumulated = f(&accumulated, next);
            }
        }
        accumulated
    }

    pub fn apply<I: Component, O: Variable>(&mut self, f: impl Fn(&I) -> O) 
    {
        for entity in self.entities.iter_mut() {
            entity.get().map(&f).map(|val| entity.set(val));
        }
    }

    pub fn apply_2<I1: Component, I2: Component, O: Variable>(&mut self, f: impl Fn(&I1, &I2) -> O)
    {
        for entity in self.entities.iter_mut() {
            if let (Some(i1), Some(i2)) = (entity.get(), entity.get()) {
                let val = f(i1, i2);
                entity.set(val)
            }
        }
    }

    pub fn apply_entity(&mut self, f: impl Fn(&mut Entity)) 
    {
        for entity in self.entities.iter_mut() {
            f(entity);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use component_derive::{Component, Variable};

    #[derive(Debug, PartialEq, Eq, Variable)] struct Count(u64);
    #[derive(Debug, PartialEq, Eq, Variable)] struct Score(u64);
    #[derive(Debug, PartialEq, Eq, Variable)] struct Name(&'static str);

    #[test]
    pub fn fetches_value_by_type() {
        let mut entity = Entity::new(1);
        entity.set(Count(123));
        entity.set(Name("Hello"));

        assert_eq!(Some(&Count(123)), entity.get::<Count>());
        assert_eq!(Some(&Name("Hello")), entity.get());
    }

    #[test]
    pub fn returns_empty_when_no_value_provided() {
        let mut entity = Entity::new(1);
        entity.set(Score(123));
        let count : Option<&Count> = entity.get();
        assert_eq!(None, count)
    }

    #[test]
    pub fn can_remove_values_from_entity() {
        let mut entity = Entity::new(1);
        entity.set(Count(123));
        assert_eq!(Some(&Count(123)), entity.get::<Count>());
        entity.remove::<Count>();
        assert_eq!(None, entity.get::<Count>());
    }

    #[test]
    pub fn can_spawn_entities() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        assert_eq!(vec![&Count(123), &Count(456)], entities.collect());
        assert_eq!(vec![&Score(123), &Score(456)], entities.collect());
    }

    #[test]
    pub fn can_fold_entities() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        assert_eq!(Score(579), entities.fold(Score(0), |Score(a), Score(b)| Score(a + b)));
    }


    #[test]
    pub fn can_map_entities() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        assert_eq!(vec![(123 + 1), (456 + 2)], entities.map(|entity| {
            let Score(s) = entity.get()?;
            Some(s + entity.id)
        }));
    }

    #[test]
    pub fn can_modify_entities() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        entities.apply(|Count(c)| Count(c + 1));

        assert_eq!(vec![&Count(124), &Count(457)], entities.collect());
    }


    #[test]
    pub fn can_modify_entities_with_multiple_args() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        entities.apply_2(|Count(c), Score(s)| Count(c + s));

        assert_eq!(vec![&Count(123), &Count(579)], entities.collect());
    }

    #[test]
    pub fn can_modify_entities_with_arbitrary_complexity() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        entities.apply_entity(|entity| {
            if let (Some(Count(c)), Some(Score(s))) = (entity.get(), entity.get()) { 
                let new_count = Count(c + s); 
                let new_score = Score(c - s);
                entity.set(new_count);
                entity.set(new_score);
            }
        });

        assert_eq!(vec![&Count(123), &Count(579)], entities.collect());
        assert_eq!(vec![&Score(333), &Score(456)], entities.collect());
    }
}