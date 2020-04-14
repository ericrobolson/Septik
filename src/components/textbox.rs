use super::*;

pub struct TextBox {
    is_active: bool,
    text: String,
    prev_text: String,
    position: Position,
}

pub enum Msg {
    ValueChanged(String),
}

impl Component<Msg> for TextBox {
    fn update(&mut self, sys_event: &SysEvent) -> std::vec::Vec<Msg> {
        let mut events = vec![];

        // Clicks
        {
            if sys_event.cursor_clicks.is_empty() == false {
                for click in sys_event.cursor_clicks.iter() {
                    if self.position.contains(&click.x, &click.y) {
                        self.is_active = true;
                        self.prev_text = self.text.clone();
                    } else {
                        if self.is_active == true {
                            self.is_active = false;

                            if self.prev_text != self.text {
                                events.push(Msg::ValueChanged(self.text.clone()));
                            }
                        }
                    }
                }
            }
        }
        if self.is_active {
            //TODO: keypresses
            // Do keypress stuff
        }

        return events;
    }
}

impl ViewComponent for TextBox {
    fn view(&self) -> backend::Node {
        Node::Empty
    }
}
