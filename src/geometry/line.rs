use na::{Vector4};
use ::geometry::*;
use rendering;
use collision::{ToCollisionObjects, CollisionObject};
use collision;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub beg: Point,
    pub end: Point
}

impl Line {
    pub fn new(beg: Point, end: Point) -> Line {
        Line {
            beg: beg,
            end: end
        }
    }

    pub fn new_ref(beg: &Point, end: &Point) -> Line {
        Line {
            beg: *beg,
            end: *end
        }
    }

    // alpha = 0 => beg, alpha = 1 => end
    pub fn get_point(&self, alpha: f64) -> Point {
        (1.0 - alpha) * self.beg  + alpha * self.end 
    }

    pub fn get_midpoint(&self) -> Point {
        self.get_point(0.5)
    }

    pub fn get_diff(&self) -> Point {
        self.end - self.beg
    }

    pub fn get_length(&self) -> f64 {
        (self.end - self.beg).norm()
    }

    pub fn get_angle(&self) -> f64 {
        let shifted_end = self.end - self.beg;
        shifted_end.y.atan2(shifted_end.x)
    }

    pub fn get_direction(&self) -> Point {
        (self.end - self.beg).normalized()
    }

    pub fn get_normal(&self) -> Point {
        let dir = self.get_diff();
        vect::get_normal_2d(dir)
    }

    pub fn get_unnormalized_normal(&self) -> Point {
        let dir = self.get_diff();
        vect::get_rot90_2d(dir)
    }

    pub fn shifted_by(&self, move_vec: Point) -> Line {
        Line {
            beg: self.beg + move_vec,
            end: self.end + move_vec
        }
    }

    pub fn get_lines_to(&self, other: Line) -> (Line, Line) {
        (Line::new(self.beg, other.beg), Line::new(self.end, other.end))
    }
}

impl Poly for Line {
    fn get_corners(&self) -> Vec<Point> {
        vec![self.beg, self.end]
    }
    fn set_corners(&mut self, corners: Vec<Point>) {
        self.beg = corners[0];
        self.end = corners[1];
    }
}

impl TwoDTransformable for Line {
    fn shift_by(&mut self, shift: Point) {
        self.beg += shift;
        self.end += shift;
    }

    fn rotate(&mut self, rot_angle: f64) {
        let rot_mat = Rotation::new(rot_angle);
        let center = self.get_point(0.5);
        self.beg = rot_mat * (self.beg - center) + center;
        self.end = rot_mat * (self.end - center) + center;
    }
}

impl ToRenderables for Line {
    fn to_renderables(&self, color: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<rendering::StandardRenderable>> {
        let line_length = (self.end - self.beg).norm();
        vec![
            Box::new(rendering::Line::new_square(self.beg, self.end, line_length / 100.0, color, depth, fixed))
        ]
    }
}

impl Line {
    pub fn render_collision_details(&self, line_info: collision::LineInfo, color: Vector4<f64>, depth: f64, fixed: bool) 
    -> Vec<Box<rendering::StandardRenderable>> {
        let coll_pos_rendering: Box<ToRenderables> = match line_info {
            collision::LineInfo::LineBeg(_) => Box::new(self.beg),
            collision::LineInfo::LineEnd(_) => Box::new(self.end),
            collision::LineInfo::Point(x, collision::LineSide::Left) => Box::new(self.get_point(1.0 - x)),
            collision::LineInfo::Point(x, collision::LineSide::Right) => Box::new(self.get_point(x)),            
            collision::LineInfo::WholeLine(_) => Box::new(self.clone()),
        };

        let (coll_pos, coll_dir) = match line_info {
            collision::LineInfo::LineBeg(dir) => (self.beg, dir),
            collision::LineInfo::LineEnd(dir) => (self.end, dir),
            collision::LineInfo::Point(x, collision::LineSide::Right) => (self.get_point(x), -self.get_normal()),            
            collision::LineInfo::Point(x, collision::LineSide::Left) => (self.get_point(1.0 - x), self.get_normal()),
            collision::LineInfo::WholeLine(collision::LineSide::Right) => (self.get_point(0.5), -self.get_normal()),
            collision::LineInfo::WholeLine(collision::LineSide::Left) => (self.get_point(0.5), self.get_normal()),            
        };

        let direction_renderable: Box<rendering::StandardRenderable> = Box::new(
            rendering::Arrow::new_for_coll_test(
                    coll_pos,
                    coll_dir,
                    color,
                    depth,
                    fixed
            )
        );

        let mut renderables = coll_pos_rendering.to_renderables(color, depth, fixed);
        renderables.push(direction_renderable);
        renderables
    }
}

impl ToCollisionObjects for Line {
    fn to_collision_objects(&self) -> Vec<CollisionObject> {
        vec![
            CollisionObject::Line(self.clone())
        ]
    }
}

/// For the line beg <=> t=0 and end <=> t=1
/// For the two values in the DualSoln the first float corresponds to a point on
/// the first line and the second float the second line
pub fn line_line_intersect_2d(line1: &Line, line2: &Line) -> DualSoln {
    let dir1 = line1.get_diff();
    let dir2 = line2.get_diff();
    let normal1 = line1.get_unnormalized_normal();
    let normal2 = line2.get_unnormalized_normal();
    if dir1.dot(&normal2) != 0.0 {
        let t1 = (line2.beg - line1.beg).dot(&normal2) / dir1.dot(&normal2);
        let t2 = (line1.beg - line2.beg).dot(&normal1) / dir2.dot(&normal1);
        return DualSoln::Two(t1, t2);
    }
    DualSoln::None
}

#[cfg(test)]
mod tests
{
    use ::geometry::{line_line_intersect_2d, Line, Point};

    #[test]
    fn line_line_intersect() {
        let line1 = Line::new(Point::new(-0.5, 0.0), Point::new(0.5, 0.0));
        let line2 = Line::new(Point::new(0.3, 1.0), Point::new(0.3, -1.0));
        let soln = line_line_intersect_2d(&line1, &line2);
        assert!(soln.both_within_zero_one(), "soln: {:?}", soln)
    }
}