#![feature(test)]
extern crate seeded_game_of_life;
extern crate test;

use seeded_game_of_life::universe::Universe;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
