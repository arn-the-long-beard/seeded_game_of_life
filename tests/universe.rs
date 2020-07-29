#[cfg(test)]
mod tests {
    use seeded_game_of_life::universe::Universe;

    #[test]
    fn big_bang_works() {
       let universe = Universe::new();


        assert_eq!(universe.width(),64);
        assert_eq!(universe.height(),64);
    }
}