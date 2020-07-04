use crate::shape::ShapeDrawable;
pub struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}
impl Line {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Line {
        Line {
            x0,
            y0,
            x1,
            y1,
        }
    }
}
impl ShapeDrawable for Line {
    fn get_cells(&self) -> Vec<(i32,i32)>{
        let mut cells: Vec<(i32,i32)> = Vec::new();

        let mut x0 = self.x0;
        let x1 = self.x1;
        let mut y0 = self.y0;
        let y1 = self.y1;

        let dx = (x1-x0).abs();
        let sx = if x0<x1 { 1 } else { -1 };
        let dy = -(y1-y0).abs();
        let sy = if y0<y1 { 1 } else { -1 }; 
        let mut err = dx+dy;
        let mut e2: i32; /* error value e_xy */
        loop {
      
            cells.push((x0, y0));
            //thick
            //cells.push((x0 + 1, y0));
            //cells.push((x0 - 1, y0));
            //cells.push((x0, y0 + 1));
            //cells.push((x0, y0 -1 ));

            //thick
            if x0==x1 && y0==y1{
                break;
            }
            e2 = 2*err;
            if e2 >= dy { err += dy; x0 += sx; } /* e_xy+e_x > 0 */
            if e2 <= dx { err += dx; y0 += sy; } 
        }
        cells
    }   
}