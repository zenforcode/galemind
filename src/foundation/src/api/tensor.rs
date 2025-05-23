pub enum Data {
    VFLOAT(Vec<f64>),
}
#[derive(PartialEq)]
pub enum DataType {
    VFLOAT,
}

pub type DataShape = Vec<usize>;
