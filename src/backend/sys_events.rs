pub struct SysEvent {
    pub cursor_clicks: Vec<EvClick>,
    pub cursor_releases: Vec<EvRelease>,
    pub cursor_moves: Vec<EvCursorMove>,
}

impl SysEvent {
    pub fn new() -> Self {
        return Self {
            cursor_clicks: vec![],
            cursor_releases: vec![],
            cursor_moves: vec![],
        };
    }

    pub fn add_cursor_move(&mut self, x: u32, y: u32) {
        let move_event = EvCursorMove { x: x, y: y };
        self.cursor_moves.push(move_event);
    }

    pub fn add_cursor_release(&mut self, x: u32, y: u32) {
        let release = EvRelease { x: x, y: y };
        self.cursor_releases.push(release);
    }

    pub fn add_cursor_click(&mut self, x: u32, y: u32) {
        let click = EvClick { x: x, y: y };
        self.cursor_clicks.push(click);
    }
}

pub struct EvClick {
    pub x: u32,
    pub y: u32,
}

pub struct EvRelease {
    pub x: u32,
    pub y: u32,
}

pub struct EvCursorMove {
    pub x: u32,
    pub y: u32,
}
pub struct KeyPress(char);
