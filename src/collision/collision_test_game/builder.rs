use super::{CollisionTestGame, CollisionTestObject};
use collision::collidable_wrapper::{CollidableWrapper, CollidableWrapperTrait};
use collision::Collider;
use geometry::{ConPoly, Line, Circle, Point};

pub struct CollisionTestBuilder {
    polys: Vec<CollidableWrapper<ConPoly, CollisionTestObject>>,
    circles: Vec<CollidableWrapper<Circle, CollisionTestObject>>,
    lines: Vec<CollidableWrapper<Line, CollisionTestObject>>,
    points: Vec<CollidableWrapper<Point, CollisionTestObject>>,
    object_index: usize
}

impl Default for CollisionTestBuilder {
    fn default() -> Self {
        CollisionTestBuilder {
            polys: Vec::new(),
            circles: Vec::new(),
            lines: Vec::new(),
            points: Vec::new(),
            object_index: 0
        }
    }
}

impl CollisionTestBuilder {
    pub fn init() -> CollisionTestBuilder {
        Self::default()
    }

    pub fn add_poly<'a> (&'a mut self, con_poly: ConPoly) -> &'a mut Self {
        let con_poly_wrapper = CollidableWrapper::new(
            con_poly,
            self.object_index,
            CollisionTestObject::Poly
        );
        self.polys.push(con_poly_wrapper);
        self.object_index += 1;        
        self
    }

    pub fn add_circle<'a> (&'a mut self, circle: Circle) -> &'a mut Self {
        let circle_wrapper = CollidableWrapper::new(
            circle,
            self.object_index,
            CollisionTestObject::Circle
        );
        self.circles.push(circle_wrapper);
        self.object_index += 1;        
        self
    }

    pub fn add_point<'a> (&'a mut self, point: Point) -> &'a mut Self {
        let point_wrapper = CollidableWrapper::new(
            point,
            self.object_index,
            CollisionTestObject::Point
        );
        self.points.push(point_wrapper);
        self.object_index += 1;
        self
    }

    pub fn add_line<'a> (&'a mut self, line: Line) -> &'a mut Self {
        let line_wrapper = CollidableWrapper::new(
            line,
            self.object_index,
            CollisionTestObject::Line
        );
        self.lines.push(line_wrapper);
        self.object_index += 1;        
        self
    }

    pub fn build_game(&mut self) -> CollisionTestGame {
        CollisionTestGame {
            polys: self.polys.clone(),
            lines: self.lines.clone(),
            points: self.points.clone(),
            circles: self.circles.clone(),
            collider: Collider,
            external_input: Default::default(),
            game_input: Default::default(),
            mouse_mov: Point::zero(),
            mouse_speed: 0.01,
            player_index: 0
        }
    }
}
