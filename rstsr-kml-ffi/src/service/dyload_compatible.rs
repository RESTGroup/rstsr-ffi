//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn KSERVICEGetVersion(ver: *mut KSERVICEVersion) -> ::core::ffi::c_int {
    dyload_lib().KSERVICEGetVersion.unwrap()(ver)
}

pub unsafe fn KmlLoadSettings(entries: *const KmlSettingEntry, envtriesLen: usize) {
    dyload_lib().KmlLoadSettings.unwrap()(entries, envtriesLen)
}

pub unsafe fn KmlStoreSettings() -> *const KmlSettingEntry {
    dyload_lib().KmlStoreSettings.unwrap()()
}

pub unsafe fn KmlVerbose() -> bool {
    dyload_lib().KmlVerbose.unwrap()()
}

pub unsafe fn KmlSetSkipZeros(arg1: KmlSkipZerosOption) {
    dyload_lib().KmlSetSkipZeros.unwrap()(arg1)
}

pub unsafe fn KmlGetSkipZeros() -> KmlSkipZerosOption {
    dyload_lib().KmlGetSkipZeros.unwrap()()
}

pub unsafe fn KmlGetCfmaAccelerationOption() -> KmlCfmaAccelerationOption {
    dyload_lib().KmlGetCfmaAccelerationOption.unwrap()()
}

pub unsafe fn KmlSetGemmToGemm3m(arg1: KmlGemmToGemM3MOption) {
    dyload_lib().KmlSetGemmToGemm3m.unwrap()(arg1)
}

pub unsafe fn KmlGetGemmToGemm3m() -> KmlGemmToGemM3MOption {
    dyload_lib().KmlGetGemmToGemm3m.unwrap()()
}

pub unsafe fn KmlSetMpiUseSharedMemory(arg1: KmlMpiUseSharedMemoryOption) {
    dyload_lib().KmlSetMpiUseSharedMemory.unwrap()(arg1)
}

pub unsafe fn KmlGetMpiUseSharedMemory() -> KmlMpiUseSharedMemoryOption {
    dyload_lib().KmlGetMpiUseSharedMemory.unwrap()()
}

pub unsafe fn KmlSetThreadUsingPolicy(arg1: KmlThreadUsingPolicyOption) {
    dyload_lib().KmlSetThreadUsingPolicy.unwrap()(arg1)
}

pub unsafe fn KmlGetThreadUsingPolicy() -> KmlThreadUsingPolicyOption {
    dyload_lib().KmlGetThreadUsingPolicy.unwrap()()
}

pub unsafe fn KmlGetPxgetrfPivotingOption() -> KmlPxgetrfPivotingOption {
    dyload_lib().KmlGetPxgetrfPivotingOption.unwrap()()
}

pub unsafe fn KmlGetEigensolverOption() -> KmlEigensolverOption {
    dyload_lib().KmlGetEigensolverOption.unwrap()()
}

pub unsafe fn KmlGetTsqrCheckOption() -> KmlTsqrCheckOption {
    dyload_lib().KmlGetTsqrCheckOption.unwrap()()
}

pub unsafe fn KmlGetFlag(flag: KmlSetting) -> *const KmlSettingValue {
    dyload_lib().KmlGetFlag.unwrap()(flag)
}

pub unsafe fn KmlGetFlagOrDefault(flag: KmlSetting) -> KmlSettingValue {
    dyload_lib().KmlGetFlagOrDefault.unwrap()(flag)
}

pub unsafe fn KmlSetFlag(flag: KmlSetting, value: KmlSettingValue) {
    dyload_lib().KmlSetFlag.unwrap()(flag, value)
}

pub unsafe fn KmlAllocate(sz: usize, align: usize) -> *mut ::core::ffi::c_void {
    dyload_lib().KmlAllocate.unwrap()(sz, align)
}

pub unsafe fn KmlCalloc(num: usize, size: usize, align: usize) -> *mut ::core::ffi::c_void {
    dyload_lib().KmlCalloc.unwrap()(num, size, align)
}

pub unsafe fn KmlFree(p: *mut ::core::ffi::c_void) {
    dyload_lib().KmlFree.unwrap()(p)
}

pub unsafe fn dsecnd() -> f64 {
    dyload_lib().dsecnd.unwrap()()
}

pub unsafe fn second() -> f32 {
    dyload_lib().second.unwrap()()
}

pub unsafe fn second_() -> f32 {
    dyload_lib().second_.unwrap()()
}

pub unsafe fn dsecnd_() -> f64 {
    dyload_lib().dsecnd_.unwrap()()
}

pub unsafe fn KmlGetNumProcs() -> ::core::ffi::c_int {
    dyload_lib().KmlGetNumProcs.unwrap()()
}

pub unsafe fn KmlGetMaxThreads() -> ::core::ffi::c_int {
    dyload_lib().KmlGetMaxThreads.unwrap()()
}

pub unsafe fn KmlSetNumThreads(numThreads: ::core::ffi::c_int) {
    dyload_lib().KmlSetNumThreads.unwrap()(numThreads)
}

pub unsafe fn KmlCannotContinue(msg: *const ::core::ffi::c_char) {
    dyload_lib().KmlCannotContinue.unwrap()(msg)
}

pub unsafe fn KmlInform(msg: *const ::core::ffi::c_char) {
    dyload_lib().KmlInform.unwrap()(msg)
}

pub unsafe fn Xerbla(
    func: *const ::core::ffi::c_char,
    param: *const ::core::ffi::c_int,
    funcLength: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    dyload_lib().Xerbla.unwrap()(func, param, funcLength)
}

pub unsafe fn xerbla_(
    func: *const ::core::ffi::c_char,
    param: *const ::core::ffi::c_int,
    funcLength: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    dyload_lib().xerbla_.unwrap()(func, param, funcLength)
}

pub unsafe fn KmlGetBuildConfig() -> *const ::core::ffi::c_char {
    dyload_lib().KmlGetBuildConfig.unwrap()()
}
