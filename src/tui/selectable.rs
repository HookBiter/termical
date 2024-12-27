pub trait Selectable {
    fn select(&mut self);
    fn deselect(&mut self);
    fn mark(&mut self);
    fn unmark(&mut self);
}
