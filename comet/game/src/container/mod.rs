use std::any::{Any, TypeId};
use std::boxed::*;
use std::collections::HashMap;
use std::mem::transmute;
use std::ops::Deref;
use std::sync::Arc;

pub type ComponentSet = HashMap<TypeId, Box<Any>>;

pub trait Container {
    fn components(&self) -> &ComponentSet;
    fn components_mut(&mut self) -> &mut ComponentSet;

    fn add_component(&mut self, component: Box<Component>) {
        self.components_mut().insert(component.typeid(), unsafe {
            transmute::<Box<Component>, Box<Any>>(component)
        });
    }

    fn component<T: 'static>(&self) -> &T where T: Component {
        unsafe {
            &**self
                .components()
                .get(&TypeId::of::<T>())
                .expect("Components are not configured!")
                .downcast_ref::<*mut T>()
                .expect("Invalid component type")
        }
    }

    fn component_mut<T: 'static>(&mut self) -> &mut T where T: Component {
        unsafe {
            &mut **self
                .components_mut()
                .get_mut(&TypeId::of::<T>())
                .expect("Components are not configured!")
                .downcast_mut::<*mut T>()
                .expect("Invalid component type")
        }
    }
}

pub trait Component: Any {
    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
