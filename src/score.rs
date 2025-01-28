use crate::engine::update::{Delta, Updatable};

pub struct Score{
    pub score: i64,
    pub turn: i64
}
impl Updatable for Score{}

#[derive(Debug)]

pub struct ScoreDelta{
    pub score: i64,
}
impl Delta for ScoreDelta{
type Target = Score;

fn new() -> Self {
    ScoreDelta{score: 0}
}
fn apply(&self, target: &mut Self::Target) -> crate::error::Result<()> {
    target.score += self.score;
    Ok(())
}
}
