use crate::shape::ShapeDrawable;
pub struct Circle {
    x0: i32,
    y0: i32,
    radius: i32,
}
impl Circle{
    pub fn new(x0: i32, y0: i32, radius: i32) -> Circle{
        Circle{
            x0,
            y0,
            radius,
        }
    }
}
impl ShapeDrawable for Circle {
    fn get_cells(&self) -> Vec<(i32,i32)>{
        let mut cells: Vec<(i32,i32)> = Vec::new();
        let x0 = self.x0;
        let y0 = self.y0;
        let mut r = self.radius;
        let mut x = -r;
        let mut y = 0;
        let mut err = 2-2*r; /* II. Quadrant */ 
        loop{
            cells.push((x0 - x,  y0 + y));
            cells.push((x0 - y,  y0 - x));
            cells.push((x0 + x,  y0 - y));
            cells.push((x0 + y,  y0 + x));
            r = err;
            if r <= y {
                y += 1;
                err += y*2+1; 
            }          /* e_xy+e_y < 0 */
            if r > x || err > y {
                x += 1;
                err += x*2+1; 
            }
            if x > 0 {
                break;
            }
        }


        return cells;
    }
}