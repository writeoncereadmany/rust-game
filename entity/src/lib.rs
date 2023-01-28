use core::any::*;
use std::collections::HashMap;

pub trait Component: Any + Clone + Sized {
    fn get(entity : &Entity) -> Option<Self>;
 }
pub trait Variable: Component { }

#[derive(Clone)]
pub struct Id(pub u64);

impl Component for Id {
    fn get(entity: &Entity) -> Option<Self> {
        Some(entity.get::<Id>()?.clone())
    }
}

// these are so we can have non-setting applications, which just fire events
impl Component for () {
    fn get(_entity: &Entity) -> Option<Self> {
        Some(())
    }
}
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

impl <A: Component, B: Component> Component for (A, B) {
    fn get(entity: &Entity) -> Option<(A, B)> {
        Some((
            entity.get::<A>()?.clone(), 
            entity.get::<B>()?.clone())
        )
    }
}

impl <A: Component, B: Component, C: Component> Component for (A, B, C) {
    fn get(entity: &Entity) -> Option<(A, B, C)> {
        Some((
            entity.get::<A>()?.clone(), 
            entity.get::<B>()?.clone(), 
            entity.get::<C>()?.clone())
        )
    }
}

impl <A: Component, B: Component, C: Component, D: Component> Component for (A, B, C, D) {
    fn get(entity: &Entity) -> Option<(A, B, C, D)> {
        Some((
            entity.get::<A>()?.clone(), 
            entity.get::<B>()?.clone(), 
            entity.get::<C>()?.clone(), 
            entity.get::<D>()?.clone())
        )
    }
}

impl <A: Component, B: Component, C: Component, D: Component, E: Component> Component for (A, B, C, D, E) {
    fn get(entity: &Entity) -> Option<(A, B, C, D, E)> {
        Some((
            entity.get::<A>()?.clone(), 
            entity.get::<B>()?.clone(), 
            entity.get::<C>()?.clone(), 
            entity.get::<D>()?.clone(),
            entity.get::<E>()?.clone())
        )
    }
}

impl <A: Component, B: Component, C: Component, D: Component, E: Component, F: Component> Component for (A, B, C, D, E, F) {
    fn get(entity: &Entity) -> Option<(A, B, C, D, E, F)> {
        Some((
            entity.get::<A>()?.clone(), 
            entity.get::<B>()?.clone(), 
            entity.get::<C>()?.clone(), 
            entity.get::<D>()?.clone(),
            entity.get::<E>()?.clone(),
            entity.get::<F>()?.clone())
        )
    }
}

impl <T: Component> Component for Option<T> {
    fn get(entity: &Entity) -> Option<Option<T>> {
        Some(entity.get::<T>().map(|component| component.clone()))
    }
}

#[derive(Clone)]
enum Not<T> {
    Not(),
    _Is(T)
}

impl <T: Component> Component for Not<T> {
    fn get(entity: &Entity) -> Option<Not<T>> {
        if entity.get::<T>().is_none() { Some(Not::Not()) } else { None }
    }
}

pub struct Entity {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn get<T: Component>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn set<T: Variable>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
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
        let entity = Entity { data: builder.with(Id(id)).data };
        self.entities.insert(id, entity);

        self.next_id += 1;
    
        id
    }

    pub fn delete<T: Component>(&mut self, id: &u64) -> Option<T> {
        T::get(&self.entities.remove(id)?)
    }

    pub fn for_each<T: Component>(&self, mut f: impl FnMut(T)) 
    {
        for entity in self.entities.values() {
            if let Some(component) = T::get(entity)
            {
                f(component);
            }
        }
    }

    pub fn for_each_pair<A: Component, B: Component>(&self, mut f: impl FnMut(&A, &B)) {
        let firsts = self.collect::<A>();
        let seconds = self.collect::<B>();
        for first in &firsts {
            for second in &seconds {
                f(first, second);
            }
        }
    }

    pub fn collect<T: Component>(&self) -> Vec<T> {
        self.entities.values().flat_map(|entity| T::get(entity)).collect()
    }

    pub fn apply<T: Component, O: Variable>(&mut self, mut f: impl FnMut(T) -> O) 
    {
        for entity in self.entities.values_mut() {
            if let Some(i) = T::get(entity) {
                let val = f(i);
                entity.set(val)  
            } 
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;


    #[derive(Debug, PartialEq, Eq, Clone, Hash)] struct Count(u64);
    #[derive(Debug, PartialEq, Eq, Clone, Hash)] struct Score(u64);
    #[derive(Debug, PartialEq, Eq, Clone, Hash)] struct Name(&'static str);

    impl Component for Count {
        fn get(entity: &Entity) -> Option<Self> {
            Some(entity.get::<Count>()?.clone())
        }    
    }

    impl Component for Score {
        fn get(entity: &Entity) -> Option<Self> {
            Some(entity.get::<Score>()?.clone())
        }    
    }

    impl Component for Name {
        fn get(entity: &Entity) -> Option<Self> {
            Some(entity.get::<Name>()?.clone())
        }    
    }

    impl Variable for Count {}
    impl Variable for Score {}
    impl Variable for Name {}

    #[test]
    pub fn fetches_value_by_type() {
        let mut entity = Entity::new(1);
        entity.set(Count(123));
        entity.set(Name("Hello"));

        assert_eq!(Some(Count(123)), Component::get(&entity));
        assert_eq!(Some(Name("Hello")), Component::get(&entity));
    }

    #[test]
    pub fn returns_empty_when_no_value_provided() {
        let mut entity = Entity::new(1);
        entity.set(Score(123));
        let count : Option<Count> = Component::get(&entity);
        assert_eq!(None, count)
    }

    #[test]
    pub fn can_remove_values_from_entity() {
        let mut entity = Entity::new(1);
        entity.set(Count(123));
        assert_eq!(Some(Count(123)), Component::get(&entity));
        entity.remove::<Count>();
        assert_eq!(None, Count::get(&entity));
    }

    #[test]
    pub fn can_spawn_entities() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        assert_eq!(set([Count(123), Count(456)]), set_(entities.collect()));
        assert_eq!(set([Score(123), Score(456)]), set_(entities.collect()));
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

        assert_eq!(set([Count(124), Count(457)]), set_(entities.collect()));
    }


    #[test]
    pub fn can_modify_entities_with_multiple_args() {
        let mut entities = Entities::new();

        entities.spawn(entity().with(Count(123)));
        entities.spawn(entity().with(Count(456)).with(Score(123)));
        entities.spawn(entity().with(Score(456)));

        entities.apply_2(|Count(c), Score(s)| Count(c + s));

        assert_eq!(set([Count(123), Count(579)]), set_(entities.collect()));
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

        assert_eq!(set([Count(123), Count(579)]), set_(entities.collect()));
        assert_eq!(set([Score(333), Score(456)]), set_(entities.collect()));
    }


    fn set<T: Hash + Eq, const N: usize>(arr: [T; N]) -> HashSet<T> {
        HashSet::from(arr)
    }

    fn set_<T: Hash + Eq>(vec: Vec<T>) -> HashSet<T> {
        HashSet::from_iter(vec)
    }

}