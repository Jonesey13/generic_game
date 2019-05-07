use crate::geometry::*;

#[derive(Clone)]
pub struct BezierQuad {
    c0: Point,
    c1: Point,
    c2: Point
}

impl BezierQuad {
    pub fn new(c0: Point, c1: Point, c2: Point) -> BezierQuad {
        BezierQuad {
            c0: c0,
            c1: c1,
            c2: c2
        }
    }

    pub fn from_line(line: Line) -> BezierQuad {
        BezierQuad {
            c0: line.beg,
            c1: line.get_point(0.5),
            c2: line.end
        }
    }
    
    pub fn eval(&self, t: f64) -> Point {
        (1.0 - t) * (1.0 - t) * self.c0  + 2.0 * t * (1.0 - t) * self.c1  + t * t * self.c2 
    }

    pub fn eval_derivative(&self, t: f64) -> Point {
        -2.0 * t * self.c0  + (2.0 - 4.0 * t) * self.c1  + 2.0 * t * self.c2
    }

    pub fn get_sub_bezier(&self, start: f64, end: f64) -> BezierQuad {
        let d0 = self.eval(start);
        let d1 = 0.5 * (end - start) * self.eval_derivative(start) + d0;
        let d2 = self.eval(end);

        BezierQuad {
            c0: d0,
            c1: d1,
            c2: d2
        }
    }
}
