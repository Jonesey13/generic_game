pub trait CollisionDataType {
    fn has_exclusion_rules() -> bool {false}
    fn can_collide(_first: &Self, _second: &Self) -> bool {true}
}