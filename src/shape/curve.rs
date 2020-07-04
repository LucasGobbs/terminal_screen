use crate::shape::ShapeDrawable;
pub struct Curve {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Curve {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> Curve {
        Curve {
            x0,
            y0,
            x1,
            y1,
            x2,
            y2,
        }
    }
}
impl ShapeDrawable for Curve {
    fn get_cells(&self) -> Vec<(i32,i32)>{
        let mut cells: Vec<(i32,i32)> = Vec::new();


        cells
    }   
}