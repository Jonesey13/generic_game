use super::{CollisionObjectResults, Collidable, CollisionObjectState, collision_logic};

pub struct Collider;

impl Collider {
    pub fn process_all<T: Clone> (mut collidables: Vec<&mut Collidable<Data=T>>) {
        for collidable in collidables.iter_mut() {
            collidable.set_collision_object_results(CollisionObjectResults::no_collision());
        }

        let mut collidables_with_objects: Vec<(&mut &mut Collidable<Data=T>, CollisionObjectState)>
            = collidables.iter_mut()
            .map(|collidable| { let collision_object = collidable.get_collision_objects()[0].clone(); (collidable, collision_object) })
            .collect();

        loop {
            if let Some ((&mut (ref mut first_collidable, ref mut first_obj), ref mut rest)) = collidables_with_objects.split_last_mut() {
                for &mut(ref mut second_collidable, ref mut second_obj) in rest.iter_mut() {
                    let (mut results1, mut results2) = Collider::process_pair(&first_obj, &second_obj);

                    if results1.collided || results2.collided {
                        let data1 = first_collidable.get_collision_data();
                        let data2 = second_collidable.get_collision_data();

                        if results1.collided {
                            results1.data = Some(data2);
                            first_collidable.set_collision_object_results(results1);
                        }

                        if results2.collided {
                            results2.data = Some(data1);
                            second_collidable.set_collision_object_results(results2);
                        }
                    }
                }
            }
            else {
                break;
            }
            collidables_with_objects.pop();
        }
    }

    fn process_pair<T: Clone> (first: &CollisionObjectState, second: &CollisionObjectState) -> (CollisionObjectResults<T>, CollisionObjectResults<T>) {
        match (first, second) {
            // Reflexive (points can't collide with points)
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => collision_logic::circ_circ_coll(&n1, &p1, &n2, &p2),
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => collision_logic::poly_poly_coll(n1, p1, n2, p2),
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2))
                => {let res = collision_logic::poly_poly_coll(n1, p1, n2, p2); (res.0.to_line_results(), res.1.to_line_results())},

            // Symmetric
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => collision_logic::circ_poly_coll(&n1, &p1, n2, p2),
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n2, &p2, n1, p1); (res.1, res.0)},

            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => {let res = collision_logic::poly_poly_coll(n2, p2, n1, p1); (res.1.to_line_results(), res.0)},
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::poly_poly_coll(n1, p1, n2, p2); (res.0, res.1.to_line_results())},

            (&CollisionObjectState::Point(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n2, p2, &n1, &p1); (res.1, res.0)},
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Point(ref n2, ref p2)) 
                => collision_logic::poly_point_coll(n1, p1, &n2, &p2),

            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n1, &p1, n2, p2); (res.0, res.1.to_line_results())},
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n2, &p2, n1, p1); (res.1.to_line_results(), res.0)},
            
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Point(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n1, p1, n2, p2); (res.0.to_line_results(), res.1)},
            (&CollisionObjectState::Point(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n2, p2, n1, p1); (res.1, res.0.to_line_results())},
            
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Point(ref n2, ref p2)) 
                => collision_logic::circ_point_coll(&n1, &p1, n2, p2),
            (&CollisionObjectState::Point(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_point_coll(&n2, &p2, n1, p1); (res.1, res.0)},
            
            _ => (CollisionObjectResults::no_collision(), CollisionObjectResults::no_collision()),
        }
    }
}