#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "dynamic_loading")]
mod dynamic_loading_specific {
    use super::*;
    use std::sync::OnceLock;

    pub unsafe fn dyload_lib() -> &'static Lib {
        static LIB: OnceLock<Lib> = OnceLock::new();
        LIB.get_or_init(|| Lib::new(vec![]))
    }
}

#[cfg(feature = "dynamic_loading")]
pub use dynamic_loading_specific::*;

/* #region general configuration */

pub(crate) mod ffi_base;
pub use ffi_base::*;

#[cfg(not(feature = "dynamic_loading"))]
pub(crate) mod ffi_extern;
#[cfg(not(feature = "dynamic_loading"))]
pub use ffi_extern::*;

#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_compatible;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_initializer;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_struct;

#[cfg(feature = "dynamic_loading")]
mod dynamic_loading_general {
    use super::*;

    pub use dyload_compatible::*;
    pub use dyload_struct::*;
}

#[cfg(feature = "dynamic_loading")]
pub use dynamic_loading_general::*;

/* #endregion */
