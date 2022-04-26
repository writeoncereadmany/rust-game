use core::any::*;
use std::collections::HashMap;

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

    pub fn set<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn remove<T: Any>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
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
        self.entities.iter().flat_map(|e| e.get() ).collect()
    }

    pub fn map<T>(&self, f: impl Fn(&Entity) -> Option<T>) -> Vec<T> {
        self.entities.iter().flat_map(f).collect()
    }

    pub fn fold<T: Any, R>(&self, initial: R, f: impl Fn(&R, &T) -> R) -> R 
    {
        let mut accumulated = initial;
        for entity in &self.entities {
            if let Some(next) = entity.get() {
                accumulated = f(&accumulated, next);
            }
        }
        accumulated
    }

    pub fn apply<I: Any, O: Any>(&mut self, f: impl Fn(&I) -> O) 
    {
        for entity in self.entities.iter_mut() {
            entity.get().map(&f).map(|val| entity.set(val));
        }
    }

    pub fn apply_2<I1: Any, I2: Any, O: Any>(&mut self, f: impl Fn(&I1, &I2) -> O)
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

    #[derive(Debug, PartialEq, Eq)] struct Count(u32);
    #[derive(Debug, PartialEq, Eq)] struct Score(u32);
    #[derive(Debug, PartialEq, Eq)] struct Name(&'static str);

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

        entities.spawn(|e| { e.set(Count(123)); });
        entities.spawn(|e| { e.set(Count(456)); e.set(Score(123)); });
        entities.spawn(|e| { e.set(Score(456)); });

        assert_eq!(vec![&Count(123), &Count(456)], entities.collect());
        assert_eq!(vec![&Score(123), &Score(456)], entities.collect());
    }

    #[test]
    pub fn can_fold_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set(Count(123)); });
        entities.spawn(|e| { e.set(Count(456)); e.set(Score(123)); });
        entities.spawn(|e| { e.set(Score(456)); });

        assert_eq!(Score(579), entities.fold(Score(0), |Score(a), Score(b)| Score(a + b)));
    }

    #[test]
    pub fn can_modify_entities() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set(Count(123)); });
        entities.spawn(|e| { e.set(Count(456)); e.set(Score(123)); });
        entities.spawn(|e| { e.set(Score(456)); });

        entities.apply(|Count(c)| Count(c + 1));

        assert_eq!(vec![&Count(124), &Count(457)], entities.collect());
    }


    #[test]
    pub fn can_modify_entities_with_multiple_args() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set(Count(123)); });
        entities.spawn(|e| { e.set(Count(456)); e.set(Score(123)); });
        entities.spawn(|e| { e.set(Score(456)); });

        entities.apply_2(|Count(c), Score(s)| Count(c + s));

        assert_eq!(vec![&Count(123), &Count(579)], entities.collect());
    }

    #[test]
    pub fn can_modify_entities_with_arbitrary_complexity() {
        let mut entities = Entities::new();

        entities.spawn(|e| { e.set(Count(123)); });
        entities.spawn(|e| { e.set(Count(456)); e.set(Score(123)); });
        entities.spawn(|e| { e.set(Score(456)); });

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