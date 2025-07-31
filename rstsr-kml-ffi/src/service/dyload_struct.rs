//! Library struct definition for dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

pub struct DyLoadLib {
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub KSERVICEGetVersion:
        Option<unsafe extern "C" fn(ver: *mut KSERVICEVersion) -> ::core::ffi::c_int>,
    pub KmlLoadSettings:
        Option<unsafe extern "C" fn(entries: *const KmlSettingEntry, envtriesLen: usize)>,
    pub KmlStoreSettings: Option<unsafe extern "C" fn() -> *const KmlSettingEntry>,
    pub KmlVerbose: Option<unsafe extern "C" fn() -> bool>,
    pub KmlSetSkipZeros: Option<unsafe extern "C" fn(arg1: KmlSkipZerosOption)>,
    pub KmlGetSkipZeros: Option<unsafe extern "C" fn() -> KmlSkipZerosOption>,
    pub KmlGetCfmaAccelerationOption: Option<unsafe extern "C" fn() -> KmlCfmaAccelerationOption>,
    pub KmlSetGemmToGemm3m: Option<unsafe extern "C" fn(arg1: KmlGemmToGemM3MOption)>,
    pub KmlGetGemmToGemm3m: Option<unsafe extern "C" fn() -> KmlGemmToGemM3MOption>,
    pub KmlSetMpiUseSharedMemory: Option<unsafe extern "C" fn(arg1: KmlMpiUseSharedMemoryOption)>,
    pub KmlGetMpiUseSharedMemory: Option<unsafe extern "C" fn() -> KmlMpiUseSharedMemoryOption>,
    pub KmlSetThreadUsingPolicy: Option<unsafe extern "C" fn(arg1: KmlThreadUsingPolicyOption)>,
    pub KmlGetThreadUsingPolicy: Option<unsafe extern "C" fn() -> KmlThreadUsingPolicyOption>,
    pub KmlGetPxgetrfPivotingOption: Option<unsafe extern "C" fn() -> KmlPxgetrfPivotingOption>,
    pub KmlGetEigensolverOption: Option<unsafe extern "C" fn() -> KmlEigensolverOption>,
    pub KmlGetTsqrCheckOption: Option<unsafe extern "C" fn() -> KmlTsqrCheckOption>,
    pub KmlGetFlag: Option<unsafe extern "C" fn(flag: KmlSetting) -> *const KmlSettingValue>,
    pub KmlGetFlagOrDefault: Option<unsafe extern "C" fn(flag: KmlSetting) -> KmlSettingValue>,
    pub KmlSetFlag: Option<unsafe extern "C" fn(flag: KmlSetting, value: KmlSettingValue)>,
    pub KmlAllocate:
        Option<unsafe extern "C" fn(sz: usize, align: usize) -> *mut ::core::ffi::c_void>,
    pub KmlCalloc: Option<
        unsafe extern "C" fn(num: usize, size: usize, align: usize) -> *mut ::core::ffi::c_void,
    >,
    pub KmlFree: Option<unsafe extern "C" fn(p: *mut ::core::ffi::c_void)>,
    pub dsecnd: Option<unsafe extern "C" fn() -> f64>,
    pub second: Option<unsafe extern "C" fn() -> f32>,
    pub second_: Option<unsafe extern "C" fn() -> f32>,
    pub dsecnd_: Option<unsafe extern "C" fn() -> f64>,
    pub KmlGetNumProcs: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub KmlGetMaxThreads: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub KmlSetNumThreads: Option<unsafe extern "C" fn(numThreads: ::core::ffi::c_int)>,
    pub KmlCannotContinue: Option<unsafe extern "C" fn(msg: *const ::core::ffi::c_char)>,
    pub KmlInform: Option<unsafe extern "C" fn(msg: *const ::core::ffi::c_char)>,
    pub Xerbla: Option<
        unsafe extern "C" fn(
            func: *const ::core::ffi::c_char,
            param: *const ::core::ffi::c_int,
            funcLength: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xerbla_: Option<
        unsafe extern "C" fn(
            func: *const ::core::ffi::c_char,
            param: *const ::core::ffi::c_int,
            funcLength: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub KmlGetBuildConfig: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
}
