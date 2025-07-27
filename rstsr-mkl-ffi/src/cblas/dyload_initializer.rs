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
            cblas_dcabs1: get_symbol(&libs, b"cblas_dcabs1\0").map(|sym| *sym),
            cblas_scabs1: get_symbol(&libs, b"cblas_scabs1\0").map(|sym| *sym),
            cblas_sdot: get_symbol(&libs, b"cblas_sdot\0").map(|sym| *sym),
            cblas_sdoti: get_symbol(&libs, b"cblas_sdoti\0").map(|sym| *sym),
            cblas_ddot: get_symbol(&libs, b"cblas_ddot\0").map(|sym| *sym),
            cblas_ddoti: get_symbol(&libs, b"cblas_ddoti\0").map(|sym| *sym),
            cblas_dsdot: get_symbol(&libs, b"cblas_dsdot\0").map(|sym| *sym),
            cblas_sdsdot: get_symbol(&libs, b"cblas_sdsdot\0").map(|sym| *sym),
            cblas_cdotu_sub: get_symbol(&libs, b"cblas_cdotu_sub\0").map(|sym| *sym),
            cblas_cdotui_sub: get_symbol(&libs, b"cblas_cdotui_sub\0").map(|sym| *sym),
            cblas_cdotc_sub: get_symbol(&libs, b"cblas_cdotc_sub\0").map(|sym| *sym),
            cblas_cdotci_sub: get_symbol(&libs, b"cblas_cdotci_sub\0").map(|sym| *sym),
            cblas_zdotu_sub: get_symbol(&libs, b"cblas_zdotu_sub\0").map(|sym| *sym),
            cblas_zdotui_sub: get_symbol(&libs, b"cblas_zdotui_sub\0").map(|sym| *sym),
            cblas_zdotc_sub: get_symbol(&libs, b"cblas_zdotc_sub\0").map(|sym| *sym),
            cblas_zdotci_sub: get_symbol(&libs, b"cblas_zdotci_sub\0").map(|sym| *sym),
            cblas_snrm2: get_symbol(&libs, b"cblas_snrm2\0").map(|sym| *sym),
            cblas_sasum: get_symbol(&libs, b"cblas_sasum\0").map(|sym| *sym),
            cblas_dnrm2: get_symbol(&libs, b"cblas_dnrm2\0").map(|sym| *sym),
            cblas_dasum: get_symbol(&libs, b"cblas_dasum\0").map(|sym| *sym),
            cblas_scnrm2: get_symbol(&libs, b"cblas_scnrm2\0").map(|sym| *sym),
            cblas_scasum: get_symbol(&libs, b"cblas_scasum\0").map(|sym| *sym),
            cblas_dznrm2: get_symbol(&libs, b"cblas_dznrm2\0").map(|sym| *sym),
            cblas_dzasum: get_symbol(&libs, b"cblas_dzasum\0").map(|sym| *sym),
            cblas_isamax: get_symbol(&libs, b"cblas_isamax\0").map(|sym| *sym),
            cblas_idamax: get_symbol(&libs, b"cblas_idamax\0").map(|sym| *sym),
            cblas_icamax: get_symbol(&libs, b"cblas_icamax\0").map(|sym| *sym),
            cblas_izamax: get_symbol(&libs, b"cblas_izamax\0").map(|sym| *sym),
            cblas_isamin: get_symbol(&libs, b"cblas_isamin\0").map(|sym| *sym),
            cblas_idamin: get_symbol(&libs, b"cblas_idamin\0").map(|sym| *sym),
            cblas_icamin: get_symbol(&libs, b"cblas_icamin\0").map(|sym| *sym),
            cblas_izamin: get_symbol(&libs, b"cblas_izamin\0").map(|sym| *sym),
            cblas_sswap: get_symbol(&libs, b"cblas_sswap\0").map(|sym| *sym),
            cblas_scopy: get_symbol(&libs, b"cblas_scopy\0").map(|sym| *sym),
            cblas_saxpy: get_symbol(&libs, b"cblas_saxpy\0").map(|sym| *sym),
            cblas_saxpby: get_symbol(&libs, b"cblas_saxpby\0").map(|sym| *sym),
            cblas_saxpyi: get_symbol(&libs, b"cblas_saxpyi\0").map(|sym| *sym),
            cblas_sgthr: get_symbol(&libs, b"cblas_sgthr\0").map(|sym| *sym),
            cblas_sgthrz: get_symbol(&libs, b"cblas_sgthrz\0").map(|sym| *sym),
            cblas_ssctr: get_symbol(&libs, b"cblas_ssctr\0").map(|sym| *sym),
            cblas_srotg: get_symbol(&libs, b"cblas_srotg\0").map(|sym| *sym),
            cblas_dswap: get_symbol(&libs, b"cblas_dswap\0").map(|sym| *sym),
            cblas_dcopy: get_symbol(&libs, b"cblas_dcopy\0").map(|sym| *sym),
            cblas_daxpy: get_symbol(&libs, b"cblas_daxpy\0").map(|sym| *sym),
            cblas_daxpby: get_symbol(&libs, b"cblas_daxpby\0").map(|sym| *sym),
            cblas_daxpyi: get_symbol(&libs, b"cblas_daxpyi\0").map(|sym| *sym),
            cblas_dgthr: get_symbol(&libs, b"cblas_dgthr\0").map(|sym| *sym),
            cblas_dgthrz: get_symbol(&libs, b"cblas_dgthrz\0").map(|sym| *sym),
            cblas_dsctr: get_symbol(&libs, b"cblas_dsctr\0").map(|sym| *sym),
            cblas_drotg: get_symbol(&libs, b"cblas_drotg\0").map(|sym| *sym),
            cblas_cswap: get_symbol(&libs, b"cblas_cswap\0").map(|sym| *sym),
            cblas_ccopy: get_symbol(&libs, b"cblas_ccopy\0").map(|sym| *sym),
            cblas_caxpy: get_symbol(&libs, b"cblas_caxpy\0").map(|sym| *sym),
            cblas_caxpby: get_symbol(&libs, b"cblas_caxpby\0").map(|sym| *sym),
            cblas_caxpyi: get_symbol(&libs, b"cblas_caxpyi\0").map(|sym| *sym),
            cblas_cgthr: get_symbol(&libs, b"cblas_cgthr\0").map(|sym| *sym),
            cblas_cgthrz: get_symbol(&libs, b"cblas_cgthrz\0").map(|sym| *sym),
            cblas_csctr: get_symbol(&libs, b"cblas_csctr\0").map(|sym| *sym),
            cblas_crotg: get_symbol(&libs, b"cblas_crotg\0").map(|sym| *sym),
            cblas_zswap: get_symbol(&libs, b"cblas_zswap\0").map(|sym| *sym),
            cblas_zcopy: get_symbol(&libs, b"cblas_zcopy\0").map(|sym| *sym),
            cblas_zaxpy: get_symbol(&libs, b"cblas_zaxpy\0").map(|sym| *sym),
            cblas_zaxpby: get_symbol(&libs, b"cblas_zaxpby\0").map(|sym| *sym),
            cblas_zaxpyi: get_symbol(&libs, b"cblas_zaxpyi\0").map(|sym| *sym),
            cblas_zgthr: get_symbol(&libs, b"cblas_zgthr\0").map(|sym| *sym),
            cblas_zgthrz: get_symbol(&libs, b"cblas_zgthrz\0").map(|sym| *sym),
            cblas_zsctr: get_symbol(&libs, b"cblas_zsctr\0").map(|sym| *sym),
            cblas_zrotg: get_symbol(&libs, b"cblas_zrotg\0").map(|sym| *sym),
            cblas_srotmg: get_symbol(&libs, b"cblas_srotmg\0").map(|sym| *sym),
            cblas_sroti: get_symbol(&libs, b"cblas_sroti\0").map(|sym| *sym),
            cblas_srotm: get_symbol(&libs, b"cblas_srotm\0").map(|sym| *sym),
            cblas_drotmg: get_symbol(&libs, b"cblas_drotmg\0").map(|sym| *sym),
            cblas_drotm: get_symbol(&libs, b"cblas_drotm\0").map(|sym| *sym),
            cblas_droti: get_symbol(&libs, b"cblas_droti\0").map(|sym| *sym),
            cblas_sscal: get_symbol(&libs, b"cblas_sscal\0").map(|sym| *sym),
            cblas_dscal: get_symbol(&libs, b"cblas_dscal\0").map(|sym| *sym),
            cblas_cscal: get_symbol(&libs, b"cblas_cscal\0").map(|sym| *sym),
            cblas_zscal: get_symbol(&libs, b"cblas_zscal\0").map(|sym| *sym),
            cblas_csscal: get_symbol(&libs, b"cblas_csscal\0").map(|sym| *sym),
            cblas_zdscal: get_symbol(&libs, b"cblas_zdscal\0").map(|sym| *sym),
            cblas_srot: get_symbol(&libs, b"cblas_srot\0").map(|sym| *sym),
            cblas_drot: get_symbol(&libs, b"cblas_drot\0").map(|sym| *sym),
            cblas_crot: get_symbol(&libs, b"cblas_crot\0").map(|sym| *sym),
            cblas_zrot: get_symbol(&libs, b"cblas_zrot\0").map(|sym| *sym),
            cblas_csrot: get_symbol(&libs, b"cblas_csrot\0").map(|sym| *sym),
            cblas_zdrot: get_symbol(&libs, b"cblas_zdrot\0").map(|sym| *sym),
            cblas_sgemv: get_symbol(&libs, b"cblas_sgemv\0").map(|sym| *sym),
            cblas_sgbmv: get_symbol(&libs, b"cblas_sgbmv\0").map(|sym| *sym),
            cblas_strmv: get_symbol(&libs, b"cblas_strmv\0").map(|sym| *sym),
            cblas_stbmv: get_symbol(&libs, b"cblas_stbmv\0").map(|sym| *sym),
            cblas_stpmv: get_symbol(&libs, b"cblas_stpmv\0").map(|sym| *sym),
            cblas_strsv: get_symbol(&libs, b"cblas_strsv\0").map(|sym| *sym),
            cblas_stbsv: get_symbol(&libs, b"cblas_stbsv\0").map(|sym| *sym),
            cblas_stpsv: get_symbol(&libs, b"cblas_stpsv\0").map(|sym| *sym),
            cblas_dgemv: get_symbol(&libs, b"cblas_dgemv\0").map(|sym| *sym),
            cblas_dgbmv: get_symbol(&libs, b"cblas_dgbmv\0").map(|sym| *sym),
            cblas_dtrmv: get_symbol(&libs, b"cblas_dtrmv\0").map(|sym| *sym),
            cblas_dtbmv: get_symbol(&libs, b"cblas_dtbmv\0").map(|sym| *sym),
            cblas_dtpmv: get_symbol(&libs, b"cblas_dtpmv\0").map(|sym| *sym),
            cblas_dtrsv: get_symbol(&libs, b"cblas_dtrsv\0").map(|sym| *sym),
            cblas_dtbsv: get_symbol(&libs, b"cblas_dtbsv\0").map(|sym| *sym),
            cblas_dtpsv: get_symbol(&libs, b"cblas_dtpsv\0").map(|sym| *sym),
            cblas_cgemv: get_symbol(&libs, b"cblas_cgemv\0").map(|sym| *sym),
            cblas_cgbmv: get_symbol(&libs, b"cblas_cgbmv\0").map(|sym| *sym),
            cblas_ctrmv: get_symbol(&libs, b"cblas_ctrmv\0").map(|sym| *sym),
            cblas_ctbmv: get_symbol(&libs, b"cblas_ctbmv\0").map(|sym| *sym),
            cblas_ctpmv: get_symbol(&libs, b"cblas_ctpmv\0").map(|sym| *sym),
            cblas_ctrsv: get_symbol(&libs, b"cblas_ctrsv\0").map(|sym| *sym),
            cblas_ctbsv: get_symbol(&libs, b"cblas_ctbsv\0").map(|sym| *sym),
            cblas_ctpsv: get_symbol(&libs, b"cblas_ctpsv\0").map(|sym| *sym),
            cblas_zgemv: get_symbol(&libs, b"cblas_zgemv\0").map(|sym| *sym),
            cblas_zgbmv: get_symbol(&libs, b"cblas_zgbmv\0").map(|sym| *sym),
            cblas_ztrmv: get_symbol(&libs, b"cblas_ztrmv\0").map(|sym| *sym),
            cblas_ztbmv: get_symbol(&libs, b"cblas_ztbmv\0").map(|sym| *sym),
            cblas_ztpmv: get_symbol(&libs, b"cblas_ztpmv\0").map(|sym| *sym),
            cblas_ztrsv: get_symbol(&libs, b"cblas_ztrsv\0").map(|sym| *sym),
            cblas_ztbsv: get_symbol(&libs, b"cblas_ztbsv\0").map(|sym| *sym),
            cblas_ztpsv: get_symbol(&libs, b"cblas_ztpsv\0").map(|sym| *sym),
            cblas_ssymv: get_symbol(&libs, b"cblas_ssymv\0").map(|sym| *sym),
            cblas_ssbmv: get_symbol(&libs, b"cblas_ssbmv\0").map(|sym| *sym),
            cblas_sspmv: get_symbol(&libs, b"cblas_sspmv\0").map(|sym| *sym),
            cblas_sger: get_symbol(&libs, b"cblas_sger\0").map(|sym| *sym),
            cblas_ssyr: get_symbol(&libs, b"cblas_ssyr\0").map(|sym| *sym),
            cblas_sspr: get_symbol(&libs, b"cblas_sspr\0").map(|sym| *sym),
            cblas_ssyr2: get_symbol(&libs, b"cblas_ssyr2\0").map(|sym| *sym),
            cblas_sspr2: get_symbol(&libs, b"cblas_sspr2\0").map(|sym| *sym),
            cblas_dsymv: get_symbol(&libs, b"cblas_dsymv\0").map(|sym| *sym),
            cblas_dsbmv: get_symbol(&libs, b"cblas_dsbmv\0").map(|sym| *sym),
            cblas_dspmv: get_symbol(&libs, b"cblas_dspmv\0").map(|sym| *sym),
            cblas_dger: get_symbol(&libs, b"cblas_dger\0").map(|sym| *sym),
            cblas_dsyr: get_symbol(&libs, b"cblas_dsyr\0").map(|sym| *sym),
            cblas_dspr: get_symbol(&libs, b"cblas_dspr\0").map(|sym| *sym),
            cblas_dsyr2: get_symbol(&libs, b"cblas_dsyr2\0").map(|sym| *sym),
            cblas_dspr2: get_symbol(&libs, b"cblas_dspr2\0").map(|sym| *sym),
            cblas_chemv: get_symbol(&libs, b"cblas_chemv\0").map(|sym| *sym),
            cblas_chbmv: get_symbol(&libs, b"cblas_chbmv\0").map(|sym| *sym),
            cblas_chpmv: get_symbol(&libs, b"cblas_chpmv\0").map(|sym| *sym),
            cblas_cgeru: get_symbol(&libs, b"cblas_cgeru\0").map(|sym| *sym),
            cblas_cgerc: get_symbol(&libs, b"cblas_cgerc\0").map(|sym| *sym),
            cblas_cher: get_symbol(&libs, b"cblas_cher\0").map(|sym| *sym),
            cblas_chpr: get_symbol(&libs, b"cblas_chpr\0").map(|sym| *sym),
            cblas_cher2: get_symbol(&libs, b"cblas_cher2\0").map(|sym| *sym),
            cblas_chpr2: get_symbol(&libs, b"cblas_chpr2\0").map(|sym| *sym),
            cblas_zhemv: get_symbol(&libs, b"cblas_zhemv\0").map(|sym| *sym),
            cblas_zhbmv: get_symbol(&libs, b"cblas_zhbmv\0").map(|sym| *sym),
            cblas_zhpmv: get_symbol(&libs, b"cblas_zhpmv\0").map(|sym| *sym),
            cblas_zgeru: get_symbol(&libs, b"cblas_zgeru\0").map(|sym| *sym),
            cblas_zgerc: get_symbol(&libs, b"cblas_zgerc\0").map(|sym| *sym),
            cblas_zher: get_symbol(&libs, b"cblas_zher\0").map(|sym| *sym),
            cblas_zhpr: get_symbol(&libs, b"cblas_zhpr\0").map(|sym| *sym),
            cblas_zher2: get_symbol(&libs, b"cblas_zher2\0").map(|sym| *sym),
            cblas_zhpr2: get_symbol(&libs, b"cblas_zhpr2\0").map(|sym| *sym),
            cblas_sgemm: get_symbol(&libs, b"cblas_sgemm\0").map(|sym| *sym),
            cblas_sgemm_batch: get_symbol(&libs, b"cblas_sgemm_batch\0").map(|sym| *sym),
            cblas_sgemm_batch_strided: get_symbol(&libs, b"cblas_sgemm_batch_strided\0")
                .map(|sym| *sym),
            cblas_sgemmt: get_symbol(&libs, b"cblas_sgemmt\0").map(|sym| *sym),
            cblas_ssymm: get_symbol(&libs, b"cblas_ssymm\0").map(|sym| *sym),
            cblas_ssyrk: get_symbol(&libs, b"cblas_ssyrk\0").map(|sym| *sym),
            cblas_ssyrk_batch_strided: get_symbol(&libs, b"cblas_ssyrk_batch_strided\0")
                .map(|sym| *sym),
            cblas_ssyrk_batch: get_symbol(&libs, b"cblas_ssyrk_batch\0").map(|sym| *sym),
            cblas_ssyr2k: get_symbol(&libs, b"cblas_ssyr2k\0").map(|sym| *sym),
            cblas_strmm: get_symbol(&libs, b"cblas_strmm\0").map(|sym| *sym),
            cblas_strmm_oop: get_symbol(&libs, b"cblas_strmm_oop\0").map(|sym| *sym),
            cblas_strsm: get_symbol(&libs, b"cblas_strsm\0").map(|sym| *sym),
            cblas_strsm_oop: get_symbol(&libs, b"cblas_strsm_oop\0").map(|sym| *sym),
            cblas_strsm_batch: get_symbol(&libs, b"cblas_strsm_batch\0").map(|sym| *sym),
            cblas_strsm_batch_strided: get_symbol(&libs, b"cblas_strsm_batch_strided\0")
                .map(|sym| *sym),
            cblas_dgemm: get_symbol(&libs, b"cblas_dgemm\0").map(|sym| *sym),
            cblas_dgemm_batch: get_symbol(&libs, b"cblas_dgemm_batch\0").map(|sym| *sym),
            cblas_dgemm_batch_strided: get_symbol(&libs, b"cblas_dgemm_batch_strided\0")
                .map(|sym| *sym),
            cblas_dgemmt: get_symbol(&libs, b"cblas_dgemmt\0").map(|sym| *sym),
            cblas_dsymm: get_symbol(&libs, b"cblas_dsymm\0").map(|sym| *sym),
            cblas_dsyrk: get_symbol(&libs, b"cblas_dsyrk\0").map(|sym| *sym),
            cblas_dsyrk_batch: get_symbol(&libs, b"cblas_dsyrk_batch\0").map(|sym| *sym),
            cblas_dsyrk_batch_strided: get_symbol(&libs, b"cblas_dsyrk_batch_strided\0")
                .map(|sym| *sym),
            cblas_dsyr2k: get_symbol(&libs, b"cblas_dsyr2k\0").map(|sym| *sym),
            cblas_dtrmm: get_symbol(&libs, b"cblas_dtrmm\0").map(|sym| *sym),
            cblas_dtrmm_oop: get_symbol(&libs, b"cblas_dtrmm_oop\0").map(|sym| *sym),
            cblas_dtrsm: get_symbol(&libs, b"cblas_dtrsm\0").map(|sym| *sym),
            cblas_dtrsm_oop: get_symbol(&libs, b"cblas_dtrsm_oop\0").map(|sym| *sym),
            cblas_dtrsm_batch: get_symbol(&libs, b"cblas_dtrsm_batch\0").map(|sym| *sym),
            cblas_dtrsm_batch_strided: get_symbol(&libs, b"cblas_dtrsm_batch_strided\0")
                .map(|sym| *sym),
            cblas_cgemm: get_symbol(&libs, b"cblas_cgemm\0").map(|sym| *sym),
            cblas_cgemm3m: get_symbol(&libs, b"cblas_cgemm3m\0").map(|sym| *sym),
            cblas_cgemm_batch: get_symbol(&libs, b"cblas_cgemm_batch\0").map(|sym| *sym),
            cblas_cgemm_batch_strided: get_symbol(&libs, b"cblas_cgemm_batch_strided\0")
                .map(|sym| *sym),
            cblas_cgemm3m_batch: get_symbol(&libs, b"cblas_cgemm3m_batch\0").map(|sym| *sym),
            cblas_cgemm3m_batch_strided: get_symbol(&libs, b"cblas_cgemm3m_batch_strided\0")
                .map(|sym| *sym),
            cblas_cgemmt: get_symbol(&libs, b"cblas_cgemmt\0").map(|sym| *sym),
            cblas_csymm: get_symbol(&libs, b"cblas_csymm\0").map(|sym| *sym),
            cblas_csyrk: get_symbol(&libs, b"cblas_csyrk\0").map(|sym| *sym),
            cblas_csyrk_batch: get_symbol(&libs, b"cblas_csyrk_batch\0").map(|sym| *sym),
            cblas_csyrk_batch_strided: get_symbol(&libs, b"cblas_csyrk_batch_strided\0")
                .map(|sym| *sym),
            cblas_csyr2k: get_symbol(&libs, b"cblas_csyr2k\0").map(|sym| *sym),
            cblas_ctrmm: get_symbol(&libs, b"cblas_ctrmm\0").map(|sym| *sym),
            cblas_ctrmm_oop: get_symbol(&libs, b"cblas_ctrmm_oop\0").map(|sym| *sym),
            cblas_ctrsm: get_symbol(&libs, b"cblas_ctrsm\0").map(|sym| *sym),
            cblas_ctrsm_oop: get_symbol(&libs, b"cblas_ctrsm_oop\0").map(|sym| *sym),
            cblas_ctrsm_batch: get_symbol(&libs, b"cblas_ctrsm_batch\0").map(|sym| *sym),
            cblas_ctrsm_batch_strided: get_symbol(&libs, b"cblas_ctrsm_batch_strided\0")
                .map(|sym| *sym),
            cblas_zgemm: get_symbol(&libs, b"cblas_zgemm\0").map(|sym| *sym),
            cblas_zgemm3m: get_symbol(&libs, b"cblas_zgemm3m\0").map(|sym| *sym),
            cblas_zgemm_batch: get_symbol(&libs, b"cblas_zgemm_batch\0").map(|sym| *sym),
            cblas_zgemm_batch_strided: get_symbol(&libs, b"cblas_zgemm_batch_strided\0")
                .map(|sym| *sym),
            cblas_zgemm3m_batch: get_symbol(&libs, b"cblas_zgemm3m_batch\0").map(|sym| *sym),
            cblas_zgemm3m_batch_strided: get_symbol(&libs, b"cblas_zgemm3m_batch_strided\0")
                .map(|sym| *sym),
            cblas_zgemmt: get_symbol(&libs, b"cblas_zgemmt\0").map(|sym| *sym),
            cblas_zsymm: get_symbol(&libs, b"cblas_zsymm\0").map(|sym| *sym),
            cblas_zsyrk: get_symbol(&libs, b"cblas_zsyrk\0").map(|sym| *sym),
            cblas_zsyrk_batch: get_symbol(&libs, b"cblas_zsyrk_batch\0").map(|sym| *sym),
            cblas_zsyrk_batch_strided: get_symbol(&libs, b"cblas_zsyrk_batch_strided\0")
                .map(|sym| *sym),
            cblas_zsyr2k: get_symbol(&libs, b"cblas_zsyr2k\0").map(|sym| *sym),
            cblas_ztrmm: get_symbol(&libs, b"cblas_ztrmm\0").map(|sym| *sym),
            cblas_ztrmm_oop: get_symbol(&libs, b"cblas_ztrmm_oop\0").map(|sym| *sym),
            cblas_ztrsm: get_symbol(&libs, b"cblas_ztrsm\0").map(|sym| *sym),
            cblas_ztrsm_oop: get_symbol(&libs, b"cblas_ztrsm_oop\0").map(|sym| *sym),
            cblas_ztrsm_batch: get_symbol(&libs, b"cblas_ztrsm_batch\0").map(|sym| *sym),
            cblas_ztrsm_batch_strided: get_symbol(&libs, b"cblas_ztrsm_batch_strided\0")
                .map(|sym| *sym),
            cblas_chemm: get_symbol(&libs, b"cblas_chemm\0").map(|sym| *sym),
            cblas_cherk: get_symbol(&libs, b"cblas_cherk\0").map(|sym| *sym),
            cblas_cher2k: get_symbol(&libs, b"cblas_cher2k\0").map(|sym| *sym),
            cblas_zhemm: get_symbol(&libs, b"cblas_zhemm\0").map(|sym| *sym),
            cblas_zherk: get_symbol(&libs, b"cblas_zherk\0").map(|sym| *sym),
            cblas_zher2k: get_symbol(&libs, b"cblas_zher2k\0").map(|sym| *sym),
            cblas_sgemm_pack_get_size: get_symbol(&libs, b"cblas_sgemm_pack_get_size\0")
                .map(|sym| *sym),
            cblas_sgemm_pack: get_symbol(&libs, b"cblas_sgemm_pack\0").map(|sym| *sym),
            cblas_sgemm_compute: get_symbol(&libs, b"cblas_sgemm_compute\0").map(|sym| *sym),
            cblas_dgemm_pack_get_size: get_symbol(&libs, b"cblas_dgemm_pack_get_size\0")
                .map(|sym| *sym),
            cblas_dgemm_pack: get_symbol(&libs, b"cblas_dgemm_pack\0").map(|sym| *sym),
            cblas_dgemm_compute: get_symbol(&libs, b"cblas_dgemm_compute\0").map(|sym| *sym),
            cblas_hgemm: get_symbol(&libs, b"cblas_hgemm\0").map(|sym| *sym),
            cblas_hgemm_pack_get_size: get_symbol(&libs, b"cblas_hgemm_pack_get_size\0")
                .map(|sym| *sym),
            cblas_hgemm_pack: get_symbol(&libs, b"cblas_hgemm_pack\0").map(|sym| *sym),
            cblas_hgemm_compute: get_symbol(&libs, b"cblas_hgemm_compute\0").map(|sym| *sym),
            cblas_gemm_s16s16s32: get_symbol(&libs, b"cblas_gemm_s16s16s32\0").map(|sym| *sym),
            cblas_gemm_s8u8s32: get_symbol(&libs, b"cblas_gemm_s8u8s32\0").map(|sym| *sym),
            cblas_gemm_bf16bf16f32: get_symbol(&libs, b"cblas_gemm_bf16bf16f32\0").map(|sym| *sym),
            cblas_gemm_f16f16f32: get_symbol(&libs, b"cblas_gemm_f16f16f32\0").map(|sym| *sym),
            cblas_gemm_e5m2e5m2f32: get_symbol(&libs, b"cblas_gemm_e5m2e5m2f32\0").map(|sym| *sym),
            cblas_gemm_e4m3e4m3f32: get_symbol(&libs, b"cblas_gemm_e4m3e4m3f32\0").map(|sym| *sym),
            cblas_gemm_s8u8s32_pack_get_size: get_symbol(
                &libs,
                b"cblas_gemm_s8u8s32_pack_get_size\0",
            )
            .map(|sym| *sym),
            cblas_gemm_s16s16s32_pack_get_size: get_symbol(
                &libs,
                b"cblas_gemm_s16s16s32_pack_get_size\0",
            )
            .map(|sym| *sym),
            cblas_gemm_bf16bf16f32_pack_get_size: get_symbol(
                &libs,
                b"cblas_gemm_bf16bf16f32_pack_get_size\0",
            )
            .map(|sym| *sym),
            cblas_gemm_f16f16f32_pack_get_size: get_symbol(
                &libs,
                b"cblas_gemm_f16f16f32_pack_get_size\0",
            )
            .map(|sym| *sym),
            cblas_gemm_e5m2e5m2f32_pack_get_size: get_symbol(
                &libs,
                b"cblas_gemm_e5m2e5m2f32_pack_get_size\0",
            )
            .map(|sym| *sym),
            cblas_gemm_e4m3e4m3f32_pack_get_size: get_symbol(
                &libs,
                b"cblas_gemm_e4m3e4m3f32_pack_get_size\0",
            )
            .map(|sym| *sym),
            cblas_gemm_s8u8s32_pack: get_symbol(&libs, b"cblas_gemm_s8u8s32_pack\0")
                .map(|sym| *sym),
            cblas_gemm_s16s16s32_pack: get_symbol(&libs, b"cblas_gemm_s16s16s32_pack\0")
                .map(|sym| *sym),
            cblas_gemm_bf16bf16f32_pack: get_symbol(&libs, b"cblas_gemm_bf16bf16f32_pack\0")
                .map(|sym| *sym),
            cblas_gemm_f16f16f32_pack: get_symbol(&libs, b"cblas_gemm_f16f16f32_pack\0")
                .map(|sym| *sym),
            cblas_gemm_e5m2e5m2f32_pack: get_symbol(&libs, b"cblas_gemm_e5m2e5m2f32_pack\0")
                .map(|sym| *sym),
            cblas_gemm_e4m3e4m3f32_pack: get_symbol(&libs, b"cblas_gemm_e4m3e4m3f32_pack\0")
                .map(|sym| *sym),
            cblas_gemm_s8u8s32_compute: get_symbol(&libs, b"cblas_gemm_s8u8s32_compute\0")
                .map(|sym| *sym),
            cblas_gemm_s16s16s32_compute: get_symbol(&libs, b"cblas_gemm_s16s16s32_compute\0")
                .map(|sym| *sym),
            cblas_gemm_bf16bf16f32_compute: get_symbol(&libs, b"cblas_gemm_bf16bf16f32_compute\0")
                .map(|sym| *sym),
            cblas_gemm_f16f16f32_compute: get_symbol(&libs, b"cblas_gemm_f16f16f32_compute\0")
                .map(|sym| *sym),
            cblas_gemm_e5m2e5m2f32_compute: get_symbol(&libs, b"cblas_gemm_e5m2e5m2f32_compute\0")
                .map(|sym| *sym),
            cblas_gemm_e4m3e4m3f32_compute: get_symbol(&libs, b"cblas_gemm_e4m3e4m3f32_compute\0")
                .map(|sym| *sym),
            mkl_cblas_jit_create_dgemm: get_symbol(&libs, b"mkl_cblas_jit_create_dgemm\0")
                .map(|sym| *sym),
            mkl_cblas_jit_create_sgemm: get_symbol(&libs, b"mkl_cblas_jit_create_sgemm\0")
                .map(|sym| *sym),
            mkl_cblas_jit_create_cgemm: get_symbol(&libs, b"mkl_cblas_jit_create_cgemm\0")
                .map(|sym| *sym),
            mkl_cblas_jit_create_zgemm: get_symbol(&libs, b"mkl_cblas_jit_create_zgemm\0")
                .map(|sym| *sym),
            mkl_jit_get_dgemm_ptr: get_symbol(&libs, b"mkl_jit_get_dgemm_ptr\0").map(|sym| *sym),
            mkl_jit_get_sgemm_ptr: get_symbol(&libs, b"mkl_jit_get_sgemm_ptr\0").map(|sym| *sym),
            mkl_jit_get_cgemm_ptr: get_symbol(&libs, b"mkl_jit_get_cgemm_ptr\0").map(|sym| *sym),
            mkl_jit_get_zgemm_ptr: get_symbol(&libs, b"mkl_jit_get_zgemm_ptr\0").map(|sym| *sym),
            mkl_jit_destroy: get_symbol(&libs, b"mkl_jit_destroy\0").map(|sym| *sym),
            cblas_saxpy_batch: get_symbol(&libs, b"cblas_saxpy_batch\0").map(|sym| *sym),
            cblas_daxpy_batch: get_symbol(&libs, b"cblas_daxpy_batch\0").map(|sym| *sym),
            cblas_caxpy_batch: get_symbol(&libs, b"cblas_caxpy_batch\0").map(|sym| *sym),
            cblas_zaxpy_batch: get_symbol(&libs, b"cblas_zaxpy_batch\0").map(|sym| *sym),
            cblas_saxpy_batch_strided: get_symbol(&libs, b"cblas_saxpy_batch_strided\0")
                .map(|sym| *sym),
            cblas_daxpy_batch_strided: get_symbol(&libs, b"cblas_daxpy_batch_strided\0")
                .map(|sym| *sym),
            cblas_caxpy_batch_strided: get_symbol(&libs, b"cblas_caxpy_batch_strided\0")
                .map(|sym| *sym),
            cblas_zaxpy_batch_strided: get_symbol(&libs, b"cblas_zaxpy_batch_strided\0")
                .map(|sym| *sym),
            cblas_scopy_batch: get_symbol(&libs, b"cblas_scopy_batch\0").map(|sym| *sym),
            cblas_dcopy_batch: get_symbol(&libs, b"cblas_dcopy_batch\0").map(|sym| *sym),
            cblas_ccopy_batch: get_symbol(&libs, b"cblas_ccopy_batch\0").map(|sym| *sym),
            cblas_zcopy_batch: get_symbol(&libs, b"cblas_zcopy_batch\0").map(|sym| *sym),
            cblas_scopy_batch_strided: get_symbol(&libs, b"cblas_scopy_batch_strided\0")
                .map(|sym| *sym),
            cblas_dcopy_batch_strided: get_symbol(&libs, b"cblas_dcopy_batch_strided\0")
                .map(|sym| *sym),
            cblas_ccopy_batch_strided: get_symbol(&libs, b"cblas_ccopy_batch_strided\0")
                .map(|sym| *sym),
            cblas_zcopy_batch_strided: get_symbol(&libs, b"cblas_zcopy_batch_strided\0")
                .map(|sym| *sym),
            cblas_sgemv_batch: get_symbol(&libs, b"cblas_sgemv_batch\0").map(|sym| *sym),
            cblas_sgemv_batch_strided: get_symbol(&libs, b"cblas_sgemv_batch_strided\0")
                .map(|sym| *sym),
            cblas_dgemv_batch: get_symbol(&libs, b"cblas_dgemv_batch\0").map(|sym| *sym),
            cblas_dgemv_batch_strided: get_symbol(&libs, b"cblas_dgemv_batch_strided\0")
                .map(|sym| *sym),
            cblas_cgemv_batch: get_symbol(&libs, b"cblas_cgemv_batch\0").map(|sym| *sym),
            cblas_cgemv_batch_strided: get_symbol(&libs, b"cblas_cgemv_batch_strided\0")
                .map(|sym| *sym),
            cblas_zgemv_batch: get_symbol(&libs, b"cblas_zgemv_batch\0").map(|sym| *sym),
            cblas_zgemv_batch_strided: get_symbol(&libs, b"cblas_zgemv_batch_strided\0")
                .map(|sym| *sym),
            cblas_sdgmm_batch: get_symbol(&libs, b"cblas_sdgmm_batch\0").map(|sym| *sym),
            cblas_sdgmm_batch_strided: get_symbol(&libs, b"cblas_sdgmm_batch_strided\0")
                .map(|sym| *sym),
            cblas_ddgmm_batch: get_symbol(&libs, b"cblas_ddgmm_batch\0").map(|sym| *sym),
            cblas_ddgmm_batch_strided: get_symbol(&libs, b"cblas_ddgmm_batch_strided\0")
                .map(|sym| *sym),
            cblas_cdgmm_batch: get_symbol(&libs, b"cblas_cdgmm_batch\0").map(|sym| *sym),
            cblas_cdgmm_batch_strided: get_symbol(&libs, b"cblas_cdgmm_batch_strided\0")
                .map(|sym| *sym),
            cblas_zdgmm_batch: get_symbol(&libs, b"cblas_zdgmm_batch\0").map(|sym| *sym),
            cblas_zdgmm_batch_strided: get_symbol(&libs, b"cblas_zdgmm_batch_strided\0")
                .map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
