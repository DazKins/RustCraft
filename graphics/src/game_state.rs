pub trait GameState {
    fn tick(&mut self);
    fn render(&mut self);
    fn init(&mut self);
}
