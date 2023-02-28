use crate::Thing;

struct ContainerOfThingRefs<'a> {
    /// A Vec of Thing objects
    pub things: Vec<Box<&'a dyn Thing>>,
}

impl<'a> ContainerOfThingRefs<'a> {
    pub fn new() -> Self {
        Self { things: Vec::new() }
    }

    pub fn add_thing(&mut self, thing: &'a dyn Thing) {
        self.things.push(Box::new(thing));
    }

    pub fn add_multiple_things(&mut self, things: &'a [&'a dyn Thing]) {
        for doc in things {
            self.add_thing(*doc);
        }
    }
}
