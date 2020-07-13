#[derive(PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0)
    }

    pub fn scale(&self, dx: f32, dy: f32) -> Vector {
        Vector::new(self.x * dx, self.y * dy)
    }

    pub fn translate(&self, dx: f32, dy: f32) -> Vector {
        Vector::new(self.x + dx, self.y + dy)
    }
}
/*
 static Offset increaseVelocity(
   Offset velocity,
   double angle,
   double force,
 ) {
   final xa = cos(angle);
   final ya = sin(angle);
   return velocity.translate(xa * force, ya * force);
 }
*/
