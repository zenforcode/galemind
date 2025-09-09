use crate::grpc_server;
use foundation::api::inference::InferParameter; // the generated proto module

impl From<grpc_server::InferParameter> for InferParameter {
    fn from(p: grpc_server::InferParameter) -> Self {
        match p.parameter_choice {
            Some(grpc_server::infer_parameter::ParameterChoice::BoolParam(b)) => Self::Bool(b),
            Some(grpc_server::infer_parameter::ParameterChoice::Int64Param(i)) => Self::Int64(i),
            Some(grpc_server::infer_parameter::ParameterChoice::F64Param(f)) => Self::Double(f),
            Some(grpc_server::infer_parameter::ParameterChoice::StringParam(s)) => Self::String(s),
            None => Self::String("".into()), // safe default or maybe return an error
        }
    }
}

impl From<InferParameter> for grpc_server::InferParameter {
    fn from(p: InferParameter) -> Self {
        use grpc_server::infer_parameter::ParameterChoice;

        let choice = match p {
            InferParameter::Bool(b) => Some(ParameterChoice::BoolParam(b)),
            InferParameter::Int64(i) => Some(ParameterChoice::Int64Param(i)),
            InferParameter::String(s) => Some(ParameterChoice::StringParam(s)),
            InferParameter::Double(d) => {
                // proto doesn’t support double? fallback, e.g., string encode
                Some(ParameterChoice::StringParam(d.to_string()))
            }
        };

        grpc_server::InferParameter {
            parameter_choice: choice,
        }
    }
}
