//! Modules representing the state of the game.

pub mod db;
pub mod game;
pub mod world;

// Playing with static lifetimes. This should, in theory, allow me to give Items
// a reference to ItemDefinition.
#[cfg(test)]
mod test {

    #[derive(Default)]
    struct ItemDefinition {
        id: i128,
    }

    #[derive(Clone, Copy)]
    struct Item<'ps> {
        def: &'ps ItemDefinition,
        quantity: i16,
    }

    #[derive(Default)]
    struct StaticDataSample {
        foo: Vec<ItemDefinition>,
    }

    #[derive(Default)]
    struct Inventory<'ps> {
        items: Vec<Item<'ps>>,
    }

    impl<'ps> Inventory<'ps> {
        fn get(&self, idx: usize) -> Item<'ps> {
            self.items[idx]
        }

        fn set(&mut self, idx: usize, item: Item<'ps>) {
            self.items.push(item.clone());
        }
    }

    struct Game<'ps> {
        inv: Inventory<'ps>,
        data: &'ps StaticDataSample,
    }

    impl<'ps> Game<'ps> {
        fn giveyourselfoneof(&mut self, idx: usize) {
            self.inv.items.push(Item {
                def: &self.data.foo[idx],
                quantity: 1,
            });
        }
    }

    #[test]
    fn aaaaa() {
        let mut definitions = StaticDataSample::default();
        definitions.foo.push(ItemDefinition { id: 100 });

        let mut game = Game {
            inv: Default::default(),
            data: &definitions,
        };
        game.giveyourselfoneof(0);
        let item = game.inv.get(0);
        game.inv.set(0, item);
    }
}
