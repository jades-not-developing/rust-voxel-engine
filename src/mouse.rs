#[derive(Default)]
pub struct Mouse {
    dx: f64,
    dy: f64,
    last_dx: f64,
    last_dy: f64,
}

impl Mouse {
    pub fn handle_move_x(&mut self, x: f64) {
        self.dx = x;
    }

    pub fn handle_move_y(&mut self, y: f64) {
        self.dy = y;
    }

    pub fn handle_move(&mut self, x: f64, y: f64) {
        self.handle_move_x(x);
        self.handle_move_y(y);
    }

    pub fn dx(&mut self) -> f64 {
        let value = self.dx - self.last_dx;
        self.last_dx = self.dx;
        value
    }

    pub fn dy(&mut self) -> f64 {
        let value = self.dy - self.last_dy;
        self.last_dy = self.dy;
        value
    }
}