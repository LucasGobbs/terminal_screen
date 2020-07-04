
use crate::shape::ShapeDrawable;
pub struct Rect {
    x0: i32,
    y0: i32,
    width: i32,
    height: i32,
    fill: bool,
}
#[allow(dead_code)]
impl Rect{
    pub fn new(x0: i32, y0: i32, width: i32, height: i32, fill: bool) -> Rect{
        Rect{
            x0,
            y0,
            width,
            height,
            fill,
        }
    }
}
impl ShapeDrawable for Rect {
    fn get_cells(&self) -> Vec<(i32,i32)>{
        let mut cells: Vec<(i32,i32)> = Vec::new();
        let x = self.x0;
        let y = self.y0;
        let width = self.width;
        let height = self.height;
        let fill = self.fill;
        for _x in x..width + x {
            for _y in y..height + y {
                if fill {
                    cells.push((_x,_y));
                } else {
                    if  _x == x || _x == width + x - 1 || _y == y || _y == height + y -1 {
                        cells.push((_x,_y));
                    }
                }
            }
        }
        return cells
    }
}