use na::{Vector2, Rotation2};
use num::Zero;
use super::{CollisionTestGame, CollisionTestObject};
use collision::collision_object_wrapper::{CollisionObjectWrapper, CollisionObjectWrapperTrait};
use collision::Collider;
use geometry::{ConPoly, Line, Circle, Point};

pub struct CollisionTestBuilder {
    polys: Vec<CollisionObjectWrapper<ConPoly, CollisionTestObject>>,
    circles: Vec<CollisionObjectWrapper<Circle, CollisionTestObject>>,
    lines: Vec<CollisionObjectWrapper<Line, CollisionTestObject>>,
    points: Vec<CollisionObjectWrapper<Point, CollisionTestObject>>,
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
        let con_poly_wrapper = CollisionObjectWrapper::new(
            con_poly,
            self.object_index,
            CollisionTestObject::Poly
        );
        self.polys.push(con_poly_wrapper);
        self.object_index += 1;        
        self
    }

    pub fn add_circle<'a> (&'a mut self, circle: Circle) -> &'a mut Self {
        let circle_wrapper = CollisionObjectWrapper::new(
            circle,
            self.object_index,
            CollisionTestObject::Circle
        );
        self.circles.push(circle_wrapper);
        self.object_index += 1;        
        self
    }

    pub fn add_point<'a> (&'a mut self, point: Point) -> &'a mut Self {
        let point_wrapper = CollisionObjectWrapper::new(
            point,
            self.object_index,
            CollisionTestObject::Point
        );
        self.points.push(point_wrapper);
        self.object_index += 1;
        self
    }

    pub fn add_line<'a> (&'a mut self, line: Line) -> &'a mut Self {
        let line_wrapper = CollisionObjectWrapper::new(
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
            mouse_mov: Vector2::zero(),
            mouse_speed: 0.01,
            player_index: 0
        }
    }
}