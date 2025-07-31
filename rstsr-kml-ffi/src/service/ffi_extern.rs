//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn KSERVICEGetVersion(ver: *mut KSERVICEVersion) -> ::core::ffi::c_int;
    pub fn KmlLoadSettings(entries: *const KmlSettingEntry, envtriesLen: usize);
    pub fn KmlStoreSettings() -> *const KmlSettingEntry;
    pub fn KmlVerbose() -> bool;
    pub fn KmlSetSkipZeros(arg1: KmlSkipZerosOption);
    pub fn KmlGetSkipZeros() -> KmlSkipZerosOption;
    pub fn KmlGetCfmaAccelerationOption() -> KmlCfmaAccelerationOption;
    pub fn KmlSetGemmToGemm3m(arg1: KmlGemmToGemM3MOption);
    pub fn KmlGetGemmToGemm3m() -> KmlGemmToGemM3MOption;
    pub fn KmlSetMpiUseSharedMemory(arg1: KmlMpiUseSharedMemoryOption);
    pub fn KmlGetMpiUseSharedMemory() -> KmlMpiUseSharedMemoryOption;
    pub fn KmlSetThreadUsingPolicy(arg1: KmlThreadUsingPolicyOption);
    pub fn KmlGetThreadUsingPolicy() -> KmlThreadUsingPolicyOption;
    pub fn KmlGetPxgetrfPivotingOption() -> KmlPxgetrfPivotingOption;
    pub fn KmlGetEigensolverOption() -> KmlEigensolverOption;
    pub fn KmlGetTsqrCheckOption() -> KmlTsqrCheckOption;
    pub fn KmlGetFlag(flag: KmlSetting) -> *const KmlSettingValue;
    pub fn KmlGetFlagOrDefault(flag: KmlSetting) -> KmlSettingValue;
    pub fn KmlSetFlag(flag: KmlSetting, value: KmlSettingValue);
    pub fn KmlAllocate(sz: usize, align: usize) -> *mut ::core::ffi::c_void;
    pub fn KmlCalloc(num: usize, size: usize, align: usize) -> *mut ::core::ffi::c_void;
    pub fn KmlFree(p: *mut ::core::ffi::c_void);
    pub fn dsecnd() -> f64;
    pub fn second() -> f32;
    pub fn second_() -> f32;
    pub fn dsecnd_() -> f64;
    pub fn KmlGetNumProcs() -> ::core::ffi::c_int;
    pub fn KmlGetMaxThreads() -> ::core::ffi::c_int;
    pub fn KmlSetNumThreads(numThreads: ::core::ffi::c_int);
    pub fn KmlCannotContinue(msg: *const ::core::ffi::c_char, ...);
    pub fn KmlInform(msg: *const ::core::ffi::c_char, ...);
    pub fn Xerbla(
        func: *const ::core::ffi::c_char,
        param: *const ::core::ffi::c_int,
        funcLength: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn xerbla_(
        func: *const ::core::ffi::c_char,
        param: *const ::core::ffi::c_int,
        funcLength: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn KmlGetBuildConfig() -> *const ::core::ffi::c_char;
}
