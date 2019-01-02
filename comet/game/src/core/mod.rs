use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use std::mem::transmute;

pub type ComponentSet = HashMap<TypeId, Arc<Component>>;

pub trait Container {
    fn components(&self) -> &ComponentSet;
    fn components_mut(&mut self) -> &mut ComponentSet;

    fn add_component(&mut self, component: Arc<Component>) {
        self.components_mut()
            .insert(component.typeid(), component);
    }

    fn component<T: 'static>(&self) -> Arc<T> where T: Component {
        let component = Box::new(self
            .components()
            .get(&TypeId::of::<T>())
            .expect("Components are not configured!")
            .clone());

        unsafe {
            *transmute::<Box<Arc<Component>>, Box<Arc<T>>>(component)
        }
    }
}

pub trait Component: Any {
    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
