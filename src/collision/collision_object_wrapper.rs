use collision::{Collidable, CollObj, CollisionObjectState, CollisionObjectResults, CollisionObjectDetails};
use geometry::{ToRenderable, TwoDTransformable};
use rendering::Renderable;
use na::{Vector2, Vector4, Rotation2};

#[derive(Clone)]
pub struct CollisionObjectWrapper<C: Clone, D: Clone> {
    collision_object: C,
    coll_results: CollisionObjectResults<D>,
    object_index: usize,
    collision_object_prev: Option<C>,
    data: D,
    player_controlled: bool,
    color: Vector4<f64>,
    has_collided_in_past: bool,
    last_collision_details: CollisionObjectDetails,
}

impl<C:Clone, D: Clone> CollisionObjectWrapper<C, D> {
    pub fn new(collision_object: C, object_index: usize, data: D) -> Self {
        CollisionObjectWrapper {
            collision_object,
            coll_results: CollisionObjectResults::no_collision(),
            object_index,
            collision_object_prev: None,
            data,
            player_controlled: false,
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            has_collided_in_past: false,
            last_collision_details: CollisionObjectDetails::None,
        }
    }

    pub fn coll_results_color() -> Vector4<f64> {
        Vector4::new(0.0, 1.0, 0.0, 1.0)
    }
}

pub trait CollisionObjectWrapperTrait: Collidable + TwoDTransformable {
    fn get_obj_pair(&self) -> CollisionObjectState;
    fn render(&self, depth: f64) -> Box<Renderable>;
    fn set_prev(&mut self);
    fn set_player_control(&mut self, flag: bool);
    fn is_player_controlled(&self) -> bool;
    fn get_object_index(&self) -> usize;
    fn get_color(&self) -> Vector4<f64>;
    fn set_color(&mut self, color: Vector4<f64>);
    fn has_collided_in_past(&self) -> bool;
    fn reset_collision_flag(&mut self);
    fn render_coll_results(&self, depth: f64) -> Vec<Box<Renderable>>;
}

impl<C: Clone + CollObj + ToRenderable + TwoDTransformable, D: Clone> CollisionObjectWrapperTrait for CollisionObjectWrapper<C, D>  {
    fn get_obj_pair(&self) -> CollisionObjectState {
        if let Some(ref obj_prev) = self.collision_object_prev {
            return self.collision_object.get_object_pair(obj_prev);
        }
        else {
            return self.collision_object.get_object_pair(&self.collision_object);
        }
    }

    fn render(&self, depth: f64) -> Box<Renderable> {
        self.collision_object.to_renderable(self.get_color(), depth, false)
    }

    fn render_coll_results(&self, depth: f64) -> Vec<Box<Renderable>> {
        self.collision_object.render_collision_details(self.last_collision_details.clone(), CollisionObjectWrapper::<C,D>::coll_results_color(), depth, false)
    }

    fn set_prev(&mut self) {
        self.collision_object_prev = Some(self.collision_object.clone());
    }

    fn set_player_control(&mut self, flag: bool) {
        self.player_controlled = flag;
    }

    fn is_player_controlled(&self) -> bool {
        self.player_controlled
    }

    fn get_object_index(&self) -> usize {
        self.object_index
    }

    fn get_color(&self) -> Vector4<f64> {
        self.color
    }

    fn set_color(&mut self, color: Vector4<f64>) {
        self.color = color;
    }

    fn has_collided_in_past(&self) -> bool {
        self.has_collided_in_past
    }

    fn reset_collision_flag(&mut self) {
        self.has_collided_in_past = false;
        self.last_collision_details = CollisionObjectDetails::None;
    }
}

impl<C: Clone + CollObj + ToRenderable + TwoDTransformable, D: Clone> Collidable for CollisionObjectWrapper<C, D> {
    type Data = D;

    fn get_collision_objects(&self) -> Vec<CollisionObjectState> {
       vec![ self.get_obj_pair()]
    }

    fn get_collision_object_results(&self) -> CollisionObjectResults<Self::Data> {
        self.coll_results.clone()
    }

    fn set_collision_object_results(&mut self, new_results: CollisionObjectResults<Self::Data>) {
        self.coll_results = new_results.clone();
        if self.has_collided() {
            self.has_collided_in_past = true;
            self.last_collision_details = new_results.details.unwrap();
        }
    }

    fn get_collision_data(&self) -> Self::Data { self.data.clone() }
}

impl<C: Clone + CollObj + ToRenderable + TwoDTransformable, D: Clone> TwoDTransformable for CollisionObjectWrapper<C, D> {
    fn shift_by(&mut self, shift: Vector2<f64>) {
        self.collision_object.shift_by(shift);
    }

    fn rotate(&mut self, rot_angle: f64) {
        self.collision_object.rotate(rot_angle);
    }
}
