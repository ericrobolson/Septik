use crate::backend;
pub use backend::{Node, Position, SysEvent};

mod textbox;

pub trait Component<TMsg> {
    fn update(&mut self, sys_events: &SysEvent) -> Vec<TMsg>;
}

pub trait ViewComponent {
    fn view(&self) -> Node;
}
