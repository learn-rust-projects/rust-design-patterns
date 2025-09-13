use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Any: For runtime type inspection and downcasting.
// Send + Sync: Ensures thread-safety — the event can be transferred and shared
// between threads safely.
pub trait Event: Any + Send + Sync {}

// This provides a blanket implementation of the Event trait for any type T that
// implements Any + Send + Sync. This allows most standard types to be used as
// events without manually implementing the trait.
impl<T: Any + Send + Sync> Event for T {}

// This creates a type alias Handler for a boxed function trait object that:
// Accepts a reference to dyn Any (so it can accept any event type).
// Implements Fn, so it can be called like a function.
// Is boxed with Box to enable dynamic dispatch.
// Is Send + Sync to ensure thread safety across threads.
type Handler = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    observers: Mutex<HashMap<TypeId, Vec<Handler>>>,
}
// register()                         post()
// +---------------------------+         +------------------------------+
// | f: Fn(&LoginEvent)        |         | e: &dyn Any (maybe LoginEvent)|
// | ↓                         |         | ↓                            |
// | move |event| {            |         | for handler in handlers      |
// |     if let Some(e) =      | ←------+|     handler(e);              |
// |         event.downcast() {          +------------------------------+
// |         f(e)              |
// |     }                     |
// | }                         |
// +---------------------------+
impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
impl EventBus {
    pub fn new() -> Self {
        Self {
            observers: Mutex::new(HashMap::new()),
        }
    }
    // 'static: This is required because you're boxing event handlers as trait
    // objects (Box<dyn Fn(&dyn Any)>) to store them in a handler list, and
    // trait objects erase lifetimes at runtime.
    pub fn register<E: Event + 'static, F>(&self, f: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        let mut map = self.observers.lock().unwrap();
        let handlers = map.entry(TypeId::of::<E>()).or_default();
        // It wraps the event handler function f into a type-erased closure and adds it
        // to the event handler list handlers.
        handlers.push(Box::new(move |event| {
            if let Some(event) = event.downcast_ref::<E>() {
                f(event);
            }
        }));
    }

    pub fn post<E: Event + 'static>(&self, event: E) {
        if let Some(handlers) = self.observers.lock().unwrap().get(&TypeId::of::<E>()) {
            for handler in handlers {
                handler(&event);
            }
        }
    }
}

#[derive(Debug)]
struct MyEvent {
    pub message: String,
}

fn main() {
    let bus = Arc::new(EventBus::new());

    bus.register(|e: &MyEvent| {
        println!("Received event: {}", e.message);
    });

    bus.post(MyEvent {
        message: "Hello, EventBus!".into(),
    });
}
