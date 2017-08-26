use collision::{Collidable, CollObj, CollObjPair, CollResults, CollDetails};
use geometry::{ToRenderable, TwoDTransformable};
use rendering::Renderable;
use na::{Vector2, Vector4, Rotation2};

#[derive(Clone)]
pub struct CollObjectWrapper<C: Clone, D: Clone> {
    coll_obj: C,
    coll_results: CollResults<D>,
    object_index: usize,
    coll_obj_prev: Option<C>,
    data: D,
    player_controlled: bool,
    color: Vector4<f64>,
    has_collided_in_past: bool,
    last_collision_details: CollDetails,
}

impl<C:Clone, D: Clone> CollObjectWrapper<C, D> {
    pub fn new(coll_obj: C, object_index: usize, data: D) -> Self {
        CollObjectWrapper {
            coll_obj,
            coll_results: CollResults::no_collision(),
            object_index,
            coll_obj_prev: None,
            data,
            player_controlled: false,
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            has_collided_in_past: false,
            last_collision_details: CollDetails::None,
        }
    }

    pub fn coll_results_color() -> Vector4<f64> {
        Vector4::new(0.0, 1.0, 0.0, 1.0)
    }
}

pub trait CollObjectWrapperTrait: Collidable + TwoDTransformable {
    fn get_obj_pair(&self) -> CollObjPair;
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

impl<C: Clone + CollObj + ToRenderable + TwoDTransformable, D: Clone> CollObjectWrapperTrait for CollObjectWrapper<C, D>  {
    fn get_obj_pair(&self) -> CollObjPair {
        if let Some(ref obj_prev) = self.coll_obj_prev {
            return self.coll_obj.get_object_pair(obj_prev);
        }
        else {
            return self.coll_obj.get_object_pair(&self.coll_obj);
        }
    }

    fn render(&self, depth: f64) -> Box<Renderable> {
        self.coll_obj.to_renderable(self.get_color(), depth, false)
    }

    fn render_coll_results(&self, depth: f64) -> Vec<Box<Renderable>> {
        self.coll_obj.render_collision_details(self.last_collision_details.clone(), CollObjectWrapper::<C,D>::coll_results_color(), depth, false)
    }

    fn set_prev(&mut self) {
        self.coll_obj_prev = Some(self.coll_obj.clone());
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
        self.last_collision_details = CollDetails::None;
    }
}

impl<C: Clone + CollObj + ToRenderable + TwoDTransformable, D: Clone> Collidable for CollObjectWrapper<C, D> {
    type Data = D;

    fn get_collision_object(&self) -> CollObjPair {
        self.get_obj_pair()
    }

    fn get_collision_results(&self) -> CollResults<Self::Data> {
        self.coll_results.clone()
    }

    fn set_collision_results(&mut self, new_results: CollResults<Self::Data>) {
        self.coll_results = new_results.clone();
        if self.has_collided() {
            self.has_collided_in_past = true;
            self.last_collision_details = new_results.details.unwrap();
        }
    }

    fn get_collision_data(&self) -> Self::Data { self.data.clone() }
}

impl<C: Clone + CollObj + ToRenderable + TwoDTransformable, D: Clone> TwoDTransformable for CollObjectWrapper<C, D> {
    fn shift_by(&mut self, shift: Vector2<f64>) {
        self.coll_obj.shift_by(shift);
    }

    fn rotate(&mut self, rot_angle: f64) {
        self.coll_obj.rotate(rot_angle);
    }
}
