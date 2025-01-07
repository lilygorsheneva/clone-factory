//! Modules representing the state of the game.

pub mod db;
pub mod game;
pub mod world;

trait Update<'b, UpdateType> {
    fn new_update(&'b self) -> UpdateType;

    fn apply_update(&mut self, new: UpdateType) -> Result<(), crate::error::Status>;
}

#[cfg(test)]
mod test {
    use super::Update;

    pub struct Placeholder {

    }

    pub struct View<'a> {
        state: usize,
        static_data: &'a Placeholder,
    }

    pub struct UpdateView<'b> {
        state: &'b usize,
        static_data: &'b Placeholder,
        update: usize,
    }

    impl<'b> Update<'b, UpdateView<'b>> for View<'_> {
        fn new_update(&'b self) -> UpdateView<'b> {
            UpdateView {
                state: &self.state,
                static_data: self.static_data,
                update: 0,
            }
        }

        fn apply_update(&mut self, new: UpdateView<'b>) -> Result<(), crate::error::Status> {
            self.state = *new.state;
            Ok(())
        }
    }
}
