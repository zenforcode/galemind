/// This module contains the core of application following the ports and adapters architecture.
/// It should not make use of heavy dependencies and should only be dedicated the inference logic.
/// As the application could evolve to a larger scale and as the project aims to fit different needs
/// this module could evolve to a library, letting sub-projects adapt it to their technical needs.
pub mod api;
pub mod inference;
