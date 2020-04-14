pub mod backend;
use backend::*;
pub mod components;
use components::*;
pub mod data_structures;

struct Root;
struct Msg;

impl Component<Msg> for Root {
    fn update(&mut self, _: &SysEvent) -> std::vec::Vec<Msg> {
        vec![]
    }
}

impl ViewComponent for Root {
    fn view(&self) -> backend::Node {
        Node::Empty
    }
}

pub fn main() {
    let mut backend = build_backend();

    let mut root = Root;
    loop {
        let events = backend.poll_events();

        root.update(&events);

        backend.render(root.view());
    }
}
