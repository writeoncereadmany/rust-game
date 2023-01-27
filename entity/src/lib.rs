use core::any::*;
use std::collections::HashMap;
use component_derive::*;

pub trait Component: Any + Clone { }
pub trait Variable: Component { }

#[derive(Clone, Constant)]
pub struct Id(pub u64);

impl Component for () {}
impl Variable for () {}

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

    pub fn get<T: Component>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn set<T: Variable>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn remove<T: Variable>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
    }
}

pub struct Entities {
    next_id: u64,
    entities: HashMap<u64, Entity>
}

impl Entities {
    pub fn new() -> Self {
        Entities{ next_id: 0, entities: HashMap::new() }
    }

    pub fn spawn(&mut self, builder: EntityBuilder) -> u64 {
        let id = self.next_id;
        let entity = Entity { id, data: builder.with(Id(id)).data };
        self.entities.insert(id, entity);

        self.next_id += 1;
    
        id
    }

    pub fn delete(&mut self, id: &u64) -> Option<Entity> {
        self.entities.remove(id)
    }

    pub fn for_each(&self, mut f: impl FnMut(&Entity)) 
    {
        for entity in self.entities.values() {
            f(entity);
        }
    }

    pub fn for_each_mut(&mut self, mut f: impl FnMut(&mut Entity)) 
    {
        for entity in self.entities.values_mut() {
            f(entity);
        }
    }

    pub fn collect<T: Component>(&self) -> Vec<&T> {
        self.entities.values().flat_map(|e| e.get() ).collect()
    }

    pub fn collect_2<T1: Component, T2: Component>(&self) -> Vec<(&T1, &T2)> {
        self.entities.values().flat_map(|e| Some((e.get()?, e.get()?)) ).collect()
    }

    pub fn collect_3<T1: Component, T2: Component, T3: Component>(&self) -> Vec<(&T1, &T2, &T3)> {
        self.entities.values().flat_map(|e| Some((e.get()?, e.get()?, e.get()?)) ).collect()
    }

    pub fn collect_4<T1: Component, T2: Component, T3: Component, T4: Component>(&self) -> Vec<(&T1, &T2, &T3, &T4)> {
        self.entities.values().flat_map(|e| Some((e.get()?, e.get()?, e.get()?, e.get()?)) ).collect()
    }

    pub fn fold<T: Component, R>(&self, initial: R, f: impl Fn(&R, &T) -> R) -> R 
    {
        let mut accumulated = initial;
        for entity in self.entities.values() {
            if let Some(next) = entity.get() {
                accumulated = f(&accumulated, next);
            }
        }
        accumulated
    }

    pub fn apply<I: Component, O: Variable>(&mut self, mut f: impl FnMut(&I) -> O) 
    {
        for entity in self.entities.values_mut() {
            if let Some(i) = entity.get() {
                let val = f(i);
                entity.set(val)  
            } 
        }
    }

    pub fn apply_2<I1: Component, I2: Component, O: Variable>(&mut self, mut f: impl FnMut(&I1, &I2) -> O)
    {
        for entity in self.entities.values_mut() {
            if let (Some(i1), Some(i2)) = (entity.get(), entity.get()) {
                let val = f(i1, i2);
                entity.set(val)
            }
        }
    }

    pub fn apply_3<I1: Component, I2: Component, I3: Component, O: Variable>(&mut self, mut f: impl FnMut(&I1, &I2, &I3) -> O)
    {
        for entity in self.entities.values_mut() {
            if let (Some(i1), Some(i2), Some(i3)) = (entity.get(), entity.get(), entity.get()) {
                let val = f(i1, i2, i3);
                entity.set(val)
            }
        }
    }

    pub fn apply_4<I1: Component, I2: Component, I3: Component, I4: Component, O: Variable>(&mut self, mut f: impl FnMut(&I1, &I2, &I3, &I4) -> O)
    {
        for entity in self.entities.values_mut() {
            if let (Some(i1), Some(i2), Some(i3), Some(i4)) = (entity.get(), entity.get(), entity.get(), entity.get()) {
                let val = f(i1, i2, i3, i4);
                entity.set(val)
            }
        }
    }

    pub fn apply_6<
        I1: Component, 
        I2: Component, 
        I3: Component, 
        I4: Component, 
        I5: Component,
        I6: Component,
        O: Variable>(&mut self, mut f: impl FnMut(&I1, &I2, &I3, &I4, &I5, &I6) -> O)
    {
        for entity in self.entities.values_mut() {
            if let (Some(i1), Some(i2), Some(i3), Some(i4), Some(i5), Some(i6)) 
                = (entity.get(), entity.get(), entity.get(), entity.get(), entity.get(), entity.get()) 
            {
                let val = f(i1, i2, i3, i4, i5, i6);
                entity.set(val)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use component_derive::{Variable};

    #[derive(Debug, PartialEq, Eq, Clone, Variable)] struct Count(u64);
    #[derive(Debug, PartialEq, Eq, Clone, Variable)] struct Score(u64);
    #[derive(Debug, PartialEq, Eq, Clone, Variable)] struct Name(&'static str);

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

        entities.for_each_mut(|entity| {
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