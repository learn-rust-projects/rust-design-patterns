use std::collections::HashMap;

// Define the prototype trait
trait Prototype {
    fn clone_box(&self) -> Box<dyn Prototype>;
    fn get_info(&self) -> String;
}

// Concrete Product A
#[derive(Clone)]
struct ConcreteA {
    pub name: String,
    pub data: Vec<i32>,
}

impl Prototype for ConcreteA {
    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
    fn get_info(&self) -> String {
        format!("ConcreteA: {}, {:?}", self.name, self.data)
    }
}

// Concrete Product B
#[derive(Clone)]
struct ConcreteB {
    pub id: u64,
    pub map: HashMap<String, String>,
}

impl Prototype for ConcreteB {
    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
    fn get_info(&self) -> String {
        format!("ConcreteB: {}, {:?}", self.id, self.map)
    }
}

// Prototype registry
struct PrototypeRegistry {
    prototypes: HashMap<String, Box<dyn Prototype>>,
}

impl PrototypeRegistry {
    fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    fn register(&mut self, name: &str, prototype: Box<dyn Prototype>) {
        self.prototypes.insert(name.to_string(), prototype);
    }

    fn create(&self, name: &str) -> Option<Box<dyn Prototype>> {
        self.prototypes.get(name).map(|p| p.clone_box())
    }
}

// Simulate a very time-consuming object creation process
fn create_complex_a() -> ConcreteA {
    println!("Simulating a very expensive object creation for ConcreteA...");
    // Imagine this takes a lot of time (e.g., loading resources, computation, etc.)
    std::thread::sleep(std::time::Duration::from_secs(2));
    ConcreteA {
        name: "Alpha".to_string(),
        data: vec![1, 2, 3],
    }
}

fn create_complex_b() -> ConcreteB {
    println!("Simulating a very expensive object creation for ConcreteB...");
    // Imagine this takes a lot of time (e.g., loading resources, computation, etc.)
    std::thread::sleep(std::time::Duration::from_secs(2));
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    ConcreteB { id: 999, map }
}

fn main() {
    let mut registry = PrototypeRegistry::new();

    // Register complex objects (expensive to create)
    let concrete_a = create_complex_a();
    let concrete_b = create_complex_b();

    registry.register("A", Box::new(concrete_a));
    registry.register("B", Box::new(concrete_b));

    // Use the prototype to quickly create new objects
    let mut clones = vec![];
    for key in ["A", "B"].iter() {
        if let Some(prototype) = registry.create(key) {
            clones.push(prototype);
        }
    }

    for c in clones {
        println!("{}", c.get_info());
    }
}
