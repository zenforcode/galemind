// This module aims to expose the ports of the inference server to the outside world.
// The stub exposed by this module should be implemented by the code provided by the corresponding
// core modules provided under the `galemind` module.
pub mod inference;
pub mod model;
pub mod tensor;
