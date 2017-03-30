pub mod view_details2d;
pub mod view_details3d;
pub use self::view_details2d::ViewDetails2D;
pub use self::view_details3d::ViewDetails3D;

#[derive(Copy, Clone, Debug)]
pub enum ViewDetails {
    TwoDim(ViewDetails2D),
    ThreeDim(ViewDetails3D)
}
