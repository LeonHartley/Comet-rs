use std::any::{Any, TypeId};
use std::boxed::*;
use std::collections::HashMap;
use std::mem::transmute;
use std::ops::Deref;
use std::sync::Arc;

pub type ComponentSet = HashMap<TypeId, Box<Any + Send + Sync>>;

pub trait Container {
    fn components(&self) -> &ComponentSet;
    fn components_mut(&mut self) -> &mut ComponentSet;

    fn add_component<T>(&mut self, component: T) where T: Component {
        self.components_mut().insert(component.typeid(), unsafe {
            transmute::<Box<Component>, Box<Any + Send + Sync>>(Box::new(component))
        });
    }

    fn component<T>(&self) -> &T where T: Component {
        &*self
            .components()
            .get(&TypeId::of::<T>())
            .expect("Components are not configured!")
            .downcast_ref::<T>()
            .expect("Invalid component type")
    }

    fn component_mut<T>(&mut self) -> &mut T where T: Component {
        &mut *self
            .components_mut()
            .get_mut(&TypeId::of::<T>())
            .expect("Components are not configured!")
            .downcast_mut::<T>()
            .expect("Invalid component type")
    }
}

pub trait Component: Any {
    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
