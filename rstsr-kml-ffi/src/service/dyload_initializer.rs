//! Library initializer implementation for dynamic-loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{Library, Symbol};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}

impl DyLoadLib {
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {
        let mut result = DyLoadLib {
            __libraries: vec![],      // dummy here, set this field later
            __libraries_path: vec![], // dummy here, set this field later
            KSERVICEGetVersion: get_symbol(&libs, b"KSERVICEGetVersion\0").map(|sym| *sym),
            KmlLoadSettings: get_symbol(&libs, b"KmlLoadSettings\0").map(|sym| *sym),
            KmlStoreSettings: get_symbol(&libs, b"KmlStoreSettings\0").map(|sym| *sym),
            KmlVerbose: get_symbol(&libs, b"KmlVerbose\0").map(|sym| *sym),
            KmlSetSkipZeros: get_symbol(&libs, b"KmlSetSkipZeros\0").map(|sym| *sym),
            KmlGetSkipZeros: get_symbol(&libs, b"KmlGetSkipZeros\0").map(|sym| *sym),
            KmlGetCfmaAccelerationOption: get_symbol(&libs, b"KmlGetCfmaAccelerationOption\0")
                .map(|sym| *sym),
            KmlSetGemmToGemm3m: get_symbol(&libs, b"KmlSetGemmToGemm3m\0").map(|sym| *sym),
            KmlGetGemmToGemm3m: get_symbol(&libs, b"KmlGetGemmToGemm3m\0").map(|sym| *sym),
            KmlSetMpiUseSharedMemory: get_symbol(&libs, b"KmlSetMpiUseSharedMemory\0")
                .map(|sym| *sym),
            KmlGetMpiUseSharedMemory: get_symbol(&libs, b"KmlGetMpiUseSharedMemory\0")
                .map(|sym| *sym),
            KmlSetThreadUsingPolicy: get_symbol(&libs, b"KmlSetThreadUsingPolicy\0")
                .map(|sym| *sym),
            KmlGetThreadUsingPolicy: get_symbol(&libs, b"KmlGetThreadUsingPolicy\0")
                .map(|sym| *sym),
            KmlGetPxgetrfPivotingOption: get_symbol(&libs, b"KmlGetPxgetrfPivotingOption\0")
                .map(|sym| *sym),
            KmlGetEigensolverOption: get_symbol(&libs, b"KmlGetEigensolverOption\0")
                .map(|sym| *sym),
            KmlGetTsqrCheckOption: get_symbol(&libs, b"KmlGetTsqrCheckOption\0").map(|sym| *sym),
            KmlGetFlag: get_symbol(&libs, b"KmlGetFlag\0").map(|sym| *sym),
            KmlGetFlagOrDefault: get_symbol(&libs, b"KmlGetFlagOrDefault\0").map(|sym| *sym),
            KmlSetFlag: get_symbol(&libs, b"KmlSetFlag\0").map(|sym| *sym),
            KmlAllocate: get_symbol(&libs, b"KmlAllocate\0").map(|sym| *sym),
            KmlCalloc: get_symbol(&libs, b"KmlCalloc\0").map(|sym| *sym),
            KmlFree: get_symbol(&libs, b"KmlFree\0").map(|sym| *sym),
            dsecnd: get_symbol(&libs, b"dsecnd\0").map(|sym| *sym),
            second: get_symbol(&libs, b"second\0").map(|sym| *sym),
            second_: get_symbol(&libs, b"second_\0").map(|sym| *sym),
            dsecnd_: get_symbol(&libs, b"dsecnd_\0").map(|sym| *sym),
            KmlGetNumProcs: get_symbol(&libs, b"KmlGetNumProcs\0").map(|sym| *sym),
            KmlGetMaxThreads: get_symbol(&libs, b"KmlGetMaxThreads\0").map(|sym| *sym),
            KmlSetNumThreads: get_symbol(&libs, b"KmlSetNumThreads\0").map(|sym| *sym),
            KmlCannotContinue: get_symbol(&libs, b"KmlCannotContinue\0").map(|sym| *sym),
            KmlInform: get_symbol(&libs, b"KmlInform\0").map(|sym| *sym),
            Xerbla: get_symbol(&libs, b"Xerbla\0").map(|sym| *sym),
            xerbla_: get_symbol(&libs, b"xerbla_\0").map(|sym| *sym),
            KmlGetBuildConfig: get_symbol(&libs, b"KmlGetBuildConfig\0").map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
