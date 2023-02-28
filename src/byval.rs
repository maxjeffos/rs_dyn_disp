use crate::Thing;

struct ContainerOfThings {
    /// A Vec of Thing objects
    pub things: Vec<Box<dyn Thing>>,
}

impl ContainerOfThings {
    pub fn new() -> Self {
        Self { things: Vec::new() }
    }

    pub fn add_thing(&mut self, thing: Box<dyn Thing>) {
        self.things.push(thing);
    }

    pub fn add_multiple_things(&mut self, things: Vec<Box<dyn Thing>>) {
        for doc in things {
            self.add_thing(doc);
        }
    }
}
