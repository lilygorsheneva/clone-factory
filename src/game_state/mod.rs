//! Modules representing the state of the game.

pub mod world;
pub mod db;
pub mod game;

// Playing with static lifetimes. This should, in theory, allow me to give Items
// a reference to ItemDefinition.
#[cfg(test)]
mod test{
    struct StaticDataSample {
        foo: i8,
        bar: i8
    } 

    struct Container_1<'pseudostatic> {
        foo: &'pseudostatic StaticDataSample
    }

    struct Container_2<'pseudostatic> {
        c1: Container_1<'pseudostatic> 
    }

    #[test]
    fn testme() {
        let staticky = StaticDataSample {foo:1, bar: 1};
        let mut c2 = Some(Container_2 {c1: {Container_1 {foo: &staticky}}});

        let c_new = Container_1{foo: c2.unwrap().c1.foo};

        c2 = None;

        assert_eq!(c_new.foo.bar, staticky.bar);
    }
}