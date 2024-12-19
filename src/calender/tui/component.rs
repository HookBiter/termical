use crossterm::event::Event as Input;
use ratatui::widgets::Widget;

pub trait Component: Widget {
    fn get_focused(&self) -> Option<Box<&dyn Component>>;
    fn handle_input(&mut self, input: Option<Input>);
}
