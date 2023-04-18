// Associated type bounds specify that a type must have a certain associated type. 
// For example, this code specifies that the generic type K must have a Key 
// associated type, which can be used to index into a collection:

trait Collection {
    type Key;
    fn get(&self, key: Self::Key) -> Option<&Self::Item>;
    type Item;
}

fn main() {
    println!("Hello, world!");
}

fn get_first<C: Collection>(collection: &C) -> Option<&C::Item> {
    collection.get(collection::Key::first())
}

