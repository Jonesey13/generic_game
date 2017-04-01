pub mod view_details2d;
pub use self::view_details2d::ViewDetails2D;
pub mod view_details3d;
pub use self::view_details3d::ViewDetails3D;
pub mod polar_viewdetails;
pub use self::polar_viewdetails::PolarViewDetails;

#[derive(Copy, Clone, Debug)]
pub enum ViewDetails {
    TwoDim(ViewDetails2D),
    ThreeDim(ViewDetails3D),
    Polar(PolarViewDetails)
}
