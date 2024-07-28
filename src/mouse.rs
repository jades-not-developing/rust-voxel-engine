#[derive(Default)]
pub struct Mouse {
    dx: f64,
    dy: f64,
    last_dx: f64,
    last_dy: f64,
    locked: bool,
}

impl Mouse {
    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn handle_move_x(&mut self, x: f64) {
        if self.locked {
            self.last_dx = x;
        }
        self.dx = x;
    }

    pub fn handle_move_y(&mut self, y: f64) {
        if self.locked {
            self.last_dy = y;
        }
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

    /// # Safety
    /// This method is marked `unsafe` because using this will not signal
    /// that the mouse has moved at all. Please use `Mouse::handle_move`
    /// for general mouse movement.
    pub unsafe fn force_set_position(&mut self, x: f64, y: f64) {
        self.dx = x;
        self.dy = y;

        self.last_dx = x;
        self.last_dy = y;
    }
}