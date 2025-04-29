use crate::galemind::api::tensor::{DataShape, DataType};

struct ModelMetaDataRequest {
    name: String,
    version: Option<String>
}

enum ModelMetaDataResponse {
    Ok(ModelMetaData),
    Err(ModelMetaDataError)
}

struct ModelMetaData {
    name: String,
    versions: Option<Vec<String>>,
    platform: String,
    inputs: Vec<MetadataTensor>,
    outputs: Vec<MetadataTensor>
}

struct MetadataTensor {
    name: String,
    datatype: DataType,
    shape: DataShape
}

struct ModelMetaDataError {
    error: String
}

trait ModelMetaDataProcessor {
    fn get_model_metadata(&self, request: ModelMetaDataRequest) -> ModelMetaDataResponse;
}