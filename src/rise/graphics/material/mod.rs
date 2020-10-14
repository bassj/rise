mod uniform;

pub use uniform::{CameraUniform, Uniform, UniformBinding};

mod material;

pub use material::{Material,  MaterialBuilder};

mod material_instance;

pub use material_instance::{MaterialInstance, MaterialInstanceBuilder};