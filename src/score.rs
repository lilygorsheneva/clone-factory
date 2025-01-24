use crate::engine::update::{Delta, Updatable};

pub struct Score(pub i64);
impl Updatable for Score{}

#[derive(Debug)]
pub struct ScoreDelta(pub i64);

impl Delta for ScoreDelta{
type Target = Score;

fn new() -> Self {
    ScoreDelta(0)
}
fn apply(&self, target: &mut Self::Target) -> crate::error::Result<()> {
    target.0 += self.0;
    Ok(())
}
}
