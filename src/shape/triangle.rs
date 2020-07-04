use crate::shape::ShapeDrawable;
use crate::shape::Line;
pub struct Triangle {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
#[allow(dead_code)]
impl Triangle{
    pub fn new(x0: i32, y0: i32,x1: i32, y1: i32,x2: i32, y2: i32) -> Triangle{
        Triangle{
            x0,
            y0,
            x1,
            y1,
            x2,
            y2,
        }
    }
}
impl ShapeDrawable for Triangle {
    fn get_cells(&self) -> Vec<(i32,i32)>{
        let mut cells: Vec<(i32,i32)> = Vec::new();
        let line_a = Line::new(self.x0,self.y0,self.x1,self.y1);
        let line_b = Line::new(self.x0,self.y0,self.x2,self.y2);
        let line_c = Line::new(self.x1,self.y1,self.x2,self.y2);

        for cell in line_a.get_cells() {
            cells.push(cell);
        }
        for cell in line_b.get_cells() {
            cells.push(cell);
        }
        for cell in line_c.get_cells() {
            cells.push(cell);
        }
        
        
        return cells;
    }
}