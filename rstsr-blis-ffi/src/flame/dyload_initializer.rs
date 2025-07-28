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
            bl1_s2: get_symbol(&libs, b"bl1_s2\0").map(|sym| *sym),
            bl1_d2: get_symbol(&libs, b"bl1_d2\0").map(|sym| *sym),
            bl1_c2: get_symbol(&libs, b"bl1_c2\0").map(|sym| *sym),
            bl1_z2: get_symbol(&libs, b"bl1_z2\0").map(|sym| *sym),
            bl1_s1: get_symbol(&libs, b"bl1_s1\0").map(|sym| *sym),
            bl1_d1: get_symbol(&libs, b"bl1_d1\0").map(|sym| *sym),
            bl1_c1: get_symbol(&libs, b"bl1_c1\0").map(|sym| *sym),
            bl1_z1: get_symbol(&libs, b"bl1_z1\0").map(|sym| *sym),
            bl1_s1h: get_symbol(&libs, b"bl1_s1h\0").map(|sym| *sym),
            bl1_d1h: get_symbol(&libs, b"bl1_d1h\0").map(|sym| *sym),
            bl1_c1h: get_symbol(&libs, b"bl1_c1h\0").map(|sym| *sym),
            bl1_z1h: get_symbol(&libs, b"bl1_z1h\0").map(|sym| *sym),
            bl1_s0: get_symbol(&libs, b"bl1_s0\0").map(|sym| *sym),
            bl1_d0: get_symbol(&libs, b"bl1_d0\0").map(|sym| *sym),
            bl1_c0: get_symbol(&libs, b"bl1_c0\0").map(|sym| *sym),
            bl1_z0: get_symbol(&libs, b"bl1_z0\0").map(|sym| *sym),
            bl1_sm1h: get_symbol(&libs, b"bl1_sm1h\0").map(|sym| *sym),
            bl1_dm1h: get_symbol(&libs, b"bl1_dm1h\0").map(|sym| *sym),
            bl1_cm1h: get_symbol(&libs, b"bl1_cm1h\0").map(|sym| *sym),
            bl1_zm1h: get_symbol(&libs, b"bl1_zm1h\0").map(|sym| *sym),
            bl1_sm1: get_symbol(&libs, b"bl1_sm1\0").map(|sym| *sym),
            bl1_dm1: get_symbol(&libs, b"bl1_dm1\0").map(|sym| *sym),
            bl1_cm1: get_symbol(&libs, b"bl1_cm1\0").map(|sym| *sym),
            bl1_zm1: get_symbol(&libs, b"bl1_zm1\0").map(|sym| *sym),
            bl1_sm2: get_symbol(&libs, b"bl1_sm2\0").map(|sym| *sym),
            bl1_dm2: get_symbol(&libs, b"bl1_dm2\0").map(|sym| *sym),
            bl1_cm2: get_symbol(&libs, b"bl1_cm2\0").map(|sym| *sym),
            bl1_zm2: get_symbol(&libs, b"bl1_zm2\0").map(|sym| *sym),
            bl1_vallocv: get_symbol(&libs, b"bl1_vallocv\0").map(|sym| *sym),
            bl1_iallocv: get_symbol(&libs, b"bl1_iallocv\0").map(|sym| *sym),
            bl1_sallocv: get_symbol(&libs, b"bl1_sallocv\0").map(|sym| *sym),
            bl1_dallocv: get_symbol(&libs, b"bl1_dallocv\0").map(|sym| *sym),
            bl1_callocv: get_symbol(&libs, b"bl1_callocv\0").map(|sym| *sym),
            bl1_zallocv: get_symbol(&libs, b"bl1_zallocv\0").map(|sym| *sym),
            bl1_vallocm: get_symbol(&libs, b"bl1_vallocm\0").map(|sym| *sym),
            bl1_iallocm: get_symbol(&libs, b"bl1_iallocm\0").map(|sym| *sym),
            bl1_sallocm: get_symbol(&libs, b"bl1_sallocm\0").map(|sym| *sym),
            bl1_dallocm: get_symbol(&libs, b"bl1_dallocm\0").map(|sym| *sym),
            bl1_callocm: get_symbol(&libs, b"bl1_callocm\0").map(|sym| *sym),
            bl1_zallocm: get_symbol(&libs, b"bl1_zallocm\0").map(|sym| *sym),
            bl1_sapdiagmv: get_symbol(&libs, b"bl1_sapdiagmv\0").map(|sym| *sym),
            bl1_dapdiagmv: get_symbol(&libs, b"bl1_dapdiagmv\0").map(|sym| *sym),
            bl1_csapdiagmv: get_symbol(&libs, b"bl1_csapdiagmv\0").map(|sym| *sym),
            bl1_capdiagmv: get_symbol(&libs, b"bl1_capdiagmv\0").map(|sym| *sym),
            bl1_zdapdiagmv: get_symbol(&libs, b"bl1_zdapdiagmv\0").map(|sym| *sym),
            bl1_zapdiagmv: get_symbol(&libs, b"bl1_zapdiagmv\0").map(|sym| *sym),
            bl1_screate_contigm: get_symbol(&libs, b"bl1_screate_contigm\0").map(|sym| *sym),
            bl1_dcreate_contigm: get_symbol(&libs, b"bl1_dcreate_contigm\0").map(|sym| *sym),
            bl1_ccreate_contigm: get_symbol(&libs, b"bl1_ccreate_contigm\0").map(|sym| *sym),
            bl1_zcreate_contigm: get_symbol(&libs, b"bl1_zcreate_contigm\0").map(|sym| *sym),
            bl1_screate_contigmt: get_symbol(&libs, b"bl1_screate_contigmt\0").map(|sym| *sym),
            bl1_dcreate_contigmt: get_symbol(&libs, b"bl1_dcreate_contigmt\0").map(|sym| *sym),
            bl1_ccreate_contigmt: get_symbol(&libs, b"bl1_ccreate_contigmt\0").map(|sym| *sym),
            bl1_zcreate_contigmt: get_symbol(&libs, b"bl1_zcreate_contigmt\0").map(|sym| *sym),
            bl1_screate_contigmr: get_symbol(&libs, b"bl1_screate_contigmr\0").map(|sym| *sym),
            bl1_dcreate_contigmr: get_symbol(&libs, b"bl1_dcreate_contigmr\0").map(|sym| *sym),
            bl1_ccreate_contigmr: get_symbol(&libs, b"bl1_ccreate_contigmr\0").map(|sym| *sym),
            bl1_zcreate_contigmr: get_symbol(&libs, b"bl1_zcreate_contigmr\0").map(|sym| *sym),
            bl1_screate_contigmsr: get_symbol(&libs, b"bl1_screate_contigmsr\0").map(|sym| *sym),
            bl1_dcreate_contigmsr: get_symbol(&libs, b"bl1_dcreate_contigmsr\0").map(|sym| *sym),
            bl1_ccreate_contigmsr: get_symbol(&libs, b"bl1_ccreate_contigmsr\0").map(|sym| *sym),
            bl1_zcreate_contigmsr: get_symbol(&libs, b"bl1_zcreate_contigmsr\0").map(|sym| *sym),
            bl1_sfree_contigm: get_symbol(&libs, b"bl1_sfree_contigm\0").map(|sym| *sym),
            bl1_dfree_contigm: get_symbol(&libs, b"bl1_dfree_contigm\0").map(|sym| *sym),
            bl1_cfree_contigm: get_symbol(&libs, b"bl1_cfree_contigm\0").map(|sym| *sym),
            bl1_zfree_contigm: get_symbol(&libs, b"bl1_zfree_contigm\0").map(|sym| *sym),
            bl1_sfree_saved_contigm: get_symbol(&libs, b"bl1_sfree_saved_contigm\0")
                .map(|sym| *sym),
            bl1_dfree_saved_contigm: get_symbol(&libs, b"bl1_dfree_saved_contigm\0")
                .map(|sym| *sym),
            bl1_cfree_saved_contigm: get_symbol(&libs, b"bl1_cfree_saved_contigm\0")
                .map(|sym| *sym),
            bl1_zfree_saved_contigm: get_symbol(&libs, b"bl1_zfree_saved_contigm\0")
                .map(|sym| *sym),
            bl1_sfree_saved_contigmr: get_symbol(&libs, b"bl1_sfree_saved_contigmr\0")
                .map(|sym| *sym),
            bl1_dfree_saved_contigmr: get_symbol(&libs, b"bl1_dfree_saved_contigmr\0")
                .map(|sym| *sym),
            bl1_cfree_saved_contigmr: get_symbol(&libs, b"bl1_cfree_saved_contigmr\0")
                .map(|sym| *sym),
            bl1_zfree_saved_contigmr: get_symbol(&libs, b"bl1_zfree_saved_contigmr\0")
                .map(|sym| *sym),
            bl1_sfree_saved_contigmsr: get_symbol(&libs, b"bl1_sfree_saved_contigmsr\0")
                .map(|sym| *sym),
            bl1_dfree_saved_contigmsr: get_symbol(&libs, b"bl1_dfree_saved_contigmsr\0")
                .map(|sym| *sym),
            bl1_cfree_saved_contigmsr: get_symbol(&libs, b"bl1_cfree_saved_contigmsr\0")
                .map(|sym| *sym),
            bl1_zfree_saved_contigmsr: get_symbol(&libs, b"bl1_zfree_saved_contigmsr\0")
                .map(|sym| *sym),
            bl1_sewinvscalv: get_symbol(&libs, b"bl1_sewinvscalv\0").map(|sym| *sym),
            bl1_dewinvscalv: get_symbol(&libs, b"bl1_dewinvscalv\0").map(|sym| *sym),
            bl1_csewinvscalv: get_symbol(&libs, b"bl1_csewinvscalv\0").map(|sym| *sym),
            bl1_cewinvscalv: get_symbol(&libs, b"bl1_cewinvscalv\0").map(|sym| *sym),
            bl1_zdewinvscalv: get_symbol(&libs, b"bl1_zdewinvscalv\0").map(|sym| *sym),
            bl1_zewinvscalv: get_symbol(&libs, b"bl1_zewinvscalv\0").map(|sym| *sym),
            bl1_sewinvscalmt: get_symbol(&libs, b"bl1_sewinvscalmt\0").map(|sym| *sym),
            bl1_dewinvscalmt: get_symbol(&libs, b"bl1_dewinvscalmt\0").map(|sym| *sym),
            bl1_csewinvscalmt: get_symbol(&libs, b"bl1_csewinvscalmt\0").map(|sym| *sym),
            bl1_cewinvscalmt: get_symbol(&libs, b"bl1_cewinvscalmt\0").map(|sym| *sym),
            bl1_zdewinvscalmt: get_symbol(&libs, b"bl1_zdewinvscalmt\0").map(|sym| *sym),
            bl1_zewinvscalmt: get_symbol(&libs, b"bl1_zewinvscalmt\0").map(|sym| *sym),
            bl1_sewscalv: get_symbol(&libs, b"bl1_sewscalv\0").map(|sym| *sym),
            bl1_dewscalv: get_symbol(&libs, b"bl1_dewscalv\0").map(|sym| *sym),
            bl1_csewscalv: get_symbol(&libs, b"bl1_csewscalv\0").map(|sym| *sym),
            bl1_cewscalv: get_symbol(&libs, b"bl1_cewscalv\0").map(|sym| *sym),
            bl1_zdewscalv: get_symbol(&libs, b"bl1_zdewscalv\0").map(|sym| *sym),
            bl1_zewscalv: get_symbol(&libs, b"bl1_zewscalv\0").map(|sym| *sym),
            bl1_sewscalmt: get_symbol(&libs, b"bl1_sewscalmt\0").map(|sym| *sym),
            bl1_dewscalmt: get_symbol(&libs, b"bl1_dewscalmt\0").map(|sym| *sym),
            bl1_csewscalmt: get_symbol(&libs, b"bl1_csewscalmt\0").map(|sym| *sym),
            bl1_cewscalmt: get_symbol(&libs, b"bl1_cewscalmt\0").map(|sym| *sym),
            bl1_zdewscalmt: get_symbol(&libs, b"bl1_zdewscalmt\0").map(|sym| *sym),
            bl1_zewscalmt: get_symbol(&libs, b"bl1_zewscalmt\0").map(|sym| *sym),
            bl1_vfree: get_symbol(&libs, b"bl1_vfree\0").map(|sym| *sym),
            bl1_ifree: get_symbol(&libs, b"bl1_ifree\0").map(|sym| *sym),
            bl1_sfree: get_symbol(&libs, b"bl1_sfree\0").map(|sym| *sym),
            bl1_dfree: get_symbol(&libs, b"bl1_dfree\0").map(|sym| *sym),
            bl1_cfree: get_symbol(&libs, b"bl1_cfree\0").map(|sym| *sym),
            bl1_zfree: get_symbol(&libs, b"bl1_zfree\0").map(|sym| *sym),
            bl1_sinverts: get_symbol(&libs, b"bl1_sinverts\0").map(|sym| *sym),
            bl1_dinverts: get_symbol(&libs, b"bl1_dinverts\0").map(|sym| *sym),
            bl1_cinverts: get_symbol(&libs, b"bl1_cinverts\0").map(|sym| *sym),
            bl1_zinverts: get_symbol(&libs, b"bl1_zinverts\0").map(|sym| *sym),
            bl1_sinvert2s: get_symbol(&libs, b"bl1_sinvert2s\0").map(|sym| *sym),
            bl1_dinvert2s: get_symbol(&libs, b"bl1_dinvert2s\0").map(|sym| *sym),
            bl1_cinvert2s: get_symbol(&libs, b"bl1_cinvert2s\0").map(|sym| *sym),
            bl1_zinvert2s: get_symbol(&libs, b"bl1_zinvert2s\0").map(|sym| *sym),
            bl1_sinvertv: get_symbol(&libs, b"bl1_sinvertv\0").map(|sym| *sym),
            bl1_dinvertv: get_symbol(&libs, b"bl1_dinvertv\0").map(|sym| *sym),
            bl1_cinvertv: get_symbol(&libs, b"bl1_cinvertv\0").map(|sym| *sym),
            bl1_zinvertv: get_symbol(&libs, b"bl1_zinvertv\0").map(|sym| *sym),
            bl1_sident: get_symbol(&libs, b"bl1_sident\0").map(|sym| *sym),
            bl1_dident: get_symbol(&libs, b"bl1_dident\0").map(|sym| *sym),
            bl1_cident: get_symbol(&libs, b"bl1_cident\0").map(|sym| *sym),
            bl1_zident: get_symbol(&libs, b"bl1_zident\0").map(|sym| *sym),
            bl1_smaxabsv: get_symbol(&libs, b"bl1_smaxabsv\0").map(|sym| *sym),
            bl1_dmaxabsv: get_symbol(&libs, b"bl1_dmaxabsv\0").map(|sym| *sym),
            bl1_cmaxabsv: get_symbol(&libs, b"bl1_cmaxabsv\0").map(|sym| *sym),
            bl1_zmaxabsv: get_symbol(&libs, b"bl1_zmaxabsv\0").map(|sym| *sym),
            bl1_smaxabsm: get_symbol(&libs, b"bl1_smaxabsm\0").map(|sym| *sym),
            bl1_dmaxabsm: get_symbol(&libs, b"bl1_dmaxabsm\0").map(|sym| *sym),
            bl1_cmaxabsm: get_symbol(&libs, b"bl1_cmaxabsm\0").map(|sym| *sym),
            bl1_zmaxabsm: get_symbol(&libs, b"bl1_zmaxabsm\0").map(|sym| *sym),
            bl1_smaxabsmr: get_symbol(&libs, b"bl1_smaxabsmr\0").map(|sym| *sym),
            bl1_dmaxabsmr: get_symbol(&libs, b"bl1_dmaxabsmr\0").map(|sym| *sym),
            bl1_cmaxabsmr: get_symbol(&libs, b"bl1_cmaxabsmr\0").map(|sym| *sym),
            bl1_zmaxabsmr: get_symbol(&libs, b"bl1_zmaxabsmr\0").map(|sym| *sym),
            bl1_srands: get_symbol(&libs, b"bl1_srands\0").map(|sym| *sym),
            bl1_drands: get_symbol(&libs, b"bl1_drands\0").map(|sym| *sym),
            bl1_crands: get_symbol(&libs, b"bl1_crands\0").map(|sym| *sym),
            bl1_zrands: get_symbol(&libs, b"bl1_zrands\0").map(|sym| *sym),
            bl1_srandv: get_symbol(&libs, b"bl1_srandv\0").map(|sym| *sym),
            bl1_drandv: get_symbol(&libs, b"bl1_drandv\0").map(|sym| *sym),
            bl1_crandv: get_symbol(&libs, b"bl1_crandv\0").map(|sym| *sym),
            bl1_zrandv: get_symbol(&libs, b"bl1_zrandv\0").map(|sym| *sym),
            bl1_srandm: get_symbol(&libs, b"bl1_srandm\0").map(|sym| *sym),
            bl1_drandm: get_symbol(&libs, b"bl1_drandm\0").map(|sym| *sym),
            bl1_crandm: get_symbol(&libs, b"bl1_crandm\0").map(|sym| *sym),
            bl1_zrandm: get_symbol(&libs, b"bl1_zrandm\0").map(|sym| *sym),
            bl1_srandmr: get_symbol(&libs, b"bl1_srandmr\0").map(|sym| *sym),
            bl1_drandmr: get_symbol(&libs, b"bl1_drandmr\0").map(|sym| *sym),
            bl1_crandmr: get_symbol(&libs, b"bl1_crandmr\0").map(|sym| *sym),
            bl1_zrandmr: get_symbol(&libs, b"bl1_zrandmr\0").map(|sym| *sym),
            bl1_set_contig_strides: get_symbol(&libs, b"bl1_set_contig_strides\0").map(|sym| *sym),
            bl1_set_dim_with_side: get_symbol(&libs, b"bl1_set_dim_with_side\0").map(|sym| *sym),
            bl1_set_dims_with_trans: get_symbol(&libs, b"bl1_set_dims_with_trans\0")
                .map(|sym| *sym),
            bl1_isetv: get_symbol(&libs, b"bl1_isetv\0").map(|sym| *sym),
            bl1_ssetv: get_symbol(&libs, b"bl1_ssetv\0").map(|sym| *sym),
            bl1_dsetv: get_symbol(&libs, b"bl1_dsetv\0").map(|sym| *sym),
            bl1_csetv: get_symbol(&libs, b"bl1_csetv\0").map(|sym| *sym),
            bl1_zsetv: get_symbol(&libs, b"bl1_zsetv\0").map(|sym| *sym),
            bl1_isetm: get_symbol(&libs, b"bl1_isetm\0").map(|sym| *sym),
            bl1_ssetm: get_symbol(&libs, b"bl1_ssetm\0").map(|sym| *sym),
            bl1_dsetm: get_symbol(&libs, b"bl1_dsetm\0").map(|sym| *sym),
            bl1_csetm: get_symbol(&libs, b"bl1_csetm\0").map(|sym| *sym),
            bl1_zsetm: get_symbol(&libs, b"bl1_zsetm\0").map(|sym| *sym),
            bl1_ssetmr: get_symbol(&libs, b"bl1_ssetmr\0").map(|sym| *sym),
            bl1_dsetmr: get_symbol(&libs, b"bl1_dsetmr\0").map(|sym| *sym),
            bl1_csetmr: get_symbol(&libs, b"bl1_csetmr\0").map(|sym| *sym),
            bl1_zsetmr: get_symbol(&libs, b"bl1_zsetmr\0").map(|sym| *sym),
            bl1_isetdiag: get_symbol(&libs, b"bl1_isetdiag\0").map(|sym| *sym),
            bl1_ssetdiag: get_symbol(&libs, b"bl1_ssetdiag\0").map(|sym| *sym),
            bl1_dsetdiag: get_symbol(&libs, b"bl1_dsetdiag\0").map(|sym| *sym),
            bl1_csetdiag: get_symbol(&libs, b"bl1_csetdiag\0").map(|sym| *sym),
            bl1_zsetdiag: get_symbol(&libs, b"bl1_zsetdiag\0").map(|sym| *sym),
            bl1_sscalediag: get_symbol(&libs, b"bl1_sscalediag\0").map(|sym| *sym),
            bl1_dscalediag: get_symbol(&libs, b"bl1_dscalediag\0").map(|sym| *sym),
            bl1_cscalediag: get_symbol(&libs, b"bl1_cscalediag\0").map(|sym| *sym),
            bl1_zscalediag: get_symbol(&libs, b"bl1_zscalediag\0").map(|sym| *sym),
            bl1_csscalediag: get_symbol(&libs, b"bl1_csscalediag\0").map(|sym| *sym),
            bl1_zdscalediag: get_symbol(&libs, b"bl1_zdscalediag\0").map(|sym| *sym),
            bl1_sshiftdiag: get_symbol(&libs, b"bl1_sshiftdiag\0").map(|sym| *sym),
            bl1_dshiftdiag: get_symbol(&libs, b"bl1_dshiftdiag\0").map(|sym| *sym),
            bl1_cshiftdiag: get_symbol(&libs, b"bl1_cshiftdiag\0").map(|sym| *sym),
            bl1_zshiftdiag: get_symbol(&libs, b"bl1_zshiftdiag\0").map(|sym| *sym),
            bl1_csshiftdiag: get_symbol(&libs, b"bl1_csshiftdiag\0").map(|sym| *sym),
            bl1_zdshiftdiag: get_symbol(&libs, b"bl1_zdshiftdiag\0").map(|sym| *sym),
            bl1_ssymmize: get_symbol(&libs, b"bl1_ssymmize\0").map(|sym| *sym),
            bl1_dsymmize: get_symbol(&libs, b"bl1_dsymmize\0").map(|sym| *sym),
            bl1_csymmize: get_symbol(&libs, b"bl1_csymmize\0").map(|sym| *sym),
            bl1_zsymmize: get_symbol(&libs, b"bl1_zsymmize\0").map(|sym| *sym),
            bl1_does_trans: get_symbol(&libs, b"bl1_does_trans\0").map(|sym| *sym),
            bl1_does_notrans: get_symbol(&libs, b"bl1_does_notrans\0").map(|sym| *sym),
            bl1_does_conj: get_symbol(&libs, b"bl1_does_conj\0").map(|sym| *sym),
            bl1_is_notrans: get_symbol(&libs, b"bl1_is_notrans\0").map(|sym| *sym),
            bl1_is_trans: get_symbol(&libs, b"bl1_is_trans\0").map(|sym| *sym),
            bl1_is_conjnotrans: get_symbol(&libs, b"bl1_is_conjnotrans\0").map(|sym| *sym),
            bl1_is_conjtrans: get_symbol(&libs, b"bl1_is_conjtrans\0").map(|sym| *sym),
            bl1_is_noconj: get_symbol(&libs, b"bl1_is_noconj\0").map(|sym| *sym),
            bl1_is_conj: get_symbol(&libs, b"bl1_is_conj\0").map(|sym| *sym),
            bl1_is_lower: get_symbol(&libs, b"bl1_is_lower\0").map(|sym| *sym),
            bl1_is_upper: get_symbol(&libs, b"bl1_is_upper\0").map(|sym| *sym),
            bl1_is_left: get_symbol(&libs, b"bl1_is_left\0").map(|sym| *sym),
            bl1_is_right: get_symbol(&libs, b"bl1_is_right\0").map(|sym| *sym),
            bl1_is_nonunit_diag: get_symbol(&libs, b"bl1_is_nonunit_diag\0").map(|sym| *sym),
            bl1_is_unit_diag: get_symbol(&libs, b"bl1_is_unit_diag\0").map(|sym| *sym),
            bl1_is_zero_diag: get_symbol(&libs, b"bl1_is_zero_diag\0").map(|sym| *sym),
            bl1_proj_trans1_to_conj: get_symbol(&libs, b"bl1_proj_trans1_to_conj\0")
                .map(|sym| *sym),
            bl1_check_storage_3m: get_symbol(&libs, b"bl1_check_storage_3m\0").map(|sym| *sym),
            bl1_check_storage_2m: get_symbol(&libs, b"bl1_check_storage_2m\0").map(|sym| *sym),
            bl1_is_row_or_col_storage: get_symbol(&libs, b"bl1_is_row_or_col_storage\0")
                .map(|sym| *sym),
            bl1_is_row_storage: get_symbol(&libs, b"bl1_is_row_storage\0").map(|sym| *sym),
            bl1_is_col_storage: get_symbol(&libs, b"bl1_is_col_storage\0").map(|sym| *sym),
            bl1_is_gen_storage: get_symbol(&libs, b"bl1_is_gen_storage\0").map(|sym| *sym),
            bl1_is_vector: get_symbol(&libs, b"bl1_is_vector\0").map(|sym| *sym),
            bl1_vector_dim: get_symbol(&libs, b"bl1_vector_dim\0").map(|sym| *sym),
            bl1_vector_inc: get_symbol(&libs, b"bl1_vector_inc\0").map(|sym| *sym),
            bl1_zero_dim1: get_symbol(&libs, b"bl1_zero_dim1\0").map(|sym| *sym),
            bl1_zero_dim2: get_symbol(&libs, b"bl1_zero_dim2\0").map(|sym| *sym),
            bl1_zero_dim3: get_symbol(&libs, b"bl1_zero_dim3\0").map(|sym| *sym),
            bl1_abort: get_symbol(&libs, b"bl1_abort\0").map(|sym| *sym),
            bl1_abort_msg: get_symbol(&libs, b"bl1_abort_msg\0").map(|sym| *sym),
            bl1_param_map_to_netlib_trans: get_symbol(&libs, b"bl1_param_map_to_netlib_trans\0")
                .map(|sym| *sym),
            bl1_param_map_to_netlib_uplo: get_symbol(&libs, b"bl1_param_map_to_netlib_uplo\0")
                .map(|sym| *sym),
            bl1_param_map_to_netlib_side: get_symbol(&libs, b"bl1_param_map_to_netlib_side\0")
                .map(|sym| *sym),
            bl1_param_map_to_netlib_diag: get_symbol(&libs, b"bl1_param_map_to_netlib_diag\0")
                .map(|sym| *sym),
            bl1_samax: get_symbol(&libs, b"bl1_samax\0").map(|sym| *sym),
            bl1_damax: get_symbol(&libs, b"bl1_damax\0").map(|sym| *sym),
            bl1_camax: get_symbol(&libs, b"bl1_camax\0").map(|sym| *sym),
            bl1_zamax: get_symbol(&libs, b"bl1_zamax\0").map(|sym| *sym),
            bl1_sasum: get_symbol(&libs, b"bl1_sasum\0").map(|sym| *sym),
            bl1_dasum: get_symbol(&libs, b"bl1_dasum\0").map(|sym| *sym),
            bl1_casum: get_symbol(&libs, b"bl1_casum\0").map(|sym| *sym),
            bl1_zasum: get_symbol(&libs, b"bl1_zasum\0").map(|sym| *sym),
            bl1_saxpy: get_symbol(&libs, b"bl1_saxpy\0").map(|sym| *sym),
            bl1_daxpy: get_symbol(&libs, b"bl1_daxpy\0").map(|sym| *sym),
            bl1_caxpy: get_symbol(&libs, b"bl1_caxpy\0").map(|sym| *sym),
            bl1_zaxpy: get_symbol(&libs, b"bl1_zaxpy\0").map(|sym| *sym),
            bl1_saxpyv: get_symbol(&libs, b"bl1_saxpyv\0").map(|sym| *sym),
            bl1_daxpyv: get_symbol(&libs, b"bl1_daxpyv\0").map(|sym| *sym),
            bl1_caxpyv: get_symbol(&libs, b"bl1_caxpyv\0").map(|sym| *sym),
            bl1_zaxpyv: get_symbol(&libs, b"bl1_zaxpyv\0").map(|sym| *sym),
            bl1_saxpymt: get_symbol(&libs, b"bl1_saxpymt\0").map(|sym| *sym),
            bl1_daxpymt: get_symbol(&libs, b"bl1_daxpymt\0").map(|sym| *sym),
            bl1_caxpymt: get_symbol(&libs, b"bl1_caxpymt\0").map(|sym| *sym),
            bl1_zaxpymt: get_symbol(&libs, b"bl1_zaxpymt\0").map(|sym| *sym),
            bl1_saxpymrt: get_symbol(&libs, b"bl1_saxpymrt\0").map(|sym| *sym),
            bl1_daxpymrt: get_symbol(&libs, b"bl1_daxpymrt\0").map(|sym| *sym),
            bl1_caxpymrt: get_symbol(&libs, b"bl1_caxpymrt\0").map(|sym| *sym),
            bl1_zaxpymrt: get_symbol(&libs, b"bl1_zaxpymrt\0").map(|sym| *sym),
            bl1_saxpysv: get_symbol(&libs, b"bl1_saxpysv\0").map(|sym| *sym),
            bl1_daxpysv: get_symbol(&libs, b"bl1_daxpysv\0").map(|sym| *sym),
            bl1_caxpysv: get_symbol(&libs, b"bl1_caxpysv\0").map(|sym| *sym),
            bl1_zaxpysv: get_symbol(&libs, b"bl1_zaxpysv\0").map(|sym| *sym),
            bl1_saxpysmt: get_symbol(&libs, b"bl1_saxpysmt\0").map(|sym| *sym),
            bl1_daxpysmt: get_symbol(&libs, b"bl1_daxpysmt\0").map(|sym| *sym),
            bl1_caxpysmt: get_symbol(&libs, b"bl1_caxpysmt\0").map(|sym| *sym),
            bl1_zaxpysmt: get_symbol(&libs, b"bl1_zaxpysmt\0").map(|sym| *sym),
            bl1_sconjv: get_symbol(&libs, b"bl1_sconjv\0").map(|sym| *sym),
            bl1_dconjv: get_symbol(&libs, b"bl1_dconjv\0").map(|sym| *sym),
            bl1_cconjv: get_symbol(&libs, b"bl1_cconjv\0").map(|sym| *sym),
            bl1_zconjv: get_symbol(&libs, b"bl1_zconjv\0").map(|sym| *sym),
            bl1_sconjm: get_symbol(&libs, b"bl1_sconjm\0").map(|sym| *sym),
            bl1_dconjm: get_symbol(&libs, b"bl1_dconjm\0").map(|sym| *sym),
            bl1_cconjm: get_symbol(&libs, b"bl1_cconjm\0").map(|sym| *sym),
            bl1_zconjm: get_symbol(&libs, b"bl1_zconjm\0").map(|sym| *sym),
            bl1_sconjmr: get_symbol(&libs, b"bl1_sconjmr\0").map(|sym| *sym),
            bl1_dconjmr: get_symbol(&libs, b"bl1_dconjmr\0").map(|sym| *sym),
            bl1_cconjmr: get_symbol(&libs, b"bl1_cconjmr\0").map(|sym| *sym),
            bl1_zconjmr: get_symbol(&libs, b"bl1_zconjmr\0").map(|sym| *sym),
            bl1_scopy: get_symbol(&libs, b"bl1_scopy\0").map(|sym| *sym),
            bl1_dcopy: get_symbol(&libs, b"bl1_dcopy\0").map(|sym| *sym),
            bl1_ccopy: get_symbol(&libs, b"bl1_ccopy\0").map(|sym| *sym),
            bl1_zcopy: get_symbol(&libs, b"bl1_zcopy\0").map(|sym| *sym),
            bl1_icopyv: get_symbol(&libs, b"bl1_icopyv\0").map(|sym| *sym),
            bl1_scopyv: get_symbol(&libs, b"bl1_scopyv\0").map(|sym| *sym),
            bl1_dcopyv: get_symbol(&libs, b"bl1_dcopyv\0").map(|sym| *sym),
            bl1_ccopyv: get_symbol(&libs, b"bl1_ccopyv\0").map(|sym| *sym),
            bl1_zcopyv: get_symbol(&libs, b"bl1_zcopyv\0").map(|sym| *sym),
            bl1_sdcopyv: get_symbol(&libs, b"bl1_sdcopyv\0").map(|sym| *sym),
            bl1_dscopyv: get_symbol(&libs, b"bl1_dscopyv\0").map(|sym| *sym),
            bl1_sccopyv: get_symbol(&libs, b"bl1_sccopyv\0").map(|sym| *sym),
            bl1_cscopyv: get_symbol(&libs, b"bl1_cscopyv\0").map(|sym| *sym),
            bl1_szcopyv: get_symbol(&libs, b"bl1_szcopyv\0").map(|sym| *sym),
            bl1_zscopyv: get_symbol(&libs, b"bl1_zscopyv\0").map(|sym| *sym),
            bl1_dccopyv: get_symbol(&libs, b"bl1_dccopyv\0").map(|sym| *sym),
            bl1_cdcopyv: get_symbol(&libs, b"bl1_cdcopyv\0").map(|sym| *sym),
            bl1_dzcopyv: get_symbol(&libs, b"bl1_dzcopyv\0").map(|sym| *sym),
            bl1_zdcopyv: get_symbol(&libs, b"bl1_zdcopyv\0").map(|sym| *sym),
            bl1_czcopyv: get_symbol(&libs, b"bl1_czcopyv\0").map(|sym| *sym),
            bl1_zccopyv: get_symbol(&libs, b"bl1_zccopyv\0").map(|sym| *sym),
            bl1_scopymr: get_symbol(&libs, b"bl1_scopymr\0").map(|sym| *sym),
            bl1_dcopymr: get_symbol(&libs, b"bl1_dcopymr\0").map(|sym| *sym),
            bl1_ccopymr: get_symbol(&libs, b"bl1_ccopymr\0").map(|sym| *sym),
            bl1_zcopymr: get_symbol(&libs, b"bl1_zcopymr\0").map(|sym| *sym),
            bl1_sscopymr: get_symbol(&libs, b"bl1_sscopymr\0").map(|sym| *sym),
            bl1_sdcopymr: get_symbol(&libs, b"bl1_sdcopymr\0").map(|sym| *sym),
            bl1_dscopymr: get_symbol(&libs, b"bl1_dscopymr\0").map(|sym| *sym),
            bl1_sccopymr: get_symbol(&libs, b"bl1_sccopymr\0").map(|sym| *sym),
            bl1_cscopymr: get_symbol(&libs, b"bl1_cscopymr\0").map(|sym| *sym),
            bl1_szcopymr: get_symbol(&libs, b"bl1_szcopymr\0").map(|sym| *sym),
            bl1_zscopymr: get_symbol(&libs, b"bl1_zscopymr\0").map(|sym| *sym),
            bl1_ddcopymr: get_symbol(&libs, b"bl1_ddcopymr\0").map(|sym| *sym),
            bl1_dccopymr: get_symbol(&libs, b"bl1_dccopymr\0").map(|sym| *sym),
            bl1_cdcopymr: get_symbol(&libs, b"bl1_cdcopymr\0").map(|sym| *sym),
            bl1_dzcopymr: get_symbol(&libs, b"bl1_dzcopymr\0").map(|sym| *sym),
            bl1_zdcopymr: get_symbol(&libs, b"bl1_zdcopymr\0").map(|sym| *sym),
            bl1_cccopymr: get_symbol(&libs, b"bl1_cccopymr\0").map(|sym| *sym),
            bl1_czcopymr: get_symbol(&libs, b"bl1_czcopymr\0").map(|sym| *sym),
            bl1_zccopymr: get_symbol(&libs, b"bl1_zccopymr\0").map(|sym| *sym),
            bl1_zzcopymr: get_symbol(&libs, b"bl1_zzcopymr\0").map(|sym| *sym),
            bl1_scopymrt: get_symbol(&libs, b"bl1_scopymrt\0").map(|sym| *sym),
            bl1_dcopymrt: get_symbol(&libs, b"bl1_dcopymrt\0").map(|sym| *sym),
            bl1_ccopymrt: get_symbol(&libs, b"bl1_ccopymrt\0").map(|sym| *sym),
            bl1_zcopymrt: get_symbol(&libs, b"bl1_zcopymrt\0").map(|sym| *sym),
            bl1_sscopymrt: get_symbol(&libs, b"bl1_sscopymrt\0").map(|sym| *sym),
            bl1_sdcopymrt: get_symbol(&libs, b"bl1_sdcopymrt\0").map(|sym| *sym),
            bl1_sccopymrt: get_symbol(&libs, b"bl1_sccopymrt\0").map(|sym| *sym),
            bl1_szcopymrt: get_symbol(&libs, b"bl1_szcopymrt\0").map(|sym| *sym),
            bl1_dscopymrt: get_symbol(&libs, b"bl1_dscopymrt\0").map(|sym| *sym),
            bl1_ddcopymrt: get_symbol(&libs, b"bl1_ddcopymrt\0").map(|sym| *sym),
            bl1_dccopymrt: get_symbol(&libs, b"bl1_dccopymrt\0").map(|sym| *sym),
            bl1_dzcopymrt: get_symbol(&libs, b"bl1_dzcopymrt\0").map(|sym| *sym),
            bl1_cscopymrt: get_symbol(&libs, b"bl1_cscopymrt\0").map(|sym| *sym),
            bl1_cdcopymrt: get_symbol(&libs, b"bl1_cdcopymrt\0").map(|sym| *sym),
            bl1_cccopymrt: get_symbol(&libs, b"bl1_cccopymrt\0").map(|sym| *sym),
            bl1_czcopymrt: get_symbol(&libs, b"bl1_czcopymrt\0").map(|sym| *sym),
            bl1_zscopymrt: get_symbol(&libs, b"bl1_zscopymrt\0").map(|sym| *sym),
            bl1_zdcopymrt: get_symbol(&libs, b"bl1_zdcopymrt\0").map(|sym| *sym),
            bl1_zccopymrt: get_symbol(&libs, b"bl1_zccopymrt\0").map(|sym| *sym),
            bl1_zzcopymrt: get_symbol(&libs, b"bl1_zzcopymrt\0").map(|sym| *sym),
            bl1_icopymt: get_symbol(&libs, b"bl1_icopymt\0").map(|sym| *sym),
            bl1_scopymt: get_symbol(&libs, b"bl1_scopymt\0").map(|sym| *sym),
            bl1_dcopymt: get_symbol(&libs, b"bl1_dcopymt\0").map(|sym| *sym),
            bl1_ccopymt: get_symbol(&libs, b"bl1_ccopymt\0").map(|sym| *sym),
            bl1_zcopymt: get_symbol(&libs, b"bl1_zcopymt\0").map(|sym| *sym),
            bl1_sscopymt: get_symbol(&libs, b"bl1_sscopymt\0").map(|sym| *sym),
            bl1_sdcopymt: get_symbol(&libs, b"bl1_sdcopymt\0").map(|sym| *sym),
            bl1_dscopymt: get_symbol(&libs, b"bl1_dscopymt\0").map(|sym| *sym),
            bl1_sccopymt: get_symbol(&libs, b"bl1_sccopymt\0").map(|sym| *sym),
            bl1_cscopymt: get_symbol(&libs, b"bl1_cscopymt\0").map(|sym| *sym),
            bl1_szcopymt: get_symbol(&libs, b"bl1_szcopymt\0").map(|sym| *sym),
            bl1_zscopymt: get_symbol(&libs, b"bl1_zscopymt\0").map(|sym| *sym),
            bl1_ddcopymt: get_symbol(&libs, b"bl1_ddcopymt\0").map(|sym| *sym),
            bl1_dccopymt: get_symbol(&libs, b"bl1_dccopymt\0").map(|sym| *sym),
            bl1_cdcopymt: get_symbol(&libs, b"bl1_cdcopymt\0").map(|sym| *sym),
            bl1_dzcopymt: get_symbol(&libs, b"bl1_dzcopymt\0").map(|sym| *sym),
            bl1_zdcopymt: get_symbol(&libs, b"bl1_zdcopymt\0").map(|sym| *sym),
            bl1_cccopymt: get_symbol(&libs, b"bl1_cccopymt\0").map(|sym| *sym),
            bl1_czcopymt: get_symbol(&libs, b"bl1_czcopymt\0").map(|sym| *sym),
            bl1_zccopymt: get_symbol(&libs, b"bl1_zccopymt\0").map(|sym| *sym),
            bl1_zzcopymt: get_symbol(&libs, b"bl1_zzcopymt\0").map(|sym| *sym),
            bl1_cdot_in: get_symbol(&libs, b"bl1_cdot_in\0").map(|sym| *sym),
            bl1_zdot_in: get_symbol(&libs, b"bl1_zdot_in\0").map(|sym| *sym),
            bl1_sdot: get_symbol(&libs, b"bl1_sdot\0").map(|sym| *sym),
            bl1_ddot: get_symbol(&libs, b"bl1_ddot\0").map(|sym| *sym),
            bl1_cdot: get_symbol(&libs, b"bl1_cdot\0").map(|sym| *sym),
            bl1_zdot: get_symbol(&libs, b"bl1_zdot\0").map(|sym| *sym),
            bl1_sdots: get_symbol(&libs, b"bl1_sdots\0").map(|sym| *sym),
            bl1_ddots: get_symbol(&libs, b"bl1_ddots\0").map(|sym| *sym),
            bl1_cdots: get_symbol(&libs, b"bl1_cdots\0").map(|sym| *sym),
            bl1_zdots: get_symbol(&libs, b"bl1_zdots\0").map(|sym| *sym),
            bl1_sdot2s: get_symbol(&libs, b"bl1_sdot2s\0").map(|sym| *sym),
            bl1_ddot2s: get_symbol(&libs, b"bl1_ddot2s\0").map(|sym| *sym),
            bl1_cdot2s: get_symbol(&libs, b"bl1_cdot2s\0").map(|sym| *sym),
            bl1_zdot2s: get_symbol(&libs, b"bl1_zdot2s\0").map(|sym| *sym),
            bl1_sfnorm: get_symbol(&libs, b"bl1_sfnorm\0").map(|sym| *sym),
            bl1_dfnorm: get_symbol(&libs, b"bl1_dfnorm\0").map(|sym| *sym),
            bl1_cfnorm: get_symbol(&libs, b"bl1_cfnorm\0").map(|sym| *sym),
            bl1_zfnorm: get_symbol(&libs, b"bl1_zfnorm\0").map(|sym| *sym),
            bl1_sinvscalv: get_symbol(&libs, b"bl1_sinvscalv\0").map(|sym| *sym),
            bl1_dinvscalv: get_symbol(&libs, b"bl1_dinvscalv\0").map(|sym| *sym),
            bl1_csinvscalv: get_symbol(&libs, b"bl1_csinvscalv\0").map(|sym| *sym),
            bl1_cinvscalv: get_symbol(&libs, b"bl1_cinvscalv\0").map(|sym| *sym),
            bl1_zdinvscalv: get_symbol(&libs, b"bl1_zdinvscalv\0").map(|sym| *sym),
            bl1_zinvscalv: get_symbol(&libs, b"bl1_zinvscalv\0").map(|sym| *sym),
            bl1_sinvscalm: get_symbol(&libs, b"bl1_sinvscalm\0").map(|sym| *sym),
            bl1_dinvscalm: get_symbol(&libs, b"bl1_dinvscalm\0").map(|sym| *sym),
            bl1_csinvscalm: get_symbol(&libs, b"bl1_csinvscalm\0").map(|sym| *sym),
            bl1_cinvscalm: get_symbol(&libs, b"bl1_cinvscalm\0").map(|sym| *sym),
            bl1_zdinvscalm: get_symbol(&libs, b"bl1_zdinvscalm\0").map(|sym| *sym),
            bl1_zinvscalm: get_symbol(&libs, b"bl1_zinvscalm\0").map(|sym| *sym),
            bl1_snrm2: get_symbol(&libs, b"bl1_snrm2\0").map(|sym| *sym),
            bl1_dnrm2: get_symbol(&libs, b"bl1_dnrm2\0").map(|sym| *sym),
            bl1_cnrm2: get_symbol(&libs, b"bl1_cnrm2\0").map(|sym| *sym),
            bl1_znrm2: get_symbol(&libs, b"bl1_znrm2\0").map(|sym| *sym),
            bl1_sscal: get_symbol(&libs, b"bl1_sscal\0").map(|sym| *sym),
            bl1_dscal: get_symbol(&libs, b"bl1_dscal\0").map(|sym| *sym),
            bl1_csscal: get_symbol(&libs, b"bl1_csscal\0").map(|sym| *sym),
            bl1_cscal: get_symbol(&libs, b"bl1_cscal\0").map(|sym| *sym),
            bl1_zdscal: get_symbol(&libs, b"bl1_zdscal\0").map(|sym| *sym),
            bl1_zscal: get_symbol(&libs, b"bl1_zscal\0").map(|sym| *sym),
            bl1_sscalv: get_symbol(&libs, b"bl1_sscalv\0").map(|sym| *sym),
            bl1_dscalv: get_symbol(&libs, b"bl1_dscalv\0").map(|sym| *sym),
            bl1_csscalv: get_symbol(&libs, b"bl1_csscalv\0").map(|sym| *sym),
            bl1_cscalv: get_symbol(&libs, b"bl1_cscalv\0").map(|sym| *sym),
            bl1_zdscalv: get_symbol(&libs, b"bl1_zdscalv\0").map(|sym| *sym),
            bl1_zscalv: get_symbol(&libs, b"bl1_zscalv\0").map(|sym| *sym),
            bl1_sscalm: get_symbol(&libs, b"bl1_sscalm\0").map(|sym| *sym),
            bl1_dscalm: get_symbol(&libs, b"bl1_dscalm\0").map(|sym| *sym),
            bl1_csscalm: get_symbol(&libs, b"bl1_csscalm\0").map(|sym| *sym),
            bl1_cscalm: get_symbol(&libs, b"bl1_cscalm\0").map(|sym| *sym),
            bl1_zdscalm: get_symbol(&libs, b"bl1_zdscalm\0").map(|sym| *sym),
            bl1_zscalm: get_symbol(&libs, b"bl1_zscalm\0").map(|sym| *sym),
            bl1_sscalmr: get_symbol(&libs, b"bl1_sscalmr\0").map(|sym| *sym),
            bl1_dscalmr: get_symbol(&libs, b"bl1_dscalmr\0").map(|sym| *sym),
            bl1_csscalmr: get_symbol(&libs, b"bl1_csscalmr\0").map(|sym| *sym),
            bl1_cscalmr: get_symbol(&libs, b"bl1_cscalmr\0").map(|sym| *sym),
            bl1_zdscalmr: get_symbol(&libs, b"bl1_zdscalmr\0").map(|sym| *sym),
            bl1_zscalmr: get_symbol(&libs, b"bl1_zscalmr\0").map(|sym| *sym),
            bl1_sswap: get_symbol(&libs, b"bl1_sswap\0").map(|sym| *sym),
            bl1_dswap: get_symbol(&libs, b"bl1_dswap\0").map(|sym| *sym),
            bl1_cswap: get_symbol(&libs, b"bl1_cswap\0").map(|sym| *sym),
            bl1_zswap: get_symbol(&libs, b"bl1_zswap\0").map(|sym| *sym),
            bl1_sswapv: get_symbol(&libs, b"bl1_sswapv\0").map(|sym| *sym),
            bl1_dswapv: get_symbol(&libs, b"bl1_dswapv\0").map(|sym| *sym),
            bl1_cswapv: get_symbol(&libs, b"bl1_cswapv\0").map(|sym| *sym),
            bl1_zswapv: get_symbol(&libs, b"bl1_zswapv\0").map(|sym| *sym),
            bl1_sswapmt: get_symbol(&libs, b"bl1_sswapmt\0").map(|sym| *sym),
            bl1_dswapmt: get_symbol(&libs, b"bl1_dswapmt\0").map(|sym| *sym),
            bl1_cswapmt: get_symbol(&libs, b"bl1_cswapmt\0").map(|sym| *sym),
            bl1_zswapmt: get_symbol(&libs, b"bl1_zswapmt\0").map(|sym| *sym),
            bl1_sgemv: get_symbol(&libs, b"bl1_sgemv\0").map(|sym| *sym),
            bl1_dgemv: get_symbol(&libs, b"bl1_dgemv\0").map(|sym| *sym),
            bl1_cgemv: get_symbol(&libs, b"bl1_cgemv\0").map(|sym| *sym),
            bl1_zgemv: get_symbol(&libs, b"bl1_zgemv\0").map(|sym| *sym),
            bl1_sgemv_blas: get_symbol(&libs, b"bl1_sgemv_blas\0").map(|sym| *sym),
            bl1_dgemv_blas: get_symbol(&libs, b"bl1_dgemv_blas\0").map(|sym| *sym),
            bl1_cgemv_blas: get_symbol(&libs, b"bl1_cgemv_blas\0").map(|sym| *sym),
            bl1_zgemv_blas: get_symbol(&libs, b"bl1_zgemv_blas\0").map(|sym| *sym),
            bl1_sger: get_symbol(&libs, b"bl1_sger\0").map(|sym| *sym),
            bl1_dger: get_symbol(&libs, b"bl1_dger\0").map(|sym| *sym),
            bl1_cger: get_symbol(&libs, b"bl1_cger\0").map(|sym| *sym),
            bl1_zger: get_symbol(&libs, b"bl1_zger\0").map(|sym| *sym),
            bl1_sger_blas: get_symbol(&libs, b"bl1_sger_blas\0").map(|sym| *sym),
            bl1_dger_blas: get_symbol(&libs, b"bl1_dger_blas\0").map(|sym| *sym),
            bl1_cgerc_blas: get_symbol(&libs, b"bl1_cgerc_blas\0").map(|sym| *sym),
            bl1_cgeru_blas: get_symbol(&libs, b"bl1_cgeru_blas\0").map(|sym| *sym),
            bl1_zgerc_blas: get_symbol(&libs, b"bl1_zgerc_blas\0").map(|sym| *sym),
            bl1_zgeru_blas: get_symbol(&libs, b"bl1_zgeru_blas\0").map(|sym| *sym),
            bl1_shemv: get_symbol(&libs, b"bl1_shemv\0").map(|sym| *sym),
            bl1_dhemv: get_symbol(&libs, b"bl1_dhemv\0").map(|sym| *sym),
            bl1_chemv: get_symbol(&libs, b"bl1_chemv\0").map(|sym| *sym),
            bl1_zhemv: get_symbol(&libs, b"bl1_zhemv\0").map(|sym| *sym),
            bl1_chemv_blas: get_symbol(&libs, b"bl1_chemv_blas\0").map(|sym| *sym),
            bl1_zhemv_blas: get_symbol(&libs, b"bl1_zhemv_blas\0").map(|sym| *sym),
            bl1_sher: get_symbol(&libs, b"bl1_sher\0").map(|sym| *sym),
            bl1_dher: get_symbol(&libs, b"bl1_dher\0").map(|sym| *sym),
            bl1_cher: get_symbol(&libs, b"bl1_cher\0").map(|sym| *sym),
            bl1_zher: get_symbol(&libs, b"bl1_zher\0").map(|sym| *sym),
            bl1_cher_blas: get_symbol(&libs, b"bl1_cher_blas\0").map(|sym| *sym),
            bl1_zher_blas: get_symbol(&libs, b"bl1_zher_blas\0").map(|sym| *sym),
            bl1_sher2: get_symbol(&libs, b"bl1_sher2\0").map(|sym| *sym),
            bl1_dher2: get_symbol(&libs, b"bl1_dher2\0").map(|sym| *sym),
            bl1_cher2: get_symbol(&libs, b"bl1_cher2\0").map(|sym| *sym),
            bl1_zher2: get_symbol(&libs, b"bl1_zher2\0").map(|sym| *sym),
            bl1_cher2_blas: get_symbol(&libs, b"bl1_cher2_blas\0").map(|sym| *sym),
            bl1_zher2_blas: get_symbol(&libs, b"bl1_zher2_blas\0").map(|sym| *sym),
            bl1_ssymv: get_symbol(&libs, b"bl1_ssymv\0").map(|sym| *sym),
            bl1_dsymv: get_symbol(&libs, b"bl1_dsymv\0").map(|sym| *sym),
            bl1_csymv: get_symbol(&libs, b"bl1_csymv\0").map(|sym| *sym),
            bl1_zsymv: get_symbol(&libs, b"bl1_zsymv\0").map(|sym| *sym),
            bl1_ssymv_blas: get_symbol(&libs, b"bl1_ssymv_blas\0").map(|sym| *sym),
            bl1_dsymv_blas: get_symbol(&libs, b"bl1_dsymv_blas\0").map(|sym| *sym),
            bl1_csymv_blas: get_symbol(&libs, b"bl1_csymv_blas\0").map(|sym| *sym),
            bl1_zsymv_blas: get_symbol(&libs, b"bl1_zsymv_blas\0").map(|sym| *sym),
            bl1_ssyr: get_symbol(&libs, b"bl1_ssyr\0").map(|sym| *sym),
            bl1_dsyr: get_symbol(&libs, b"bl1_dsyr\0").map(|sym| *sym),
            bl1_csyr: get_symbol(&libs, b"bl1_csyr\0").map(|sym| *sym),
            bl1_zsyr: get_symbol(&libs, b"bl1_zsyr\0").map(|sym| *sym),
            bl1_ssyr_blas: get_symbol(&libs, b"bl1_ssyr_blas\0").map(|sym| *sym),
            bl1_dsyr_blas: get_symbol(&libs, b"bl1_dsyr_blas\0").map(|sym| *sym),
            bl1_csyr_blas: get_symbol(&libs, b"bl1_csyr_blas\0").map(|sym| *sym),
            bl1_zsyr_blas: get_symbol(&libs, b"bl1_zsyr_blas\0").map(|sym| *sym),
            bl1_ssyr2: get_symbol(&libs, b"bl1_ssyr2\0").map(|sym| *sym),
            bl1_dsyr2: get_symbol(&libs, b"bl1_dsyr2\0").map(|sym| *sym),
            bl1_csyr2: get_symbol(&libs, b"bl1_csyr2\0").map(|sym| *sym),
            bl1_zsyr2: get_symbol(&libs, b"bl1_zsyr2\0").map(|sym| *sym),
            bl1_ssyr2_blas: get_symbol(&libs, b"bl1_ssyr2_blas\0").map(|sym| *sym),
            bl1_dsyr2_blas: get_symbol(&libs, b"bl1_dsyr2_blas\0").map(|sym| *sym),
            bl1_csyr2_blas: get_symbol(&libs, b"bl1_csyr2_blas\0").map(|sym| *sym),
            bl1_zsyr2_blas: get_symbol(&libs, b"bl1_zsyr2_blas\0").map(|sym| *sym),
            bl1_strmv: get_symbol(&libs, b"bl1_strmv\0").map(|sym| *sym),
            bl1_dtrmv: get_symbol(&libs, b"bl1_dtrmv\0").map(|sym| *sym),
            bl1_ctrmv: get_symbol(&libs, b"bl1_ctrmv\0").map(|sym| *sym),
            bl1_ztrmv: get_symbol(&libs, b"bl1_ztrmv\0").map(|sym| *sym),
            bl1_strmv_blas: get_symbol(&libs, b"bl1_strmv_blas\0").map(|sym| *sym),
            bl1_dtrmv_blas: get_symbol(&libs, b"bl1_dtrmv_blas\0").map(|sym| *sym),
            bl1_ctrmv_blas: get_symbol(&libs, b"bl1_ctrmv_blas\0").map(|sym| *sym),
            bl1_ztrmv_blas: get_symbol(&libs, b"bl1_ztrmv_blas\0").map(|sym| *sym),
            bl1_strsv: get_symbol(&libs, b"bl1_strsv\0").map(|sym| *sym),
            bl1_dtrsv: get_symbol(&libs, b"bl1_dtrsv\0").map(|sym| *sym),
            bl1_ctrsv: get_symbol(&libs, b"bl1_ctrsv\0").map(|sym| *sym),
            bl1_ztrsv: get_symbol(&libs, b"bl1_ztrsv\0").map(|sym| *sym),
            bl1_strsv_blas: get_symbol(&libs, b"bl1_strsv_blas\0").map(|sym| *sym),
            bl1_dtrsv_blas: get_symbol(&libs, b"bl1_dtrsv_blas\0").map(|sym| *sym),
            bl1_ctrsv_blas: get_symbol(&libs, b"bl1_ctrsv_blas\0").map(|sym| *sym),
            bl1_ztrsv_blas: get_symbol(&libs, b"bl1_ztrsv_blas\0").map(|sym| *sym),
            bl1_strmvsx: get_symbol(&libs, b"bl1_strmvsx\0").map(|sym| *sym),
            bl1_dtrmvsx: get_symbol(&libs, b"bl1_dtrmvsx\0").map(|sym| *sym),
            bl1_ctrmvsx: get_symbol(&libs, b"bl1_ctrmvsx\0").map(|sym| *sym),
            bl1_ztrmvsx: get_symbol(&libs, b"bl1_ztrmvsx\0").map(|sym| *sym),
            bl1_strsvsx: get_symbol(&libs, b"bl1_strsvsx\0").map(|sym| *sym),
            bl1_dtrsvsx: get_symbol(&libs, b"bl1_dtrsvsx\0").map(|sym| *sym),
            bl1_ctrsvsx: get_symbol(&libs, b"bl1_ctrsvsx\0").map(|sym| *sym),
            bl1_ztrsvsx: get_symbol(&libs, b"bl1_ztrsvsx\0").map(|sym| *sym),
            bl1_sgemm: get_symbol(&libs, b"bl1_sgemm\0").map(|sym| *sym),
            bl1_dgemm: get_symbol(&libs, b"bl1_dgemm\0").map(|sym| *sym),
            bl1_cgemm: get_symbol(&libs, b"bl1_cgemm\0").map(|sym| *sym),
            bl1_zgemm: get_symbol(&libs, b"bl1_zgemm\0").map(|sym| *sym),
            bl1_sgemm_blas: get_symbol(&libs, b"bl1_sgemm_blas\0").map(|sym| *sym),
            bl1_dgemm_blas: get_symbol(&libs, b"bl1_dgemm_blas\0").map(|sym| *sym),
            bl1_cgemm_blas: get_symbol(&libs, b"bl1_cgemm_blas\0").map(|sym| *sym),
            bl1_zgemm_blas: get_symbol(&libs, b"bl1_zgemm_blas\0").map(|sym| *sym),
            bl1_shemm: get_symbol(&libs, b"bl1_shemm\0").map(|sym| *sym),
            bl1_dhemm: get_symbol(&libs, b"bl1_dhemm\0").map(|sym| *sym),
            bl1_chemm: get_symbol(&libs, b"bl1_chemm\0").map(|sym| *sym),
            bl1_zhemm: get_symbol(&libs, b"bl1_zhemm\0").map(|sym| *sym),
            bl1_chemm_blas: get_symbol(&libs, b"bl1_chemm_blas\0").map(|sym| *sym),
            bl1_zhemm_blas: get_symbol(&libs, b"bl1_zhemm_blas\0").map(|sym| *sym),
            bl1_sherk: get_symbol(&libs, b"bl1_sherk\0").map(|sym| *sym),
            bl1_dherk: get_symbol(&libs, b"bl1_dherk\0").map(|sym| *sym),
            bl1_cherk: get_symbol(&libs, b"bl1_cherk\0").map(|sym| *sym),
            bl1_zherk: get_symbol(&libs, b"bl1_zherk\0").map(|sym| *sym),
            bl1_cherk_blas: get_symbol(&libs, b"bl1_cherk_blas\0").map(|sym| *sym),
            bl1_zherk_blas: get_symbol(&libs, b"bl1_zherk_blas\0").map(|sym| *sym),
            bl1_sher2k: get_symbol(&libs, b"bl1_sher2k\0").map(|sym| *sym),
            bl1_dher2k: get_symbol(&libs, b"bl1_dher2k\0").map(|sym| *sym),
            bl1_cher2k: get_symbol(&libs, b"bl1_cher2k\0").map(|sym| *sym),
            bl1_zher2k: get_symbol(&libs, b"bl1_zher2k\0").map(|sym| *sym),
            bl1_cher2k_blas: get_symbol(&libs, b"bl1_cher2k_blas\0").map(|sym| *sym),
            bl1_zher2k_blas: get_symbol(&libs, b"bl1_zher2k_blas\0").map(|sym| *sym),
            bl1_ssymm: get_symbol(&libs, b"bl1_ssymm\0").map(|sym| *sym),
            bl1_dsymm: get_symbol(&libs, b"bl1_dsymm\0").map(|sym| *sym),
            bl1_csymm: get_symbol(&libs, b"bl1_csymm\0").map(|sym| *sym),
            bl1_zsymm: get_symbol(&libs, b"bl1_zsymm\0").map(|sym| *sym),
            bl1_ssymm_blas: get_symbol(&libs, b"bl1_ssymm_blas\0").map(|sym| *sym),
            bl1_dsymm_blas: get_symbol(&libs, b"bl1_dsymm_blas\0").map(|sym| *sym),
            bl1_csymm_blas: get_symbol(&libs, b"bl1_csymm_blas\0").map(|sym| *sym),
            bl1_zsymm_blas: get_symbol(&libs, b"bl1_zsymm_blas\0").map(|sym| *sym),
            bl1_ssyrk: get_symbol(&libs, b"bl1_ssyrk\0").map(|sym| *sym),
            bl1_dsyrk: get_symbol(&libs, b"bl1_dsyrk\0").map(|sym| *sym),
            bl1_csyrk: get_symbol(&libs, b"bl1_csyrk\0").map(|sym| *sym),
            bl1_zsyrk: get_symbol(&libs, b"bl1_zsyrk\0").map(|sym| *sym),
            bl1_ssyrk_blas: get_symbol(&libs, b"bl1_ssyrk_blas\0").map(|sym| *sym),
            bl1_dsyrk_blas: get_symbol(&libs, b"bl1_dsyrk_blas\0").map(|sym| *sym),
            bl1_csyrk_blas: get_symbol(&libs, b"bl1_csyrk_blas\0").map(|sym| *sym),
            bl1_zsyrk_blas: get_symbol(&libs, b"bl1_zsyrk_blas\0").map(|sym| *sym),
            bl1_ssyr2k: get_symbol(&libs, b"bl1_ssyr2k\0").map(|sym| *sym),
            bl1_dsyr2k: get_symbol(&libs, b"bl1_dsyr2k\0").map(|sym| *sym),
            bl1_csyr2k: get_symbol(&libs, b"bl1_csyr2k\0").map(|sym| *sym),
            bl1_zsyr2k: get_symbol(&libs, b"bl1_zsyr2k\0").map(|sym| *sym),
            bl1_ssyr2k_blas: get_symbol(&libs, b"bl1_ssyr2k_blas\0").map(|sym| *sym),
            bl1_dsyr2k_blas: get_symbol(&libs, b"bl1_dsyr2k_blas\0").map(|sym| *sym),
            bl1_csyr2k_blas: get_symbol(&libs, b"bl1_csyr2k_blas\0").map(|sym| *sym),
            bl1_zsyr2k_blas: get_symbol(&libs, b"bl1_zsyr2k_blas\0").map(|sym| *sym),
            bl1_strmm: get_symbol(&libs, b"bl1_strmm\0").map(|sym| *sym),
            bl1_dtrmm: get_symbol(&libs, b"bl1_dtrmm\0").map(|sym| *sym),
            bl1_ctrmm: get_symbol(&libs, b"bl1_ctrmm\0").map(|sym| *sym),
            bl1_ztrmm: get_symbol(&libs, b"bl1_ztrmm\0").map(|sym| *sym),
            bl1_strmm_blas: get_symbol(&libs, b"bl1_strmm_blas\0").map(|sym| *sym),
            bl1_dtrmm_blas: get_symbol(&libs, b"bl1_dtrmm_blas\0").map(|sym| *sym),
            bl1_ctrmm_blas: get_symbol(&libs, b"bl1_ctrmm_blas\0").map(|sym| *sym),
            bl1_ztrmm_blas: get_symbol(&libs, b"bl1_ztrmm_blas\0").map(|sym| *sym),
            bl1_strsm: get_symbol(&libs, b"bl1_strsm\0").map(|sym| *sym),
            bl1_dtrsm: get_symbol(&libs, b"bl1_dtrsm\0").map(|sym| *sym),
            bl1_ctrsm: get_symbol(&libs, b"bl1_ctrsm\0").map(|sym| *sym),
            bl1_ztrsm: get_symbol(&libs, b"bl1_ztrsm\0").map(|sym| *sym),
            bl1_strsm_blas: get_symbol(&libs, b"bl1_strsm_blas\0").map(|sym| *sym),
            bl1_dtrsm_blas: get_symbol(&libs, b"bl1_dtrsm_blas\0").map(|sym| *sym),
            bl1_ctrsm_blas: get_symbol(&libs, b"bl1_ctrsm_blas\0").map(|sym| *sym),
            bl1_ztrsm_blas: get_symbol(&libs, b"bl1_ztrsm_blas\0").map(|sym| *sym),
            bl1_strmmsx: get_symbol(&libs, b"bl1_strmmsx\0").map(|sym| *sym),
            bl1_dtrmmsx: get_symbol(&libs, b"bl1_dtrmmsx\0").map(|sym| *sym),
            bl1_ctrmmsx: get_symbol(&libs, b"bl1_ctrmmsx\0").map(|sym| *sym),
            bl1_ztrmmsx: get_symbol(&libs, b"bl1_ztrmmsx\0").map(|sym| *sym),
            bl1_strsmsx: get_symbol(&libs, b"bl1_strsmsx\0").map(|sym| *sym),
            bl1_dtrsmsx: get_symbol(&libs, b"bl1_dtrsmsx\0").map(|sym| *sym),
            bl1_ctrsmsx: get_symbol(&libs, b"bl1_ctrsmsx\0").map(|sym| *sym),
            bl1_ztrsmsx: get_symbol(&libs, b"bl1_ztrsmsx\0").map(|sym| *sym),
            bl1_saxmyv2: get_symbol(&libs, b"bl1_saxmyv2\0").map(|sym| *sym),
            bl1_daxmyv2: get_symbol(&libs, b"bl1_daxmyv2\0").map(|sym| *sym),
            bl1_caxmyv2: get_symbol(&libs, b"bl1_caxmyv2\0").map(|sym| *sym),
            bl1_zaxmyv2: get_symbol(&libs, b"bl1_zaxmyv2\0").map(|sym| *sym),
            bl1_saxpyv2b: get_symbol(&libs, b"bl1_saxpyv2b\0").map(|sym| *sym),
            bl1_daxpyv2b: get_symbol(&libs, b"bl1_daxpyv2b\0").map(|sym| *sym),
            bl1_caxpyv2b: get_symbol(&libs, b"bl1_caxpyv2b\0").map(|sym| *sym),
            bl1_zaxpyv2b: get_symbol(&libs, b"bl1_zaxpyv2b\0").map(|sym| *sym),
            bl1_saxpyv3b: get_symbol(&libs, b"bl1_saxpyv3b\0").map(|sym| *sym),
            bl1_daxpyv3b: get_symbol(&libs, b"bl1_daxpyv3b\0").map(|sym| *sym),
            bl1_caxpyv3b: get_symbol(&libs, b"bl1_caxpyv3b\0").map(|sym| *sym),
            bl1_zaxpyv3b: get_symbol(&libs, b"bl1_zaxpyv3b\0").map(|sym| *sym),
            bl1_saxpyv2bdotaxpy: get_symbol(&libs, b"bl1_saxpyv2bdotaxpy\0").map(|sym| *sym),
            bl1_daxpyv2bdotaxpy: get_symbol(&libs, b"bl1_daxpyv2bdotaxpy\0").map(|sym| *sym),
            bl1_caxpyv2bdotaxpy: get_symbol(&libs, b"bl1_caxpyv2bdotaxpy\0").map(|sym| *sym),
            bl1_zaxpyv2bdotaxpy: get_symbol(&libs, b"bl1_zaxpyv2bdotaxpy\0").map(|sym| *sym),
            bl1_sdotsv2: get_symbol(&libs, b"bl1_sdotsv2\0").map(|sym| *sym),
            bl1_ddotsv2: get_symbol(&libs, b"bl1_ddotsv2\0").map(|sym| *sym),
            bl1_cdotsv2: get_symbol(&libs, b"bl1_cdotsv2\0").map(|sym| *sym),
            bl1_zdotsv2: get_symbol(&libs, b"bl1_zdotsv2\0").map(|sym| *sym),
            bl1_sdotsv3: get_symbol(&libs, b"bl1_sdotsv3\0").map(|sym| *sym),
            bl1_ddotsv3: get_symbol(&libs, b"bl1_ddotsv3\0").map(|sym| *sym),
            bl1_cdotsv3: get_symbol(&libs, b"bl1_cdotsv3\0").map(|sym| *sym),
            bl1_zdotsv3: get_symbol(&libs, b"bl1_zdotsv3\0").map(|sym| *sym),
            bl1_sdotaxpy: get_symbol(&libs, b"bl1_sdotaxpy\0").map(|sym| *sym),
            bl1_ddotaxpy: get_symbol(&libs, b"bl1_ddotaxpy\0").map(|sym| *sym),
            bl1_cdotaxpy: get_symbol(&libs, b"bl1_cdotaxpy\0").map(|sym| *sym),
            bl1_zdotaxpy: get_symbol(&libs, b"bl1_zdotaxpy\0").map(|sym| *sym),
            bl1_sdotaxmyv2: get_symbol(&libs, b"bl1_sdotaxmyv2\0").map(|sym| *sym),
            bl1_ddotaxmyv2: get_symbol(&libs, b"bl1_ddotaxmyv2\0").map(|sym| *sym),
            bl1_cdotaxmyv2: get_symbol(&libs, b"bl1_cdotaxmyv2\0").map(|sym| *sym),
            bl1_zdotaxmyv2: get_symbol(&libs, b"bl1_zdotaxmyv2\0").map(|sym| *sym),
            bl1_sdotv2axpyv2b: get_symbol(&libs, b"bl1_sdotv2axpyv2b\0").map(|sym| *sym),
            bl1_ddotv2axpyv2b: get_symbol(&libs, b"bl1_ddotv2axpyv2b\0").map(|sym| *sym),
            bl1_cdotv2axpyv2b: get_symbol(&libs, b"bl1_cdotv2axpyv2b\0").map(|sym| *sym),
            bl1_zdotv2axpyv2b: get_symbol(&libs, b"bl1_zdotv2axpyv2b\0").map(|sym| *sym),
            bl1_zaxpyv2bdots: get_symbol(&libs, b"bl1_zaxpyv2bdots\0").map(|sym| *sym),
            isamax_: get_symbol(&libs, b"isamax_\0").map(|sym| *sym),
            idamax_: get_symbol(&libs, b"idamax_\0").map(|sym| *sym),
            icamax_: get_symbol(&libs, b"icamax_\0").map(|sym| *sym),
            izamax_: get_symbol(&libs, b"izamax_\0").map(|sym| *sym),
            sasum_: get_symbol(&libs, b"sasum_\0").map(|sym| *sym),
            dasum_: get_symbol(&libs, b"dasum_\0").map(|sym| *sym),
            scasum_: get_symbol(&libs, b"scasum_\0").map(|sym| *sym),
            dzasum_: get_symbol(&libs, b"dzasum_\0").map(|sym| *sym),
            saxpy_: get_symbol(&libs, b"saxpy_\0").map(|sym| *sym),
            daxpy_: get_symbol(&libs, b"daxpy_\0").map(|sym| *sym),
            caxpy_: get_symbol(&libs, b"caxpy_\0").map(|sym| *sym),
            zaxpy_: get_symbol(&libs, b"zaxpy_\0").map(|sym| *sym),
            scopy_: get_symbol(&libs, b"scopy_\0").map(|sym| *sym),
            dcopy_: get_symbol(&libs, b"dcopy_\0").map(|sym| *sym),
            ccopy_: get_symbol(&libs, b"ccopy_\0").map(|sym| *sym),
            zcopy_: get_symbol(&libs, b"zcopy_\0").map(|sym| *sym),
            sdot_: get_symbol(&libs, b"sdot_\0").map(|sym| *sym),
            ddot_: get_symbol(&libs, b"ddot_\0").map(|sym| *sym),
            cdotu_: get_symbol(&libs, b"cdotu_\0").map(|sym| *sym),
            cdotc_: get_symbol(&libs, b"cdotc_\0").map(|sym| *sym),
            zdotu_: get_symbol(&libs, b"zdotu_\0").map(|sym| *sym),
            zdotc_: get_symbol(&libs, b"zdotc_\0").map(|sym| *sym),
            snrm2_: get_symbol(&libs, b"snrm2_\0").map(|sym| *sym),
            dnrm2_: get_symbol(&libs, b"dnrm2_\0").map(|sym| *sym),
            scnrm2_: get_symbol(&libs, b"scnrm2_\0").map(|sym| *sym),
            dznrm2_: get_symbol(&libs, b"dznrm2_\0").map(|sym| *sym),
            sscal_: get_symbol(&libs, b"sscal_\0").map(|sym| *sym),
            dscal_: get_symbol(&libs, b"dscal_\0").map(|sym| *sym),
            cscal_: get_symbol(&libs, b"cscal_\0").map(|sym| *sym),
            csscal_: get_symbol(&libs, b"csscal_\0").map(|sym| *sym),
            zscal_: get_symbol(&libs, b"zscal_\0").map(|sym| *sym),
            zdscal_: get_symbol(&libs, b"zdscal_\0").map(|sym| *sym),
            sswap_: get_symbol(&libs, b"sswap_\0").map(|sym| *sym),
            dswap_: get_symbol(&libs, b"dswap_\0").map(|sym| *sym),
            cswap_: get_symbol(&libs, b"cswap_\0").map(|sym| *sym),
            zswap_: get_symbol(&libs, b"zswap_\0").map(|sym| *sym),
            sgemv_: get_symbol(&libs, b"sgemv_\0").map(|sym| *sym),
            dgemv_: get_symbol(&libs, b"dgemv_\0").map(|sym| *sym),
            cgemv_: get_symbol(&libs, b"cgemv_\0").map(|sym| *sym),
            zgemv_: get_symbol(&libs, b"zgemv_\0").map(|sym| *sym),
            sger_: get_symbol(&libs, b"sger_\0").map(|sym| *sym),
            dger_: get_symbol(&libs, b"dger_\0").map(|sym| *sym),
            cgerc_: get_symbol(&libs, b"cgerc_\0").map(|sym| *sym),
            cgeru_: get_symbol(&libs, b"cgeru_\0").map(|sym| *sym),
            zgerc_: get_symbol(&libs, b"zgerc_\0").map(|sym| *sym),
            zgeru_: get_symbol(&libs, b"zgeru_\0").map(|sym| *sym),
            chemv_: get_symbol(&libs, b"chemv_\0").map(|sym| *sym),
            zhemv_: get_symbol(&libs, b"zhemv_\0").map(|sym| *sym),
            cher_: get_symbol(&libs, b"cher_\0").map(|sym| *sym),
            zher_: get_symbol(&libs, b"zher_\0").map(|sym| *sym),
            cher2_: get_symbol(&libs, b"cher2_\0").map(|sym| *sym),
            zher2_: get_symbol(&libs, b"zher2_\0").map(|sym| *sym),
            ssymv_: get_symbol(&libs, b"ssymv_\0").map(|sym| *sym),
            dsymv_: get_symbol(&libs, b"dsymv_\0").map(|sym| *sym),
            ssyr_: get_symbol(&libs, b"ssyr_\0").map(|sym| *sym),
            dsyr_: get_symbol(&libs, b"dsyr_\0").map(|sym| *sym),
            ssyr2_: get_symbol(&libs, b"ssyr2_\0").map(|sym| *sym),
            dsyr2_: get_symbol(&libs, b"dsyr2_\0").map(|sym| *sym),
            strmv_: get_symbol(&libs, b"strmv_\0").map(|sym| *sym),
            dtrmv_: get_symbol(&libs, b"dtrmv_\0").map(|sym| *sym),
            ctrmv_: get_symbol(&libs, b"ctrmv_\0").map(|sym| *sym),
            ztrmv_: get_symbol(&libs, b"ztrmv_\0").map(|sym| *sym),
            strsv_: get_symbol(&libs, b"strsv_\0").map(|sym| *sym),
            dtrsv_: get_symbol(&libs, b"dtrsv_\0").map(|sym| *sym),
            ctrsv_: get_symbol(&libs, b"ctrsv_\0").map(|sym| *sym),
            ztrsv_: get_symbol(&libs, b"ztrsv_\0").map(|sym| *sym),
            sgemm_: get_symbol(&libs, b"sgemm_\0").map(|sym| *sym),
            dgemm_: get_symbol(&libs, b"dgemm_\0").map(|sym| *sym),
            cgemm_: get_symbol(&libs, b"cgemm_\0").map(|sym| *sym),
            zgemm_: get_symbol(&libs, b"zgemm_\0").map(|sym| *sym),
            chemm_: get_symbol(&libs, b"chemm_\0").map(|sym| *sym),
            zhemm_: get_symbol(&libs, b"zhemm_\0").map(|sym| *sym),
            cherk_: get_symbol(&libs, b"cherk_\0").map(|sym| *sym),
            zherk_: get_symbol(&libs, b"zherk_\0").map(|sym| *sym),
            cher2k_: get_symbol(&libs, b"cher2k_\0").map(|sym| *sym),
            zher2k_: get_symbol(&libs, b"zher2k_\0").map(|sym| *sym),
            ssymm_: get_symbol(&libs, b"ssymm_\0").map(|sym| *sym),
            dsymm_: get_symbol(&libs, b"dsymm_\0").map(|sym| *sym),
            csymm_: get_symbol(&libs, b"csymm_\0").map(|sym| *sym),
            zsymm_: get_symbol(&libs, b"zsymm_\0").map(|sym| *sym),
            ssyrk_: get_symbol(&libs, b"ssyrk_\0").map(|sym| *sym),
            dsyrk_: get_symbol(&libs, b"dsyrk_\0").map(|sym| *sym),
            csyrk_: get_symbol(&libs, b"csyrk_\0").map(|sym| *sym),
            zsyrk_: get_symbol(&libs, b"zsyrk_\0").map(|sym| *sym),
            ssyr2k_: get_symbol(&libs, b"ssyr2k_\0").map(|sym| *sym),
            dsyr2k_: get_symbol(&libs, b"dsyr2k_\0").map(|sym| *sym),
            csyr2k_: get_symbol(&libs, b"csyr2k_\0").map(|sym| *sym),
            zsyr2k_: get_symbol(&libs, b"zsyr2k_\0").map(|sym| *sym),
            strmm_: get_symbol(&libs, b"strmm_\0").map(|sym| *sym),
            dtrmm_: get_symbol(&libs, b"dtrmm_\0").map(|sym| *sym),
            ctrmm_: get_symbol(&libs, b"ctrmm_\0").map(|sym| *sym),
            ztrmm_: get_symbol(&libs, b"ztrmm_\0").map(|sym| *sym),
            strsm_: get_symbol(&libs, b"strsm_\0").map(|sym| *sym),
            dtrsm_: get_symbol(&libs, b"dtrsm_\0").map(|sym| *sym),
            ctrsm_: get_symbol(&libs, b"ctrsm_\0").map(|sym| *sym),
            ztrsm_: get_symbol(&libs, b"ztrsm_\0").map(|sym| *sym),
            FLA_Cntl_obj_free: get_symbol(&libs, b"FLA_Cntl_obj_free\0").map(|sym| *sym),
            FLA_Cntl_axpy_obj_create: get_symbol(&libs, b"FLA_Cntl_axpy_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_axpyt_obj_create: get_symbol(&libs, b"FLA_Cntl_axpyt_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_copy_obj_create: get_symbol(&libs, b"FLA_Cntl_copy_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_copyt_obj_create: get_symbol(&libs, b"FLA_Cntl_copyt_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_copyr_obj_create: get_symbol(&libs, b"FLA_Cntl_copyr_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_scal_obj_create: get_symbol(&libs, b"FLA_Cntl_scal_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_scalr_obj_create: get_symbol(&libs, b"FLA_Cntl_scalr_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_swap_obj_create: get_symbol(&libs, b"FLA_Cntl_swap_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_tpose_obj_create: get_symbol(&libs, b"FLA_Cntl_tpose_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_gemv_obj_create: get_symbol(&libs, b"FLA_Cntl_gemv_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_trsv_obj_create: get_symbol(&libs, b"FLA_Cntl_trsv_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_gemm_obj_create: get_symbol(&libs, b"FLA_Cntl_gemm_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_hemm_obj_create: get_symbol(&libs, b"FLA_Cntl_hemm_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_herk_obj_create: get_symbol(&libs, b"FLA_Cntl_herk_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_her2k_obj_create: get_symbol(&libs, b"FLA_Cntl_her2k_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_symm_obj_create: get_symbol(&libs, b"FLA_Cntl_symm_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_syrk_obj_create: get_symbol(&libs, b"FLA_Cntl_syrk_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_syr2k_obj_create: get_symbol(&libs, b"FLA_Cntl_syr2k_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_trmm_obj_create: get_symbol(&libs, b"FLA_Cntl_trmm_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_trsm_obj_create: get_symbol(&libs, b"FLA_Cntl_trsm_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_chol_obj_create: get_symbol(&libs, b"FLA_Cntl_chol_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_lu_obj_create: get_symbol(&libs, b"FLA_Cntl_lu_obj_create\0").map(|sym| *sym),
            FLA_Cntl_appiv_obj_create: get_symbol(&libs, b"FLA_Cntl_appiv_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_qrut_obj_create: get_symbol(&libs, b"FLA_Cntl_qrut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_qr2ut_obj_create: get_symbol(&libs, b"FLA_Cntl_qr2ut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_lqut_obj_create: get_symbol(&libs, b"FLA_Cntl_lqut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_caqr2ut_obj_create: get_symbol(&libs, b"FLA_Cntl_caqr2ut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_hessut_obj_create: get_symbol(&libs, b"FLA_Cntl_hessut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_tridiagut_obj_create: get_symbol(&libs, b"FLA_Cntl_tridiagut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_bidiagut_obj_create: get_symbol(&libs, b"FLA_Cntl_bidiagut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_trinv_obj_create: get_symbol(&libs, b"FLA_Cntl_trinv_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_ttmm_obj_create: get_symbol(&libs, b"FLA_Cntl_ttmm_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_sylv_obj_create: get_symbol(&libs, b"FLA_Cntl_sylv_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_lyap_obj_create: get_symbol(&libs, b"FLA_Cntl_lyap_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_spdinv_obj_create: get_symbol(&libs, b"FLA_Cntl_spdinv_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_apqut_obj_create: get_symbol(&libs, b"FLA_Cntl_apqut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_apq2ut_obj_create: get_symbol(&libs, b"FLA_Cntl_apq2ut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_apcaq2ut_obj_create: get_symbol(&libs, b"FLA_Cntl_apcaq2ut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_qrutinc_obj_create: get_symbol(&libs, b"FLA_Cntl_qrutinc_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_apqutinc_obj_create: get_symbol(&libs, b"FLA_Cntl_apqutinc_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_caqrutinc_obj_create: get_symbol(&libs, b"FLA_Cntl_caqrutinc_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_apcaqutinc_obj_create: get_symbol(&libs, b"FLA_Cntl_apcaqutinc_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_uddateut_obj_create: get_symbol(&libs, b"FLA_Cntl_uddateut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_apqudut_obj_create: get_symbol(&libs, b"FLA_Cntl_apqudut_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_uddateutinc_obj_create: get_symbol(
                &libs,
                b"FLA_Cntl_uddateutinc_obj_create\0",
            )
            .map(|sym| *sym),
            FLA_Cntl_apqudutinc_obj_create: get_symbol(&libs, b"FLA_Cntl_apqudutinc_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_eig_gest_obj_create: get_symbol(&libs, b"FLA_Cntl_eig_gest_obj_create\0")
                .map(|sym| *sym),
            FLA_Cntl_init_flamec: get_symbol(&libs, b"FLA_Cntl_init_flamec\0").map(|sym| *sym),
            FLA_Cntl_finalize_flamec: get_symbol(&libs, b"FLA_Cntl_finalize_flamec\0")
                .map(|sym| *sym),
            FLA_Transpose_cntl_init: get_symbol(&libs, b"FLA_Transpose_cntl_init\0")
                .map(|sym| *sym),
            FLA_Transpose_cntl_finalize: get_symbol(&libs, b"FLA_Transpose_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Axpy_cntl_init: get_symbol(&libs, b"FLA_Axpy_cntl_init\0").map(|sym| *sym),
            FLA_Axpyt_cntl_init: get_symbol(&libs, b"FLA_Axpyt_cntl_init\0").map(|sym| *sym),
            FLA_Copy_cntl_init: get_symbol(&libs, b"FLA_Copy_cntl_init\0").map(|sym| *sym),
            FLA_Copyt_cntl_init: get_symbol(&libs, b"FLA_Copyt_cntl_init\0").map(|sym| *sym),
            FLA_Copyr_cntl_init: get_symbol(&libs, b"FLA_Copyr_cntl_init\0").map(|sym| *sym),
            FLA_Scal_cntl_init: get_symbol(&libs, b"FLA_Scal_cntl_init\0").map(|sym| *sym),
            FLA_Scalr_cntl_init: get_symbol(&libs, b"FLA_Scalr_cntl_init\0").map(|sym| *sym),
            FLA_Axpy_cntl_finalize: get_symbol(&libs, b"FLA_Axpy_cntl_finalize\0").map(|sym| *sym),
            FLA_Axpyt_cntl_finalize: get_symbol(&libs, b"FLA_Axpyt_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Copy_cntl_finalize: get_symbol(&libs, b"FLA_Copy_cntl_finalize\0").map(|sym| *sym),
            FLA_Copyt_cntl_finalize: get_symbol(&libs, b"FLA_Copyt_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Copyr_cntl_finalize: get_symbol(&libs, b"FLA_Copyr_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Scal_cntl_finalize: get_symbol(&libs, b"FLA_Scal_cntl_finalize\0").map(|sym| *sym),
            FLA_Scalr_cntl_finalize: get_symbol(&libs, b"FLA_Scalr_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Gemv_cntl_init: get_symbol(&libs, b"FLA_Gemv_cntl_init\0").map(|sym| *sym),
            FLA_Trsv_cntl_init: get_symbol(&libs, b"FLA_Trsv_cntl_init\0").map(|sym| *sym),
            FLA_Gemv_cntl_finalize: get_symbol(&libs, b"FLA_Gemv_cntl_finalize\0").map(|sym| *sym),
            FLA_Trsv_cntl_finalize: get_symbol(&libs, b"FLA_Trsv_cntl_finalize\0").map(|sym| *sym),
            FLA_Gemm_cntl_init: get_symbol(&libs, b"FLA_Gemm_cntl_init\0").map(|sym| *sym),
            FLA_Hemm_cntl_init: get_symbol(&libs, b"FLA_Hemm_cntl_init\0").map(|sym| *sym),
            FLA_Herk_cntl_init: get_symbol(&libs, b"FLA_Herk_cntl_init\0").map(|sym| *sym),
            FLA_Her2k_cntl_init: get_symbol(&libs, b"FLA_Her2k_cntl_init\0").map(|sym| *sym),
            FLA_Symm_cntl_init: get_symbol(&libs, b"FLA_Symm_cntl_init\0").map(|sym| *sym),
            FLA_Syrk_cntl_init: get_symbol(&libs, b"FLA_Syrk_cntl_init\0").map(|sym| *sym),
            FLA_Syr2k_cntl_init: get_symbol(&libs, b"FLA_Syr2k_cntl_init\0").map(|sym| *sym),
            FLA_Trmm_cntl_init: get_symbol(&libs, b"FLA_Trmm_cntl_init\0").map(|sym| *sym),
            FLA_Trsm_cntl_init: get_symbol(&libs, b"FLA_Trsm_cntl_init\0").map(|sym| *sym),
            FLA_Gemm_cntl_finalize: get_symbol(&libs, b"FLA_Gemm_cntl_finalize\0").map(|sym| *sym),
            FLA_Hemm_cntl_finalize: get_symbol(&libs, b"FLA_Hemm_cntl_finalize\0").map(|sym| *sym),
            FLA_Herk_cntl_finalize: get_symbol(&libs, b"FLA_Herk_cntl_finalize\0").map(|sym| *sym),
            FLA_Her2k_cntl_finalize: get_symbol(&libs, b"FLA_Her2k_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Symm_cntl_finalize: get_symbol(&libs, b"FLA_Symm_cntl_finalize\0").map(|sym| *sym),
            FLA_Syrk_cntl_finalize: get_symbol(&libs, b"FLA_Syrk_cntl_finalize\0").map(|sym| *sym),
            FLA_Syr2k_cntl_finalize: get_symbol(&libs, b"FLA_Syr2k_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Trmm_cntl_finalize: get_symbol(&libs, b"FLA_Trmm_cntl_finalize\0").map(|sym| *sym),
            FLA_Trsm_cntl_finalize: get_symbol(&libs, b"FLA_Trsm_cntl_finalize\0").map(|sym| *sym),
            FLA_Apply_pivots_cntl_init: get_symbol(&libs, b"FLA_Apply_pivots_cntl_init\0")
                .map(|sym| *sym),
            FLA_Chol_cntl_init: get_symbol(&libs, b"FLA_Chol_cntl_init\0").map(|sym| *sym),
            FLA_LU_piv_cntl_init: get_symbol(&libs, b"FLA_LU_piv_cntl_init\0").map(|sym| *sym),
            FLA_LU_nopiv_cntl_init: get_symbol(&libs, b"FLA_LU_nopiv_cntl_init\0").map(|sym| *sym),
            FLA_QR_UT_cntl_init: get_symbol(&libs, b"FLA_QR_UT_cntl_init\0").map(|sym| *sym),
            FLA_QR2_UT_cntl_init: get_symbol(&libs, b"FLA_QR2_UT_cntl_init\0").map(|sym| *sym),
            FLA_LQ_UT_cntl_init: get_symbol(&libs, b"FLA_LQ_UT_cntl_init\0").map(|sym| *sym),
            FLA_CAQR2_UT_cntl_init: get_symbol(&libs, b"FLA_CAQR2_UT_cntl_init\0").map(|sym| *sym),
            FLA_UDdate_UT_cntl_init: get_symbol(&libs, b"FLA_UDdate_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Hess_UT_cntl_init: get_symbol(&libs, b"FLA_Hess_UT_cntl_init\0").map(|sym| *sym),
            FLA_Tridiag_UT_cntl_init: get_symbol(&libs, b"FLA_Tridiag_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_cntl_init: get_symbol(&libs, b"FLA_Bidiag_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Trinv_cntl_init: get_symbol(&libs, b"FLA_Trinv_cntl_init\0").map(|sym| *sym),
            FLA_Ttmm_cntl_init: get_symbol(&libs, b"FLA_Ttmm_cntl_init\0").map(|sym| *sym),
            FLA_Sylv_cntl_init: get_symbol(&libs, b"FLA_Sylv_cntl_init\0").map(|sym| *sym),
            FLA_Lyap_cntl_init: get_symbol(&libs, b"FLA_Lyap_cntl_init\0").map(|sym| *sym),
            FLA_SPDinv_cntl_init: get_symbol(&libs, b"FLA_SPDinv_cntl_init\0").map(|sym| *sym),
            FLA_Apply_Q_UT_cntl_init: get_symbol(&libs, b"FLA_Apply_Q_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_cntl_init: get_symbol(&libs, b"FLA_Apply_Q2_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_cntl_init: get_symbol(&libs, b"FLA_Apply_CAQ2_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT_cntl_init: get_symbol(&libs, b"FLA_Apply_QUD_UT_cntl_init\0")
                .map(|sym| *sym),
            FLA_Eig_gest_cntl_init: get_symbol(&libs, b"FLA_Eig_gest_cntl_init\0").map(|sym| *sym),
            FLA_Apply_pivots_cntl_finalize: get_symbol(&libs, b"FLA_Apply_pivots_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Chol_cntl_finalize: get_symbol(&libs, b"FLA_Chol_cntl_finalize\0").map(|sym| *sym),
            FLA_LU_piv_cntl_finalize: get_symbol(&libs, b"FLA_LU_piv_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_LU_nopiv_cntl_finalize: get_symbol(&libs, b"FLA_LU_nopiv_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_QR_UT_cntl_finalize: get_symbol(&libs, b"FLA_QR_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_QR2_UT_cntl_finalize: get_symbol(&libs, b"FLA_QR2_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_LQ_UT_cntl_finalize: get_symbol(&libs, b"FLA_LQ_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_CAQR2_UT_cntl_finalize: get_symbol(&libs, b"FLA_CAQR2_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_cntl_finalize: get_symbol(&libs, b"FLA_UDdate_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Hess_UT_cntl_finalize: get_symbol(&libs, b"FLA_Hess_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_cntl_finalize: get_symbol(&libs, b"FLA_Tridiag_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_cntl_finalize: get_symbol(&libs, b"FLA_Bidiag_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Trinv_cntl_finalize: get_symbol(&libs, b"FLA_Trinv_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Ttmm_cntl_finalize: get_symbol(&libs, b"FLA_Ttmm_cntl_finalize\0").map(|sym| *sym),
            FLA_Sylv_cntl_finalize: get_symbol(&libs, b"FLA_Sylv_cntl_finalize\0").map(|sym| *sym),
            FLA_Lyap_cntl_finalize: get_symbol(&libs, b"FLA_Lyap_cntl_finalize\0").map(|sym| *sym),
            FLA_SPDinv_cntl_finalize: get_symbol(&libs, b"FLA_SPDinv_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_cntl_finalize: get_symbol(&libs, b"FLA_Apply_Q_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_cntl_finalize: get_symbol(&libs, b"FLA_Apply_Q2_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_cntl_finalize: get_symbol(
                &libs,
                b"FLA_Apply_CAQ2_UT_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLA_Apply_QUD_UT_cntl_finalize: get_symbol(&libs, b"FLA_Apply_QUD_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Eig_gest_cntl_finalize: get_symbol(&libs, b"FLA_Eig_gest_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Cntl_init_flash: get_symbol(&libs, b"FLA_Cntl_init_flash\0").map(|sym| *sym),
            FLA_Cntl_finalize_flash: get_symbol(&libs, b"FLA_Cntl_finalize_flash\0")
                .map(|sym| *sym),
            FLASH_Transpose_cntl_init: get_symbol(&libs, b"FLASH_Transpose_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Transpose_cntl_finalize: get_symbol(&libs, b"FLASH_Transpose_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Axpy_cntl_init: get_symbol(&libs, b"FLASH_Axpy_cntl_init\0").map(|sym| *sym),
            FLASH_Axpyt_cntl_init: get_symbol(&libs, b"FLASH_Axpyt_cntl_init\0").map(|sym| *sym),
            FLASH_Copy_cntl_init: get_symbol(&libs, b"FLASH_Copy_cntl_init\0").map(|sym| *sym),
            FLASH_Copyt_cntl_init: get_symbol(&libs, b"FLASH_Copyt_cntl_init\0").map(|sym| *sym),
            FLASH_Copyr_cntl_init: get_symbol(&libs, b"FLASH_Copyr_cntl_init\0").map(|sym| *sym),
            FLASH_Scal_cntl_init: get_symbol(&libs, b"FLASH_Scal_cntl_init\0").map(|sym| *sym),
            FLASH_Scalr_cntl_init: get_symbol(&libs, b"FLASH_Scalr_cntl_init\0").map(|sym| *sym),
            FLASH_Axpy_cntl_finalize: get_symbol(&libs, b"FLASH_Axpy_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Axpyt_cntl_finalize: get_symbol(&libs, b"FLASH_Axpyt_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Copy_cntl_finalize: get_symbol(&libs, b"FLASH_Copy_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Copyt_cntl_finalize: get_symbol(&libs, b"FLASH_Copyt_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Copyr_cntl_finalize: get_symbol(&libs, b"FLASH_Copyr_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Scal_cntl_finalize: get_symbol(&libs, b"FLASH_Scal_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Scalr_cntl_finalize: get_symbol(&libs, b"FLASH_Scalr_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Gemv_cntl_init: get_symbol(&libs, b"FLASH_Gemv_cntl_init\0").map(|sym| *sym),
            FLASH_Trsv_cntl_init: get_symbol(&libs, b"FLASH_Trsv_cntl_init\0").map(|sym| *sym),
            FLASH_Gemv_cntl_finalize: get_symbol(&libs, b"FLASH_Gemv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Trsv_cntl_finalize: get_symbol(&libs, b"FLASH_Trsv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Gemm_cntl_init: get_symbol(&libs, b"FLASH_Gemm_cntl_init\0").map(|sym| *sym),
            FLASH_Hemm_cntl_init: get_symbol(&libs, b"FLASH_Hemm_cntl_init\0").map(|sym| *sym),
            FLASH_Herk_cntl_init: get_symbol(&libs, b"FLASH_Herk_cntl_init\0").map(|sym| *sym),
            FLASH_Her2k_cntl_init: get_symbol(&libs, b"FLASH_Her2k_cntl_init\0").map(|sym| *sym),
            FLASH_Symm_cntl_init: get_symbol(&libs, b"FLASH_Symm_cntl_init\0").map(|sym| *sym),
            FLASH_Syrk_cntl_init: get_symbol(&libs, b"FLASH_Syrk_cntl_init\0").map(|sym| *sym),
            FLASH_Syr2k_cntl_init: get_symbol(&libs, b"FLASH_Syr2k_cntl_init\0").map(|sym| *sym),
            FLASH_Trmm_cntl_init: get_symbol(&libs, b"FLASH_Trmm_cntl_init\0").map(|sym| *sym),
            FLASH_Trsm_cntl_init: get_symbol(&libs, b"FLASH_Trsm_cntl_init\0").map(|sym| *sym),
            FLASH_Gemm_cntl_finalize: get_symbol(&libs, b"FLASH_Gemm_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Hemm_cntl_finalize: get_symbol(&libs, b"FLASH_Hemm_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Herk_cntl_finalize: get_symbol(&libs, b"FLASH_Herk_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Her2k_cntl_finalize: get_symbol(&libs, b"FLASH_Her2k_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Symm_cntl_finalize: get_symbol(&libs, b"FLASH_Symm_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Syrk_cntl_finalize: get_symbol(&libs, b"FLASH_Syrk_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Syr2k_cntl_finalize: get_symbol(&libs, b"FLASH_Syr2k_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Trmm_cntl_finalize: get_symbol(&libs, b"FLASH_Trmm_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Trsm_cntl_finalize: get_symbol(&libs, b"FLASH_Trsm_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Apply_pivots_cntl_init: get_symbol(&libs, b"FLASH_Apply_pivots_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Chol_cntl_init: get_symbol(&libs, b"FLASH_Chol_cntl_init\0").map(|sym| *sym),
            FLASH_LU_nopiv_cntl_init: get_symbol(&libs, b"FLASH_LU_nopiv_cntl_init\0")
                .map(|sym| *sym),
            FLASH_LU_piv_cntl_init: get_symbol(&libs, b"FLASH_LU_piv_cntl_init\0").map(|sym| *sym),
            FLASH_LU_incpiv_cntl_init: get_symbol(&libs, b"FLASH_LU_incpiv_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Trinv_cntl_init: get_symbol(&libs, b"FLASH_Trinv_cntl_init\0").map(|sym| *sym),
            FLASH_Ttmm_cntl_init: get_symbol(&libs, b"FLASH_Ttmm_cntl_init\0").map(|sym| *sym),
            FLASH_SPDinv_cntl_init: get_symbol(&libs, b"FLASH_SPDinv_cntl_init\0").map(|sym| *sym),
            FLASH_Sylv_cntl_init: get_symbol(&libs, b"FLASH_Sylv_cntl_init\0").map(|sym| *sym),
            FLASH_Lyap_cntl_init: get_symbol(&libs, b"FLASH_Lyap_cntl_init\0").map(|sym| *sym),
            FLASH_QR_UT_cntl_init: get_symbol(&libs, b"FLASH_QR_UT_cntl_init\0").map(|sym| *sym),
            FLASH_QR2_UT_cntl_init: get_symbol(&libs, b"FLASH_QR2_UT_cntl_init\0").map(|sym| *sym),
            FLASH_LQ_UT_cntl_init: get_symbol(&libs, b"FLASH_LQ_UT_cntl_init\0").map(|sym| *sym),
            FLASH_CAQR2_UT_cntl_init: get_symbol(&libs, b"FLASH_CAQR2_UT_cntl_init\0")
                .map(|sym| *sym),
            FLASH_UDdate_UT_cntl_init: get_symbol(&libs, b"FLASH_UDdate_UT_cntl_init\0")
                .map(|sym| *sym),
            FLASH_QR_UT_inc_cntl_init: get_symbol(&libs, b"FLASH_QR_UT_inc_cntl_init\0")
                .map(|sym| *sym),
            FLASH_CAQR_UT_inc_cntl_init: get_symbol(&libs, b"FLASH_CAQR_UT_inc_cntl_init\0")
                .map(|sym| *sym),
            FLASH_UDdate_UT_inc_cntl_init: get_symbol(&libs, b"FLASH_UDdate_UT_inc_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_Q_UT_cntl_init: get_symbol(&libs, b"FLASH_Apply_Q_UT_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_Q2_UT_cntl_init: get_symbol(&libs, b"FLASH_Apply_Q2_UT_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_CAQ2_UT_cntl_init: get_symbol(&libs, b"FLASH_Apply_CAQ2_UT_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_QUD_UT_cntl_init: get_symbol(&libs, b"FLASH_Apply_QUD_UT_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_Q_UT_inc_cntl_init: get_symbol(&libs, b"FLASH_Apply_Q_UT_inc_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_CAQ_UT_inc_cntl_init: get_symbol(
                &libs,
                b"FLASH_Apply_CAQ_UT_inc_cntl_init\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_QUD_UT_inc_cntl_init: get_symbol(
                &libs,
                b"FLASH_Apply_QUD_UT_inc_cntl_init\0",
            )
            .map(|sym| *sym),
            FLASH_Eig_gest_cntl_init: get_symbol(&libs, b"FLASH_Eig_gest_cntl_init\0")
                .map(|sym| *sym),
            FLASH_Apply_pivots_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_pivots_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Chol_cntl_finalize: get_symbol(&libs, b"FLASH_Chol_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_LU_nopiv_cntl_finalize: get_symbol(&libs, b"FLASH_LU_nopiv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_LU_piv_cntl_finalize: get_symbol(&libs, b"FLASH_LU_piv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_LU_incpiv_cntl_finalize: get_symbol(&libs, b"FLASH_LU_incpiv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Trinv_cntl_finalize: get_symbol(&libs, b"FLASH_Trinv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Ttmm_cntl_finalize: get_symbol(&libs, b"FLASH_Ttmm_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_SPDinv_cntl_finalize: get_symbol(&libs, b"FLASH_SPDinv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Sylv_cntl_finalize: get_symbol(&libs, b"FLASH_Sylv_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Lyap_cntl_finalize: get_symbol(&libs, b"FLASH_Lyap_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_QR_UT_cntl_finalize: get_symbol(&libs, b"FLASH_QR_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_QR2_UT_cntl_finalize: get_symbol(&libs, b"FLASH_QR2_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_LQ_UT_cntl_finalize: get_symbol(&libs, b"FLASH_LQ_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_CAQR2_UT_cntl_finalize: get_symbol(&libs, b"FLASH_CAQR2_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_UDdate_UT_cntl_finalize: get_symbol(&libs, b"FLASH_UDdate_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_QR_UT_inc_cntl_finalize: get_symbol(&libs, b"FLASH_QR_UT_inc_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_CAQR_UT_inc_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_CAQR_UT_inc_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_UDdate_UT_inc_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_UDdate_UT_inc_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_Q_UT_cntl_finalize: get_symbol(&libs, b"FLASH_Apply_Q_UT_cntl_finalize\0")
                .map(|sym| *sym),
            FLASH_Apply_Q2_UT_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_Q2_UT_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_CAQ2_UT_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_CAQ2_UT_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_QUD_UT_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_QUD_UT_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_Q_UT_inc_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_Q_UT_inc_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_CAQ_UT_inc_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_CAQ_UT_inc_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_QUD_UT_inc_cntl_finalize: get_symbol(
                &libs,
                b"FLASH_Apply_QUD_UT_inc_cntl_finalize\0",
            )
            .map(|sym| *sym),
            FLASH_Eig_gest_cntl_finalize: get_symbol(&libs, b"FLASH_Eig_gest_cntl_finalize\0")
                .map(|sym| *sym),
            FLA_Cntl_init: get_symbol(&libs, b"FLA_Cntl_init\0").map(|sym| *sym),
            FLA_Cntl_finalize: get_symbol(&libs, b"FLA_Cntl_finalize\0").map(|sym| *sym),
            FLA_Blocksize_create: get_symbol(&libs, b"FLA_Blocksize_create\0").map(|sym| *sym),
            FLA_Blocksize_create_copy: get_symbol(&libs, b"FLA_Blocksize_create_copy\0")
                .map(|sym| *sym),
            FLA_Blocksize_set: get_symbol(&libs, b"FLA_Blocksize_set\0").map(|sym| *sym),
            FLA_Blocksize_scale: get_symbol(&libs, b"FLA_Blocksize_scale\0").map(|sym| *sym),
            FLA_Blocksize_free: get_symbol(&libs, b"FLA_Blocksize_free\0").map(|sym| *sym),
            FLA_Blocksize_extract: get_symbol(&libs, b"FLA_Blocksize_extract\0").map(|sym| *sym),
            FLA_Query_blocksizes: get_symbol(&libs, b"FLA_Query_blocksizes\0").map(|sym| *sym),
            FLA_Query_blocksize: get_symbol(&libs, b"FLA_Query_blocksize\0").map(|sym| *sym),
            FLA_Determine_blocksize: get_symbol(&libs, b"FLA_Determine_blocksize\0")
                .map(|sym| *sym),
            FLA_determine_matrix_size: get_symbol(&libs, b"FLA_determine_matrix_size\0")
                .map(|sym| *sym),
            FLA_Check_error_level: get_symbol(&libs, b"FLA_Check_error_level\0").map(|sym| *sym),
            FLA_Check_error_level_set: get_symbol(&libs, b"FLA_Check_error_level_set\0")
                .map(|sym| *sym),
            FLA_Check_error_code_helper: get_symbol(&libs, b"FLA_Check_error_code_helper\0")
                .map(|sym| *sym),
            FLA_Check_valid_side: get_symbol(&libs, b"FLA_Check_valid_side\0").map(|sym| *sym),
            FLA_Check_valid_uplo: get_symbol(&libs, b"FLA_Check_valid_uplo\0").map(|sym| *sym),
            FLA_Check_valid_trans: get_symbol(&libs, b"FLA_Check_valid_trans\0").map(|sym| *sym),
            FLA_Check_valid_diag: get_symbol(&libs, b"FLA_Check_valid_diag\0").map(|sym| *sym),
            FLA_Check_valid_conj: get_symbol(&libs, b"FLA_Check_valid_conj\0").map(|sym| *sym),
            FLA_Check_valid_direct: get_symbol(&libs, b"FLA_Check_valid_direct\0").map(|sym| *sym),
            FLA_Check_valid_storev: get_symbol(&libs, b"FLA_Check_valid_storev\0").map(|sym| *sym),
            FLA_Check_valid_inverse: get_symbol(&libs, b"FLA_Check_valid_inverse\0")
                .map(|sym| *sym),
            FLA_Check_valid_datatype: get_symbol(&libs, b"FLA_Check_valid_datatype\0")
                .map(|sym| *sym),
            FLA_Check_valid_object_datatype: get_symbol(
                &libs,
                b"FLA_Check_valid_object_datatype\0",
            )
            .map(|sym| *sym),
            FLA_Check_valid_evd_type: get_symbol(&libs, b"FLA_Check_valid_evd_type\0")
                .map(|sym| *sym),
            FLA_Check_valid_svd_type: get_symbol(&libs, b"FLA_Check_valid_svd_type\0")
                .map(|sym| *sym),
            FLA_Check_valid_svd_type_combination: get_symbol(
                &libs,
                b"FLA_Check_valid_svd_type_combination\0",
            )
            .map(|sym| *sym),
            FLA_Check_valid_svd_type_and_trans_combination: get_symbol(
                &libs,
                b"FLA_Check_valid_svd_type_and_trans_combination\0",
            )
            .map(|sym| *sym),
            FLA_Check_floating_datatype: get_symbol(&libs, b"FLA_Check_floating_datatype\0")
                .map(|sym| *sym),
            FLA_Check_int_datatype: get_symbol(&libs, b"FLA_Check_int_datatype\0").map(|sym| *sym),
            FLA_Check_real_datatype: get_symbol(&libs, b"FLA_Check_real_datatype\0")
                .map(|sym| *sym),
            FLA_Check_complex_datatype: get_symbol(&libs, b"FLA_Check_complex_datatype\0")
                .map(|sym| *sym),
            FLA_Check_floating_object: get_symbol(&libs, b"FLA_Check_floating_object\0")
                .map(|sym| *sym),
            FLA_Check_int_object: get_symbol(&libs, b"FLA_Check_int_object\0").map(|sym| *sym),
            FLA_Check_real_object: get_symbol(&libs, b"FLA_Check_real_object\0").map(|sym| *sym),
            FLA_Check_comparable_object: get_symbol(&libs, b"FLA_Check_comparable_object\0")
                .map(|sym| *sym),
            FLA_Check_complex_object: get_symbol(&libs, b"FLA_Check_complex_object\0")
                .map(|sym| *sym),
            FLA_Check_consistent_datatype: get_symbol(&libs, b"FLA_Check_consistent_datatype\0")
                .map(|sym| *sym),
            FLA_Check_consistent_object_datatype: get_symbol(
                &libs,
                b"FLA_Check_consistent_object_datatype\0",
            )
            .map(|sym| *sym),
            FLA_Check_identical_object_precision: get_symbol(
                &libs,
                b"FLA_Check_identical_object_precision\0",
            )
            .map(|sym| *sym),
            FLA_Check_square: get_symbol(&libs, b"FLA_Check_square\0").map(|sym| *sym),
            FLA_Check_if_scalar: get_symbol(&libs, b"FLA_Check_if_scalar\0").map(|sym| *sym),
            FLA_Check_if_vector: get_symbol(&libs, b"FLA_Check_if_vector\0").map(|sym| *sym),
            FLA_Check_conformal_dims: get_symbol(&libs, b"FLA_Check_conformal_dims\0")
                .map(|sym| *sym),
            FLA_Check_matrix_matrix_dims: get_symbol(&libs, b"FLA_Check_matrix_matrix_dims\0")
                .map(|sym| *sym),
            FLA_Check_matrix_vector_dims: get_symbol(&libs, b"FLA_Check_matrix_vector_dims\0")
                .map(|sym| *sym),
            FLA_Check_equal_vector_dims: get_symbol(&libs, b"FLA_Check_equal_vector_dims\0")
                .map(|sym| *sym),
            FLA_Check_conj1_trans_and_datatype: get_symbol(
                &libs,
                b"FLA_Check_conj1_trans_and_datatype\0",
            )
            .map(|sym| *sym),
            FLA_Check_hess_indices: get_symbol(&libs, b"FLA_Check_hess_indices\0").map(|sym| *sym),
            FLA_Check_null_pointer: get_symbol(&libs, b"FLA_Check_null_pointer\0").map(|sym| *sym),
            FLA_Check_object_dims: get_symbol(&libs, b"FLA_Check_object_dims\0").map(|sym| *sym),
            FLA_Check_valid_pivot_type: get_symbol(&libs, b"FLA_Check_valid_pivot_type\0")
                .map(|sym| *sym),
            FLA_Check_malloc_pointer: get_symbol(&libs, b"FLA_Check_malloc_pointer\0")
                .map(|sym| *sym),
            FLA_Check_base_buffer_mismatch: get_symbol(&libs, b"FLA_Check_base_buffer_mismatch\0")
                .map(|sym| *sym),
            FLA_Check_adjacent_objects_2x2: get_symbol(&libs, b"FLA_Check_adjacent_objects_2x2\0")
                .map(|sym| *sym),
            FLA_Check_adjacent_objects_2x1: get_symbol(&libs, b"FLA_Check_adjacent_objects_2x1\0")
                .map(|sym| *sym),
            FLA_Check_adjacent_objects_1x2: get_symbol(&libs, b"FLA_Check_adjacent_objects_1x2\0")
                .map(|sym| *sym),
            FLA_Check_blocksize_value: get_symbol(&libs, b"FLA_Check_blocksize_value\0")
                .map(|sym| *sym),
            FLA_Check_blocksize_object: get_symbol(&libs, b"FLA_Check_blocksize_object\0")
                .map(|sym| *sym),
            FLA_Check_file_descriptor: get_symbol(&libs, b"FLA_Check_file_descriptor\0")
                .map(|sym| *sym),
            FLA_Check_lseek_result: get_symbol(&libs, b"FLA_Check_lseek_result\0").map(|sym| *sym),
            FLA_Check_close_result: get_symbol(&libs, b"FLA_Check_close_result\0").map(|sym| *sym),
            FLA_Check_unlink_result: get_symbol(&libs, b"FLA_Check_unlink_result\0")
                .map(|sym| *sym),
            FLA_Check_read_result: get_symbol(&libs, b"FLA_Check_read_result\0").map(|sym| *sym),
            FLA_Check_write_result: get_symbol(&libs, b"FLA_Check_write_result\0").map(|sym| *sym),
            FLA_Check_valid_quadrant: get_symbol(&libs, b"FLA_Check_valid_quadrant\0")
                .map(|sym| *sym),
            FLA_Check_vector_dim_min: get_symbol(&libs, b"FLA_Check_vector_dim_min\0")
                .map(|sym| *sym),
            FLA_Check_pthread_create_result: get_symbol(
                &libs,
                b"FLA_Check_pthread_create_result\0",
            )
            .map(|sym| *sym),
            FLA_Check_pthread_join_result: get_symbol(&libs, b"FLA_Check_pthread_join_result\0")
                .map(|sym| *sym),
            FLA_Check_valid_isgn_value: get_symbol(&libs, b"FLA_Check_valid_isgn_value\0")
                .map(|sym| *sym),
            FLA_Check_sylv_matrix_dims: get_symbol(&libs, b"FLA_Check_sylv_matrix_dims\0")
                .map(|sym| *sym),
            FLA_Check_chol_failure: get_symbol(&libs, b"FLA_Check_chol_failure\0").map(|sym| *sym),
            FLA_Check_valid_elemtype: get_symbol(&libs, b"FLA_Check_valid_elemtype\0")
                .map(|sym| *sym),
            FLA_Check_posix_memalign_failure: get_symbol(
                &libs,
                b"FLA_Check_posix_memalign_failure\0",
            )
            .map(|sym| *sym),
            FLA_Check_submatrix_dims_and_offset: get_symbol(
                &libs,
                b"FLA_Check_submatrix_dims_and_offset\0",
            )
            .map(|sym| *sym),
            FLA_Check_object_scalar_elemtype: get_symbol(
                &libs,
                b"FLA_Check_object_scalar_elemtype\0",
            )
            .map(|sym| *sym),
            FLA_Check_object_matrix_elemtype: get_symbol(
                &libs,
                b"FLA_Check_object_matrix_elemtype\0",
            )
            .map(|sym| *sym),
            FLA_Check_num_threads: get_symbol(&libs, b"FLA_Check_num_threads\0").map(|sym| *sym),
            FLA_Check_conj_and_datatype: get_symbol(&libs, b"FLA_Check_conj_and_datatype\0")
                .map(|sym| *sym),
            FLA_Check_valid_complex_trans: get_symbol(&libs, b"FLA_Check_valid_complex_trans\0")
                .map(|sym| *sym),
            FLA_Check_valid_real_trans: get_symbol(&libs, b"FLA_Check_valid_real_trans\0")
                .map(|sym| *sym),
            FLA_Check_valid_blas_trans: get_symbol(&libs, b"FLA_Check_valid_blas_trans\0")
                .map(|sym| *sym),
            FLA_Check_nonconstant_datatype: get_symbol(&libs, b"FLA_Check_nonconstant_datatype\0")
                .map(|sym| *sym),
            FLA_Check_nonconstant_object: get_symbol(&libs, b"FLA_Check_nonconstant_object\0")
                .map(|sym| *sym),
            FLA_Check_identical_object_datatype: get_symbol(
                &libs,
                b"FLA_Check_identical_object_datatype\0",
            )
            .map(|sym| *sym),
            FLA_Check_divide_by_zero: get_symbol(&libs, b"FLA_Check_divide_by_zero\0")
                .map(|sym| *sym),
            FLA_Check_identical_object_elemtype: get_symbol(
                &libs,
                b"FLA_Check_identical_object_elemtype\0",
            )
            .map(|sym| *sym),
            FLA_Check_pivot_index_range: get_symbol(&libs, b"FLA_Check_pivot_index_range\0")
                .map(|sym| *sym),
            FLA_Check_householder_panel_dims: get_symbol(
                &libs,
                b"FLA_Check_householder_panel_dims\0",
            )
            .map(|sym| *sym),
            FLA_Check_object_length_equals: get_symbol(&libs, b"FLA_Check_object_length_equals\0")
                .map(|sym| *sym),
            FLA_Check_object_width_equals: get_symbol(&libs, b"FLA_Check_object_width_equals\0")
                .map(|sym| *sym),
            FLA_Check_object_length_min: get_symbol(&libs, b"FLA_Check_object_length_min\0")
                .map(|sym| *sym),
            FLA_Check_object_width_min: get_symbol(&libs, b"FLA_Check_object_width_min\0")
                .map(|sym| *sym),
            FLA_Check_valid_error_level: get_symbol(&libs, b"FLA_Check_valid_error_level\0")
                .map(|sym| *sym),
            FLA_Check_attempted_repart_2x2: get_symbol(&libs, b"FLA_Check_attempted_repart_2x2\0")
                .map(|sym| *sym),
            FLA_Check_attempted_repart_2x1: get_symbol(&libs, b"FLA_Check_attempted_repart_2x1\0")
                .map(|sym| *sym),
            FLA_Check_attempted_repart_1x2: get_symbol(&libs, b"FLA_Check_attempted_repart_1x2\0")
                .map(|sym| *sym),
            FLA_Check_valid_leftright_side: get_symbol(&libs, b"FLA_Check_valid_leftright_side\0")
                .map(|sym| *sym),
            FLA_Check_valid_topbottom_side: get_symbol(&libs, b"FLA_Check_valid_topbottom_side\0")
                .map(|sym| *sym),
            FLA_Check_matrix_strides: get_symbol(&libs, b"FLA_Check_matrix_strides\0")
                .map(|sym| *sym),
            FLA_Check_vector_dim: get_symbol(&libs, b"FLA_Check_vector_dim\0").map(|sym| *sym),
            FLA_Check_row_vector: get_symbol(&libs, b"FLA_Check_row_vector\0").map(|sym| *sym),
            FLA_Check_col_vector: get_symbol(&libs, b"FLA_Check_col_vector\0").map(|sym| *sym),
            FLA_Check_valid_machval: get_symbol(&libs, b"FLA_Check_valid_machval\0")
                .map(|sym| *sym),
            FLA_Check_valid_diag_offset: get_symbol(&libs, b"FLA_Check_valid_diag_offset\0")
                .map(|sym| *sym),
            FLA_Check_col_storage: get_symbol(&libs, b"FLA_Check_col_storage\0").map(|sym| *sym),
            FLA_Check_row_storage: get_symbol(&libs, b"FLA_Check_row_storage\0").map(|sym| *sym),
            FLA_Error_string_for_code: get_symbol(&libs, b"FLA_Error_string_for_code\0")
                .map(|sym| *sym),
            FLA_Error_messages_init: get_symbol(&libs, b"FLA_Error_messages_init\0")
                .map(|sym| *sym),
            FLA_Print_message: get_symbol(&libs, b"FLA_Print_message\0").map(|sym| *sym),
            FLA_Abort: get_symbol(&libs, b"FLA_Abort\0").map(|sym| *sym),
            FLA_Init: get_symbol(&libs, b"FLA_Init\0").map(|sym| *sym),
            FLA_Finalize: get_symbol(&libs, b"FLA_Finalize\0").map(|sym| *sym),
            FLA_Initialized: get_symbol(&libs, b"FLA_Initialized\0").map(|sym| *sym),
            FLA_Init_safe: get_symbol(&libs, b"FLA_Init_safe\0").map(|sym| *sym),
            FLA_Finalize_safe: get_symbol(&libs, b"FLA_Finalize_safe\0").map(|sym| *sym),
            FLA_Init_constants: get_symbol(&libs, b"FLA_Init_constants\0").map(|sym| *sym),
            FLA_Finalize_constants: get_symbol(&libs, b"FLA_Finalize_constants\0").map(|sym| *sym),
            FLA_Init_numerical_constants: get_symbol(&libs, b"FLA_Init_numerical_constants\0")
                .map(|sym| *sym),
            FLA_Finalize_numerical_constants: get_symbol(
                &libs,
                b"FLA_Finalize_numerical_constants\0",
            )
            .map(|sym| *sym),
            FLA_Lock_init: get_symbol(&libs, b"FLA_Lock_init\0").map(|sym| *sym),
            FLA_Lock_destroy: get_symbol(&libs, b"FLA_Lock_destroy\0").map(|sym| *sym),
            FLA_Lock_acquire: get_symbol(&libs, b"FLA_Lock_acquire\0").map(|sym| *sym),
            FLA_Lock_release: get_symbol(&libs, b"FLA_Lock_release\0").map(|sym| *sym),
            FLA_RWLock_init: get_symbol(&libs, b"FLA_RWLock_init\0").map(|sym| *sym),
            FLA_RWLock_destroy: get_symbol(&libs, b"FLA_RWLock_destroy\0").map(|sym| *sym),
            FLA_RWLock_write_acquire: get_symbol(&libs, b"FLA_RWLock_write_acquire\0")
                .map(|sym| *sym),
            FLA_RWLock_read_acquire: get_symbol(&libs, b"FLA_RWLock_read_acquire\0")
                .map(|sym| *sym),
            FLA_RWLock_release: get_symbol(&libs, b"FLA_RWLock_release\0").map(|sym| *sym),
            FLA_Memory_leak_counter_init: get_symbol(&libs, b"FLA_Memory_leak_counter_init\0")
                .map(|sym| *sym),
            FLA_Memory_leak_counter_finalize: get_symbol(
                &libs,
                b"FLA_Memory_leak_counter_finalize\0",
            )
            .map(|sym| *sym),
            FLA_Memory_leak_counter_status: get_symbol(&libs, b"FLA_Memory_leak_counter_status\0")
                .map(|sym| *sym),
            FLA_Memory_leak_counter_set: get_symbol(&libs, b"FLA_Memory_leak_counter_set\0")
                .map(|sym| *sym),
            FLA_malloc: get_symbol(&libs, b"FLA_malloc\0").map(|sym| *sym),
            FLA_realloc: get_symbol(&libs, b"FLA_realloc\0").map(|sym| *sym),
            FLA_buff_malloc: get_symbol(&libs, b"FLA_buff_malloc\0").map(|sym| *sym),
            FLA_free: get_symbol(&libs, b"FLA_free\0").map(|sym| *sym),
            FLA_buff_free: get_symbol(&libs, b"FLA_buff_free\0").map(|sym| *sym),
            FLA_Obj_copy_view: get_symbol(&libs, b"FLA_Obj_copy_view\0").map(|sym| *sym),
            FLA_Obj_extract_real_scalar: get_symbol(&libs, b"FLA_Obj_extract_real_scalar\0")
                .map(|sym| *sym),
            FLA_Obj_extract_complex_scalar: get_symbol(&libs, b"FLA_Obj_extract_complex_scalar\0")
                .map(|sym| *sym),
            FLA_Obj_extract_real_part: get_symbol(&libs, b"FLA_Obj_extract_real_part\0")
                .map(|sym| *sym),
            FLA_Obj_extract_imag_part: get_symbol(&libs, b"FLA_Obj_extract_imag_part\0")
                .map(|sym| *sym),
            FLA_Obj_set_real_part: get_symbol(&libs, b"FLA_Obj_set_real_part\0").map(|sym| *sym),
            FLA_Obj_set_imag_part: get_symbol(&libs, b"FLA_Obj_set_imag_part\0").map(|sym| *sym),
            FLA_Obj_show: get_symbol(&libs, b"FLA_Obj_show\0").map(|sym| *sym),
            FLA_Obj_fshow: get_symbol(&libs, b"FLA_Obj_fshow\0").map(|sym| *sym),
            FLA_Obj_copy_view_check: get_symbol(&libs, b"FLA_Obj_copy_view_check\0")
                .map(|sym| *sym),
            FLA_Obj_extract_real_scalar_check: get_symbol(
                &libs,
                b"FLA_Obj_extract_real_scalar_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_extract_complex_scalar_check: get_symbol(
                &libs,
                b"FLA_Obj_extract_complex_scalar_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_extract_real_part_check: get_symbol(
                &libs,
                b"FLA_Obj_extract_real_part_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_extract_imag_part_check: get_symbol(
                &libs,
                b"FLA_Obj_extract_imag_part_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_set_real_part_check: get_symbol(&libs, b"FLA_Obj_set_real_part_check\0")
                .map(|sym| *sym),
            FLA_Obj_set_imag_part_check: get_symbol(&libs, b"FLA_Obj_set_imag_part_check\0")
                .map(|sym| *sym),
            FLA_Obj_show_check: get_symbol(&libs, b"FLA_Obj_show_check\0").map(|sym| *sym),
            FLA_Obj_fshow_check: get_symbol(&libs, b"FLA_Obj_fshow_check\0").map(|sym| *sym),
            FLA_Copy_buffer_to_object: get_symbol(&libs, b"FLA_Copy_buffer_to_object\0")
                .map(|sym| *sym),
            FLA_Copy_object_to_buffer: get_symbol(&libs, b"FLA_Copy_object_to_buffer\0")
                .map(|sym| *sym),
            FLA_Copy_buffer_to_object_check: get_symbol(
                &libs,
                b"FLA_Copy_buffer_to_object_check\0",
            )
            .map(|sym| *sym),
            FLA_Copy_object_to_buffer_check: get_symbol(
                &libs,
                b"FLA_Copy_object_to_buffer_check\0",
            )
            .map(|sym| *sym),
            FLA_Axpy_buffer_to_object: get_symbol(&libs, b"FLA_Axpy_buffer_to_object\0")
                .map(|sym| *sym),
            FLA_Axpy_object_to_buffer: get_symbol(&libs, b"FLA_Axpy_object_to_buffer\0")
                .map(|sym| *sym),
            FLA_Axpy_buffer_to_object_check: get_symbol(
                &libs,
                b"FLA_Axpy_buffer_to_object_check\0",
            )
            .map(|sym| *sym),
            FLA_Axpy_object_to_buffer_check: get_symbol(
                &libs,
                b"FLA_Axpy_object_to_buffer_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_nullify: get_symbol(&libs, b"FLA_Obj_nullify\0").map(|sym| *sym),
            FLA_Obj_create: get_symbol(&libs, b"FLA_Obj_create\0").map(|sym| *sym),
            FLA_Obj_create_ext: get_symbol(&libs, b"FLA_Obj_create_ext\0").map(|sym| *sym),
            FLA_Obj_create_conf_to: get_symbol(&libs, b"FLA_Obj_create_conf_to\0").map(|sym| *sym),
            FLA_Obj_create_copy_of: get_symbol(&libs, b"FLA_Obj_create_copy_of\0").map(|sym| *sym),
            FLA_Obj_create_without_buffer: get_symbol(&libs, b"FLA_Obj_create_without_buffer\0")
                .map(|sym| *sym),
            FLA_Obj_create_constant: get_symbol(&libs, b"FLA_Obj_create_constant\0")
                .map(|sym| *sym),
            FLA_Obj_create_constant_ext: get_symbol(&libs, b"FLA_Obj_create_constant_ext\0")
                .map(|sym| *sym),
            FLA_Obj_create_complex_constant: get_symbol(
                &libs,
                b"FLA_Obj_create_complex_constant\0",
            )
            .map(|sym| *sym),
            FLA_Obj_attach_buffer: get_symbol(&libs, b"FLA_Obj_attach_buffer\0").map(|sym| *sym),
            FLA_Obj_create_buffer: get_symbol(&libs, b"FLA_Obj_create_buffer\0").map(|sym| *sym),
            FLA_Obj_free: get_symbol(&libs, b"FLA_Obj_free\0").map(|sym| *sym),
            FLA_Obj_free_without_buffer: get_symbol(&libs, b"FLA_Obj_free_without_buffer\0")
                .map(|sym| *sym),
            FLA_Obj_free_buffer: get_symbol(&libs, b"FLA_Obj_free_buffer\0").map(|sym| *sym),
            FLA_align_ldim: get_symbol(&libs, b"FLA_align_ldim\0").map(|sym| *sym),
            FLA_compute_num_elem: get_symbol(&libs, b"FLA_compute_num_elem\0").map(|sym| *sym),
            FLA_adjust_strides: get_symbol(&libs, b"FLA_adjust_strides\0").map(|sym| *sym),
            FLA_Obj_flip_base: get_symbol(&libs, b"FLA_Obj_flip_base\0").map(|sym| *sym),
            FLA_Obj_flip_view: get_symbol(&libs, b"FLA_Obj_flip_view\0").map(|sym| *sym),
            FLA_Obj_create_ext_check: get_symbol(&libs, b"FLA_Obj_create_ext_check\0")
                .map(|sym| *sym),
            FLA_Obj_create_conf_to_check: get_symbol(&libs, b"FLA_Obj_create_conf_to_check\0")
                .map(|sym| *sym),
            FLA_Obj_create_without_buffer_check: get_symbol(
                &libs,
                b"FLA_Obj_create_without_buffer_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_create_constant_check: get_symbol(&libs, b"FLA_Obj_create_constant_check\0")
                .map(|sym| *sym),
            FLA_Obj_create_constant_ext_check: get_symbol(
                &libs,
                b"FLA_Obj_create_constant_ext_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_create_complex_constant_check: get_symbol(
                &libs,
                b"FLA_Obj_create_complex_constant_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_attach_buffer_check: get_symbol(&libs, b"FLA_Obj_attach_buffer_check\0")
                .map(|sym| *sym),
            FLA_Obj_create_buffer_check: get_symbol(&libs, b"FLA_Obj_create_buffer_check\0")
                .map(|sym| *sym),
            FLA_Obj_free_check: get_symbol(&libs, b"FLA_Obj_free_check\0").map(|sym| *sym),
            FLA_Obj_free_without_buffer_check: get_symbol(
                &libs,
                b"FLA_Obj_free_without_buffer_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_free_buffer_check: get_symbol(&libs, b"FLA_Obj_free_buffer_check\0")
                .map(|sym| *sym),
            FLA_Obj_create_buffer_task: get_symbol(&libs, b"FLA_Obj_create_buffer_task\0")
                .map(|sym| *sym),
            FLA_Obj_free_buffer_task: get_symbol(&libs, b"FLA_Obj_free_buffer_task\0")
                .map(|sym| *sym),
            FLA_Obj_datatype: get_symbol(&libs, b"FLA_Obj_datatype\0").map(|sym| *sym),
            FLA_Obj_datatype_proj_to_real: get_symbol(&libs, b"FLA_Obj_datatype_proj_to_real\0")
                .map(|sym| *sym),
            FLA_Obj_datatype_proj_to_complex: get_symbol(
                &libs,
                b"FLA_Obj_datatype_proj_to_complex\0",
            )
            .map(|sym| *sym),
            FLA_Obj_elemtype: get_symbol(&libs, b"FLA_Obj_elemtype\0").map(|sym| *sym),
            FLA_Obj_datatype_size: get_symbol(&libs, b"FLA_Obj_datatype_size\0").map(|sym| *sym),
            FLA_Obj_elem_size: get_symbol(&libs, b"FLA_Obj_elem_size\0").map(|sym| *sym),
            FLA_Obj_length: get_symbol(&libs, b"FLA_Obj_length\0").map(|sym| *sym),
            FLA_Obj_width: get_symbol(&libs, b"FLA_Obj_width\0").map(|sym| *sym),
            FLA_Obj_structure: get_symbol(&libs, b"FLA_Obj_structure\0").map(|sym| *sym),
            FLA_Obj_vector_dim: get_symbol(&libs, b"FLA_Obj_vector_dim\0").map(|sym| *sym),
            FLA_Obj_vector_inc: get_symbol(&libs, b"FLA_Obj_vector_inc\0").map(|sym| *sym),
            FLA_Obj_min_dim: get_symbol(&libs, b"FLA_Obj_min_dim\0").map(|sym| *sym),
            FLA_Obj_max_dim: get_symbol(&libs, b"FLA_Obj_max_dim\0").map(|sym| *sym),
            FLA_Obj_row_stride: get_symbol(&libs, b"FLA_Obj_row_stride\0").map(|sym| *sym),
            FLA_Obj_col_stride: get_symbol(&libs, b"FLA_Obj_col_stride\0").map(|sym| *sym),
            FLA_Obj_row_offset: get_symbol(&libs, b"FLA_Obj_row_offset\0").map(|sym| *sym),
            FLA_Obj_col_offset: get_symbol(&libs, b"FLA_Obj_col_offset\0").map(|sym| *sym),
            FLA_Obj_base_length: get_symbol(&libs, b"FLA_Obj_base_length\0").map(|sym| *sym),
            FLA_Obj_base_width: get_symbol(&libs, b"FLA_Obj_base_width\0").map(|sym| *sym),
            FLA_Obj_num_elem_alloc: get_symbol(&libs, b"FLA_Obj_num_elem_alloc\0").map(|sym| *sym),
            FLA_Obj_base_buffer: get_symbol(&libs, b"FLA_Obj_base_buffer\0").map(|sym| *sym),
            FLA_Obj_buffer_at_view: get_symbol(&libs, b"FLA_Obj_buffer_at_view\0").map(|sym| *sym),
            FLA_Obj_buffer_is_null: get_symbol(&libs, b"FLA_Obj_buffer_is_null\0").map(|sym| *sym),
            FLA_Obj_is_int: get_symbol(&libs, b"FLA_Obj_is_int\0").map(|sym| *sym),
            FLA_Obj_is_floating_point: get_symbol(&libs, b"FLA_Obj_is_floating_point\0")
                .map(|sym| *sym),
            FLA_Obj_is_constant: get_symbol(&libs, b"FLA_Obj_is_constant\0").map(|sym| *sym),
            FLA_Obj_is_real: get_symbol(&libs, b"FLA_Obj_is_real\0").map(|sym| *sym),
            FLA_Obj_is_complex: get_symbol(&libs, b"FLA_Obj_is_complex\0").map(|sym| *sym),
            FLA_Obj_is_single_precision: get_symbol(&libs, b"FLA_Obj_is_single_precision\0")
                .map(|sym| *sym),
            FLA_Obj_is_double_precision: get_symbol(&libs, b"FLA_Obj_is_double_precision\0")
                .map(|sym| *sym),
            FLA_Obj_is_scalar: get_symbol(&libs, b"FLA_Obj_is_scalar\0").map(|sym| *sym),
            FLA_Obj_is_vector: get_symbol(&libs, b"FLA_Obj_is_vector\0").map(|sym| *sym),
            FLA_Obj_has_zero_dim: get_symbol(&libs, b"FLA_Obj_has_zero_dim\0").map(|sym| *sym),
            FLA_Obj_is_row_major: get_symbol(&libs, b"FLA_Obj_is_row_major\0").map(|sym| *sym),
            FLA_Obj_is_col_major: get_symbol(&libs, b"FLA_Obj_is_col_major\0").map(|sym| *sym),
            FLA_Obj_is_conformal_to: get_symbol(&libs, b"FLA_Obj_is_conformal_to\0")
                .map(|sym| *sym),
            FLA_Obj_is: get_symbol(&libs, b"FLA_Obj_is\0").map(|sym| *sym),
            FLA_Obj_is_identical: get_symbol(&libs, b"FLA_Obj_is_identical\0").map(|sym| *sym),
            FLA_Obj_is_overlapped: get_symbol(&libs, b"FLA_Obj_is_overlapped\0").map(|sym| *sym),
            FLA_Obj_equals: get_symbol(&libs, b"FLA_Obj_equals\0").map(|sym| *sym),
            FLA_Obj_gt: get_symbol(&libs, b"FLA_Obj_gt\0").map(|sym| *sym),
            FLA_Obj_ge: get_symbol(&libs, b"FLA_Obj_ge\0").map(|sym| *sym),
            FLA_Obj_lt: get_symbol(&libs, b"FLA_Obj_lt\0").map(|sym| *sym),
            FLA_Obj_le: get_symbol(&libs, b"FLA_Obj_le\0").map(|sym| *sym),
            FLA_Submatrix_at: get_symbol(&libs, b"FLA_Submatrix_at\0").map(|sym| *sym),
            FLA_Obj_has_nan: get_symbol(&libs, b"FLA_Obj_has_nan\0").map(|sym| *sym),
            FLA_Obj_datatype_check: get_symbol(&libs, b"FLA_Obj_datatype_check\0").map(|sym| *sym),
            FLA_Obj_datatype_proj_to_real_check: get_symbol(
                &libs,
                b"FLA_Obj_datatype_proj_to_real_check\0",
            )
            .map(|sym| *sym),
            FLA_Obj_elemtype_check: get_symbol(&libs, b"FLA_Obj_elemtype_check\0").map(|sym| *sym),
            FLA_Obj_datatype_size_check: get_symbol(&libs, b"FLA_Obj_datatype_size_check\0")
                .map(|sym| *sym),
            FLA_Obj_elem_size_check: get_symbol(&libs, b"FLA_Obj_elem_size_check\0")
                .map(|sym| *sym),
            FLA_Obj_buffer_at_view_check: get_symbol(&libs, b"FLA_Obj_buffer_at_view_check\0")
                .map(|sym| *sym),
            FLA_Obj_equals_check: get_symbol(&libs, b"FLA_Obj_equals_check\0").map(|sym| *sym),
            FLA_Obj_gt_check: get_symbol(&libs, b"FLA_Obj_gt_check\0").map(|sym| *sym),
            FLA_Obj_ge_check: get_symbol(&libs, b"FLA_Obj_ge_check\0").map(|sym| *sym),
            FLA_Obj_lt_check: get_symbol(&libs, b"FLA_Obj_lt_check\0").map(|sym| *sym),
            FLA_Obj_le_check: get_symbol(&libs, b"FLA_Obj_le_check\0").map(|sym| *sym),
            FLA_Submatrix_at_check: get_symbol(&libs, b"FLA_Submatrix_at_check\0").map(|sym| *sym),
            FLA_Obj_has_nan_check: get_symbol(&libs, b"FLA_Obj_has_nan_check\0").map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_trans: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_trans\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_uplo: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_uplo\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_side: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_side\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_diag: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_diag\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_direct: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_direct\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_storev: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_storev\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_evd_type: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_evd_type\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_svd_type: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_svd_type\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_netlib_machval: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_netlib_machval\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_blis_trans: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_blis_trans\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_blis_conj: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_blis_conj\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_blis_uplo: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_blis_uplo\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_blis_side: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_blis_side\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_flame_to_blis_diag: get_symbol(
                &libs,
                b"FLA_Param_map_flame_to_blis_diag\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_blis_to_flame_trans: get_symbol(
                &libs,
                b"FLA_Param_map_blis_to_flame_trans\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_blis_to_flame_uplo: get_symbol(
                &libs,
                b"FLA_Param_map_blis_to_flame_uplo\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_blis_to_flame_side: get_symbol(
                &libs,
                b"FLA_Param_map_blis_to_flame_side\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_blis_to_flame_diag: get_symbol(
                &libs,
                b"FLA_Param_map_blis_to_flame_diag\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_trans: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_trans\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_uplo: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_uplo\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_side: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_side\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_diag: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_diag\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_storev: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_storev\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_direct: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_direct\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_char_to_flame_inv: get_symbol(
                &libs,
                b"FLA_Param_map_char_to_flame_inv\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_netlib_to_flame_trans: get_symbol(
                &libs,
                b"FLA_Param_map_netlib_to_flame_trans\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_netlib_to_flame_uplo: get_symbol(
                &libs,
                b"FLA_Param_map_netlib_to_flame_uplo\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_netlib_to_flame_side: get_symbol(
                &libs,
                b"FLA_Param_map_netlib_to_flame_side\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_netlib_to_flame_diag: get_symbol(
                &libs,
                b"FLA_Param_map_netlib_to_flame_diag\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_netlib_to_flame_inv: get_symbol(
                &libs,
                b"FLA_Param_map_netlib_to_flame_inv\0",
            )
            .map(|sym| *sym),
            FLA_Param_map_netlib_to_flame_svd_type: get_symbol(
                &libs,
                b"FLA_Param_map_netlib_to_flame_svd_type\0",
            )
            .map(|sym| *sym),
            FLA_Part_2x2: get_symbol(&libs, b"FLA_Part_2x2\0").map(|sym| *sym),
            FLA_Part_2x1: get_symbol(&libs, b"FLA_Part_2x1\0").map(|sym| *sym),
            FLA_Part_1x2: get_symbol(&libs, b"FLA_Part_1x2\0").map(|sym| *sym),
            FLA_Merge_2x2: get_symbol(&libs, b"FLA_Merge_2x2\0").map(|sym| *sym),
            FLA_Merge_2x1: get_symbol(&libs, b"FLA_Merge_2x1\0").map(|sym| *sym),
            FLA_Merge_1x2: get_symbol(&libs, b"FLA_Merge_1x2\0").map(|sym| *sym),
            FLA_Repart_2x2_to_3x3: get_symbol(&libs, b"FLA_Repart_2x2_to_3x3\0").map(|sym| *sym),
            FLA_Repart_2x1_to_3x1: get_symbol(&libs, b"FLA_Repart_2x1_to_3x1\0").map(|sym| *sym),
            FLA_Repart_1x2_to_1x3: get_symbol(&libs, b"FLA_Repart_1x2_to_1x3\0").map(|sym| *sym),
            FLA_Cont_with_3x3_to_2x2: get_symbol(&libs, b"FLA_Cont_with_3x3_to_2x2\0")
                .map(|sym| *sym),
            FLA_Cont_with_3x1_to_2x1: get_symbol(&libs, b"FLA_Cont_with_3x1_to_2x1\0")
                .map(|sym| *sym),
            FLA_Cont_with_1x3_to_1x2: get_symbol(&libs, b"FLA_Cont_with_1x3_to_1x2\0")
                .map(|sym| *sym),
            FLA_Repart_3x3_to_5x5: get_symbol(&libs, b"FLA_Repart_3x3_to_5x5\0").map(|sym| *sym),
            FLA_Cont_with_5x5_to_3x3: get_symbol(&libs, b"FLA_Cont_with_5x5_to_3x3\0")
                .map(|sym| *sym),
            FLA_Part_2x2_check: get_symbol(&libs, b"FLA_Part_2x2_check\0").map(|sym| *sym),
            FLA_Part_2x1_check: get_symbol(&libs, b"FLA_Part_2x1_check\0").map(|sym| *sym),
            FLA_Part_1x2_check: get_symbol(&libs, b"FLA_Part_1x2_check\0").map(|sym| *sym),
            FLA_Merge_2x2_check: get_symbol(&libs, b"FLA_Merge_2x2_check\0").map(|sym| *sym),
            FLA_Merge_2x1_check: get_symbol(&libs, b"FLA_Merge_2x1_check\0").map(|sym| *sym),
            FLA_Merge_1x2_check: get_symbol(&libs, b"FLA_Merge_1x2_check\0").map(|sym| *sym),
            FLA_Repart_2x2_to_3x3_check: get_symbol(&libs, b"FLA_Repart_2x2_to_3x3_check\0")
                .map(|sym| *sym),
            FLA_Repart_2x1_to_3x1_check: get_symbol(&libs, b"FLA_Repart_2x1_to_3x1_check\0")
                .map(|sym| *sym),
            FLA_Repart_1x2_to_1x3_check: get_symbol(&libs, b"FLA_Repart_1x2_to_1x3_check\0")
                .map(|sym| *sym),
            FLA_Cont_with_3x3_to_2x2_check: get_symbol(&libs, b"FLA_Cont_with_3x3_to_2x2_check\0")
                .map(|sym| *sym),
            FLA_Cont_with_3x1_to_2x1_check: get_symbol(&libs, b"FLA_Cont_with_3x1_to_2x1_check\0")
                .map(|sym| *sym),
            FLA_Cont_with_1x3_to_1x2_check: get_symbol(&libs, b"FLA_Cont_with_1x3_to_1x2_check\0")
                .map(|sym| *sym),
            FLA_random_float: get_symbol(&libs, b"FLA_random_float\0").map(|sym| *sym),
            FLA_random_double: get_symbol(&libs, b"FLA_random_double\0").map(|sym| *sym),
            FLA_random_scomplex: get_symbol(&libs, b"FLA_random_scomplex\0").map(|sym| *sym),
            FLA_random_dcomplex: get_symbol(&libs, b"FLA_random_dcomplex\0").map(|sym| *sym),
            FLA_Absolute_square: get_symbol(&libs, b"FLA_Absolute_square\0").map(|sym| *sym),
            FLA_Absolute_value: get_symbol(&libs, b"FLA_Absolute_value\0").map(|sym| *sym),
            FLA_Clock: get_symbol(&libs, b"FLA_Clock\0").map(|sym| *sym),
            FLA_Conjugate: get_symbol(&libs, b"FLA_Conjugate\0").map(|sym| *sym),
            FLA_Conjugate_r: get_symbol(&libs, b"FLA_Conjugate_r\0").map(|sym| *sym),
            FLA_Fill_with_linear_dist: get_symbol(&libs, b"FLA_Fill_with_linear_dist\0")
                .map(|sym| *sym),
            FLA_Fill_with_inverse_dist: get_symbol(&libs, b"FLA_Fill_with_inverse_dist\0")
                .map(|sym| *sym),
            FLA_Fill_with_geometric_dist: get_symbol(&libs, b"FLA_Fill_with_geometric_dist\0")
                .map(|sym| *sym),
            FLA_Fill_with_random_dist: get_symbol(&libs, b"FLA_Fill_with_random_dist\0")
                .map(|sym| *sym),
            FLA_Fill_with_logarithmic_dist: get_symbol(&libs, b"FLA_Fill_with_logarithmic_dist\0")
                .map(|sym| *sym),
            FLA_Fill_with_cluster_dist: get_symbol(&libs, b"FLA_Fill_with_cluster_dist\0")
                .map(|sym| *sym),
            FLA_Hermitianize: get_symbol(&libs, b"FLA_Hermitianize\0").map(|sym| *sym),
            FLA_Invert: get_symbol(&libs, b"FLA_Invert\0").map(|sym| *sym),
            FLA_Inv_scal_elemwise: get_symbol(&libs, b"FLA_Inv_scal_elemwise\0").map(|sym| *sym),
            FLA_Max_abs_value: get_symbol(&libs, b"FLA_Max_abs_value\0").map(|sym| *sym),
            FLA_Max_abs_value_herm: get_symbol(&libs, b"FLA_Max_abs_value_herm\0").map(|sym| *sym),
            FLA_Max_elemwise_diff: get_symbol(&libs, b"FLA_Max_elemwise_diff\0").map(|sym| *sym),
            FLA_Mult_add: get_symbol(&libs, b"FLA_Mult_add\0").map(|sym| *sym),
            FLA_Negate: get_symbol(&libs, b"FLA_Negate\0").map(|sym| *sym),
            FLA_Norm1: get_symbol(&libs, b"FLA_Norm1\0").map(|sym| *sym),
            FLA_Norm_inf: get_symbol(&libs, b"FLA_Norm_inf\0").map(|sym| *sym),
            FLA_Norm_frob: get_symbol(&libs, b"FLA_Norm_frob\0").map(|sym| *sym),
            FLA_Pow: get_symbol(&libs, b"FLA_Pow\0").map(|sym| *sym),
            FLA_Random_matrix: get_symbol(&libs, b"FLA_Random_matrix\0").map(|sym| *sym),
            FLA_Random_herm_matrix: get_symbol(&libs, b"FLA_Random_herm_matrix\0").map(|sym| *sym),
            FLA_Random_symm_matrix: get_symbol(&libs, b"FLA_Random_symm_matrix\0").map(|sym| *sym),
            FLA_Random_spd_matrix: get_symbol(&libs, b"FLA_Random_spd_matrix\0").map(|sym| *sym),
            FLA_Random_tri_matrix: get_symbol(&libs, b"FLA_Random_tri_matrix\0").map(|sym| *sym),
            FLA_Random_unitary_matrix: get_symbol(&libs, b"FLA_Random_unitary_matrix\0")
                .map(|sym| *sym),
            FLA_Scal_elemwise: get_symbol(&libs, b"FLA_Scal_elemwise\0").map(|sym| *sym),
            FLA_Setr: get_symbol(&libs, b"FLA_Setr\0").map(|sym| *sym),
            FLA_Shift_pivots_to_check: get_symbol(&libs, b"FLA_Shift_pivots_to_check\0")
                .map(|sym| *sym),
            FLA_Sqrt: get_symbol(&libs, b"FLA_Sqrt\0").map(|sym| *sym),
            FLA_Symmetrize: get_symbol(&libs, b"FLA_Symmetrize\0").map(|sym| *sym),
            FLA_Triangularize: get_symbol(&libs, b"FLA_Triangularize\0").map(|sym| *sym),
            FLA_Transpose: get_symbol(&libs, b"FLA_Transpose\0").map(|sym| *sym),
            FLA_Set: get_symbol(&libs, b"FLA_Set\0").map(|sym| *sym),
            FLA_Set_diag: get_symbol(&libs, b"FLA_Set_diag\0").map(|sym| *sym),
            FLA_Set_offdiag: get_symbol(&libs, b"FLA_Set_offdiag\0").map(|sym| *sym),
            FLA_Set_to_identity: get_symbol(&libs, b"FLA_Set_to_identity\0").map(|sym| *sym),
            FLA_Add_to_diag: get_symbol(&libs, b"FLA_Add_to_diag\0").map(|sym| *sym),
            FLA_Shift_diag: get_symbol(&libs, b"FLA_Shift_diag\0").map(|sym| *sym),
            FLA_Scale_diag: get_symbol(&libs, b"FLA_Scale_diag\0").map(|sym| *sym),
            FLA_Set_diagonal_vector: get_symbol(&libs, b"FLA_Set_diagonal_vector\0")
                .map(|sym| *sym),
            FLA_Set_diagonal_matrix: get_symbol(&libs, b"FLA_Set_diagonal_matrix\0")
                .map(|sym| *sym),
            FLA_Absolute_square_check: get_symbol(&libs, b"FLA_Absolute_square_check\0")
                .map(|sym| *sym),
            FLA_Absolute_value_check: get_symbol(&libs, b"FLA_Absolute_value_check\0")
                .map(|sym| *sym),
            FLA_Conjugate_check: get_symbol(&libs, b"FLA_Conjugate_check\0").map(|sym| *sym),
            FLA_Conjugate_r_check: get_symbol(&libs, b"FLA_Conjugate_r_check\0").map(|sym| *sym),
            FLA_Fill_with_linear_dist_check: get_symbol(
                &libs,
                b"FLA_Fill_with_linear_dist_check\0",
            )
            .map(|sym| *sym),
            FLA_Fill_with_inverse_dist_check: get_symbol(
                &libs,
                b"FLA_Fill_with_inverse_dist_check\0",
            )
            .map(|sym| *sym),
            FLA_Fill_with_geometric_dist_check: get_symbol(
                &libs,
                b"FLA_Fill_with_geometric_dist_check\0",
            )
            .map(|sym| *sym),
            FLA_Fill_with_random_dist_check: get_symbol(
                &libs,
                b"FLA_Fill_with_random_dist_check\0",
            )
            .map(|sym| *sym),
            FLA_Fill_with_logarithmic_dist_check: get_symbol(
                &libs,
                b"FLA_Fill_with_logarithmic_dist_check\0",
            )
            .map(|sym| *sym),
            FLA_Fill_with_cluster_dist_check: get_symbol(
                &libs,
                b"FLA_Fill_with_cluster_dist_check\0",
            )
            .map(|sym| *sym),
            FLA_Hermitianize_check: get_symbol(&libs, b"FLA_Hermitianize_check\0").map(|sym| *sym),
            FLA_Invert_check: get_symbol(&libs, b"FLA_Invert_check\0").map(|sym| *sym),
            FLA_Inv_scal_elemwise_check: get_symbol(&libs, b"FLA_Inv_scal_elemwise_check\0")
                .map(|sym| *sym),
            FLA_Max_abs_value_check: get_symbol(&libs, b"FLA_Max_abs_value_check\0")
                .map(|sym| *sym),
            FLA_Max_abs_value_herm_check: get_symbol(&libs, b"FLA_Max_abs_value_herm_check\0")
                .map(|sym| *sym),
            FLA_Max_elemwise_diff_check: get_symbol(&libs, b"FLA_Max_elemwise_diff_check\0")
                .map(|sym| *sym),
            FLA_Mult_add_check: get_symbol(&libs, b"FLA_Mult_add_check\0").map(|sym| *sym),
            FLA_Negate_check: get_symbol(&libs, b"FLA_Negate_check\0").map(|sym| *sym),
            FLA_Norm1_check: get_symbol(&libs, b"FLA_Norm1_check\0").map(|sym| *sym),
            FLA_Norm_inf_check: get_symbol(&libs, b"FLA_Norm_inf_check\0").map(|sym| *sym),
            FLA_Norm_frob_check: get_symbol(&libs, b"FLA_Norm_frob_check\0").map(|sym| *sym),
            FLA_Pow_check: get_symbol(&libs, b"FLA_Pow_check\0").map(|sym| *sym),
            FLA_Random_matrix_check: get_symbol(&libs, b"FLA_Random_matrix_check\0")
                .map(|sym| *sym),
            FLA_Random_herm_matrix_check: get_symbol(&libs, b"FLA_Random_herm_matrix_check\0")
                .map(|sym| *sym),
            FLA_Random_symm_matrix_check: get_symbol(&libs, b"FLA_Random_symm_matrix_check\0")
                .map(|sym| *sym),
            FLA_Random_spd_matrix_check: get_symbol(&libs, b"FLA_Random_spd_matrix_check\0")
                .map(|sym| *sym),
            FLA_Random_tri_matrix_check: get_symbol(&libs, b"FLA_Random_tri_matrix_check\0")
                .map(|sym| *sym),
            FLA_Random_unitary_matrix_check: get_symbol(
                &libs,
                b"FLA_Random_unitary_matrix_check\0",
            )
            .map(|sym| *sym),
            FLA_Scal_elemwise_check: get_symbol(&libs, b"FLA_Scal_elemwise_check\0")
                .map(|sym| *sym),
            FLA_Setr_check: get_symbol(&libs, b"FLA_Setr_check\0").map(|sym| *sym),
            FLA_Sort_check: get_symbol(&libs, b"FLA_Sort_check\0").map(|sym| *sym),
            FLA_Sqrt_check: get_symbol(&libs, b"FLA_Sqrt_check\0").map(|sym| *sym),
            FLA_Symmetrize_check: get_symbol(&libs, b"FLA_Symmetrize_check\0").map(|sym| *sym),
            FLA_Triangularize_check: get_symbol(&libs, b"FLA_Triangularize_check\0")
                .map(|sym| *sym),
            FLA_Transpose_check: get_symbol(&libs, b"FLA_Transpose_check\0").map(|sym| *sym),
            FLA_Set_check: get_symbol(&libs, b"FLA_Set_check\0").map(|sym| *sym),
            FLA_Set_diag_check: get_symbol(&libs, b"FLA_Set_diag_check\0").map(|sym| *sym),
            FLA_Set_to_identity_check: get_symbol(&libs, b"FLA_Set_to_identity_check\0")
                .map(|sym| *sym),
            FLA_Add_to_diag_check: get_symbol(&libs, b"FLA_Add_to_diag_check\0").map(|sym| *sym),
            FLA_Shift_diag_check: get_symbol(&libs, b"FLA_Shift_diag_check\0").map(|sym| *sym),
            FLA_Scale_diag_check: get_symbol(&libs, b"FLA_Scale_diag_check\0").map(|sym| *sym),
            FLA_Transpose_blk_var1: get_symbol(&libs, b"FLA_Transpose_blk_var1\0").map(|sym| *sym),
            FLA_Transpose_blk_var2: get_symbol(&libs, b"FLA_Transpose_blk_var2\0").map(|sym| *sym),
            FLA_Transpose_unb_var1: get_symbol(&libs, b"FLA_Transpose_unb_var1\0").map(|sym| *sym),
            FLA_Transpose_unb_var2: get_symbol(&libs, b"FLA_Transpose_unb_var2\0").map(|sym| *sym),
            FLA_Swap_t_blk_var1: get_symbol(&libs, b"FLA_Swap_t_blk_var1\0").map(|sym| *sym),
            FLA_Swap_t_blk_var2: get_symbol(&libs, b"FLA_Swap_t_blk_var2\0").map(|sym| *sym),
            FLA_Sort: get_symbol(&libs, b"FLA_Sort\0").map(|sym| *sym),
            FLA_Sort_f_ops: get_symbol(&libs, b"FLA_Sort_f_ops\0").map(|sym| *sym),
            FLA_Sort_b_ops: get_symbol(&libs, b"FLA_Sort_b_ops\0").map(|sym| *sym),
            FLA_Sort_f_opd: get_symbol(&libs, b"FLA_Sort_f_opd\0").map(|sym| *sym),
            FLA_Sort_b_opd: get_symbol(&libs, b"FLA_Sort_b_opd\0").map(|sym| *sym),
            FLA_Househ2_UT: get_symbol(&libs, b"FLA_Househ2_UT\0").map(|sym| *sym),
            FLA_Househ2_UT_l_ops: get_symbol(&libs, b"FLA_Househ2_UT_l_ops\0").map(|sym| *sym),
            FLA_Househ2_UT_l_opd: get_symbol(&libs, b"FLA_Househ2_UT_l_opd\0").map(|sym| *sym),
            FLA_Househ2_UT_l_opc: get_symbol(&libs, b"FLA_Househ2_UT_l_opc\0").map(|sym| *sym),
            FLA_Househ2_UT_l_opz: get_symbol(&libs, b"FLA_Househ2_UT_l_opz\0").map(|sym| *sym),
            FLA_Househ2_UT_r_ops: get_symbol(&libs, b"FLA_Househ2_UT_r_ops\0").map(|sym| *sym),
            FLA_Househ2_UT_r_opd: get_symbol(&libs, b"FLA_Househ2_UT_r_opd\0").map(|sym| *sym),
            FLA_Househ2_UT_r_opc: get_symbol(&libs, b"FLA_Househ2_UT_r_opc\0").map(|sym| *sym),
            FLA_Househ2_UT_r_opz: get_symbol(&libs, b"FLA_Househ2_UT_r_opz\0").map(|sym| *sym),
            FLA_Househ3UD_UT: get_symbol(&libs, b"FLA_Househ3UD_UT\0").map(|sym| *sym),
            FLA_Househ3UD_UT_ops: get_symbol(&libs, b"FLA_Househ3UD_UT_ops\0").map(|sym| *sym),
            FLA_Househ3UD_UT_opd: get_symbol(&libs, b"FLA_Househ3UD_UT_opd\0").map(|sym| *sym),
            FLA_Househ3UD_UT_opc: get_symbol(&libs, b"FLA_Househ3UD_UT_opc\0").map(|sym| *sym),
            FLA_Househ3UD_UT_opz: get_symbol(&libs, b"FLA_Househ3UD_UT_opz\0").map(|sym| *sym),
            FLA_Househ2s_UT: get_symbol(&libs, b"FLA_Househ2s_UT\0").map(|sym| *sym),
            FLA_Househ2s_UT_l_ops: get_symbol(&libs, b"FLA_Househ2s_UT_l_ops\0").map(|sym| *sym),
            FLA_Househ2s_UT_l_opd: get_symbol(&libs, b"FLA_Househ2s_UT_l_opd\0").map(|sym| *sym),
            FLA_Househ2s_UT_l_opc: get_symbol(&libs, b"FLA_Househ2s_UT_l_opc\0").map(|sym| *sym),
            FLA_Househ2s_UT_l_opz: get_symbol(&libs, b"FLA_Househ2s_UT_l_opz\0").map(|sym| *sym),
            FLA_Househ2s_UT_r_ops: get_symbol(&libs, b"FLA_Househ2s_UT_r_ops\0").map(|sym| *sym),
            FLA_Househ2s_UT_r_opd: get_symbol(&libs, b"FLA_Househ2s_UT_r_opd\0").map(|sym| *sym),
            FLA_Househ2s_UT_r_opc: get_symbol(&libs, b"FLA_Househ2s_UT_r_opc\0").map(|sym| *sym),
            FLA_Househ2s_UT_r_opz: get_symbol(&libs, b"FLA_Househ2s_UT_r_opz\0").map(|sym| *sym),
            FLA_Hev_2x2: get_symbol(&libs, b"FLA_Hev_2x2\0").map(|sym| *sym),
            FLA_Hev_2x2_ops: get_symbol(&libs, b"FLA_Hev_2x2_ops\0").map(|sym| *sym),
            FLA_Hev_2x2_opd: get_symbol(&libs, b"FLA_Hev_2x2_opd\0").map(|sym| *sym),
            FLA_Hevv_2x2: get_symbol(&libs, b"FLA_Hevv_2x2\0").map(|sym| *sym),
            FLA_Hevv_2x2_ops: get_symbol(&libs, b"FLA_Hevv_2x2_ops\0").map(|sym| *sym),
            FLA_Hevv_2x2_opd: get_symbol(&libs, b"FLA_Hevv_2x2_opd\0").map(|sym| *sym),
            FLA_Hevv_2x2_opc: get_symbol(&libs, b"FLA_Hevv_2x2_opc\0").map(|sym| *sym),
            FLA_Hevv_2x2_opz: get_symbol(&libs, b"FLA_Hevv_2x2_opz\0").map(|sym| *sym),
            FLA_Wilkshift_tridiag: get_symbol(&libs, b"FLA_Wilkshift_tridiag\0").map(|sym| *sym),
            FLA_Wilkshift_tridiag_ops: get_symbol(&libs, b"FLA_Wilkshift_tridiag_ops\0")
                .map(|sym| *sym),
            FLA_Wilkshift_tridiag_opd: get_symbol(&libs, b"FLA_Wilkshift_tridiag_opd\0")
                .map(|sym| *sym),
            FLA_Pythag2: get_symbol(&libs, b"FLA_Pythag2\0").map(|sym| *sym),
            FLA_Pythag2_ops: get_symbol(&libs, b"FLA_Pythag2_ops\0").map(|sym| *sym),
            FLA_Pythag2_opd: get_symbol(&libs, b"FLA_Pythag2_opd\0").map(|sym| *sym),
            FLA_Pythag3: get_symbol(&libs, b"FLA_Pythag3\0").map(|sym| *sym),
            FLA_Pythag3_ops: get_symbol(&libs, b"FLA_Pythag3_ops\0").map(|sym| *sym),
            FLA_Pythag3_opd: get_symbol(&libs, b"FLA_Pythag3_opd\0").map(|sym| *sym),
            FLA_Sort_evd: get_symbol(&libs, b"FLA_Sort_evd\0").map(|sym| *sym),
            FLA_Sort_evd_f_ops: get_symbol(&libs, b"FLA_Sort_evd_f_ops\0").map(|sym| *sym),
            FLA_Sort_evd_b_ops: get_symbol(&libs, b"FLA_Sort_evd_b_ops\0").map(|sym| *sym),
            FLA_Sort_evd_f_opd: get_symbol(&libs, b"FLA_Sort_evd_f_opd\0").map(|sym| *sym),
            FLA_Sort_evd_b_opd: get_symbol(&libs, b"FLA_Sort_evd_b_opd\0").map(|sym| *sym),
            FLA_Sort_evd_f_opc: get_symbol(&libs, b"FLA_Sort_evd_f_opc\0").map(|sym| *sym),
            FLA_Sort_evd_b_opc: get_symbol(&libs, b"FLA_Sort_evd_b_opc\0").map(|sym| *sym),
            FLA_Sort_evd_f_opz: get_symbol(&libs, b"FLA_Sort_evd_f_opz\0").map(|sym| *sym),
            FLA_Sort_evd_b_opz: get_symbol(&libs, b"FLA_Sort_evd_b_opz\0").map(|sym| *sym),
            FLA_Sort_bsvd_ext: get_symbol(&libs, b"FLA_Sort_bsvd_ext\0").map(|sym| *sym),
            FLA_Sort_bsvd_ext_f_ops: get_symbol(&libs, b"FLA_Sort_bsvd_ext_f_ops\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_b_ops: get_symbol(&libs, b"FLA_Sort_bsvd_ext_b_ops\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_f_opd: get_symbol(&libs, b"FLA_Sort_bsvd_ext_f_opd\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_b_opd: get_symbol(&libs, b"FLA_Sort_bsvd_ext_b_opd\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_f_opc: get_symbol(&libs, b"FLA_Sort_bsvd_ext_f_opc\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_b_opc: get_symbol(&libs, b"FLA_Sort_bsvd_ext_b_opc\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_f_opz: get_symbol(&libs, b"FLA_Sort_bsvd_ext_f_opz\0")
                .map(|sym| *sym),
            FLA_Sort_bsvd_ext_b_opz: get_symbol(&libs, b"FLA_Sort_bsvd_ext_b_opz\0")
                .map(|sym| *sym),
            FLA_Sort_svd: get_symbol(&libs, b"FLA_Sort_svd\0").map(|sym| *sym),
            FLA_Sort_svd_f_ops: get_symbol(&libs, b"FLA_Sort_svd_f_ops\0").map(|sym| *sym),
            FLA_Sort_svd_b_ops: get_symbol(&libs, b"FLA_Sort_svd_b_ops\0").map(|sym| *sym),
            FLA_Sort_svd_f_opd: get_symbol(&libs, b"FLA_Sort_svd_f_opd\0").map(|sym| *sym),
            FLA_Sort_svd_b_opd: get_symbol(&libs, b"FLA_Sort_svd_b_opd\0").map(|sym| *sym),
            FLA_Sort_svd_f_opc: get_symbol(&libs, b"FLA_Sort_svd_f_opc\0").map(|sym| *sym),
            FLA_Sort_svd_b_opc: get_symbol(&libs, b"FLA_Sort_svd_b_opc\0").map(|sym| *sym),
            FLA_Sort_svd_f_opz: get_symbol(&libs, b"FLA_Sort_svd_f_opz\0").map(|sym| *sym),
            FLA_Sort_svd_b_opz: get_symbol(&libs, b"FLA_Sort_svd_b_opz\0").map(|sym| *sym),
            FLA_Sv_2x2: get_symbol(&libs, b"FLA_Sv_2x2\0").map(|sym| *sym),
            FLA_Sv_2x2_ops: get_symbol(&libs, b"FLA_Sv_2x2_ops\0").map(|sym| *sym),
            FLA_Sv_2x2_opd: get_symbol(&libs, b"FLA_Sv_2x2_opd\0").map(|sym| *sym),
            FLA_Svv_2x2: get_symbol(&libs, b"FLA_Svv_2x2\0").map(|sym| *sym),
            FLA_Svv_2x2_ops: get_symbol(&libs, b"FLA_Svv_2x2_ops\0").map(|sym| *sym),
            FLA_Svv_2x2_opd: get_symbol(&libs, b"FLA_Svv_2x2_opd\0").map(|sym| *sym),
            FLA_Mach_params: get_symbol(&libs, b"FLA_Mach_params\0").map(|sym| *sym),
            FLA_Mach_params_ops: get_symbol(&libs, b"FLA_Mach_params_ops\0").map(|sym| *sym),
            FLA_Mach_params_opd: get_symbol(&libs, b"FLA_Mach_params_opd\0").map(|sym| *sym),
            FLA_Apply_diag_matrix: get_symbol(&libs, b"FLA_Apply_diag_matrix\0").map(|sym| *sym),
            FLA_Shift_pivots_to: get_symbol(&libs, b"FLA_Shift_pivots_to\0").map(|sym| *sym),
            FLA_Form_perm_matrix: get_symbol(&libs, b"FLA_Form_perm_matrix\0").map(|sym| *sym),
            FLA_LU_find_zero_on_diagonal: get_symbol(&libs, b"FLA_LU_find_zero_on_diagonal\0")
                .map(|sym| *sym),
            fla_dlamch: get_symbol(&libs, b"fla_dlamch\0").map(|sym| *sym),
            fla_slamch: get_symbol(&libs, b"fla_slamch\0").map(|sym| *sym),
            fla_lsame: get_symbol(&libs, b"fla_lsame\0").map(|sym| *sym),
            fla_pow_di: get_symbol(&libs, b"fla_pow_di\0").map(|sym| *sym),
            fla_pow_ri: get_symbol(&libs, b"fla_pow_ri\0").map(|sym| *sym),
            FLA_Househ2_UT_check: get_symbol(&libs, b"FLA_Househ2_UT_check\0").map(|sym| *sym),
            FLA_Househ3UD_UT_check: get_symbol(&libs, b"FLA_Househ3UD_UT_check\0").map(|sym| *sym),
            FLA_Househ2s_UT_check: get_symbol(&libs, b"FLA_Househ2s_UT_check\0").map(|sym| *sym),
            FLA_Givens2_check: get_symbol(&libs, b"FLA_Givens2_check\0").map(|sym| *sym),
            FLA_Apply_GTG_check: get_symbol(&libs, b"FLA_Apply_GTG_check\0").map(|sym| *sym),
            FLA_Apply_G_1x2_check: get_symbol(&libs, b"FLA_Apply_G_1x2_check\0").map(|sym| *sym),
            FLA_Apply_G_mx2_check: get_symbol(&libs, b"FLA_Apply_G_mx2_check\0").map(|sym| *sym),
            FLA_Apply_G_check: get_symbol(&libs, b"FLA_Apply_G_check\0").map(|sym| *sym),
            FLA_Wilkshift_tridiag_check: get_symbol(&libs, b"FLA_Wilkshift_tridiag_check\0")
                .map(|sym| *sym),
            FLA_Wilkshift_bidiag_check: get_symbol(&libs, b"FLA_Wilkshift_bidiag_check\0")
                .map(|sym| *sym),
            FLA_Introduce_bulge_check: get_symbol(&libs, b"FLA_Introduce_bulge_check\0")
                .map(|sym| *sym),
            FLA_Mach_params_check: get_symbol(&libs, b"FLA_Mach_params_check\0").map(|sym| *sym),
            FLA_Sort_evd_check: get_symbol(&libs, b"FLA_Sort_evd_check\0").map(|sym| *sym),
            FLA_Sort_svd_check: get_symbol(&libs, b"FLA_Sort_svd_check\0").map(|sym| *sym),
            FLA_Apply_diag_matrix_check: get_symbol(&libs, b"FLA_Apply_diag_matrix_check\0")
                .map(|sym| *sym),
            FLA_Form_perm_matrix_check: get_symbol(&libs, b"FLA_Form_perm_matrix_check\0")
                .map(|sym| *sym),
            FLA_LU_find_zero_on_diagonal_check: get_symbol(
                &libs,
                b"FLA_LU_find_zero_on_diagonal_check\0",
            )
            .map(|sym| *sym),
            FLA_Asum: get_symbol(&libs, b"FLA_Asum\0").map(|sym| *sym),
            FLA_Axpy: get_symbol(&libs, b"FLA_Axpy\0").map(|sym| *sym),
            FLA_Axpys: get_symbol(&libs, b"FLA_Axpys\0").map(|sym| *sym),
            FLA_Axpyt: get_symbol(&libs, b"FLA_Axpyt\0").map(|sym| *sym),
            FLA_Axpyrt: get_symbol(&libs, b"FLA_Axpyrt\0").map(|sym| *sym),
            FLA_Copy: get_symbol(&libs, b"FLA_Copy\0").map(|sym| *sym),
            FLA_Copyr: get_symbol(&libs, b"FLA_Copyr\0").map(|sym| *sym),
            FLA_Copyrt: get_symbol(&libs, b"FLA_Copyrt\0").map(|sym| *sym),
            FLA_Copyt: get_symbol(&libs, b"FLA_Copyt\0").map(|sym| *sym),
            FLA_Dot: get_symbol(&libs, b"FLA_Dot\0").map(|sym| *sym),
            FLA_Dot2cs: get_symbol(&libs, b"FLA_Dot2cs\0").map(|sym| *sym),
            FLA_Dot2s: get_symbol(&libs, b"FLA_Dot2s\0").map(|sym| *sym),
            FLA_Dotc: get_symbol(&libs, b"FLA_Dotc\0").map(|sym| *sym),
            FLA_Dotcs: get_symbol(&libs, b"FLA_Dotcs\0").map(|sym| *sym),
            FLA_Dots: get_symbol(&libs, b"FLA_Dots\0").map(|sym| *sym),
            FLA_Amax: get_symbol(&libs, b"FLA_Amax\0").map(|sym| *sym),
            FLA_Inv_scal: get_symbol(&libs, b"FLA_Inv_scal\0").map(|sym| *sym),
            FLA_Inv_scalc: get_symbol(&libs, b"FLA_Inv_scalc\0").map(|sym| *sym),
            FLA_Nrm2: get_symbol(&libs, b"FLA_Nrm2\0").map(|sym| *sym),
            FLA_Scal: get_symbol(&libs, b"FLA_Scal\0").map(|sym| *sym),
            FLA_Scalc: get_symbol(&libs, b"FLA_Scalc\0").map(|sym| *sym),
            FLA_Scalr: get_symbol(&libs, b"FLA_Scalr\0").map(|sym| *sym),
            FLA_Swap: get_symbol(&libs, b"FLA_Swap\0").map(|sym| *sym),
            FLA_Swapt: get_symbol(&libs, b"FLA_Swapt\0").map(|sym| *sym),
            FLA_Axpy_task: get_symbol(&libs, b"FLA_Axpy_task\0").map(|sym| *sym),
            FLA_Axpyt_task: get_symbol(&libs, b"FLA_Axpyt_task\0").map(|sym| *sym),
            FLA_Copy_task: get_symbol(&libs, b"FLA_Copy_task\0").map(|sym| *sym),
            FLA_Copyt_task: get_symbol(&libs, b"FLA_Copyt_task\0").map(|sym| *sym),
            FLA_Copyr_task: get_symbol(&libs, b"FLA_Copyr_task\0").map(|sym| *sym),
            FLA_Scal_task: get_symbol(&libs, b"FLA_Scal_task\0").map(|sym| *sym),
            FLA_Scalr_task: get_symbol(&libs, b"FLA_Scalr_task\0").map(|sym| *sym),
            FLA_Axpyt_n_task: get_symbol(&libs, b"FLA_Axpyt_n_task\0").map(|sym| *sym),
            FLA_Axpyt_t_task: get_symbol(&libs, b"FLA_Axpyt_t_task\0").map(|sym| *sym),
            FLA_Axpyt_c_task: get_symbol(&libs, b"FLA_Axpyt_c_task\0").map(|sym| *sym),
            FLA_Axpyt_h_task: get_symbol(&libs, b"FLA_Axpyt_h_task\0").map(|sym| *sym),
            FLA_Copyt_n_task: get_symbol(&libs, b"FLA_Copyt_n_task\0").map(|sym| *sym),
            FLA_Copyt_t_task: get_symbol(&libs, b"FLA_Copyt_t_task\0").map(|sym| *sym),
            FLA_Copyt_c_task: get_symbol(&libs, b"FLA_Copyt_c_task\0").map(|sym| *sym),
            FLA_Copyt_h_task: get_symbol(&libs, b"FLA_Copyt_h_task\0").map(|sym| *sym),
            FLA_Copyr_l_task: get_symbol(&libs, b"FLA_Copyr_l_task\0").map(|sym| *sym),
            FLA_Copyr_u_task: get_symbol(&libs, b"FLA_Copyr_u_task\0").map(|sym| *sym),
            FLA_Scalr_l_task: get_symbol(&libs, b"FLA_Scalr_l_task\0").map(|sym| *sym),
            FLA_Scalr_u_task: get_symbol(&libs, b"FLA_Scalr_u_task\0").map(|sym| *sym),
            FLA_Asum_external: get_symbol(&libs, b"FLA_Asum_external\0").map(|sym| *sym),
            FLA_Axpy_external: get_symbol(&libs, b"FLA_Axpy_external\0").map(|sym| *sym),
            FLA_Axpys_external: get_symbol(&libs, b"FLA_Axpys_external\0").map(|sym| *sym),
            FLA_Axpyt_external: get_symbol(&libs, b"FLA_Axpyt_external\0").map(|sym| *sym),
            FLA_Axpyrt_external: get_symbol(&libs, b"FLA_Axpyrt_external\0").map(|sym| *sym),
            FLA_Copy_external: get_symbol(&libs, b"FLA_Copy_external\0").map(|sym| *sym),
            FLA_Copyr_external: get_symbol(&libs, b"FLA_Copyr_external\0").map(|sym| *sym),
            FLA_Copyrt_external: get_symbol(&libs, b"FLA_Copyrt_external\0").map(|sym| *sym),
            FLA_Copyt_external: get_symbol(&libs, b"FLA_Copyt_external\0").map(|sym| *sym),
            FLA_Dot_external: get_symbol(&libs, b"FLA_Dot_external\0").map(|sym| *sym),
            FLA_Dotc_external: get_symbol(&libs, b"FLA_Dotc_external\0").map(|sym| *sym),
            FLA_Dots_external: get_symbol(&libs, b"FLA_Dots_external\0").map(|sym| *sym),
            FLA_Dotcs_external: get_symbol(&libs, b"FLA_Dotcs_external\0").map(|sym| *sym),
            FLA_Dot2s_external: get_symbol(&libs, b"FLA_Dot2s_external\0").map(|sym| *sym),
            FLA_Dot2cs_external: get_symbol(&libs, b"FLA_Dot2cs_external\0").map(|sym| *sym),
            FLA_Amax_external: get_symbol(&libs, b"FLA_Amax_external\0").map(|sym| *sym),
            FLA_Inv_scal_external: get_symbol(&libs, b"FLA_Inv_scal_external\0").map(|sym| *sym),
            FLA_Inv_scalc_external: get_symbol(&libs, b"FLA_Inv_scalc_external\0").map(|sym| *sym),
            FLA_Nrm2_external: get_symbol(&libs, b"FLA_Nrm2_external\0").map(|sym| *sym),
            FLA_Scal_external: get_symbol(&libs, b"FLA_Scal_external\0").map(|sym| *sym),
            FLA_Scalc_external: get_symbol(&libs, b"FLA_Scalc_external\0").map(|sym| *sym),
            FLA_Scalr_external: get_symbol(&libs, b"FLA_Scalr_external\0").map(|sym| *sym),
            FLA_Swap_external: get_symbol(&libs, b"FLA_Swap_external\0").map(|sym| *sym),
            FLA_Swapt_external: get_symbol(&libs, b"FLA_Swapt_external\0").map(|sym| *sym),
            FLA_Axpy_external_gpu: get_symbol(&libs, b"FLA_Axpy_external_gpu\0").map(|sym| *sym),
            FLA_Copy_external_gpu: get_symbol(&libs, b"FLA_Copy_external_gpu\0").map(|sym| *sym),
            FLA_Scal_external_gpu: get_symbol(&libs, b"FLA_Scal_external_gpu\0").map(|sym| *sym),
            FLA_Scalr_external_gpu: get_symbol(&libs, b"FLA_Scalr_external_gpu\0").map(|sym| *sym),
            FLA_Asum_check: get_symbol(&libs, b"FLA_Asum_check\0").map(|sym| *sym),
            FLA_Axpy_check: get_symbol(&libs, b"FLA_Axpy_check\0").map(|sym| *sym),
            FLA_Axpys_check: get_symbol(&libs, b"FLA_Axpys_check\0").map(|sym| *sym),
            FLA_Axpyt_check: get_symbol(&libs, b"FLA_Axpyt_check\0").map(|sym| *sym),
            FLA_Axpyrt_check: get_symbol(&libs, b"FLA_Axpyrt_check\0").map(|sym| *sym),
            FLA_Copy_check: get_symbol(&libs, b"FLA_Copy_check\0").map(|sym| *sym),
            FLA_Copyr_check: get_symbol(&libs, b"FLA_Copyr_check\0").map(|sym| *sym),
            FLA_Copyrt_check: get_symbol(&libs, b"FLA_Copyrt_check\0").map(|sym| *sym),
            FLA_Copyt_check: get_symbol(&libs, b"FLA_Copyt_check\0").map(|sym| *sym),
            FLA_Dot_check: get_symbol(&libs, b"FLA_Dot_check\0").map(|sym| *sym),
            FLA_Dotc_check: get_symbol(&libs, b"FLA_Dotc_check\0").map(|sym| *sym),
            FLA_Dots_check: get_symbol(&libs, b"FLA_Dots_check\0").map(|sym| *sym),
            FLA_Dotcs_check: get_symbol(&libs, b"FLA_Dotcs_check\0").map(|sym| *sym),
            FLA_Dot2s_check: get_symbol(&libs, b"FLA_Dot2s_check\0").map(|sym| *sym),
            FLA_Dot2cs_check: get_symbol(&libs, b"FLA_Dot2cs_check\0").map(|sym| *sym),
            FLA_Amax_check: get_symbol(&libs, b"FLA_Amax_check\0").map(|sym| *sym),
            FLA_Inv_scal_check: get_symbol(&libs, b"FLA_Inv_scal_check\0").map(|sym| *sym),
            FLA_Inv_scalc_check: get_symbol(&libs, b"FLA_Inv_scalc_check\0").map(|sym| *sym),
            FLA_Nrm2_check: get_symbol(&libs, b"FLA_Nrm2_check\0").map(|sym| *sym),
            FLA_Scal_check: get_symbol(&libs, b"FLA_Scal_check\0").map(|sym| *sym),
            FLA_Scalc_check: get_symbol(&libs, b"FLA_Scalc_check\0").map(|sym| *sym),
            FLA_Scalr_check: get_symbol(&libs, b"FLA_Scalr_check\0").map(|sym| *sym),
            FLA_Swap_check: get_symbol(&libs, b"FLA_Swap_check\0").map(|sym| *sym),
            FLA_Swapt_check: get_symbol(&libs, b"FLA_Swapt_check\0").map(|sym| *sym),
            FLA_Axpy_internal_check: get_symbol(&libs, b"FLA_Axpy_internal_check\0")
                .map(|sym| *sym),
            FLA_Axpyt_internal_check: get_symbol(&libs, b"FLA_Axpyt_internal_check\0")
                .map(|sym| *sym),
            FLA_Copy_internal_check: get_symbol(&libs, b"FLA_Copy_internal_check\0")
                .map(|sym| *sym),
            FLA_Copyt_internal_check: get_symbol(&libs, b"FLA_Copyt_internal_check\0")
                .map(|sym| *sym),
            FLA_Copyr_internal_check: get_symbol(&libs, b"FLA_Copyr_internal_check\0")
                .map(|sym| *sym),
            FLA_Scal_internal_check: get_symbol(&libs, b"FLA_Scal_internal_check\0")
                .map(|sym| *sym),
            FLA_Scalr_internal_check: get_symbol(&libs, b"FLA_Scalr_internal_check\0")
                .map(|sym| *sym),
            FLA_Gemv: get_symbol(&libs, b"FLA_Gemv\0").map(|sym| *sym),
            FLA_Gemvc: get_symbol(&libs, b"FLA_Gemvc\0").map(|sym| *sym),
            FLA_Ger: get_symbol(&libs, b"FLA_Ger\0").map(|sym| *sym),
            FLA_Gerc: get_symbol(&libs, b"FLA_Gerc\0").map(|sym| *sym),
            FLA_Hemv: get_symbol(&libs, b"FLA_Hemv\0").map(|sym| *sym),
            FLA_Hemvc: get_symbol(&libs, b"FLA_Hemvc\0").map(|sym| *sym),
            FLA_Her: get_symbol(&libs, b"FLA_Her\0").map(|sym| *sym),
            FLA_Herc: get_symbol(&libs, b"FLA_Herc\0").map(|sym| *sym),
            FLA_Her2: get_symbol(&libs, b"FLA_Her2\0").map(|sym| *sym),
            FLA_Her2c: get_symbol(&libs, b"FLA_Her2c\0").map(|sym| *sym),
            FLA_Symv: get_symbol(&libs, b"FLA_Symv\0").map(|sym| *sym),
            FLA_Syr: get_symbol(&libs, b"FLA_Syr\0").map(|sym| *sym),
            FLA_Syr2: get_symbol(&libs, b"FLA_Syr2\0").map(|sym| *sym),
            FLA_Trmv: get_symbol(&libs, b"FLA_Trmv\0").map(|sym| *sym),
            FLA_Trmvsx: get_symbol(&libs, b"FLA_Trmvsx\0").map(|sym| *sym),
            FLA_Trsv: get_symbol(&libs, b"FLA_Trsv\0").map(|sym| *sym),
            FLA_Trsvsx: get_symbol(&libs, b"FLA_Trsvsx\0").map(|sym| *sym),
            FLA_Gemv_task: get_symbol(&libs, b"FLA_Gemv_task\0").map(|sym| *sym),
            FLA_Trsv_task: get_symbol(&libs, b"FLA_Trsv_task\0").map(|sym| *sym),
            FLA_Gemv_h_task: get_symbol(&libs, b"FLA_Gemv_h_task\0").map(|sym| *sym),
            FLA_Gemv_n_task: get_symbol(&libs, b"FLA_Gemv_n_task\0").map(|sym| *sym),
            FLA_Gemv_t_task: get_symbol(&libs, b"FLA_Gemv_t_task\0").map(|sym| *sym),
            FLA_Trsv_lc_task: get_symbol(&libs, b"FLA_Trsv_lc_task\0").map(|sym| *sym),
            FLA_Trsv_ln_task: get_symbol(&libs, b"FLA_Trsv_ln_task\0").map(|sym| *sym),
            FLA_Trsv_lt_task: get_symbol(&libs, b"FLA_Trsv_lt_task\0").map(|sym| *sym),
            FLA_Trsv_uc_task: get_symbol(&libs, b"FLA_Trsv_uc_task\0").map(|sym| *sym),
            FLA_Trsv_un_task: get_symbol(&libs, b"FLA_Trsv_un_task\0").map(|sym| *sym),
            FLA_Trsv_ut_task: get_symbol(&libs, b"FLA_Trsv_ut_task\0").map(|sym| *sym),
            FLA_Gemv_external: get_symbol(&libs, b"FLA_Gemv_external\0").map(|sym| *sym),
            FLA_Gemvc_external: get_symbol(&libs, b"FLA_Gemvc_external\0").map(|sym| *sym),
            FLA_Ger_external: get_symbol(&libs, b"FLA_Ger_external\0").map(|sym| *sym),
            FLA_Gerc_external: get_symbol(&libs, b"FLA_Gerc_external\0").map(|sym| *sym),
            FLA_Hemv_external: get_symbol(&libs, b"FLA_Hemv_external\0").map(|sym| *sym),
            FLA_Hemvc_external: get_symbol(&libs, b"FLA_Hemvc_external\0").map(|sym| *sym),
            FLA_Her_external: get_symbol(&libs, b"FLA_Her_external\0").map(|sym| *sym),
            FLA_Herc_external: get_symbol(&libs, b"FLA_Herc_external\0").map(|sym| *sym),
            FLA_Her2_external: get_symbol(&libs, b"FLA_Her2_external\0").map(|sym| *sym),
            FLA_Her2c_external: get_symbol(&libs, b"FLA_Her2c_external\0").map(|sym| *sym),
            FLA_Symv_external: get_symbol(&libs, b"FLA_Symv_external\0").map(|sym| *sym),
            FLA_Syr_external: get_symbol(&libs, b"FLA_Syr_external\0").map(|sym| *sym),
            FLA_Syr2_external: get_symbol(&libs, b"FLA_Syr2_external\0").map(|sym| *sym),
            FLA_Trmv_external: get_symbol(&libs, b"FLA_Trmv_external\0").map(|sym| *sym),
            FLA_Trmvsx_external: get_symbol(&libs, b"FLA_Trmvsx_external\0").map(|sym| *sym),
            FLA_Trsv_external: get_symbol(&libs, b"FLA_Trsv_external\0").map(|sym| *sym),
            FLA_Trsvsx_external: get_symbol(&libs, b"FLA_Trsvsx_external\0").map(|sym| *sym),
            FLA_Gemv_external_gpu: get_symbol(&libs, b"FLA_Gemv_external_gpu\0").map(|sym| *sym),
            FLA_Trsv_external_gpu: get_symbol(&libs, b"FLA_Trsv_external_gpu\0").map(|sym| *sym),
            FLA_Gemv_check: get_symbol(&libs, b"FLA_Gemv_check\0").map(|sym| *sym),
            FLA_Gemvc_check: get_symbol(&libs, b"FLA_Gemvc_check\0").map(|sym| *sym),
            FLA_Ger_check: get_symbol(&libs, b"FLA_Ger_check\0").map(|sym| *sym),
            FLA_Gerc_check: get_symbol(&libs, b"FLA_Gerc_check\0").map(|sym| *sym),
            FLA_Hemv_check: get_symbol(&libs, b"FLA_Hemv_check\0").map(|sym| *sym),
            FLA_Hemvc_check: get_symbol(&libs, b"FLA_Hemvc_check\0").map(|sym| *sym),
            FLA_Her_check: get_symbol(&libs, b"FLA_Her_check\0").map(|sym| *sym),
            FLA_Herc_check: get_symbol(&libs, b"FLA_Herc_check\0").map(|sym| *sym),
            FLA_Her2_check: get_symbol(&libs, b"FLA_Her2_check\0").map(|sym| *sym),
            FLA_Her2c_check: get_symbol(&libs, b"FLA_Her2c_check\0").map(|sym| *sym),
            FLA_Symv_check: get_symbol(&libs, b"FLA_Symv_check\0").map(|sym| *sym),
            FLA_Syr_check: get_symbol(&libs, b"FLA_Syr_check\0").map(|sym| *sym),
            FLA_Syr2_check: get_symbol(&libs, b"FLA_Syr2_check\0").map(|sym| *sym),
            FLA_Trmv_check: get_symbol(&libs, b"FLA_Trmv_check\0").map(|sym| *sym),
            FLA_Trmvsx_check: get_symbol(&libs, b"FLA_Trmvsx_check\0").map(|sym| *sym),
            FLA_Trsv_check: get_symbol(&libs, b"FLA_Trsv_check\0").map(|sym| *sym),
            FLA_Trsvsx_check: get_symbol(&libs, b"FLA_Trsvsx_check\0").map(|sym| *sym),
            FLA_Gemv_internal_check: get_symbol(&libs, b"FLA_Gemv_internal_check\0")
                .map(|sym| *sym),
            FLA_Trsv_internal_check: get_symbol(&libs, b"FLA_Trsv_internal_check\0")
                .map(|sym| *sym),
            FLA_Gemm: get_symbol(&libs, b"FLA_Gemm\0").map(|sym| *sym),
            FLA_Hemm: get_symbol(&libs, b"FLA_Hemm\0").map(|sym| *sym),
            FLA_Herk: get_symbol(&libs, b"FLA_Herk\0").map(|sym| *sym),
            FLA_Her2k: get_symbol(&libs, b"FLA_Her2k\0").map(|sym| *sym),
            FLA_Symm: get_symbol(&libs, b"FLA_Symm\0").map(|sym| *sym),
            FLA_Syrk: get_symbol(&libs, b"FLA_Syrk\0").map(|sym| *sym),
            FLA_Syr2k: get_symbol(&libs, b"FLA_Syr2k\0").map(|sym| *sym),
            FLA_Trmm: get_symbol(&libs, b"FLA_Trmm\0").map(|sym| *sym),
            FLA_Trmmsx: get_symbol(&libs, b"FLA_Trmmsx\0").map(|sym| *sym),
            FLA_Trsm: get_symbol(&libs, b"FLA_Trsm\0").map(|sym| *sym),
            FLA_Trsmsx: get_symbol(&libs, b"FLA_Trsmsx\0").map(|sym| *sym),
            FLA_Gemp: get_symbol(&libs, b"FLA_Gemp\0").map(|sym| *sym),
            FLA_Gepm: get_symbol(&libs, b"FLA_Gepm\0").map(|sym| *sym),
            FLA_Gepp: get_symbol(&libs, b"FLA_Gepp\0").map(|sym| *sym),
            FLA_Gemm_task: get_symbol(&libs, b"FLA_Gemm_task\0").map(|sym| *sym),
            FLA_Hemm_task: get_symbol(&libs, b"FLA_Hemm_task\0").map(|sym| *sym),
            FLA_Herk_task: get_symbol(&libs, b"FLA_Herk_task\0").map(|sym| *sym),
            FLA_Her2k_task: get_symbol(&libs, b"FLA_Her2k_task\0").map(|sym| *sym),
            FLA_Symm_task: get_symbol(&libs, b"FLA_Symm_task\0").map(|sym| *sym),
            FLA_Syrk_task: get_symbol(&libs, b"FLA_Syrk_task\0").map(|sym| *sym),
            FLA_Syr2k_task: get_symbol(&libs, b"FLA_Syr2k_task\0").map(|sym| *sym),
            FLA_Trmm_task: get_symbol(&libs, b"FLA_Trmm_task\0").map(|sym| *sym),
            FLA_Trsm_task: get_symbol(&libs, b"FLA_Trsm_task\0").map(|sym| *sym),
            FLA_Gemm_cc_task: get_symbol(&libs, b"FLA_Gemm_cc_task\0").map(|sym| *sym),
            FLA_Gemm_ch_task: get_symbol(&libs, b"FLA_Gemm_ch_task\0").map(|sym| *sym),
            FLA_Gemm_cn_task: get_symbol(&libs, b"FLA_Gemm_cn_task\0").map(|sym| *sym),
            FLA_Gemm_ct_task: get_symbol(&libs, b"FLA_Gemm_ct_task\0").map(|sym| *sym),
            FLA_Gemm_hc_task: get_symbol(&libs, b"FLA_Gemm_hc_task\0").map(|sym| *sym),
            FLA_Gemm_hh_task: get_symbol(&libs, b"FLA_Gemm_hh_task\0").map(|sym| *sym),
            FLA_Gemm_hn_task: get_symbol(&libs, b"FLA_Gemm_hn_task\0").map(|sym| *sym),
            FLA_Gemm_ht_task: get_symbol(&libs, b"FLA_Gemm_ht_task\0").map(|sym| *sym),
            FLA_Gemm_nc_task: get_symbol(&libs, b"FLA_Gemm_nc_task\0").map(|sym| *sym),
            FLA_Gemm_nh_task: get_symbol(&libs, b"FLA_Gemm_nh_task\0").map(|sym| *sym),
            FLA_Gemm_nn_task: get_symbol(&libs, b"FLA_Gemm_nn_task\0").map(|sym| *sym),
            FLA_Gemm_nt_task: get_symbol(&libs, b"FLA_Gemm_nt_task\0").map(|sym| *sym),
            FLA_Gemm_tc_task: get_symbol(&libs, b"FLA_Gemm_tc_task\0").map(|sym| *sym),
            FLA_Gemm_th_task: get_symbol(&libs, b"FLA_Gemm_th_task\0").map(|sym| *sym),
            FLA_Gemm_tn_task: get_symbol(&libs, b"FLA_Gemm_tn_task\0").map(|sym| *sym),
            FLA_Gemm_tt_task: get_symbol(&libs, b"FLA_Gemm_tt_task\0").map(|sym| *sym),
            FLA_Hemm_ll_task: get_symbol(&libs, b"FLA_Hemm_ll_task\0").map(|sym| *sym),
            FLA_Hemm_lu_task: get_symbol(&libs, b"FLA_Hemm_lu_task\0").map(|sym| *sym),
            FLA_Hemm_rl_task: get_symbol(&libs, b"FLA_Hemm_rl_task\0").map(|sym| *sym),
            FLA_Hemm_ru_task: get_symbol(&libs, b"FLA_Hemm_ru_task\0").map(|sym| *sym),
            FLA_Her2k_ln_task: get_symbol(&libs, b"FLA_Her2k_ln_task\0").map(|sym| *sym),
            FLA_Her2k_lh_task: get_symbol(&libs, b"FLA_Her2k_lh_task\0").map(|sym| *sym),
            FLA_Her2k_un_task: get_symbol(&libs, b"FLA_Her2k_un_task\0").map(|sym| *sym),
            FLA_Her2k_uh_task: get_symbol(&libs, b"FLA_Her2k_uh_task\0").map(|sym| *sym),
            FLA_Herk_ln_task: get_symbol(&libs, b"FLA_Herk_ln_task\0").map(|sym| *sym),
            FLA_Herk_lh_task: get_symbol(&libs, b"FLA_Herk_lh_task\0").map(|sym| *sym),
            FLA_Herk_un_task: get_symbol(&libs, b"FLA_Herk_un_task\0").map(|sym| *sym),
            FLA_Herk_uh_task: get_symbol(&libs, b"FLA_Herk_uh_task\0").map(|sym| *sym),
            FLA_Symm_ll_task: get_symbol(&libs, b"FLA_Symm_ll_task\0").map(|sym| *sym),
            FLA_Symm_lu_task: get_symbol(&libs, b"FLA_Symm_lu_task\0").map(|sym| *sym),
            FLA_Symm_rl_task: get_symbol(&libs, b"FLA_Symm_rl_task\0").map(|sym| *sym),
            FLA_Symm_ru_task: get_symbol(&libs, b"FLA_Symm_ru_task\0").map(|sym| *sym),
            FLA_Syr2k_ln_task: get_symbol(&libs, b"FLA_Syr2k_ln_task\0").map(|sym| *sym),
            FLA_Syr2k_lt_task: get_symbol(&libs, b"FLA_Syr2k_lt_task\0").map(|sym| *sym),
            FLA_Syr2k_un_task: get_symbol(&libs, b"FLA_Syr2k_un_task\0").map(|sym| *sym),
            FLA_Syr2k_ut_task: get_symbol(&libs, b"FLA_Syr2k_ut_task\0").map(|sym| *sym),
            FLA_Syrk_ln_task: get_symbol(&libs, b"FLA_Syrk_ln_task\0").map(|sym| *sym),
            FLA_Syrk_lt_task: get_symbol(&libs, b"FLA_Syrk_lt_task\0").map(|sym| *sym),
            FLA_Syrk_un_task: get_symbol(&libs, b"FLA_Syrk_un_task\0").map(|sym| *sym),
            FLA_Syrk_ut_task: get_symbol(&libs, b"FLA_Syrk_ut_task\0").map(|sym| *sym),
            FLA_Trmm_llc_task: get_symbol(&libs, b"FLA_Trmm_llc_task\0").map(|sym| *sym),
            FLA_Trmm_llh_task: get_symbol(&libs, b"FLA_Trmm_llh_task\0").map(|sym| *sym),
            FLA_Trmm_lln_task: get_symbol(&libs, b"FLA_Trmm_lln_task\0").map(|sym| *sym),
            FLA_Trmm_llt_task: get_symbol(&libs, b"FLA_Trmm_llt_task\0").map(|sym| *sym),
            FLA_Trmm_luc_task: get_symbol(&libs, b"FLA_Trmm_luc_task\0").map(|sym| *sym),
            FLA_Trmm_luh_task: get_symbol(&libs, b"FLA_Trmm_luh_task\0").map(|sym| *sym),
            FLA_Trmm_lun_task: get_symbol(&libs, b"FLA_Trmm_lun_task\0").map(|sym| *sym),
            FLA_Trmm_lut_task: get_symbol(&libs, b"FLA_Trmm_lut_task\0").map(|sym| *sym),
            FLA_Trmm_rlc_task: get_symbol(&libs, b"FLA_Trmm_rlc_task\0").map(|sym| *sym),
            FLA_Trmm_rlh_task: get_symbol(&libs, b"FLA_Trmm_rlh_task\0").map(|sym| *sym),
            FLA_Trmm_rln_task: get_symbol(&libs, b"FLA_Trmm_rln_task\0").map(|sym| *sym),
            FLA_Trmm_rlt_task: get_symbol(&libs, b"FLA_Trmm_rlt_task\0").map(|sym| *sym),
            FLA_Trmm_ruc_task: get_symbol(&libs, b"FLA_Trmm_ruc_task\0").map(|sym| *sym),
            FLA_Trmm_ruh_task: get_symbol(&libs, b"FLA_Trmm_ruh_task\0").map(|sym| *sym),
            FLA_Trmm_run_task: get_symbol(&libs, b"FLA_Trmm_run_task\0").map(|sym| *sym),
            FLA_Trmm_rut_task: get_symbol(&libs, b"FLA_Trmm_rut_task\0").map(|sym| *sym),
            FLA_Trsm_llc_task: get_symbol(&libs, b"FLA_Trsm_llc_task\0").map(|sym| *sym),
            FLA_Trsm_llh_task: get_symbol(&libs, b"FLA_Trsm_llh_task\0").map(|sym| *sym),
            FLA_Trsm_lln_task: get_symbol(&libs, b"FLA_Trsm_lln_task\0").map(|sym| *sym),
            FLA_Trsm_llt_task: get_symbol(&libs, b"FLA_Trsm_llt_task\0").map(|sym| *sym),
            FLA_Trsm_luc_task: get_symbol(&libs, b"FLA_Trsm_luc_task\0").map(|sym| *sym),
            FLA_Trsm_luh_task: get_symbol(&libs, b"FLA_Trsm_luh_task\0").map(|sym| *sym),
            FLA_Trsm_lun_task: get_symbol(&libs, b"FLA_Trsm_lun_task\0").map(|sym| *sym),
            FLA_Trsm_lut_task: get_symbol(&libs, b"FLA_Trsm_lut_task\0").map(|sym| *sym),
            FLA_Trsm_rlc_task: get_symbol(&libs, b"FLA_Trsm_rlc_task\0").map(|sym| *sym),
            FLA_Trsm_rlh_task: get_symbol(&libs, b"FLA_Trsm_rlh_task\0").map(|sym| *sym),
            FLA_Trsm_rln_task: get_symbol(&libs, b"FLA_Trsm_rln_task\0").map(|sym| *sym),
            FLA_Trsm_rlt_task: get_symbol(&libs, b"FLA_Trsm_rlt_task\0").map(|sym| *sym),
            FLA_Trsm_ruc_task: get_symbol(&libs, b"FLA_Trsm_ruc_task\0").map(|sym| *sym),
            FLA_Trsm_ruh_task: get_symbol(&libs, b"FLA_Trsm_ruh_task\0").map(|sym| *sym),
            FLA_Trsm_run_task: get_symbol(&libs, b"FLA_Trsm_run_task\0").map(|sym| *sym),
            FLA_Trsm_rut_task: get_symbol(&libs, b"FLA_Trsm_rut_task\0").map(|sym| *sym),
            FLA_Gemm_external: get_symbol(&libs, b"FLA_Gemm_external\0").map(|sym| *sym),
            FLA_Hemm_external: get_symbol(&libs, b"FLA_Hemm_external\0").map(|sym| *sym),
            FLA_Herk_external: get_symbol(&libs, b"FLA_Herk_external\0").map(|sym| *sym),
            FLA_Her2k_external: get_symbol(&libs, b"FLA_Her2k_external\0").map(|sym| *sym),
            FLA_Symm_external: get_symbol(&libs, b"FLA_Symm_external\0").map(|sym| *sym),
            FLA_Syrk_external: get_symbol(&libs, b"FLA_Syrk_external\0").map(|sym| *sym),
            FLA_Syr2k_external: get_symbol(&libs, b"FLA_Syr2k_external\0").map(|sym| *sym),
            FLA_Trmm_external: get_symbol(&libs, b"FLA_Trmm_external\0").map(|sym| *sym),
            FLA_Trsm_external: get_symbol(&libs, b"FLA_Trsm_external\0").map(|sym| *sym),
            FLA_Trmmsx_external: get_symbol(&libs, b"FLA_Trmmsx_external\0").map(|sym| *sym),
            FLA_Trsmsx_external: get_symbol(&libs, b"FLA_Trsmsx_external\0").map(|sym| *sym),
            FLA_Gemm_external_gpu: get_symbol(&libs, b"FLA_Gemm_external_gpu\0").map(|sym| *sym),
            FLA_Hemm_external_gpu: get_symbol(&libs, b"FLA_Hemm_external_gpu\0").map(|sym| *sym),
            FLA_Herk_external_gpu: get_symbol(&libs, b"FLA_Herk_external_gpu\0").map(|sym| *sym),
            FLA_Her2k_external_gpu: get_symbol(&libs, b"FLA_Her2k_external_gpu\0").map(|sym| *sym),
            FLA_Symm_external_gpu: get_symbol(&libs, b"FLA_Symm_external_gpu\0").map(|sym| *sym),
            FLA_Syrk_external_gpu: get_symbol(&libs, b"FLA_Syrk_external_gpu\0").map(|sym| *sym),
            FLA_Syr2k_external_gpu: get_symbol(&libs, b"FLA_Syr2k_external_gpu\0").map(|sym| *sym),
            FLA_Trmm_external_gpu: get_symbol(&libs, b"FLA_Trmm_external_gpu\0").map(|sym| *sym),
            FLA_Trsm_external_gpu: get_symbol(&libs, b"FLA_Trsm_external_gpu\0").map(|sym| *sym),
            FLA_Gemm_check: get_symbol(&libs, b"FLA_Gemm_check\0").map(|sym| *sym),
            FLA_Hemm_check: get_symbol(&libs, b"FLA_Hemm_check\0").map(|sym| *sym),
            FLA_Her2k_check: get_symbol(&libs, b"FLA_Her2k_check\0").map(|sym| *sym),
            FLA_Herk_check: get_symbol(&libs, b"FLA_Herk_check\0").map(|sym| *sym),
            FLA_Symm_check: get_symbol(&libs, b"FLA_Symm_check\0").map(|sym| *sym),
            FLA_Syr2k_check: get_symbol(&libs, b"FLA_Syr2k_check\0").map(|sym| *sym),
            FLA_Syrk_check: get_symbol(&libs, b"FLA_Syrk_check\0").map(|sym| *sym),
            FLA_Trmm_check: get_symbol(&libs, b"FLA_Trmm_check\0").map(|sym| *sym),
            FLA_Trmmsx_check: get_symbol(&libs, b"FLA_Trmmsx_check\0").map(|sym| *sym),
            FLA_Trsm_check: get_symbol(&libs, b"FLA_Trsm_check\0").map(|sym| *sym),
            FLA_Trsmsx_check: get_symbol(&libs, b"FLA_Trsmsx_check\0").map(|sym| *sym),
            FLA_Gemm_internal_check: get_symbol(&libs, b"FLA_Gemm_internal_check\0")
                .map(|sym| *sym),
            FLA_Hemm_internal_check: get_symbol(&libs, b"FLA_Hemm_internal_check\0")
                .map(|sym| *sym),
            FLA_Herk_internal_check: get_symbol(&libs, b"FLA_Herk_internal_check\0")
                .map(|sym| *sym),
            FLA_Her2k_internal_check: get_symbol(&libs, b"FLA_Her2k_internal_check\0")
                .map(|sym| *sym),
            FLA_Symm_internal_check: get_symbol(&libs, b"FLA_Symm_internal_check\0")
                .map(|sym| *sym),
            FLA_Syrk_internal_check: get_symbol(&libs, b"FLA_Syrk_internal_check\0")
                .map(|sym| *sym),
            FLA_Syr2k_internal_check: get_symbol(&libs, b"FLA_Syr2k_internal_check\0")
                .map(|sym| *sym),
            FLA_Trmm_internal_check: get_symbol(&libs, b"FLA_Trmm_internal_check\0")
                .map(|sym| *sym),
            FLA_Trsm_internal_check: get_symbol(&libs, b"FLA_Trsm_internal_check\0")
                .map(|sym| *sym),
            FLA_Chol: get_symbol(&libs, b"FLA_Chol\0").map(|sym| *sym),
            FLA_LU_nopiv: get_symbol(&libs, b"FLA_LU_nopiv\0").map(|sym| *sym),
            FLA_LU_piv: get_symbol(&libs, b"FLA_LU_piv\0").map(|sym| *sym),
            FLA_QR_UT: get_symbol(&libs, b"FLA_QR_UT\0").map(|sym| *sym),
            FLA_QR_UT_piv: get_symbol(&libs, b"FLA_QR_UT_piv\0").map(|sym| *sym),
            FLA_LQ_UT: get_symbol(&libs, b"FLA_LQ_UT\0").map(|sym| *sym),
            FLA_Trinv: get_symbol(&libs, b"FLA_Trinv\0").map(|sym| *sym),
            FLA_Ttmm: get_symbol(&libs, b"FLA_Ttmm\0").map(|sym| *sym),
            FLA_Sylv: get_symbol(&libs, b"FLA_Sylv\0").map(|sym| *sym),
            FLA_SPDinv: get_symbol(&libs, b"FLA_SPDinv\0").map(|sym| *sym),
            FLA_Hess_UT: get_symbol(&libs, b"FLA_Hess_UT\0").map(|sym| *sym),
            FLA_Eig_gest: get_symbol(&libs, b"FLA_Eig_gest\0").map(|sym| *sym),
            FLA_Accum_T_UT: get_symbol(&libs, b"FLA_Accum_T_UT\0").map(|sym| *sym),
            FLA_Apply_H2_UT: get_symbol(&libs, b"FLA_Apply_H2_UT\0").map(|sym| *sym),
            FLA_Apply_HUD_UT: get_symbol(&libs, b"FLA_Apply_HUD_UT\0").map(|sym| *sym),
            FLA_Apply_Q_UT: get_symbol(&libs, b"FLA_Apply_Q_UT\0").map(|sym| *sym),
            FLA_Apply_pivots: get_symbol(&libs, b"FLA_Apply_pivots\0").map(|sym| *sym),
            FLA_Chol_task: get_symbol(&libs, b"FLA_Chol_task\0").map(|sym| *sym),
            FLA_Chol_l_task: get_symbol(&libs, b"FLA_Chol_l_task\0").map(|sym| *sym),
            FLA_Chol_u_task: get_symbol(&libs, b"FLA_Chol_u_task\0").map(|sym| *sym),
            FLA_LU_piv_macro_task: get_symbol(&libs, b"FLA_LU_piv_macro_task\0").map(|sym| *sym),
            FLA_Apply_pivots_task: get_symbol(&libs, b"FLA_Apply_pivots_task\0").map(|sym| *sym),
            FLA_Apply_pivots_ln_task: get_symbol(&libs, b"FLA_Apply_pivots_ln_task\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_macro_task: get_symbol(&libs, b"FLA_Apply_pivots_macro_task\0")
                .map(|sym| *sym),
            FLA_LU_nopiv_task: get_symbol(&libs, b"FLA_LU_nopiv_task\0").map(|sym| *sym),
            FLA_LU_piv_task: get_symbol(&libs, b"FLA_LU_piv_task\0").map(|sym| *sym),
            FLA_LU_piv_copy_task: get_symbol(&libs, b"FLA_LU_piv_copy_task\0").map(|sym| *sym),
            FLA_Trsm_piv_task: get_symbol(&libs, b"FLA_Trsm_piv_task\0").map(|sym| *sym),
            FLA_SA_LU_task: get_symbol(&libs, b"FLA_SA_LU_task\0").map(|sym| *sym),
            FLA_SA_FS_task: get_symbol(&libs, b"FLA_SA_FS_task\0").map(|sym| *sym),
            FLA_Trinv_task: get_symbol(&libs, b"FLA_Trinv_task\0").map(|sym| *sym),
            FLA_Trinv_ln_task: get_symbol(&libs, b"FLA_Trinv_ln_task\0").map(|sym| *sym),
            FLA_Trinv_lu_task: get_symbol(&libs, b"FLA_Trinv_lu_task\0").map(|sym| *sym),
            FLA_Trinv_un_task: get_symbol(&libs, b"FLA_Trinv_un_task\0").map(|sym| *sym),
            FLA_Trinv_uu_task: get_symbol(&libs, b"FLA_Trinv_uu_task\0").map(|sym| *sym),
            FLA_Ttmm_task: get_symbol(&libs, b"FLA_Ttmm_task\0").map(|sym| *sym),
            FLA_Ttmm_l_task: get_symbol(&libs, b"FLA_Ttmm_l_task\0").map(|sym| *sym),
            FLA_Ttmm_u_task: get_symbol(&libs, b"FLA_Ttmm_u_task\0").map(|sym| *sym),
            FLA_Sylv_task: get_symbol(&libs, b"FLA_Sylv_task\0").map(|sym| *sym),
            FLA_Sylv_nn_task: get_symbol(&libs, b"FLA_Sylv_nn_task\0").map(|sym| *sym),
            FLA_Sylv_nh_task: get_symbol(&libs, b"FLA_Sylv_nh_task\0").map(|sym| *sym),
            FLA_Sylv_hn_task: get_symbol(&libs, b"FLA_Sylv_hn_task\0").map(|sym| *sym),
            FLA_Sylv_hh_task: get_symbol(&libs, b"FLA_Sylv_hh_task\0").map(|sym| *sym),
            FLA_Lyap_task: get_symbol(&libs, b"FLA_Lyap_task\0").map(|sym| *sym),
            FLA_Lyap_n_task: get_symbol(&libs, b"FLA_Lyap_n_task\0").map(|sym| *sym),
            FLA_Lyap_h_task: get_symbol(&libs, b"FLA_Lyap_h_task\0").map(|sym| *sym),
            FLA_Apply_Q_UT_task: get_symbol(&libs, b"FLA_Apply_Q_UT_task\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lhbc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfc_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfc_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfr_task: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfr_task\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_task: get_symbol(&libs, b"FLA_Apply_Q2_UT_task\0").map(|sym| *sym),
            FLA_Apply_Q2_UT_lhfc_task: get_symbol(&libs, b"FLA_Apply_Q2_UT_lhfc_task\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_task: get_symbol(&libs, b"FLA_Apply_CAQ2_UT_task\0").map(|sym| *sym),
            FLA_Apply_CAQ2_UT_lhfc_task: get_symbol(&libs, b"FLA_Apply_CAQ2_UT_lhfc_task\0")
                .map(|sym| *sym),
            FLA_QR2_UT_task: get_symbol(&libs, b"FLA_QR2_UT_task\0").map(|sym| *sym),
            FLA_CAQR2_UT_task: get_symbol(&libs, b"FLA_CAQR2_UT_task\0").map(|sym| *sym),
            FLA_QR_UT_macro_task: get_symbol(&libs, b"FLA_QR_UT_macro_task\0").map(|sym| *sym),
            FLA_QR_UT_task: get_symbol(&libs, b"FLA_QR_UT_task\0").map(|sym| *sym),
            FLA_QR_UT_copy_task: get_symbol(&libs, b"FLA_QR_UT_copy_task\0").map(|sym| *sym),
            FLA_LQ_UT_macro_task: get_symbol(&libs, b"FLA_LQ_UT_macro_task\0").map(|sym| *sym),
            FLA_LQ_UT_task: get_symbol(&libs, b"FLA_LQ_UT_task\0").map(|sym| *sym),
            FLA_UDdate_UT_task: get_symbol(&libs, b"FLA_UDdate_UT_task\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_task: get_symbol(&libs, b"FLA_Apply_QUD_UT_task\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_lhfc_task: get_symbol(&libs, b"FLA_Apply_QUD_UT_lhfc_task\0")
                .map(|sym| *sym),
            FLA_Eig_gest_task: get_symbol(&libs, b"FLA_Eig_gest_task\0").map(|sym| *sym),
            FLA_Eig_gest_il_task: get_symbol(&libs, b"FLA_Eig_gest_il_task\0").map(|sym| *sym),
            FLA_Eig_gest_iu_task: get_symbol(&libs, b"FLA_Eig_gest_iu_task\0").map(|sym| *sym),
            FLA_Eig_gest_nl_task: get_symbol(&libs, b"FLA_Eig_gest_nl_task\0").map(|sym| *sym),
            FLA_Eig_gest_nu_task: get_symbol(&libs, b"FLA_Eig_gest_nu_task\0").map(|sym| *sym),
            FLA_Apply_Q_blk_external: get_symbol(&libs, b"FLA_Apply_Q_blk_external\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_unb_external: get_symbol(&libs, b"FLA_Apply_pivots_unb_external\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_unb_ext: get_symbol(&libs, b"FLA_Apply_pivots_ln_unb_ext\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_macro_external: get_symbol(
                &libs,
                b"FLA_Apply_pivots_macro_external\0",
            )
            .map(|sym| *sym),
            FLA_Chol_blk_external: get_symbol(&libs, b"FLA_Chol_blk_external\0").map(|sym| *sym),
            FLA_Chol_l_blk_ext: get_symbol(&libs, b"FLA_Chol_l_blk_ext\0").map(|sym| *sym),
            FLA_Chol_u_blk_ext: get_symbol(&libs, b"FLA_Chol_u_blk_ext\0").map(|sym| *sym),
            FLA_Chol_unb_external: get_symbol(&libs, b"FLA_Chol_unb_external\0").map(|sym| *sym),
            FLA_Chol_l_unb_ext: get_symbol(&libs, b"FLA_Chol_l_unb_ext\0").map(|sym| *sym),
            FLA_Chol_u_unb_ext: get_symbol(&libs, b"FLA_Chol_u_unb_ext\0").map(|sym| *sym),
            FLA_LU_piv_blk_external: get_symbol(&libs, b"FLA_LU_piv_blk_external\0")
                .map(|sym| *sym),
            FLA_LU_piv_blk_ext: get_symbol(&libs, b"FLA_LU_piv_blk_ext\0").map(|sym| *sym),
            FLA_LU_piv_unb_external: get_symbol(&libs, b"FLA_LU_piv_unb_external\0")
                .map(|sym| *sym),
            FLA_LU_piv_unb_ext: get_symbol(&libs, b"FLA_LU_piv_unb_ext\0").map(|sym| *sym),
            FLA_QR_blk_external: get_symbol(&libs, b"FLA_QR_blk_external\0").map(|sym| *sym),
            FLA_QR_unb_external: get_symbol(&libs, b"FLA_QR_unb_external\0").map(|sym| *sym),
            FLA_LQ_blk_external: get_symbol(&libs, b"FLA_LQ_blk_external\0").map(|sym| *sym),
            FLA_LQ_unb_external: get_symbol(&libs, b"FLA_LQ_unb_external\0").map(|sym| *sym),
            FLA_Hess_blk_external: get_symbol(&libs, b"FLA_Hess_blk_external\0").map(|sym| *sym),
            FLA_Hess_unb_external: get_symbol(&libs, b"FLA_Hess_unb_external\0").map(|sym| *sym),
            FLA_Tridiag_blk_external: get_symbol(&libs, b"FLA_Tridiag_blk_external\0")
                .map(|sym| *sym),
            FLA_Tridiag_unb_external: get_symbol(&libs, b"FLA_Tridiag_unb_external\0")
                .map(|sym| *sym),
            FLA_Bidiag_blk_external: get_symbol(&libs, b"FLA_Bidiag_blk_external\0")
                .map(|sym| *sym),
            FLA_Bidiag_unb_external: get_symbol(&libs, b"FLA_Bidiag_unb_external\0")
                .map(|sym| *sym),
            FLA_QR_form_Q_external: get_symbol(&libs, b"FLA_QR_form_Q_external\0").map(|sym| *sym),
            FLA_Tridiag_form_Q_external: get_symbol(&libs, b"FLA_Tridiag_form_Q_external\0")
                .map(|sym| *sym),
            FLA_Tridiag_apply_Q_external: get_symbol(&libs, b"FLA_Tridiag_apply_Q_external\0")
                .map(|sym| *sym),
            FLA_Bidiag_form_U_external: get_symbol(&libs, b"FLA_Bidiag_form_U_external\0")
                .map(|sym| *sym),
            FLA_Bidiag_form_V_external: get_symbol(&libs, b"FLA_Bidiag_form_V_external\0")
                .map(|sym| *sym),
            FLA_Bidiag_apply_U_external: get_symbol(&libs, b"FLA_Bidiag_apply_U_external\0")
                .map(|sym| *sym),
            FLA_Bidiag_apply_V_external: get_symbol(&libs, b"FLA_Bidiag_apply_V_external\0")
                .map(|sym| *sym),
            FLA_Trinv_blk_external: get_symbol(&libs, b"FLA_Trinv_blk_external\0").map(|sym| *sym),
            FLA_Trinv_ln_blk_ext: get_symbol(&libs, b"FLA_Trinv_ln_blk_ext\0").map(|sym| *sym),
            FLA_Trinv_lu_blk_ext: get_symbol(&libs, b"FLA_Trinv_lu_blk_ext\0").map(|sym| *sym),
            FLA_Trinv_un_blk_ext: get_symbol(&libs, b"FLA_Trinv_un_blk_ext\0").map(|sym| *sym),
            FLA_Trinv_uu_blk_ext: get_symbol(&libs, b"FLA_Trinv_uu_blk_ext\0").map(|sym| *sym),
            FLA_Trinv_unb_external: get_symbol(&libs, b"FLA_Trinv_unb_external\0").map(|sym| *sym),
            FLA_Trinv_ln_unb_ext: get_symbol(&libs, b"FLA_Trinv_ln_unb_ext\0").map(|sym| *sym),
            FLA_Trinv_lu_unb_ext: get_symbol(&libs, b"FLA_Trinv_lu_unb_ext\0").map(|sym| *sym),
            FLA_Trinv_un_unb_ext: get_symbol(&libs, b"FLA_Trinv_un_unb_ext\0").map(|sym| *sym),
            FLA_Trinv_uu_unb_ext: get_symbol(&libs, b"FLA_Trinv_uu_unb_ext\0").map(|sym| *sym),
            FLA_Ttmm_blk_external: get_symbol(&libs, b"FLA_Ttmm_blk_external\0").map(|sym| *sym),
            FLA_Ttmm_l_blk_ext: get_symbol(&libs, b"FLA_Ttmm_l_blk_ext\0").map(|sym| *sym),
            FLA_Ttmm_u_blk_ext: get_symbol(&libs, b"FLA_Ttmm_u_blk_ext\0").map(|sym| *sym),
            FLA_Ttmm_unb_external: get_symbol(&libs, b"FLA_Ttmm_unb_external\0").map(|sym| *sym),
            FLA_Ttmm_l_unb_ext: get_symbol(&libs, b"FLA_Ttmm_l_unb_ext\0").map(|sym| *sym),
            FLA_Ttmm_u_unb_ext: get_symbol(&libs, b"FLA_Ttmm_u_unb_ext\0").map(|sym| *sym),
            FLA_Sylv_blk_external: get_symbol(&libs, b"FLA_Sylv_blk_external\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_ext: get_symbol(&libs, b"FLA_Sylv_nn_blk_ext\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_ext: get_symbol(&libs, b"FLA_Sylv_nh_blk_ext\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_ext: get_symbol(&libs, b"FLA_Sylv_hn_blk_ext\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_ext: get_symbol(&libs, b"FLA_Sylv_hh_blk_ext\0").map(|sym| *sym),
            FLA_Sylv_unb_external: get_symbol(&libs, b"FLA_Sylv_unb_external\0").map(|sym| *sym),
            FLA_Sylv_nn_unb_ext: get_symbol(&libs, b"FLA_Sylv_nn_unb_ext\0").map(|sym| *sym),
            FLA_Sylv_nh_unb_ext: get_symbol(&libs, b"FLA_Sylv_nh_unb_ext\0").map(|sym| *sym),
            FLA_Sylv_hn_unb_ext: get_symbol(&libs, b"FLA_Sylv_hn_unb_ext\0").map(|sym| *sym),
            FLA_Sylv_hh_unb_ext: get_symbol(&libs, b"FLA_Sylv_hh_unb_ext\0").map(|sym| *sym),
            FLA_SPDinv_blk_external: get_symbol(&libs, b"FLA_SPDinv_blk_external\0")
                .map(|sym| *sym),
            FLA_Eig_gest_blk_external: get_symbol(&libs, b"FLA_Eig_gest_blk_external\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_blk_ext: get_symbol(&libs, b"FLA_Eig_gest_il_blk_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_blk_ext: get_symbol(&libs, b"FLA_Eig_gest_iu_blk_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_blk_ext: get_symbol(&libs, b"FLA_Eig_gest_nl_blk_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_blk_ext: get_symbol(&libs, b"FLA_Eig_gest_nu_blk_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_unb_external: get_symbol(&libs, b"FLA_Eig_gest_unb_external\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_unb_ext: get_symbol(&libs, b"FLA_Eig_gest_il_unb_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_unb_ext: get_symbol(&libs, b"FLA_Eig_gest_iu_unb_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_unb_ext: get_symbol(&libs, b"FLA_Eig_gest_nl_unb_ext\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_unb_ext: get_symbol(&libs, b"FLA_Eig_gest_nu_unb_ext\0")
                .map(|sym| *sym),
            FLA_Tevd_external: get_symbol(&libs, b"FLA_Tevd_external\0").map(|sym| *sym),
            FLA_Tevdd_external: get_symbol(&libs, b"FLA_Tevdd_external\0").map(|sym| *sym),
            FLA_Tevdr_external: get_symbol(&libs, b"FLA_Tevdr_external\0").map(|sym| *sym),
            FLA_Hevd_external: get_symbol(&libs, b"FLA_Hevd_external\0").map(|sym| *sym),
            FLA_Hevdd_external: get_symbol(&libs, b"FLA_Hevdd_external\0").map(|sym| *sym),
            FLA_Hevdr_external: get_symbol(&libs, b"FLA_Hevdr_external\0").map(|sym| *sym),
            FLA_Bsvd_external: get_symbol(&libs, b"FLA_Bsvd_external\0").map(|sym| *sym),
            FLA_Bsvdd_external: get_symbol(&libs, b"FLA_Bsvdd_external\0").map(|sym| *sym),
            FLA_Svd_external: get_symbol(&libs, b"FLA_Svd_external\0").map(|sym| *sym),
            FLA_Svdd_external: get_symbol(&libs, b"FLA_Svdd_external\0").map(|sym| *sym),
            FLA_Chol_check: get_symbol(&libs, b"FLA_Chol_check\0").map(|sym| *sym),
            FLA_Chol_solve_check: get_symbol(&libs, b"FLA_Chol_solve_check\0").map(|sym| *sym),
            FLA_LU_nopiv_check: get_symbol(&libs, b"FLA_LU_nopiv_check\0").map(|sym| *sym),
            FLA_LU_nopiv_solve_check: get_symbol(&libs, b"FLA_LU_nopiv_solve_check\0")
                .map(|sym| *sym),
            FLA_LU_piv_check: get_symbol(&libs, b"FLA_LU_piv_check\0").map(|sym| *sym),
            FLA_LU_piv_solve_check: get_symbol(&libs, b"FLA_LU_piv_solve_check\0").map(|sym| *sym),
            FLA_LU_incpiv_check: get_symbol(&libs, b"FLA_LU_incpiv_check\0").map(|sym| *sym),
            FLA_LU_incpiv_solve_check: get_symbol(&libs, b"FLA_LU_incpiv_solve_check\0")
                .map(|sym| *sym),
            FLA_FS_incpiv_check: get_symbol(&libs, b"FLA_FS_incpiv_check\0").map(|sym| *sym),
            FLA_QR_check: get_symbol(&libs, b"FLA_QR_check\0").map(|sym| *sym),
            FLA_QR_UT_check: get_symbol(&libs, b"FLA_QR_UT_check\0").map(|sym| *sym),
            FLA_QR_UT_solve_check: get_symbol(&libs, b"FLA_QR_UT_solve_check\0").map(|sym| *sym),
            FLA_QR_UT_recover_tau_check: get_symbol(&libs, b"FLA_QR_UT_recover_tau_check\0")
                .map(|sym| *sym),
            FLA_QR_UT_form_Q_check: get_symbol(&libs, b"FLA_QR_UT_form_Q_check\0").map(|sym| *sym),
            FLA_LQ_check: get_symbol(&libs, b"FLA_LQ_check\0").map(|sym| *sym),
            FLA_LQ_UT_check: get_symbol(&libs, b"FLA_LQ_UT_check\0").map(|sym| *sym),
            FLA_LQ_UT_solve_check: get_symbol(&libs, b"FLA_LQ_UT_solve_check\0").map(|sym| *sym),
            FLA_LQ_UT_recover_tau_check: get_symbol(&libs, b"FLA_LQ_UT_recover_tau_check\0")
                .map(|sym| *sym),
            FLA_LQ_UT_form_Q_check: get_symbol(&libs, b"FLA_LQ_UT_form_Q_check\0").map(|sym| *sym),
            FLA_Hess_check: get_symbol(&libs, b"FLA_Hess_check\0").map(|sym| *sym),
            FLA_Hess_UT_check: get_symbol(&libs, b"FLA_Hess_UT_check\0").map(|sym| *sym),
            FLA_Hess_UT_recover_tau_check: get_symbol(&libs, b"FLA_Hess_UT_recover_tau_check\0")
                .map(|sym| *sym),
            FLA_Tridiag_check: get_symbol(&libs, b"FLA_Tridiag_check\0").map(|sym| *sym),
            FLA_Tridiag_UT_check: get_symbol(&libs, b"FLA_Tridiag_UT_check\0").map(|sym| *sym),
            FLA_Tridiag_UT_recover_tau_check: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_recover_tau_check\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_scale_diagonals_check: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_scale_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_extract_diagonals_check: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_extract_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_extract_real_diagonals_check: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_extract_real_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_realify_check: get_symbol(&libs, b"FLA_Tridiag_UT_realify_check\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_realify_subdiagonal_check: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_realify_subdiagonal_check\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_check: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_check\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_check: get_symbol(&libs, b"FLA_Tridiag_UT_form_Q_check\0")
                .map(|sym| *sym),
            FLA_Trinv_check: get_symbol(&libs, b"FLA_Trinv_check\0").map(|sym| *sym),
            FLA_Bidiag_check: get_symbol(&libs, b"FLA_Bidiag_check\0").map(|sym| *sym),
            FLA_Bidiag_UT_check: get_symbol(&libs, b"FLA_Bidiag_UT_check\0").map(|sym| *sym),
            FLA_Bidiag_UT_recover_tau_check: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_recover_tau_check\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_extract_diagonals_check: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_extract_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_extract_real_diagonals_check: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_extract_real_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_scale_diagonals_check: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_scale_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_realify_check: get_symbol(&libs, b"FLA_Bidiag_UT_realify_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_realify_diagonals_check: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_realify_diagonals_check\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_form_U_check: get_symbol(&libs, b"FLA_Bidiag_UT_form_U_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_form_V_check: get_symbol(&libs, b"FLA_Bidiag_UT_form_V_check\0")
                .map(|sym| *sym),
            FLA_Ttmm_check: get_symbol(&libs, b"FLA_Ttmm_check\0").map(|sym| *sym),
            FLA_Sylv_check: get_symbol(&libs, b"FLA_Sylv_check\0").map(|sym| *sym),
            FLA_Lyap_check: get_symbol(&libs, b"FLA_Lyap_check\0").map(|sym| *sym),
            FLA_SPDinv_check: get_symbol(&libs, b"FLA_SPDinv_check\0").map(|sym| *sym),
            FLA_Eig_gest_check: get_symbol(&libs, b"FLA_Eig_gest_check\0").map(|sym| *sym),
            FLA_Apply_Q_check: get_symbol(&libs, b"FLA_Apply_Q_check\0").map(|sym| *sym),
            FLA_QR_form_Q_check: get_symbol(&libs, b"FLA_QR_form_Q_check\0").map(|sym| *sym),
            FLA_Tridiag_form_Q_check: get_symbol(&libs, b"FLA_Tridiag_form_Q_check\0")
                .map(|sym| *sym),
            FLA_Tridiag_apply_Q_check: get_symbol(&libs, b"FLA_Tridiag_apply_Q_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_form_U_check: get_symbol(&libs, b"FLA_Bidiag_form_U_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_form_V_check: get_symbol(&libs, b"FLA_Bidiag_form_V_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_apply_U_check: get_symbol(&libs, b"FLA_Bidiag_apply_U_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_apply_V_check: get_symbol(&libs, b"FLA_Bidiag_apply_V_check\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_check: get_symbol(&libs, b"FLA_Apply_Q_UT_check\0").map(|sym| *sym),
            FLA_Apply_Q2_UT_check: get_symbol(&libs, b"FLA_Apply_Q2_UT_check\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_check: get_symbol(&libs, b"FLA_Apply_QUD_UT_check\0").map(|sym| *sym),
            FLA_Apply_pivots_check: get_symbol(&libs, b"FLA_Apply_pivots_check\0").map(|sym| *sym),
            FLA_QR2_UT_check: get_symbol(&libs, b"FLA_QR2_UT_check\0").map(|sym| *sym),
            FLA_CAQR2_UT_check: get_symbol(&libs, b"FLA_CAQR2_UT_check\0").map(|sym| *sym),
            FLA_QR_UT_inc_check: get_symbol(&libs, b"FLA_QR_UT_inc_check\0").map(|sym| *sym),
            FLA_Apply_Q_UT_inc_check: get_symbol(&libs, b"FLA_Apply_Q_UT_inc_check\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ_UT_inc_check: get_symbol(&libs, b"FLA_Apply_CAQ_UT_inc_check\0")
                .map(|sym| *sym),
            FLA_QR_UT_inc_solve_check: get_symbol(&libs, b"FLA_QR_UT_inc_solve_check\0")
                .map(|sym| *sym),
            FLA_CAQR_UT_inc_solve_check: get_symbol(&libs, b"FLA_CAQR_UT_inc_solve_check\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_check: get_symbol(&libs, b"FLA_UDdate_UT_check\0").map(|sym| *sym),
            FLA_UDdate_UT_update_rhs_check: get_symbol(&libs, b"FLA_UDdate_UT_update_rhs_check\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_solve_check: get_symbol(&libs, b"FLA_UDdate_UT_solve_check\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_inc_check: get_symbol(&libs, b"FLA_UDdate_UT_inc_check\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_inc_update_rhs_check: get_symbol(
                &libs,
                b"FLA_UDdate_UT_inc_update_rhs_check\0",
            )
            .map(|sym| *sym),
            FLA_UDdate_UT_inc_solve_check: get_symbol(&libs, b"FLA_UDdate_UT_inc_solve_check\0")
                .map(|sym| *sym),
            FLA_CAQR_UT_inc_check: get_symbol(&libs, b"FLA_CAQR_UT_inc_check\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_inc_check: get_symbol(&libs, b"FLA_Apply_QUD_UT_inc_check\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_check: get_symbol(&libs, b"FLA_Apply_H2_UT_check\0").map(|sym| *sym),
            FLA_Apply_HUD_UT_check: get_symbol(&libs, b"FLA_Apply_HUD_UT_check\0").map(|sym| *sym),
            FLA_Accum_T_UT_check: get_symbol(&libs, b"FLA_Accum_T_UT_check\0").map(|sym| *sym),
            FLA_Tevd_compute_scaling_check: get_symbol(&libs, b"FLA_Tevd_compute_scaling_check\0")
                .map(|sym| *sym),
            FLA_Hevd_compute_scaling_check: get_symbol(&libs, b"FLA_Hevd_compute_scaling_check\0")
                .map(|sym| *sym),
            FLA_Hevd_check: get_symbol(&libs, b"FLA_Hevd_check\0").map(|sym| *sym),
            FLA_Hevdd_check: get_symbol(&libs, b"FLA_Hevdd_check\0").map(|sym| *sym),
            FLA_Hevdr_check: get_symbol(&libs, b"FLA_Hevdr_check\0").map(|sym| *sym),
            FLA_Bsvd_check: get_symbol(&libs, b"FLA_Bsvd_check\0").map(|sym| *sym),
            FLA_Bsvd_ext_check: get_symbol(&libs, b"FLA_Bsvd_ext_check\0").map(|sym| *sym),
            FLA_Bsvd_compute_scaling_check: get_symbol(&libs, b"FLA_Bsvd_compute_scaling_check\0")
                .map(|sym| *sym),
            FLA_Svd_compute_scaling_check: get_symbol(&libs, b"FLA_Svd_compute_scaling_check\0")
                .map(|sym| *sym),
            FLA_Svd_check: get_symbol(&libs, b"FLA_Svd_check\0").map(|sym| *sym),
            FLA_Svd_ext_check: get_symbol(&libs, b"FLA_Svd_ext_check\0").map(|sym| *sym),
            FLA_Svdd_check: get_symbol(&libs, b"FLA_Svdd_check\0").map(|sym| *sym),
            FLA_Chol_internal_check: get_symbol(&libs, b"FLA_Chol_internal_check\0")
                .map(|sym| *sym),
            FLA_LU_nopiv_internal_check: get_symbol(&libs, b"FLA_LU_nopiv_internal_check\0")
                .map(|sym| *sym),
            FLA_Trinv_internal_check: get_symbol(&libs, b"FLA_Trinv_internal_check\0")
                .map(|sym| *sym),
            FLA_Ttmm_internal_check: get_symbol(&libs, b"FLA_Ttmm_internal_check\0")
                .map(|sym| *sym),
            FLA_SPDinv_internal_check: get_symbol(&libs, b"FLA_SPDinv_internal_check\0")
                .map(|sym| *sym),
            FLA_Sylv_internal_check: get_symbol(&libs, b"FLA_Sylv_internal_check\0")
                .map(|sym| *sym),
            FLA_Lyap_internal_check: get_symbol(&libs, b"FLA_Lyap_internal_check\0")
                .map(|sym| *sym),
            FLA_QR_UT_internal_check: get_symbol(&libs, b"FLA_QR_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_QR_UT_copy_internal_check: get_symbol(&libs, b"FLA_QR_UT_copy_internal_check\0")
                .map(|sym| *sym),
            FLA_QR2_UT_internal_check: get_symbol(&libs, b"FLA_QR2_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_CAQR2_UT_internal_check: get_symbol(&libs, b"FLA_CAQR2_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_LQ_UT_internal_check: get_symbol(&libs, b"FLA_LQ_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_Hess_UT_internal_check: get_symbol(&libs, b"FLA_Hess_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_internal_check: get_symbol(&libs, b"FLA_Tridiag_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_internal_check: get_symbol(&libs, b"FLA_Bidiag_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_internal_check: get_symbol(&libs, b"FLA_UDdate_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_internal_check: get_symbol(&libs, b"FLA_Apply_Q_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_internal_check: get_symbol(&libs, b"FLA_Apply_Q2_UT_internal_check\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_internal_check: get_symbol(
                &libs,
                b"FLA_Apply_CAQ2_UT_internal_check\0",
            )
            .map(|sym| *sym),
            FLA_Apply_QUD_UT_internal_check: get_symbol(
                &libs,
                b"FLA_Apply_QUD_UT_internal_check\0",
            )
            .map(|sym| *sym),
            FLA_Apply_Q_UT_inc_internal_check: get_symbol(
                &libs,
                b"FLA_Apply_Q_UT_inc_internal_check\0",
            )
            .map(|sym| *sym),
            FLA_Apply_CAQ_UT_inc_internal_check: get_symbol(
                &libs,
                b"FLA_Apply_CAQ_UT_inc_internal_check\0",
            )
            .map(|sym| *sym),
            FLA_Apply_QUD_UT_inc_internal_check: get_symbol(
                &libs,
                b"FLA_Apply_QUD_UT_inc_internal_check\0",
            )
            .map(|sym| *sym),
            FLA_Eig_gest_internal_check: get_symbol(&libs, b"FLA_Eig_gest_internal_check\0")
                .map(|sym| *sym),
            FLA_Axpy_blk_var1: get_symbol(&libs, b"FLA_Axpy_blk_var1\0").map(|sym| *sym),
            FLA_Axpy_blk_var2: get_symbol(&libs, b"FLA_Axpy_blk_var2\0").map(|sym| *sym),
            FLA_Axpy_blk_var3: get_symbol(&libs, b"FLA_Axpy_blk_var3\0").map(|sym| *sym),
            FLA_Axpy_blk_var4: get_symbol(&libs, b"FLA_Axpy_blk_var4\0").map(|sym| *sym),
            FLA_Axpy_internal: get_symbol(&libs, b"FLA_Axpy_internal\0").map(|sym| *sym),
            FLA_Axpyt_n_blk_var1: get_symbol(&libs, b"FLA_Axpyt_n_blk_var1\0").map(|sym| *sym),
            FLA_Axpyt_n_blk_var2: get_symbol(&libs, b"FLA_Axpyt_n_blk_var2\0").map(|sym| *sym),
            FLA_Axpyt_n_blk_var3: get_symbol(&libs, b"FLA_Axpyt_n_blk_var3\0").map(|sym| *sym),
            FLA_Axpyt_n_blk_var4: get_symbol(&libs, b"FLA_Axpyt_n_blk_var4\0").map(|sym| *sym),
            FLA_Axpyt_t_blk_var1: get_symbol(&libs, b"FLA_Axpyt_t_blk_var1\0").map(|sym| *sym),
            FLA_Axpyt_t_blk_var2: get_symbol(&libs, b"FLA_Axpyt_t_blk_var2\0").map(|sym| *sym),
            FLA_Axpyt_t_blk_var3: get_symbol(&libs, b"FLA_Axpyt_t_blk_var3\0").map(|sym| *sym),
            FLA_Axpyt_t_blk_var4: get_symbol(&libs, b"FLA_Axpyt_t_blk_var4\0").map(|sym| *sym),
            FLA_Axpyt_c_blk_var1: get_symbol(&libs, b"FLA_Axpyt_c_blk_var1\0").map(|sym| *sym),
            FLA_Axpyt_c_blk_var2: get_symbol(&libs, b"FLA_Axpyt_c_blk_var2\0").map(|sym| *sym),
            FLA_Axpyt_c_blk_var3: get_symbol(&libs, b"FLA_Axpyt_c_blk_var3\0").map(|sym| *sym),
            FLA_Axpyt_c_blk_var4: get_symbol(&libs, b"FLA_Axpyt_c_blk_var4\0").map(|sym| *sym),
            FLA_Axpyt_h_blk_var1: get_symbol(&libs, b"FLA_Axpyt_h_blk_var1\0").map(|sym| *sym),
            FLA_Axpyt_h_blk_var2: get_symbol(&libs, b"FLA_Axpyt_h_blk_var2\0").map(|sym| *sym),
            FLA_Axpyt_h_blk_var3: get_symbol(&libs, b"FLA_Axpyt_h_blk_var3\0").map(|sym| *sym),
            FLA_Axpyt_h_blk_var4: get_symbol(&libs, b"FLA_Axpyt_h_blk_var4\0").map(|sym| *sym),
            FLA_Axpyt_internal: get_symbol(&libs, b"FLA_Axpyt_internal\0").map(|sym| *sym),
            FLA_Axpyt_n: get_symbol(&libs, b"FLA_Axpyt_n\0").map(|sym| *sym),
            FLA_Axpyt_t: get_symbol(&libs, b"FLA_Axpyt_t\0").map(|sym| *sym),
            FLA_Axpyt_c: get_symbol(&libs, b"FLA_Axpyt_c\0").map(|sym| *sym),
            FLA_Axpyt_h: get_symbol(&libs, b"FLA_Axpyt_h\0").map(|sym| *sym),
            FLA_Copy_blk_var1: get_symbol(&libs, b"FLA_Copy_blk_var1\0").map(|sym| *sym),
            FLA_Copy_blk_var2: get_symbol(&libs, b"FLA_Copy_blk_var2\0").map(|sym| *sym),
            FLA_Copy_blk_var3: get_symbol(&libs, b"FLA_Copy_blk_var3\0").map(|sym| *sym),
            FLA_Copy_blk_var4: get_symbol(&libs, b"FLA_Copy_blk_var4\0").map(|sym| *sym),
            FLA_Copy_internal: get_symbol(&libs, b"FLA_Copy_internal\0").map(|sym| *sym),
            FLA_Copyt_n_blk_var1: get_symbol(&libs, b"FLA_Copyt_n_blk_var1\0").map(|sym| *sym),
            FLA_Copyt_n_blk_var2: get_symbol(&libs, b"FLA_Copyt_n_blk_var2\0").map(|sym| *sym),
            FLA_Copyt_n_blk_var3: get_symbol(&libs, b"FLA_Copyt_n_blk_var3\0").map(|sym| *sym),
            FLA_Copyt_n_blk_var4: get_symbol(&libs, b"FLA_Copyt_n_blk_var4\0").map(|sym| *sym),
            FLA_Copyt_t_blk_var1: get_symbol(&libs, b"FLA_Copyt_t_blk_var1\0").map(|sym| *sym),
            FLA_Copyt_t_blk_var2: get_symbol(&libs, b"FLA_Copyt_t_blk_var2\0").map(|sym| *sym),
            FLA_Copyt_t_blk_var3: get_symbol(&libs, b"FLA_Copyt_t_blk_var3\0").map(|sym| *sym),
            FLA_Copyt_t_blk_var4: get_symbol(&libs, b"FLA_Copyt_t_blk_var4\0").map(|sym| *sym),
            FLA_Copyt_c_blk_var1: get_symbol(&libs, b"FLA_Copyt_c_blk_var1\0").map(|sym| *sym),
            FLA_Copyt_c_blk_var2: get_symbol(&libs, b"FLA_Copyt_c_blk_var2\0").map(|sym| *sym),
            FLA_Copyt_c_blk_var3: get_symbol(&libs, b"FLA_Copyt_c_blk_var3\0").map(|sym| *sym),
            FLA_Copyt_c_blk_var4: get_symbol(&libs, b"FLA_Copyt_c_blk_var4\0").map(|sym| *sym),
            FLA_Copyt_h_blk_var1: get_symbol(&libs, b"FLA_Copyt_h_blk_var1\0").map(|sym| *sym),
            FLA_Copyt_h_blk_var2: get_symbol(&libs, b"FLA_Copyt_h_blk_var2\0").map(|sym| *sym),
            FLA_Copyt_h_blk_var3: get_symbol(&libs, b"FLA_Copyt_h_blk_var3\0").map(|sym| *sym),
            FLA_Copyt_h_blk_var4: get_symbol(&libs, b"FLA_Copyt_h_blk_var4\0").map(|sym| *sym),
            FLA_Copyt_internal: get_symbol(&libs, b"FLA_Copyt_internal\0").map(|sym| *sym),
            FLA_Copyt_n: get_symbol(&libs, b"FLA_Copyt_n\0").map(|sym| *sym),
            FLA_Copyt_t: get_symbol(&libs, b"FLA_Copyt_t\0").map(|sym| *sym),
            FLA_Copyt_c: get_symbol(&libs, b"FLA_Copyt_c\0").map(|sym| *sym),
            FLA_Copyt_h: get_symbol(&libs, b"FLA_Copyt_h\0").map(|sym| *sym),
            FLA_Copyr_l_blk_var1: get_symbol(&libs, b"FLA_Copyr_l_blk_var1\0").map(|sym| *sym),
            FLA_Copyr_l_blk_var2: get_symbol(&libs, b"FLA_Copyr_l_blk_var2\0").map(|sym| *sym),
            FLA_Copyr_l_blk_var3: get_symbol(&libs, b"FLA_Copyr_l_blk_var3\0").map(|sym| *sym),
            FLA_Copyr_l_blk_var4: get_symbol(&libs, b"FLA_Copyr_l_blk_var4\0").map(|sym| *sym),
            FLA_Copyr_u_blk_var1: get_symbol(&libs, b"FLA_Copyr_u_blk_var1\0").map(|sym| *sym),
            FLA_Copyr_u_blk_var2: get_symbol(&libs, b"FLA_Copyr_u_blk_var2\0").map(|sym| *sym),
            FLA_Copyr_u_blk_var3: get_symbol(&libs, b"FLA_Copyr_u_blk_var3\0").map(|sym| *sym),
            FLA_Copyr_u_blk_var4: get_symbol(&libs, b"FLA_Copyr_u_blk_var4\0").map(|sym| *sym),
            FLASH_Copyr: get_symbol(&libs, b"FLASH_Copyr\0").map(|sym| *sym),
            FLA_Copyr_internal: get_symbol(&libs, b"FLA_Copyr_internal\0").map(|sym| *sym),
            FLA_Copyr_l: get_symbol(&libs, b"FLA_Copyr_l\0").map(|sym| *sym),
            FLA_Copyr_u: get_symbol(&libs, b"FLA_Copyr_u\0").map(|sym| *sym),
            FLA_Scal_blk_var1: get_symbol(&libs, b"FLA_Scal_blk_var1\0").map(|sym| *sym),
            FLA_Scal_blk_var2: get_symbol(&libs, b"FLA_Scal_blk_var2\0").map(|sym| *sym),
            FLA_Scal_blk_var3: get_symbol(&libs, b"FLA_Scal_blk_var3\0").map(|sym| *sym),
            FLA_Scal_blk_var4: get_symbol(&libs, b"FLA_Scal_blk_var4\0").map(|sym| *sym),
            FLA_Scal_internal: get_symbol(&libs, b"FLA_Scal_internal\0").map(|sym| *sym),
            FLA_Scalr_l_blk_var1: get_symbol(&libs, b"FLA_Scalr_l_blk_var1\0").map(|sym| *sym),
            FLA_Scalr_l_blk_var2: get_symbol(&libs, b"FLA_Scalr_l_blk_var2\0").map(|sym| *sym),
            FLA_Scalr_l_blk_var3: get_symbol(&libs, b"FLA_Scalr_l_blk_var3\0").map(|sym| *sym),
            FLA_Scalr_l_blk_var4: get_symbol(&libs, b"FLA_Scalr_l_blk_var4\0").map(|sym| *sym),
            FLA_Scalr_u_blk_var1: get_symbol(&libs, b"FLA_Scalr_u_blk_var1\0").map(|sym| *sym),
            FLA_Scalr_u_blk_var2: get_symbol(&libs, b"FLA_Scalr_u_blk_var2\0").map(|sym| *sym),
            FLA_Scalr_u_blk_var3: get_symbol(&libs, b"FLA_Scalr_u_blk_var3\0").map(|sym| *sym),
            FLA_Scalr_u_blk_var4: get_symbol(&libs, b"FLA_Scalr_u_blk_var4\0").map(|sym| *sym),
            FLA_Scalr_internal: get_symbol(&libs, b"FLA_Scalr_internal\0").map(|sym| *sym),
            FLA_Scalr_l: get_symbol(&libs, b"FLA_Scalr_l\0").map(|sym| *sym),
            FLA_Scalr_u: get_symbol(&libs, b"FLA_Scalr_u\0").map(|sym| *sym),
            FLA_Gemv_h_blk_var1: get_symbol(&libs, b"FLA_Gemv_h_blk_var1\0").map(|sym| *sym),
            FLA_Gemv_h_blk_var2: get_symbol(&libs, b"FLA_Gemv_h_blk_var2\0").map(|sym| *sym),
            FLA_Gemv_h_blk_var5: get_symbol(&libs, b"FLA_Gemv_h_blk_var5\0").map(|sym| *sym),
            FLA_Gemv_h_blk_var6: get_symbol(&libs, b"FLA_Gemv_h_blk_var6\0").map(|sym| *sym),
            FLA_Gemv_n_blk_var1: get_symbol(&libs, b"FLA_Gemv_n_blk_var1\0").map(|sym| *sym),
            FLA_Gemv_n_blk_var2: get_symbol(&libs, b"FLA_Gemv_n_blk_var2\0").map(|sym| *sym),
            FLA_Gemv_n_blk_var5: get_symbol(&libs, b"FLA_Gemv_n_blk_var5\0").map(|sym| *sym),
            FLA_Gemv_n_blk_var6: get_symbol(&libs, b"FLA_Gemv_n_blk_var6\0").map(|sym| *sym),
            FLA_Gemv_t_blk_var1: get_symbol(&libs, b"FLA_Gemv_t_blk_var1\0").map(|sym| *sym),
            FLA_Gemv_t_blk_var2: get_symbol(&libs, b"FLA_Gemv_t_blk_var2\0").map(|sym| *sym),
            FLA_Gemv_t_blk_var5: get_symbol(&libs, b"FLA_Gemv_t_blk_var5\0").map(|sym| *sym),
            FLA_Gemv_t_blk_var6: get_symbol(&libs, b"FLA_Gemv_t_blk_var6\0").map(|sym| *sym),
            FLA_Gemv_internal: get_symbol(&libs, b"FLA_Gemv_internal\0").map(|sym| *sym),
            FLA_Gemv_h: get_symbol(&libs, b"FLA_Gemv_h\0").map(|sym| *sym),
            FLA_Gemv_n: get_symbol(&libs, b"FLA_Gemv_n\0").map(|sym| *sym),
            FLA_Gemv_t: get_symbol(&libs, b"FLA_Gemv_t\0").map(|sym| *sym),
            FLA_Trsv_lc_blk_var1: get_symbol(&libs, b"FLA_Trsv_lc_blk_var1\0").map(|sym| *sym),
            FLA_Trsv_lc_blk_var2: get_symbol(&libs, b"FLA_Trsv_lc_blk_var2\0").map(|sym| *sym),
            FLA_Trsv_ln_blk_var1: get_symbol(&libs, b"FLA_Trsv_ln_blk_var1\0").map(|sym| *sym),
            FLA_Trsv_ln_blk_var2: get_symbol(&libs, b"FLA_Trsv_ln_blk_var2\0").map(|sym| *sym),
            FLA_Trsv_lt_blk_var1: get_symbol(&libs, b"FLA_Trsv_lt_blk_var1\0").map(|sym| *sym),
            FLA_Trsv_lt_blk_var2: get_symbol(&libs, b"FLA_Trsv_lt_blk_var2\0").map(|sym| *sym),
            FLA_Trsv_uc_blk_var1: get_symbol(&libs, b"FLA_Trsv_uc_blk_var1\0").map(|sym| *sym),
            FLA_Trsv_uc_blk_var2: get_symbol(&libs, b"FLA_Trsv_uc_blk_var2\0").map(|sym| *sym),
            FLA_Trsv_un_blk_var1: get_symbol(&libs, b"FLA_Trsv_un_blk_var1\0").map(|sym| *sym),
            FLA_Trsv_un_blk_var2: get_symbol(&libs, b"FLA_Trsv_un_blk_var2\0").map(|sym| *sym),
            FLA_Trsv_ut_blk_var1: get_symbol(&libs, b"FLA_Trsv_ut_blk_var1\0").map(|sym| *sym),
            FLA_Trsv_ut_blk_var2: get_symbol(&libs, b"FLA_Trsv_ut_blk_var2\0").map(|sym| *sym),
            FLA_Trsv_internal: get_symbol(&libs, b"FLA_Trsv_internal\0").map(|sym| *sym),
            FLA_Trsv_lc: get_symbol(&libs, b"FLA_Trsv_lc\0").map(|sym| *sym),
            FLA_Trsv_ln: get_symbol(&libs, b"FLA_Trsv_ln\0").map(|sym| *sym),
            FLA_Trsv_lt: get_symbol(&libs, b"FLA_Trsv_lt\0").map(|sym| *sym),
            FLA_Trsv_uc: get_symbol(&libs, b"FLA_Trsv_uc\0").map(|sym| *sym),
            FLA_Trsv_un: get_symbol(&libs, b"FLA_Trsv_un\0").map(|sym| *sym),
            FLA_Trsv_ut: get_symbol(&libs, b"FLA_Trsv_ut\0").map(|sym| *sym),
            FLA_Gemm_cc_blk_var1: get_symbol(&libs, b"FLA_Gemm_cc_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_cc_blk_var2: get_symbol(&libs, b"FLA_Gemm_cc_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_cc_blk_var3: get_symbol(&libs, b"FLA_Gemm_cc_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_cc_blk_var4: get_symbol(&libs, b"FLA_Gemm_cc_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_cc_blk_var5: get_symbol(&libs, b"FLA_Gemm_cc_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_cc_blk_var6: get_symbol(&libs, b"FLA_Gemm_cc_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_cc_unb_var1: get_symbol(&libs, b"FLA_Gemm_cc_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_cc_unb_var2: get_symbol(&libs, b"FLA_Gemm_cc_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_cc_unb_var3: get_symbol(&libs, b"FLA_Gemm_cc_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_cc_unb_var4: get_symbol(&libs, b"FLA_Gemm_cc_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_cc_unb_var5: get_symbol(&libs, b"FLA_Gemm_cc_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_cc_unb_var6: get_symbol(&libs, b"FLA_Gemm_cc_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_ch_blk_var1: get_symbol(&libs, b"FLA_Gemm_ch_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_ch_blk_var2: get_symbol(&libs, b"FLA_Gemm_ch_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_ch_blk_var3: get_symbol(&libs, b"FLA_Gemm_ch_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_ch_blk_var4: get_symbol(&libs, b"FLA_Gemm_ch_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_ch_blk_var5: get_symbol(&libs, b"FLA_Gemm_ch_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_ch_blk_var6: get_symbol(&libs, b"FLA_Gemm_ch_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_ch_unb_var1: get_symbol(&libs, b"FLA_Gemm_ch_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_ch_unb_var2: get_symbol(&libs, b"FLA_Gemm_ch_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_ch_unb_var3: get_symbol(&libs, b"FLA_Gemm_ch_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_ch_unb_var4: get_symbol(&libs, b"FLA_Gemm_ch_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_ch_unb_var5: get_symbol(&libs, b"FLA_Gemm_ch_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_ch_unb_var6: get_symbol(&libs, b"FLA_Gemm_ch_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_cn_blk_var1: get_symbol(&libs, b"FLA_Gemm_cn_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_cn_blk_var2: get_symbol(&libs, b"FLA_Gemm_cn_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_cn_blk_var3: get_symbol(&libs, b"FLA_Gemm_cn_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_cn_blk_var4: get_symbol(&libs, b"FLA_Gemm_cn_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_cn_blk_var5: get_symbol(&libs, b"FLA_Gemm_cn_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_cn_blk_var6: get_symbol(&libs, b"FLA_Gemm_cn_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_cn_unb_var1: get_symbol(&libs, b"FLA_Gemm_cn_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_cn_unb_var2: get_symbol(&libs, b"FLA_Gemm_cn_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_cn_unb_var3: get_symbol(&libs, b"FLA_Gemm_cn_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_cn_unb_var4: get_symbol(&libs, b"FLA_Gemm_cn_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_cn_unb_var5: get_symbol(&libs, b"FLA_Gemm_cn_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_cn_unb_var6: get_symbol(&libs, b"FLA_Gemm_cn_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_ct_blk_var1: get_symbol(&libs, b"FLA_Gemm_ct_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_ct_blk_var2: get_symbol(&libs, b"FLA_Gemm_ct_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_ct_blk_var3: get_symbol(&libs, b"FLA_Gemm_ct_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_ct_blk_var4: get_symbol(&libs, b"FLA_Gemm_ct_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_ct_blk_var5: get_symbol(&libs, b"FLA_Gemm_ct_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_ct_blk_var6: get_symbol(&libs, b"FLA_Gemm_ct_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_ct_unb_var1: get_symbol(&libs, b"FLA_Gemm_ct_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_ct_unb_var2: get_symbol(&libs, b"FLA_Gemm_ct_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_ct_unb_var3: get_symbol(&libs, b"FLA_Gemm_ct_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_ct_unb_var4: get_symbol(&libs, b"FLA_Gemm_ct_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_ct_unb_var5: get_symbol(&libs, b"FLA_Gemm_ct_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_ct_unb_var6: get_symbol(&libs, b"FLA_Gemm_ct_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_hc_blk_var1: get_symbol(&libs, b"FLA_Gemm_hc_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_hc_blk_var2: get_symbol(&libs, b"FLA_Gemm_hc_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_hc_blk_var3: get_symbol(&libs, b"FLA_Gemm_hc_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_hc_blk_var4: get_symbol(&libs, b"FLA_Gemm_hc_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_hc_blk_var5: get_symbol(&libs, b"FLA_Gemm_hc_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_hc_blk_var6: get_symbol(&libs, b"FLA_Gemm_hc_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_hc_unb_var1: get_symbol(&libs, b"FLA_Gemm_hc_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_hc_unb_var2: get_symbol(&libs, b"FLA_Gemm_hc_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_hc_unb_var3: get_symbol(&libs, b"FLA_Gemm_hc_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_hc_unb_var4: get_symbol(&libs, b"FLA_Gemm_hc_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_hc_unb_var5: get_symbol(&libs, b"FLA_Gemm_hc_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_hc_unb_var6: get_symbol(&libs, b"FLA_Gemm_hc_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_hh_blk_var1: get_symbol(&libs, b"FLA_Gemm_hh_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_hh_blk_var2: get_symbol(&libs, b"FLA_Gemm_hh_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_hh_blk_var3: get_symbol(&libs, b"FLA_Gemm_hh_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_hh_blk_var4: get_symbol(&libs, b"FLA_Gemm_hh_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_hh_blk_var5: get_symbol(&libs, b"FLA_Gemm_hh_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_hh_blk_var6: get_symbol(&libs, b"FLA_Gemm_hh_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_hh_unb_var1: get_symbol(&libs, b"FLA_Gemm_hh_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_hh_unb_var2: get_symbol(&libs, b"FLA_Gemm_hh_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_hh_unb_var3: get_symbol(&libs, b"FLA_Gemm_hh_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_hh_unb_var4: get_symbol(&libs, b"FLA_Gemm_hh_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_hh_unb_var5: get_symbol(&libs, b"FLA_Gemm_hh_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_hh_unb_var6: get_symbol(&libs, b"FLA_Gemm_hh_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_hn_blk_var1: get_symbol(&libs, b"FLA_Gemm_hn_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_hn_blk_var2: get_symbol(&libs, b"FLA_Gemm_hn_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_hn_blk_var3: get_symbol(&libs, b"FLA_Gemm_hn_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_hn_blk_var4: get_symbol(&libs, b"FLA_Gemm_hn_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_hn_blk_var5: get_symbol(&libs, b"FLA_Gemm_hn_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_hn_blk_var6: get_symbol(&libs, b"FLA_Gemm_hn_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_hn_unb_var1: get_symbol(&libs, b"FLA_Gemm_hn_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_hn_unb_var2: get_symbol(&libs, b"FLA_Gemm_hn_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_hn_unb_var3: get_symbol(&libs, b"FLA_Gemm_hn_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_hn_unb_var4: get_symbol(&libs, b"FLA_Gemm_hn_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_hn_unb_var5: get_symbol(&libs, b"FLA_Gemm_hn_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_hn_unb_var6: get_symbol(&libs, b"FLA_Gemm_hn_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_ht_blk_var1: get_symbol(&libs, b"FLA_Gemm_ht_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_ht_blk_var2: get_symbol(&libs, b"FLA_Gemm_ht_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_ht_blk_var3: get_symbol(&libs, b"FLA_Gemm_ht_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_ht_blk_var4: get_symbol(&libs, b"FLA_Gemm_ht_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_ht_blk_var5: get_symbol(&libs, b"FLA_Gemm_ht_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_ht_blk_var6: get_symbol(&libs, b"FLA_Gemm_ht_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_ht_unb_var1: get_symbol(&libs, b"FLA_Gemm_ht_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_ht_unb_var2: get_symbol(&libs, b"FLA_Gemm_ht_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_ht_unb_var3: get_symbol(&libs, b"FLA_Gemm_ht_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_ht_unb_var4: get_symbol(&libs, b"FLA_Gemm_ht_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_ht_unb_var5: get_symbol(&libs, b"FLA_Gemm_ht_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_ht_unb_var6: get_symbol(&libs, b"FLA_Gemm_ht_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_nc_blk_var1: get_symbol(&libs, b"FLA_Gemm_nc_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_nc_blk_var2: get_symbol(&libs, b"FLA_Gemm_nc_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_nc_blk_var3: get_symbol(&libs, b"FLA_Gemm_nc_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_nc_blk_var4: get_symbol(&libs, b"FLA_Gemm_nc_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_nc_blk_var5: get_symbol(&libs, b"FLA_Gemm_nc_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_nc_blk_var6: get_symbol(&libs, b"FLA_Gemm_nc_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_nc_unb_var1: get_symbol(&libs, b"FLA_Gemm_nc_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_nc_unb_var2: get_symbol(&libs, b"FLA_Gemm_nc_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_nc_unb_var3: get_symbol(&libs, b"FLA_Gemm_nc_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_nc_unb_var4: get_symbol(&libs, b"FLA_Gemm_nc_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_nc_unb_var5: get_symbol(&libs, b"FLA_Gemm_nc_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_nc_unb_var6: get_symbol(&libs, b"FLA_Gemm_nc_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_nh_blk_var1: get_symbol(&libs, b"FLA_Gemm_nh_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_nh_blk_var2: get_symbol(&libs, b"FLA_Gemm_nh_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_nh_blk_var3: get_symbol(&libs, b"FLA_Gemm_nh_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_nh_blk_var4: get_symbol(&libs, b"FLA_Gemm_nh_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_nh_blk_var5: get_symbol(&libs, b"FLA_Gemm_nh_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_nh_blk_var6: get_symbol(&libs, b"FLA_Gemm_nh_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_nh_unb_var1: get_symbol(&libs, b"FLA_Gemm_nh_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_nh_unb_var2: get_symbol(&libs, b"FLA_Gemm_nh_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_nh_unb_var3: get_symbol(&libs, b"FLA_Gemm_nh_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_nh_unb_var4: get_symbol(&libs, b"FLA_Gemm_nh_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_nh_unb_var5: get_symbol(&libs, b"FLA_Gemm_nh_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_nh_unb_var6: get_symbol(&libs, b"FLA_Gemm_nh_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_nn_blk_var1: get_symbol(&libs, b"FLA_Gemm_nn_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_nn_blk_var2: get_symbol(&libs, b"FLA_Gemm_nn_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_nn_blk_var3: get_symbol(&libs, b"FLA_Gemm_nn_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_nn_blk_var4: get_symbol(&libs, b"FLA_Gemm_nn_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_nn_blk_var5: get_symbol(&libs, b"FLA_Gemm_nn_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_nn_blk_var6: get_symbol(&libs, b"FLA_Gemm_nn_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_nn_unb_var1: get_symbol(&libs, b"FLA_Gemm_nn_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_nn_unb_var2: get_symbol(&libs, b"FLA_Gemm_nn_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_nn_unb_var3: get_symbol(&libs, b"FLA_Gemm_nn_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_nn_unb_var4: get_symbol(&libs, b"FLA_Gemm_nn_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_nn_unb_var5: get_symbol(&libs, b"FLA_Gemm_nn_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_nn_unb_var6: get_symbol(&libs, b"FLA_Gemm_nn_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_nt_blk_var1: get_symbol(&libs, b"FLA_Gemm_nt_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_nt_blk_var2: get_symbol(&libs, b"FLA_Gemm_nt_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_nt_blk_var3: get_symbol(&libs, b"FLA_Gemm_nt_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_nt_blk_var4: get_symbol(&libs, b"FLA_Gemm_nt_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_nt_blk_var5: get_symbol(&libs, b"FLA_Gemm_nt_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_nt_blk_var6: get_symbol(&libs, b"FLA_Gemm_nt_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_nt_unb_var1: get_symbol(&libs, b"FLA_Gemm_nt_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_nt_unb_var2: get_symbol(&libs, b"FLA_Gemm_nt_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_nt_unb_var3: get_symbol(&libs, b"FLA_Gemm_nt_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_nt_unb_var4: get_symbol(&libs, b"FLA_Gemm_nt_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_nt_unb_var5: get_symbol(&libs, b"FLA_Gemm_nt_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_nt_unb_var6: get_symbol(&libs, b"FLA_Gemm_nt_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_tc_blk_var1: get_symbol(&libs, b"FLA_Gemm_tc_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_tc_blk_var2: get_symbol(&libs, b"FLA_Gemm_tc_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_tc_blk_var3: get_symbol(&libs, b"FLA_Gemm_tc_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_tc_blk_var4: get_symbol(&libs, b"FLA_Gemm_tc_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_tc_blk_var5: get_symbol(&libs, b"FLA_Gemm_tc_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_tc_blk_var6: get_symbol(&libs, b"FLA_Gemm_tc_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_tc_unb_var1: get_symbol(&libs, b"FLA_Gemm_tc_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_tc_unb_var2: get_symbol(&libs, b"FLA_Gemm_tc_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_tc_unb_var3: get_symbol(&libs, b"FLA_Gemm_tc_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_tc_unb_var4: get_symbol(&libs, b"FLA_Gemm_tc_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_tc_unb_var5: get_symbol(&libs, b"FLA_Gemm_tc_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_tc_unb_var6: get_symbol(&libs, b"FLA_Gemm_tc_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_th_blk_var1: get_symbol(&libs, b"FLA_Gemm_th_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_th_blk_var2: get_symbol(&libs, b"FLA_Gemm_th_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_th_blk_var3: get_symbol(&libs, b"FLA_Gemm_th_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_th_blk_var4: get_symbol(&libs, b"FLA_Gemm_th_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_th_blk_var5: get_symbol(&libs, b"FLA_Gemm_th_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_th_blk_var6: get_symbol(&libs, b"FLA_Gemm_th_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_th_unb_var1: get_symbol(&libs, b"FLA_Gemm_th_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_th_unb_var2: get_symbol(&libs, b"FLA_Gemm_th_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_th_unb_var3: get_symbol(&libs, b"FLA_Gemm_th_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_th_unb_var4: get_symbol(&libs, b"FLA_Gemm_th_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_th_unb_var5: get_symbol(&libs, b"FLA_Gemm_th_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_th_unb_var6: get_symbol(&libs, b"FLA_Gemm_th_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_tn_blk_var1: get_symbol(&libs, b"FLA_Gemm_tn_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_tn_blk_var2: get_symbol(&libs, b"FLA_Gemm_tn_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_tn_blk_var3: get_symbol(&libs, b"FLA_Gemm_tn_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_tn_blk_var4: get_symbol(&libs, b"FLA_Gemm_tn_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_tn_blk_var5: get_symbol(&libs, b"FLA_Gemm_tn_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_tn_blk_var6: get_symbol(&libs, b"FLA_Gemm_tn_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_tn_unb_var1: get_symbol(&libs, b"FLA_Gemm_tn_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_tn_unb_var2: get_symbol(&libs, b"FLA_Gemm_tn_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_tn_unb_var3: get_symbol(&libs, b"FLA_Gemm_tn_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_tn_unb_var4: get_symbol(&libs, b"FLA_Gemm_tn_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_tn_unb_var5: get_symbol(&libs, b"FLA_Gemm_tn_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_tn_unb_var6: get_symbol(&libs, b"FLA_Gemm_tn_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_tt_blk_var1: get_symbol(&libs, b"FLA_Gemm_tt_blk_var1\0").map(|sym| *sym),
            FLA_Gemm_tt_blk_var2: get_symbol(&libs, b"FLA_Gemm_tt_blk_var2\0").map(|sym| *sym),
            FLA_Gemm_tt_blk_var3: get_symbol(&libs, b"FLA_Gemm_tt_blk_var3\0").map(|sym| *sym),
            FLA_Gemm_tt_blk_var4: get_symbol(&libs, b"FLA_Gemm_tt_blk_var4\0").map(|sym| *sym),
            FLA_Gemm_tt_blk_var5: get_symbol(&libs, b"FLA_Gemm_tt_blk_var5\0").map(|sym| *sym),
            FLA_Gemm_tt_blk_var6: get_symbol(&libs, b"FLA_Gemm_tt_blk_var6\0").map(|sym| *sym),
            FLA_Gemm_tt_unb_var1: get_symbol(&libs, b"FLA_Gemm_tt_unb_var1\0").map(|sym| *sym),
            FLA_Gemm_tt_unb_var2: get_symbol(&libs, b"FLA_Gemm_tt_unb_var2\0").map(|sym| *sym),
            FLA_Gemm_tt_unb_var3: get_symbol(&libs, b"FLA_Gemm_tt_unb_var3\0").map(|sym| *sym),
            FLA_Gemm_tt_unb_var4: get_symbol(&libs, b"FLA_Gemm_tt_unb_var4\0").map(|sym| *sym),
            FLA_Gemm_tt_unb_var5: get_symbol(&libs, b"FLA_Gemm_tt_unb_var5\0").map(|sym| *sym),
            FLA_Gemm_tt_unb_var6: get_symbol(&libs, b"FLA_Gemm_tt_unb_var6\0").map(|sym| *sym),
            FLA_Gemm_internal: get_symbol(&libs, b"FLA_Gemm_internal\0").map(|sym| *sym),
            FLA_Gemm_cc: get_symbol(&libs, b"FLA_Gemm_cc\0").map(|sym| *sym),
            FLA_Gemm_ch: get_symbol(&libs, b"FLA_Gemm_ch\0").map(|sym| *sym),
            FLA_Gemm_cn: get_symbol(&libs, b"FLA_Gemm_cn\0").map(|sym| *sym),
            FLA_Gemm_ct: get_symbol(&libs, b"FLA_Gemm_ct\0").map(|sym| *sym),
            FLA_Gemm_hc: get_symbol(&libs, b"FLA_Gemm_hc\0").map(|sym| *sym),
            FLA_Gemm_hh: get_symbol(&libs, b"FLA_Gemm_hh\0").map(|sym| *sym),
            FLA_Gemm_hn: get_symbol(&libs, b"FLA_Gemm_hn\0").map(|sym| *sym),
            FLA_Gemm_ht: get_symbol(&libs, b"FLA_Gemm_ht\0").map(|sym| *sym),
            FLA_Gemm_nc: get_symbol(&libs, b"FLA_Gemm_nc\0").map(|sym| *sym),
            FLA_Gemm_nh: get_symbol(&libs, b"FLA_Gemm_nh\0").map(|sym| *sym),
            FLA_Gemm_nn: get_symbol(&libs, b"FLA_Gemm_nn\0").map(|sym| *sym),
            FLA_Gemm_nt: get_symbol(&libs, b"FLA_Gemm_nt\0").map(|sym| *sym),
            FLA_Gemm_tc: get_symbol(&libs, b"FLA_Gemm_tc\0").map(|sym| *sym),
            FLA_Gemm_th: get_symbol(&libs, b"FLA_Gemm_th\0").map(|sym| *sym),
            FLA_Gemm_tn: get_symbol(&libs, b"FLA_Gemm_tn\0").map(|sym| *sym),
            FLA_Gemm_tt: get_symbol(&libs, b"FLA_Gemm_tt\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var1: get_symbol(&libs, b"FLA_Hemm_ll_blk_var1\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var2: get_symbol(&libs, b"FLA_Hemm_ll_blk_var2\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var3: get_symbol(&libs, b"FLA_Hemm_ll_blk_var3\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var4: get_symbol(&libs, b"FLA_Hemm_ll_blk_var4\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var5: get_symbol(&libs, b"FLA_Hemm_ll_blk_var5\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var6: get_symbol(&libs, b"FLA_Hemm_ll_blk_var6\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var7: get_symbol(&libs, b"FLA_Hemm_ll_blk_var7\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var8: get_symbol(&libs, b"FLA_Hemm_ll_blk_var8\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var9: get_symbol(&libs, b"FLA_Hemm_ll_blk_var9\0").map(|sym| *sym),
            FLA_Hemm_ll_blk_var10: get_symbol(&libs, b"FLA_Hemm_ll_blk_var10\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var1: get_symbol(&libs, b"FLA_Hemm_ll_unb_var1\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var2: get_symbol(&libs, b"FLA_Hemm_ll_unb_var2\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var3: get_symbol(&libs, b"FLA_Hemm_ll_unb_var3\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var4: get_symbol(&libs, b"FLA_Hemm_ll_unb_var4\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var5: get_symbol(&libs, b"FLA_Hemm_ll_unb_var5\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var6: get_symbol(&libs, b"FLA_Hemm_ll_unb_var6\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var7: get_symbol(&libs, b"FLA_Hemm_ll_unb_var7\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var8: get_symbol(&libs, b"FLA_Hemm_ll_unb_var8\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var9: get_symbol(&libs, b"FLA_Hemm_ll_unb_var9\0").map(|sym| *sym),
            FLA_Hemm_ll_unb_var10: get_symbol(&libs, b"FLA_Hemm_ll_unb_var10\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var1: get_symbol(&libs, b"FLA_Hemm_lu_blk_var1\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var2: get_symbol(&libs, b"FLA_Hemm_lu_blk_var2\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var3: get_symbol(&libs, b"FLA_Hemm_lu_blk_var3\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var4: get_symbol(&libs, b"FLA_Hemm_lu_blk_var4\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var5: get_symbol(&libs, b"FLA_Hemm_lu_blk_var5\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var6: get_symbol(&libs, b"FLA_Hemm_lu_blk_var6\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var7: get_symbol(&libs, b"FLA_Hemm_lu_blk_var7\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var8: get_symbol(&libs, b"FLA_Hemm_lu_blk_var8\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var9: get_symbol(&libs, b"FLA_Hemm_lu_blk_var9\0").map(|sym| *sym),
            FLA_Hemm_lu_blk_var10: get_symbol(&libs, b"FLA_Hemm_lu_blk_var10\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var1: get_symbol(&libs, b"FLA_Hemm_lu_unb_var1\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var2: get_symbol(&libs, b"FLA_Hemm_lu_unb_var2\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var3: get_symbol(&libs, b"FLA_Hemm_lu_unb_var3\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var4: get_symbol(&libs, b"FLA_Hemm_lu_unb_var4\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var5: get_symbol(&libs, b"FLA_Hemm_lu_unb_var5\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var6: get_symbol(&libs, b"FLA_Hemm_lu_unb_var6\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var7: get_symbol(&libs, b"FLA_Hemm_lu_unb_var7\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var8: get_symbol(&libs, b"FLA_Hemm_lu_unb_var8\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var9: get_symbol(&libs, b"FLA_Hemm_lu_unb_var9\0").map(|sym| *sym),
            FLA_Hemm_lu_unb_var10: get_symbol(&libs, b"FLA_Hemm_lu_unb_var10\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var1: get_symbol(&libs, b"FLA_Hemm_rl_blk_var1\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var2: get_symbol(&libs, b"FLA_Hemm_rl_blk_var2\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var3: get_symbol(&libs, b"FLA_Hemm_rl_blk_var3\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var4: get_symbol(&libs, b"FLA_Hemm_rl_blk_var4\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var5: get_symbol(&libs, b"FLA_Hemm_rl_blk_var5\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var6: get_symbol(&libs, b"FLA_Hemm_rl_blk_var6\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var7: get_symbol(&libs, b"FLA_Hemm_rl_blk_var7\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var8: get_symbol(&libs, b"FLA_Hemm_rl_blk_var8\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var9: get_symbol(&libs, b"FLA_Hemm_rl_blk_var9\0").map(|sym| *sym),
            FLA_Hemm_rl_blk_var10: get_symbol(&libs, b"FLA_Hemm_rl_blk_var10\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var1: get_symbol(&libs, b"FLA_Hemm_rl_unb_var1\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var2: get_symbol(&libs, b"FLA_Hemm_rl_unb_var2\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var3: get_symbol(&libs, b"FLA_Hemm_rl_unb_var3\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var4: get_symbol(&libs, b"FLA_Hemm_rl_unb_var4\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var5: get_symbol(&libs, b"FLA_Hemm_rl_unb_var5\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var6: get_symbol(&libs, b"FLA_Hemm_rl_unb_var6\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var7: get_symbol(&libs, b"FLA_Hemm_rl_unb_var7\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var8: get_symbol(&libs, b"FLA_Hemm_rl_unb_var8\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var9: get_symbol(&libs, b"FLA_Hemm_rl_unb_var9\0").map(|sym| *sym),
            FLA_Hemm_rl_unb_var10: get_symbol(&libs, b"FLA_Hemm_rl_unb_var10\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var1: get_symbol(&libs, b"FLA_Hemm_ru_blk_var1\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var2: get_symbol(&libs, b"FLA_Hemm_ru_blk_var2\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var3: get_symbol(&libs, b"FLA_Hemm_ru_blk_var3\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var4: get_symbol(&libs, b"FLA_Hemm_ru_blk_var4\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var5: get_symbol(&libs, b"FLA_Hemm_ru_blk_var5\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var6: get_symbol(&libs, b"FLA_Hemm_ru_blk_var6\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var7: get_symbol(&libs, b"FLA_Hemm_ru_blk_var7\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var8: get_symbol(&libs, b"FLA_Hemm_ru_blk_var8\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var9: get_symbol(&libs, b"FLA_Hemm_ru_blk_var9\0").map(|sym| *sym),
            FLA_Hemm_ru_blk_var10: get_symbol(&libs, b"FLA_Hemm_ru_blk_var10\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var1: get_symbol(&libs, b"FLA_Hemm_ru_unb_var1\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var2: get_symbol(&libs, b"FLA_Hemm_ru_unb_var2\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var3: get_symbol(&libs, b"FLA_Hemm_ru_unb_var3\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var4: get_symbol(&libs, b"FLA_Hemm_ru_unb_var4\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var5: get_symbol(&libs, b"FLA_Hemm_ru_unb_var5\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var6: get_symbol(&libs, b"FLA_Hemm_ru_unb_var6\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var7: get_symbol(&libs, b"FLA_Hemm_ru_unb_var7\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var8: get_symbol(&libs, b"FLA_Hemm_ru_unb_var8\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var9: get_symbol(&libs, b"FLA_Hemm_ru_unb_var9\0").map(|sym| *sym),
            FLA_Hemm_ru_unb_var10: get_symbol(&libs, b"FLA_Hemm_ru_unb_var10\0").map(|sym| *sym),
            FLA_Hemm_internal: get_symbol(&libs, b"FLA_Hemm_internal\0").map(|sym| *sym),
            FLA_Hemm_ll: get_symbol(&libs, b"FLA_Hemm_ll\0").map(|sym| *sym),
            FLA_Hemm_lu: get_symbol(&libs, b"FLA_Hemm_lu\0").map(|sym| *sym),
            FLA_Hemm_rl: get_symbol(&libs, b"FLA_Hemm_rl\0").map(|sym| *sym),
            FLA_Hemm_ru: get_symbol(&libs, b"FLA_Hemm_ru\0").map(|sym| *sym),
            FLA_Herk_lh_blk_var1: get_symbol(&libs, b"FLA_Herk_lh_blk_var1\0").map(|sym| *sym),
            FLA_Herk_lh_blk_var2: get_symbol(&libs, b"FLA_Herk_lh_blk_var2\0").map(|sym| *sym),
            FLA_Herk_lh_blk_var3: get_symbol(&libs, b"FLA_Herk_lh_blk_var3\0").map(|sym| *sym),
            FLA_Herk_lh_blk_var4: get_symbol(&libs, b"FLA_Herk_lh_blk_var4\0").map(|sym| *sym),
            FLA_Herk_lh_blk_var5: get_symbol(&libs, b"FLA_Herk_lh_blk_var5\0").map(|sym| *sym),
            FLA_Herk_lh_blk_var6: get_symbol(&libs, b"FLA_Herk_lh_blk_var6\0").map(|sym| *sym),
            FLA_Herk_lh_unb_var1: get_symbol(&libs, b"FLA_Herk_lh_unb_var1\0").map(|sym| *sym),
            FLA_Herk_lh_unb_var2: get_symbol(&libs, b"FLA_Herk_lh_unb_var2\0").map(|sym| *sym),
            FLA_Herk_lh_unb_var3: get_symbol(&libs, b"FLA_Herk_lh_unb_var3\0").map(|sym| *sym),
            FLA_Herk_lh_unb_var4: get_symbol(&libs, b"FLA_Herk_lh_unb_var4\0").map(|sym| *sym),
            FLA_Herk_lh_unb_var5: get_symbol(&libs, b"FLA_Herk_lh_unb_var5\0").map(|sym| *sym),
            FLA_Herk_lh_unb_var6: get_symbol(&libs, b"FLA_Herk_lh_unb_var6\0").map(|sym| *sym),
            FLA_Herk_ln_blk_var1: get_symbol(&libs, b"FLA_Herk_ln_blk_var1\0").map(|sym| *sym),
            FLA_Herk_ln_blk_var2: get_symbol(&libs, b"FLA_Herk_ln_blk_var2\0").map(|sym| *sym),
            FLA_Herk_ln_blk_var3: get_symbol(&libs, b"FLA_Herk_ln_blk_var3\0").map(|sym| *sym),
            FLA_Herk_ln_blk_var4: get_symbol(&libs, b"FLA_Herk_ln_blk_var4\0").map(|sym| *sym),
            FLA_Herk_ln_blk_var5: get_symbol(&libs, b"FLA_Herk_ln_blk_var5\0").map(|sym| *sym),
            FLA_Herk_ln_blk_var6: get_symbol(&libs, b"FLA_Herk_ln_blk_var6\0").map(|sym| *sym),
            FLA_Herk_ln_unb_var1: get_symbol(&libs, b"FLA_Herk_ln_unb_var1\0").map(|sym| *sym),
            FLA_Herk_ln_unb_var2: get_symbol(&libs, b"FLA_Herk_ln_unb_var2\0").map(|sym| *sym),
            FLA_Herk_ln_unb_var3: get_symbol(&libs, b"FLA_Herk_ln_unb_var3\0").map(|sym| *sym),
            FLA_Herk_ln_unb_var4: get_symbol(&libs, b"FLA_Herk_ln_unb_var4\0").map(|sym| *sym),
            FLA_Herk_ln_unb_var5: get_symbol(&libs, b"FLA_Herk_ln_unb_var5\0").map(|sym| *sym),
            FLA_Herk_ln_unb_var6: get_symbol(&libs, b"FLA_Herk_ln_unb_var6\0").map(|sym| *sym),
            FLA_Herk_uh_blk_var1: get_symbol(&libs, b"FLA_Herk_uh_blk_var1\0").map(|sym| *sym),
            FLA_Herk_uh_blk_var2: get_symbol(&libs, b"FLA_Herk_uh_blk_var2\0").map(|sym| *sym),
            FLA_Herk_uh_blk_var3: get_symbol(&libs, b"FLA_Herk_uh_blk_var3\0").map(|sym| *sym),
            FLA_Herk_uh_blk_var4: get_symbol(&libs, b"FLA_Herk_uh_blk_var4\0").map(|sym| *sym),
            FLA_Herk_uh_blk_var5: get_symbol(&libs, b"FLA_Herk_uh_blk_var5\0").map(|sym| *sym),
            FLA_Herk_uh_blk_var6: get_symbol(&libs, b"FLA_Herk_uh_blk_var6\0").map(|sym| *sym),
            FLA_Herk_uh_unb_var1: get_symbol(&libs, b"FLA_Herk_uh_unb_var1\0").map(|sym| *sym),
            FLA_Herk_uh_unb_var2: get_symbol(&libs, b"FLA_Herk_uh_unb_var2\0").map(|sym| *sym),
            FLA_Herk_uh_unb_var3: get_symbol(&libs, b"FLA_Herk_uh_unb_var3\0").map(|sym| *sym),
            FLA_Herk_uh_unb_var4: get_symbol(&libs, b"FLA_Herk_uh_unb_var4\0").map(|sym| *sym),
            FLA_Herk_uh_unb_var5: get_symbol(&libs, b"FLA_Herk_uh_unb_var5\0").map(|sym| *sym),
            FLA_Herk_uh_unb_var6: get_symbol(&libs, b"FLA_Herk_uh_unb_var6\0").map(|sym| *sym),
            FLA_Herk_un_blk_var1: get_symbol(&libs, b"FLA_Herk_un_blk_var1\0").map(|sym| *sym),
            FLA_Herk_un_blk_var2: get_symbol(&libs, b"FLA_Herk_un_blk_var2\0").map(|sym| *sym),
            FLA_Herk_un_blk_var3: get_symbol(&libs, b"FLA_Herk_un_blk_var3\0").map(|sym| *sym),
            FLA_Herk_un_blk_var4: get_symbol(&libs, b"FLA_Herk_un_blk_var4\0").map(|sym| *sym),
            FLA_Herk_un_blk_var5: get_symbol(&libs, b"FLA_Herk_un_blk_var5\0").map(|sym| *sym),
            FLA_Herk_un_blk_var6: get_symbol(&libs, b"FLA_Herk_un_blk_var6\0").map(|sym| *sym),
            FLA_Herk_un_unb_var1: get_symbol(&libs, b"FLA_Herk_un_unb_var1\0").map(|sym| *sym),
            FLA_Herk_un_unb_var2: get_symbol(&libs, b"FLA_Herk_un_unb_var2\0").map(|sym| *sym),
            FLA_Herk_un_unb_var3: get_symbol(&libs, b"FLA_Herk_un_unb_var3\0").map(|sym| *sym),
            FLA_Herk_un_unb_var4: get_symbol(&libs, b"FLA_Herk_un_unb_var4\0").map(|sym| *sym),
            FLA_Herk_un_unb_var5: get_symbol(&libs, b"FLA_Herk_un_unb_var5\0").map(|sym| *sym),
            FLA_Herk_un_unb_var6: get_symbol(&libs, b"FLA_Herk_un_unb_var6\0").map(|sym| *sym),
            FLA_Herk_internal: get_symbol(&libs, b"FLA_Herk_internal\0").map(|sym| *sym),
            FLA_Herk_lh: get_symbol(&libs, b"FLA_Herk_lh\0").map(|sym| *sym),
            FLA_Herk_ln: get_symbol(&libs, b"FLA_Herk_ln\0").map(|sym| *sym),
            FLA_Herk_uh: get_symbol(&libs, b"FLA_Herk_uh\0").map(|sym| *sym),
            FLA_Herk_un: get_symbol(&libs, b"FLA_Herk_un\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var1: get_symbol(&libs, b"FLA_Her2k_lh_blk_var1\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var2: get_symbol(&libs, b"FLA_Her2k_lh_blk_var2\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var3: get_symbol(&libs, b"FLA_Her2k_lh_blk_var3\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var4: get_symbol(&libs, b"FLA_Her2k_lh_blk_var4\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var5: get_symbol(&libs, b"FLA_Her2k_lh_blk_var5\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var6: get_symbol(&libs, b"FLA_Her2k_lh_blk_var6\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var7: get_symbol(&libs, b"FLA_Her2k_lh_blk_var7\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var8: get_symbol(&libs, b"FLA_Her2k_lh_blk_var8\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var9: get_symbol(&libs, b"FLA_Her2k_lh_blk_var9\0").map(|sym| *sym),
            FLA_Her2k_lh_blk_var10: get_symbol(&libs, b"FLA_Her2k_lh_blk_var10\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var1: get_symbol(&libs, b"FLA_Her2k_lh_unb_var1\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var2: get_symbol(&libs, b"FLA_Her2k_lh_unb_var2\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var3: get_symbol(&libs, b"FLA_Her2k_lh_unb_var3\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var4: get_symbol(&libs, b"FLA_Her2k_lh_unb_var4\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var5: get_symbol(&libs, b"FLA_Her2k_lh_unb_var5\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var6: get_symbol(&libs, b"FLA_Her2k_lh_unb_var6\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var7: get_symbol(&libs, b"FLA_Her2k_lh_unb_var7\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var8: get_symbol(&libs, b"FLA_Her2k_lh_unb_var8\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var9: get_symbol(&libs, b"FLA_Her2k_lh_unb_var9\0").map(|sym| *sym),
            FLA_Her2k_lh_unb_var10: get_symbol(&libs, b"FLA_Her2k_lh_unb_var10\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var1: get_symbol(&libs, b"FLA_Her2k_ln_blk_var1\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var2: get_symbol(&libs, b"FLA_Her2k_ln_blk_var2\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var3: get_symbol(&libs, b"FLA_Her2k_ln_blk_var3\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var4: get_symbol(&libs, b"FLA_Her2k_ln_blk_var4\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var5: get_symbol(&libs, b"FLA_Her2k_ln_blk_var5\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var6: get_symbol(&libs, b"FLA_Her2k_ln_blk_var6\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var7: get_symbol(&libs, b"FLA_Her2k_ln_blk_var7\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var8: get_symbol(&libs, b"FLA_Her2k_ln_blk_var8\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var9: get_symbol(&libs, b"FLA_Her2k_ln_blk_var9\0").map(|sym| *sym),
            FLA_Her2k_ln_blk_var10: get_symbol(&libs, b"FLA_Her2k_ln_blk_var10\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var1: get_symbol(&libs, b"FLA_Her2k_ln_unb_var1\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var2: get_symbol(&libs, b"FLA_Her2k_ln_unb_var2\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var3: get_symbol(&libs, b"FLA_Her2k_ln_unb_var3\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var4: get_symbol(&libs, b"FLA_Her2k_ln_unb_var4\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var5: get_symbol(&libs, b"FLA_Her2k_ln_unb_var5\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var6: get_symbol(&libs, b"FLA_Her2k_ln_unb_var6\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var7: get_symbol(&libs, b"FLA_Her2k_ln_unb_var7\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var8: get_symbol(&libs, b"FLA_Her2k_ln_unb_var8\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var9: get_symbol(&libs, b"FLA_Her2k_ln_unb_var9\0").map(|sym| *sym),
            FLA_Her2k_ln_unb_var10: get_symbol(&libs, b"FLA_Her2k_ln_unb_var10\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var1: get_symbol(&libs, b"FLA_Her2k_uh_blk_var1\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var2: get_symbol(&libs, b"FLA_Her2k_uh_blk_var2\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var3: get_symbol(&libs, b"FLA_Her2k_uh_blk_var3\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var4: get_symbol(&libs, b"FLA_Her2k_uh_blk_var4\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var5: get_symbol(&libs, b"FLA_Her2k_uh_blk_var5\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var6: get_symbol(&libs, b"FLA_Her2k_uh_blk_var6\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var7: get_symbol(&libs, b"FLA_Her2k_uh_blk_var7\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var8: get_symbol(&libs, b"FLA_Her2k_uh_blk_var8\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var9: get_symbol(&libs, b"FLA_Her2k_uh_blk_var9\0").map(|sym| *sym),
            FLA_Her2k_uh_blk_var10: get_symbol(&libs, b"FLA_Her2k_uh_blk_var10\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var1: get_symbol(&libs, b"FLA_Her2k_uh_unb_var1\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var2: get_symbol(&libs, b"FLA_Her2k_uh_unb_var2\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var3: get_symbol(&libs, b"FLA_Her2k_uh_unb_var3\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var4: get_symbol(&libs, b"FLA_Her2k_uh_unb_var4\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var5: get_symbol(&libs, b"FLA_Her2k_uh_unb_var5\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var6: get_symbol(&libs, b"FLA_Her2k_uh_unb_var6\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var7: get_symbol(&libs, b"FLA_Her2k_uh_unb_var7\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var8: get_symbol(&libs, b"FLA_Her2k_uh_unb_var8\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var9: get_symbol(&libs, b"FLA_Her2k_uh_unb_var9\0").map(|sym| *sym),
            FLA_Her2k_uh_unb_var10: get_symbol(&libs, b"FLA_Her2k_uh_unb_var10\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var1: get_symbol(&libs, b"FLA_Her2k_un_blk_var1\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var2: get_symbol(&libs, b"FLA_Her2k_un_blk_var2\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var3: get_symbol(&libs, b"FLA_Her2k_un_blk_var3\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var4: get_symbol(&libs, b"FLA_Her2k_un_blk_var4\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var5: get_symbol(&libs, b"FLA_Her2k_un_blk_var5\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var6: get_symbol(&libs, b"FLA_Her2k_un_blk_var6\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var7: get_symbol(&libs, b"FLA_Her2k_un_blk_var7\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var8: get_symbol(&libs, b"FLA_Her2k_un_blk_var8\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var9: get_symbol(&libs, b"FLA_Her2k_un_blk_var9\0").map(|sym| *sym),
            FLA_Her2k_un_blk_var10: get_symbol(&libs, b"FLA_Her2k_un_blk_var10\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var1: get_symbol(&libs, b"FLA_Her2k_un_unb_var1\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var2: get_symbol(&libs, b"FLA_Her2k_un_unb_var2\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var3: get_symbol(&libs, b"FLA_Her2k_un_unb_var3\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var4: get_symbol(&libs, b"FLA_Her2k_un_unb_var4\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var5: get_symbol(&libs, b"FLA_Her2k_un_unb_var5\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var6: get_symbol(&libs, b"FLA_Her2k_un_unb_var6\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var7: get_symbol(&libs, b"FLA_Her2k_un_unb_var7\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var8: get_symbol(&libs, b"FLA_Her2k_un_unb_var8\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var9: get_symbol(&libs, b"FLA_Her2k_un_unb_var9\0").map(|sym| *sym),
            FLA_Her2k_un_unb_var10: get_symbol(&libs, b"FLA_Her2k_un_unb_var10\0").map(|sym| *sym),
            FLA_Her2k_internal: get_symbol(&libs, b"FLA_Her2k_internal\0").map(|sym| *sym),
            FLA_Her2k_lh: get_symbol(&libs, b"FLA_Her2k_lh\0").map(|sym| *sym),
            FLA_Her2k_ln: get_symbol(&libs, b"FLA_Her2k_ln\0").map(|sym| *sym),
            FLA_Her2k_uh: get_symbol(&libs, b"FLA_Her2k_uh\0").map(|sym| *sym),
            FLA_Her2k_un: get_symbol(&libs, b"FLA_Her2k_un\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var1: get_symbol(&libs, b"FLA_Symm_ll_blk_var1\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var2: get_symbol(&libs, b"FLA_Symm_ll_blk_var2\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var3: get_symbol(&libs, b"FLA_Symm_ll_blk_var3\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var4: get_symbol(&libs, b"FLA_Symm_ll_blk_var4\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var5: get_symbol(&libs, b"FLA_Symm_ll_blk_var5\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var6: get_symbol(&libs, b"FLA_Symm_ll_blk_var6\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var7: get_symbol(&libs, b"FLA_Symm_ll_blk_var7\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var8: get_symbol(&libs, b"FLA_Symm_ll_blk_var8\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var9: get_symbol(&libs, b"FLA_Symm_ll_blk_var9\0").map(|sym| *sym),
            FLA_Symm_ll_blk_var10: get_symbol(&libs, b"FLA_Symm_ll_blk_var10\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var1: get_symbol(&libs, b"FLA_Symm_ll_unb_var1\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var2: get_symbol(&libs, b"FLA_Symm_ll_unb_var2\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var3: get_symbol(&libs, b"FLA_Symm_ll_unb_var3\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var4: get_symbol(&libs, b"FLA_Symm_ll_unb_var4\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var5: get_symbol(&libs, b"FLA_Symm_ll_unb_var5\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var6: get_symbol(&libs, b"FLA_Symm_ll_unb_var6\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var7: get_symbol(&libs, b"FLA_Symm_ll_unb_var7\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var8: get_symbol(&libs, b"FLA_Symm_ll_unb_var8\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var9: get_symbol(&libs, b"FLA_Symm_ll_unb_var9\0").map(|sym| *sym),
            FLA_Symm_ll_unb_var10: get_symbol(&libs, b"FLA_Symm_ll_unb_var10\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var1: get_symbol(&libs, b"FLA_Symm_lu_blk_var1\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var2: get_symbol(&libs, b"FLA_Symm_lu_blk_var2\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var3: get_symbol(&libs, b"FLA_Symm_lu_blk_var3\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var4: get_symbol(&libs, b"FLA_Symm_lu_blk_var4\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var5: get_symbol(&libs, b"FLA_Symm_lu_blk_var5\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var6: get_symbol(&libs, b"FLA_Symm_lu_blk_var6\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var7: get_symbol(&libs, b"FLA_Symm_lu_blk_var7\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var8: get_symbol(&libs, b"FLA_Symm_lu_blk_var8\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var9: get_symbol(&libs, b"FLA_Symm_lu_blk_var9\0").map(|sym| *sym),
            FLA_Symm_lu_blk_var10: get_symbol(&libs, b"FLA_Symm_lu_blk_var10\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var1: get_symbol(&libs, b"FLA_Symm_lu_unb_var1\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var2: get_symbol(&libs, b"FLA_Symm_lu_unb_var2\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var3: get_symbol(&libs, b"FLA_Symm_lu_unb_var3\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var4: get_symbol(&libs, b"FLA_Symm_lu_unb_var4\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var5: get_symbol(&libs, b"FLA_Symm_lu_unb_var5\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var6: get_symbol(&libs, b"FLA_Symm_lu_unb_var6\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var7: get_symbol(&libs, b"FLA_Symm_lu_unb_var7\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var8: get_symbol(&libs, b"FLA_Symm_lu_unb_var8\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var9: get_symbol(&libs, b"FLA_Symm_lu_unb_var9\0").map(|sym| *sym),
            FLA_Symm_lu_unb_var10: get_symbol(&libs, b"FLA_Symm_lu_unb_var10\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var1: get_symbol(&libs, b"FLA_Symm_rl_blk_var1\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var2: get_symbol(&libs, b"FLA_Symm_rl_blk_var2\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var3: get_symbol(&libs, b"FLA_Symm_rl_blk_var3\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var4: get_symbol(&libs, b"FLA_Symm_rl_blk_var4\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var5: get_symbol(&libs, b"FLA_Symm_rl_blk_var5\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var6: get_symbol(&libs, b"FLA_Symm_rl_blk_var6\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var7: get_symbol(&libs, b"FLA_Symm_rl_blk_var7\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var8: get_symbol(&libs, b"FLA_Symm_rl_blk_var8\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var9: get_symbol(&libs, b"FLA_Symm_rl_blk_var9\0").map(|sym| *sym),
            FLA_Symm_rl_blk_var10: get_symbol(&libs, b"FLA_Symm_rl_blk_var10\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var1: get_symbol(&libs, b"FLA_Symm_rl_unb_var1\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var2: get_symbol(&libs, b"FLA_Symm_rl_unb_var2\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var3: get_symbol(&libs, b"FLA_Symm_rl_unb_var3\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var4: get_symbol(&libs, b"FLA_Symm_rl_unb_var4\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var5: get_symbol(&libs, b"FLA_Symm_rl_unb_var5\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var6: get_symbol(&libs, b"FLA_Symm_rl_unb_var6\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var7: get_symbol(&libs, b"FLA_Symm_rl_unb_var7\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var8: get_symbol(&libs, b"FLA_Symm_rl_unb_var8\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var9: get_symbol(&libs, b"FLA_Symm_rl_unb_var9\0").map(|sym| *sym),
            FLA_Symm_rl_unb_var10: get_symbol(&libs, b"FLA_Symm_rl_unb_var10\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var1: get_symbol(&libs, b"FLA_Symm_ru_blk_var1\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var2: get_symbol(&libs, b"FLA_Symm_ru_blk_var2\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var3: get_symbol(&libs, b"FLA_Symm_ru_blk_var3\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var4: get_symbol(&libs, b"FLA_Symm_ru_blk_var4\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var5: get_symbol(&libs, b"FLA_Symm_ru_blk_var5\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var6: get_symbol(&libs, b"FLA_Symm_ru_blk_var6\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var7: get_symbol(&libs, b"FLA_Symm_ru_blk_var7\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var8: get_symbol(&libs, b"FLA_Symm_ru_blk_var8\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var9: get_symbol(&libs, b"FLA_Symm_ru_blk_var9\0").map(|sym| *sym),
            FLA_Symm_ru_blk_var10: get_symbol(&libs, b"FLA_Symm_ru_blk_var10\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var1: get_symbol(&libs, b"FLA_Symm_ru_unb_var1\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var2: get_symbol(&libs, b"FLA_Symm_ru_unb_var2\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var3: get_symbol(&libs, b"FLA_Symm_ru_unb_var3\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var4: get_symbol(&libs, b"FLA_Symm_ru_unb_var4\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var5: get_symbol(&libs, b"FLA_Symm_ru_unb_var5\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var6: get_symbol(&libs, b"FLA_Symm_ru_unb_var6\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var7: get_symbol(&libs, b"FLA_Symm_ru_unb_var7\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var8: get_symbol(&libs, b"FLA_Symm_ru_unb_var8\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var9: get_symbol(&libs, b"FLA_Symm_ru_unb_var9\0").map(|sym| *sym),
            FLA_Symm_ru_unb_var10: get_symbol(&libs, b"FLA_Symm_ru_unb_var10\0").map(|sym| *sym),
            FLA_Symm_internal: get_symbol(&libs, b"FLA_Symm_internal\0").map(|sym| *sym),
            FLA_Symm_ll: get_symbol(&libs, b"FLA_Symm_ll\0").map(|sym| *sym),
            FLA_Symm_lu: get_symbol(&libs, b"FLA_Symm_lu\0").map(|sym| *sym),
            FLA_Symm_rl: get_symbol(&libs, b"FLA_Symm_rl\0").map(|sym| *sym),
            FLA_Symm_ru: get_symbol(&libs, b"FLA_Symm_ru\0").map(|sym| *sym),
            FLA_Syrk_ln_blk_var1: get_symbol(&libs, b"FLA_Syrk_ln_blk_var1\0").map(|sym| *sym),
            FLA_Syrk_ln_blk_var2: get_symbol(&libs, b"FLA_Syrk_ln_blk_var2\0").map(|sym| *sym),
            FLA_Syrk_ln_blk_var3: get_symbol(&libs, b"FLA_Syrk_ln_blk_var3\0").map(|sym| *sym),
            FLA_Syrk_ln_blk_var4: get_symbol(&libs, b"FLA_Syrk_ln_blk_var4\0").map(|sym| *sym),
            FLA_Syrk_ln_blk_var5: get_symbol(&libs, b"FLA_Syrk_ln_blk_var5\0").map(|sym| *sym),
            FLA_Syrk_ln_blk_var6: get_symbol(&libs, b"FLA_Syrk_ln_blk_var6\0").map(|sym| *sym),
            FLA_Syrk_ln_unb_var1: get_symbol(&libs, b"FLA_Syrk_ln_unb_var1\0").map(|sym| *sym),
            FLA_Syrk_ln_unb_var2: get_symbol(&libs, b"FLA_Syrk_ln_unb_var2\0").map(|sym| *sym),
            FLA_Syrk_ln_unb_var3: get_symbol(&libs, b"FLA_Syrk_ln_unb_var3\0").map(|sym| *sym),
            FLA_Syrk_ln_unb_var4: get_symbol(&libs, b"FLA_Syrk_ln_unb_var4\0").map(|sym| *sym),
            FLA_Syrk_ln_unb_var5: get_symbol(&libs, b"FLA_Syrk_ln_unb_var5\0").map(|sym| *sym),
            FLA_Syrk_ln_unb_var6: get_symbol(&libs, b"FLA_Syrk_ln_unb_var6\0").map(|sym| *sym),
            FLA_Syrk_lt_blk_var1: get_symbol(&libs, b"FLA_Syrk_lt_blk_var1\0").map(|sym| *sym),
            FLA_Syrk_lt_blk_var2: get_symbol(&libs, b"FLA_Syrk_lt_blk_var2\0").map(|sym| *sym),
            FLA_Syrk_lt_blk_var3: get_symbol(&libs, b"FLA_Syrk_lt_blk_var3\0").map(|sym| *sym),
            FLA_Syrk_lt_blk_var4: get_symbol(&libs, b"FLA_Syrk_lt_blk_var4\0").map(|sym| *sym),
            FLA_Syrk_lt_blk_var5: get_symbol(&libs, b"FLA_Syrk_lt_blk_var5\0").map(|sym| *sym),
            FLA_Syrk_lt_blk_var6: get_symbol(&libs, b"FLA_Syrk_lt_blk_var6\0").map(|sym| *sym),
            FLA_Syrk_lt_unb_var1: get_symbol(&libs, b"FLA_Syrk_lt_unb_var1\0").map(|sym| *sym),
            FLA_Syrk_lt_unb_var2: get_symbol(&libs, b"FLA_Syrk_lt_unb_var2\0").map(|sym| *sym),
            FLA_Syrk_lt_unb_var3: get_symbol(&libs, b"FLA_Syrk_lt_unb_var3\0").map(|sym| *sym),
            FLA_Syrk_lt_unb_var4: get_symbol(&libs, b"FLA_Syrk_lt_unb_var4\0").map(|sym| *sym),
            FLA_Syrk_lt_unb_var5: get_symbol(&libs, b"FLA_Syrk_lt_unb_var5\0").map(|sym| *sym),
            FLA_Syrk_lt_unb_var6: get_symbol(&libs, b"FLA_Syrk_lt_unb_var6\0").map(|sym| *sym),
            FLA_Syrk_un_blk_var1: get_symbol(&libs, b"FLA_Syrk_un_blk_var1\0").map(|sym| *sym),
            FLA_Syrk_un_blk_var2: get_symbol(&libs, b"FLA_Syrk_un_blk_var2\0").map(|sym| *sym),
            FLA_Syrk_un_blk_var3: get_symbol(&libs, b"FLA_Syrk_un_blk_var3\0").map(|sym| *sym),
            FLA_Syrk_un_blk_var4: get_symbol(&libs, b"FLA_Syrk_un_blk_var4\0").map(|sym| *sym),
            FLA_Syrk_un_blk_var5: get_symbol(&libs, b"FLA_Syrk_un_blk_var5\0").map(|sym| *sym),
            FLA_Syrk_un_blk_var6: get_symbol(&libs, b"FLA_Syrk_un_blk_var6\0").map(|sym| *sym),
            FLA_Syrk_un_unb_var1: get_symbol(&libs, b"FLA_Syrk_un_unb_var1\0").map(|sym| *sym),
            FLA_Syrk_un_unb_var2: get_symbol(&libs, b"FLA_Syrk_un_unb_var2\0").map(|sym| *sym),
            FLA_Syrk_un_unb_var3: get_symbol(&libs, b"FLA_Syrk_un_unb_var3\0").map(|sym| *sym),
            FLA_Syrk_un_unb_var4: get_symbol(&libs, b"FLA_Syrk_un_unb_var4\0").map(|sym| *sym),
            FLA_Syrk_un_unb_var5: get_symbol(&libs, b"FLA_Syrk_un_unb_var5\0").map(|sym| *sym),
            FLA_Syrk_un_unb_var6: get_symbol(&libs, b"FLA_Syrk_un_unb_var6\0").map(|sym| *sym),
            FLA_Syrk_ut_blk_var1: get_symbol(&libs, b"FLA_Syrk_ut_blk_var1\0").map(|sym| *sym),
            FLA_Syrk_ut_blk_var2: get_symbol(&libs, b"FLA_Syrk_ut_blk_var2\0").map(|sym| *sym),
            FLA_Syrk_ut_blk_var3: get_symbol(&libs, b"FLA_Syrk_ut_blk_var3\0").map(|sym| *sym),
            FLA_Syrk_ut_blk_var4: get_symbol(&libs, b"FLA_Syrk_ut_blk_var4\0").map(|sym| *sym),
            FLA_Syrk_ut_blk_var5: get_symbol(&libs, b"FLA_Syrk_ut_blk_var5\0").map(|sym| *sym),
            FLA_Syrk_ut_blk_var6: get_symbol(&libs, b"FLA_Syrk_ut_blk_var6\0").map(|sym| *sym),
            FLA_Syrk_ut_unb_var1: get_symbol(&libs, b"FLA_Syrk_ut_unb_var1\0").map(|sym| *sym),
            FLA_Syrk_ut_unb_var2: get_symbol(&libs, b"FLA_Syrk_ut_unb_var2\0").map(|sym| *sym),
            FLA_Syrk_ut_unb_var3: get_symbol(&libs, b"FLA_Syrk_ut_unb_var3\0").map(|sym| *sym),
            FLA_Syrk_ut_unb_var4: get_symbol(&libs, b"FLA_Syrk_ut_unb_var4\0").map(|sym| *sym),
            FLA_Syrk_ut_unb_var5: get_symbol(&libs, b"FLA_Syrk_ut_unb_var5\0").map(|sym| *sym),
            FLA_Syrk_ut_unb_var6: get_symbol(&libs, b"FLA_Syrk_ut_unb_var6\0").map(|sym| *sym),
            FLA_Syrk_internal: get_symbol(&libs, b"FLA_Syrk_internal\0").map(|sym| *sym),
            FLA_Syrk_ln: get_symbol(&libs, b"FLA_Syrk_ln\0").map(|sym| *sym),
            FLA_Syrk_lt: get_symbol(&libs, b"FLA_Syrk_lt\0").map(|sym| *sym),
            FLA_Syrk_un: get_symbol(&libs, b"FLA_Syrk_un\0").map(|sym| *sym),
            FLA_Syrk_ut: get_symbol(&libs, b"FLA_Syrk_ut\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var1: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var1\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var2: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var2\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var3: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var3\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var4: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var4\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var5: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var5\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var6: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var6\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var7: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var7\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var8: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var8\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var9: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var9\0").map(|sym| *sym),
            FLA_Syr2k_ln_blk_var10: get_symbol(&libs, b"FLA_Syr2k_ln_blk_var10\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var1: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var1\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var2: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var2\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var3: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var3\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var4: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var4\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var5: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var5\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var6: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var6\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var7: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var7\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var8: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var8\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var9: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var9\0").map(|sym| *sym),
            FLA_Syr2k_ln_unb_var10: get_symbol(&libs, b"FLA_Syr2k_ln_unb_var10\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var1: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var1\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var2: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var2\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var3: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var3\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var4: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var4\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var5: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var5\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var6: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var6\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var7: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var7\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var8: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var8\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var9: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var9\0").map(|sym| *sym),
            FLA_Syr2k_lt_blk_var10: get_symbol(&libs, b"FLA_Syr2k_lt_blk_var10\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var1: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var1\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var2: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var2\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var3: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var3\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var4: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var4\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var5: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var5\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var6: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var6\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var7: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var7\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var8: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var8\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var9: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var9\0").map(|sym| *sym),
            FLA_Syr2k_lt_unb_var10: get_symbol(&libs, b"FLA_Syr2k_lt_unb_var10\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var1: get_symbol(&libs, b"FLA_Syr2k_un_blk_var1\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var2: get_symbol(&libs, b"FLA_Syr2k_un_blk_var2\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var3: get_symbol(&libs, b"FLA_Syr2k_un_blk_var3\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var4: get_symbol(&libs, b"FLA_Syr2k_un_blk_var4\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var5: get_symbol(&libs, b"FLA_Syr2k_un_blk_var5\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var6: get_symbol(&libs, b"FLA_Syr2k_un_blk_var6\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var7: get_symbol(&libs, b"FLA_Syr2k_un_blk_var7\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var8: get_symbol(&libs, b"FLA_Syr2k_un_blk_var8\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var9: get_symbol(&libs, b"FLA_Syr2k_un_blk_var9\0").map(|sym| *sym),
            FLA_Syr2k_un_blk_var10: get_symbol(&libs, b"FLA_Syr2k_un_blk_var10\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var1: get_symbol(&libs, b"FLA_Syr2k_un_unb_var1\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var2: get_symbol(&libs, b"FLA_Syr2k_un_unb_var2\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var3: get_symbol(&libs, b"FLA_Syr2k_un_unb_var3\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var4: get_symbol(&libs, b"FLA_Syr2k_un_unb_var4\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var5: get_symbol(&libs, b"FLA_Syr2k_un_unb_var5\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var6: get_symbol(&libs, b"FLA_Syr2k_un_unb_var6\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var7: get_symbol(&libs, b"FLA_Syr2k_un_unb_var7\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var8: get_symbol(&libs, b"FLA_Syr2k_un_unb_var8\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var9: get_symbol(&libs, b"FLA_Syr2k_un_unb_var9\0").map(|sym| *sym),
            FLA_Syr2k_un_unb_var10: get_symbol(&libs, b"FLA_Syr2k_un_unb_var10\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var1: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var1\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var2: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var2\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var3: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var3\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var4: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var4\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var5: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var5\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var6: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var6\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var7: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var7\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var8: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var8\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var9: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var9\0").map(|sym| *sym),
            FLA_Syr2k_ut_blk_var10: get_symbol(&libs, b"FLA_Syr2k_ut_blk_var10\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var1: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var1\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var2: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var2\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var3: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var3\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var4: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var4\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var5: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var5\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var6: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var6\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var7: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var7\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var8: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var8\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var9: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var9\0").map(|sym| *sym),
            FLA_Syr2k_ut_unb_var10: get_symbol(&libs, b"FLA_Syr2k_ut_unb_var10\0").map(|sym| *sym),
            FLA_Syr2k_internal: get_symbol(&libs, b"FLA_Syr2k_internal\0").map(|sym| *sym),
            FLA_Syr2k_ln: get_symbol(&libs, b"FLA_Syr2k_ln\0").map(|sym| *sym),
            FLA_Syr2k_lt: get_symbol(&libs, b"FLA_Syr2k_lt\0").map(|sym| *sym),
            FLA_Syr2k_un: get_symbol(&libs, b"FLA_Syr2k_un\0").map(|sym| *sym),
            FLA_Syr2k_ut: get_symbol(&libs, b"FLA_Syr2k_ut\0").map(|sym| *sym),
            FLA_Trmm_llc_blk_var1: get_symbol(&libs, b"FLA_Trmm_llc_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_llc_blk_var2: get_symbol(&libs, b"FLA_Trmm_llc_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_llc_blk_var3: get_symbol(&libs, b"FLA_Trmm_llc_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_llc_blk_var4: get_symbol(&libs, b"FLA_Trmm_llc_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_llc_unb_var1: get_symbol(&libs, b"FLA_Trmm_llc_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_llc_unb_var2: get_symbol(&libs, b"FLA_Trmm_llc_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_llc_unb_var3: get_symbol(&libs, b"FLA_Trmm_llc_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_llc_unb_var4: get_symbol(&libs, b"FLA_Trmm_llc_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_llh_blk_var1: get_symbol(&libs, b"FLA_Trmm_llh_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_llh_blk_var2: get_symbol(&libs, b"FLA_Trmm_llh_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_llh_blk_var3: get_symbol(&libs, b"FLA_Trmm_llh_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_llh_blk_var4: get_symbol(&libs, b"FLA_Trmm_llh_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_llh_unb_var1: get_symbol(&libs, b"FLA_Trmm_llh_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_llh_unb_var2: get_symbol(&libs, b"FLA_Trmm_llh_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_llh_unb_var3: get_symbol(&libs, b"FLA_Trmm_llh_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_llh_unb_var4: get_symbol(&libs, b"FLA_Trmm_llh_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_lln_blk_var1: get_symbol(&libs, b"FLA_Trmm_lln_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_lln_blk_var2: get_symbol(&libs, b"FLA_Trmm_lln_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_lln_blk_var3: get_symbol(&libs, b"FLA_Trmm_lln_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_lln_blk_var4: get_symbol(&libs, b"FLA_Trmm_lln_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_lln_unb_var1: get_symbol(&libs, b"FLA_Trmm_lln_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_lln_unb_var2: get_symbol(&libs, b"FLA_Trmm_lln_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_lln_unb_var3: get_symbol(&libs, b"FLA_Trmm_lln_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_lln_unb_var4: get_symbol(&libs, b"FLA_Trmm_lln_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_llt_blk_var1: get_symbol(&libs, b"FLA_Trmm_llt_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_llt_blk_var2: get_symbol(&libs, b"FLA_Trmm_llt_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_llt_blk_var3: get_symbol(&libs, b"FLA_Trmm_llt_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_llt_blk_var4: get_symbol(&libs, b"FLA_Trmm_llt_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_llt_unb_var1: get_symbol(&libs, b"FLA_Trmm_llt_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_llt_unb_var2: get_symbol(&libs, b"FLA_Trmm_llt_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_llt_unb_var3: get_symbol(&libs, b"FLA_Trmm_llt_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_llt_unb_var4: get_symbol(&libs, b"FLA_Trmm_llt_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_luc_blk_var1: get_symbol(&libs, b"FLA_Trmm_luc_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_luc_blk_var2: get_symbol(&libs, b"FLA_Trmm_luc_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_luc_blk_var3: get_symbol(&libs, b"FLA_Trmm_luc_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_luc_blk_var4: get_symbol(&libs, b"FLA_Trmm_luc_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_luc_unb_var1: get_symbol(&libs, b"FLA_Trmm_luc_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_luc_unb_var2: get_symbol(&libs, b"FLA_Trmm_luc_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_luc_unb_var3: get_symbol(&libs, b"FLA_Trmm_luc_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_luc_unb_var4: get_symbol(&libs, b"FLA_Trmm_luc_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_luh_blk_var1: get_symbol(&libs, b"FLA_Trmm_luh_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_luh_blk_var2: get_symbol(&libs, b"FLA_Trmm_luh_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_luh_blk_var3: get_symbol(&libs, b"FLA_Trmm_luh_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_luh_blk_var4: get_symbol(&libs, b"FLA_Trmm_luh_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_luh_unb_var1: get_symbol(&libs, b"FLA_Trmm_luh_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_luh_unb_var2: get_symbol(&libs, b"FLA_Trmm_luh_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_luh_unb_var3: get_symbol(&libs, b"FLA_Trmm_luh_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_luh_unb_var4: get_symbol(&libs, b"FLA_Trmm_luh_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_lun_blk_var1: get_symbol(&libs, b"FLA_Trmm_lun_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_lun_blk_var2: get_symbol(&libs, b"FLA_Trmm_lun_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_lun_blk_var3: get_symbol(&libs, b"FLA_Trmm_lun_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_lun_blk_var4: get_symbol(&libs, b"FLA_Trmm_lun_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_lun_unb_var1: get_symbol(&libs, b"FLA_Trmm_lun_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_lun_unb_var2: get_symbol(&libs, b"FLA_Trmm_lun_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_lun_unb_var3: get_symbol(&libs, b"FLA_Trmm_lun_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_lun_unb_var4: get_symbol(&libs, b"FLA_Trmm_lun_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_lut_blk_var1: get_symbol(&libs, b"FLA_Trmm_lut_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_lut_blk_var2: get_symbol(&libs, b"FLA_Trmm_lut_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_lut_blk_var3: get_symbol(&libs, b"FLA_Trmm_lut_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_lut_blk_var4: get_symbol(&libs, b"FLA_Trmm_lut_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_lut_unb_var1: get_symbol(&libs, b"FLA_Trmm_lut_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_lut_unb_var2: get_symbol(&libs, b"FLA_Trmm_lut_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_lut_unb_var3: get_symbol(&libs, b"FLA_Trmm_lut_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_lut_unb_var4: get_symbol(&libs, b"FLA_Trmm_lut_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_rlc_blk_var1: get_symbol(&libs, b"FLA_Trmm_rlc_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_rlc_blk_var2: get_symbol(&libs, b"FLA_Trmm_rlc_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_rlc_blk_var3: get_symbol(&libs, b"FLA_Trmm_rlc_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_rlc_blk_var4: get_symbol(&libs, b"FLA_Trmm_rlc_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_rlc_unb_var1: get_symbol(&libs, b"FLA_Trmm_rlc_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_rlc_unb_var2: get_symbol(&libs, b"FLA_Trmm_rlc_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_rlc_unb_var3: get_symbol(&libs, b"FLA_Trmm_rlc_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_rlc_unb_var4: get_symbol(&libs, b"FLA_Trmm_rlc_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_rlh_blk_var1: get_symbol(&libs, b"FLA_Trmm_rlh_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_rlh_blk_var2: get_symbol(&libs, b"FLA_Trmm_rlh_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_rlh_blk_var3: get_symbol(&libs, b"FLA_Trmm_rlh_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_rlh_blk_var4: get_symbol(&libs, b"FLA_Trmm_rlh_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_rlh_unb_var1: get_symbol(&libs, b"FLA_Trmm_rlh_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_rlh_unb_var2: get_symbol(&libs, b"FLA_Trmm_rlh_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_rlh_unb_var3: get_symbol(&libs, b"FLA_Trmm_rlh_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_rlh_unb_var4: get_symbol(&libs, b"FLA_Trmm_rlh_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_rln_blk_var1: get_symbol(&libs, b"FLA_Trmm_rln_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_rln_blk_var2: get_symbol(&libs, b"FLA_Trmm_rln_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_rln_blk_var3: get_symbol(&libs, b"FLA_Trmm_rln_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_rln_blk_var4: get_symbol(&libs, b"FLA_Trmm_rln_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_rln_unb_var1: get_symbol(&libs, b"FLA_Trmm_rln_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_rln_unb_var2: get_symbol(&libs, b"FLA_Trmm_rln_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_rln_unb_var3: get_symbol(&libs, b"FLA_Trmm_rln_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_rln_unb_var4: get_symbol(&libs, b"FLA_Trmm_rln_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_rlt_blk_var1: get_symbol(&libs, b"FLA_Trmm_rlt_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_rlt_blk_var2: get_symbol(&libs, b"FLA_Trmm_rlt_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_rlt_blk_var3: get_symbol(&libs, b"FLA_Trmm_rlt_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_rlt_blk_var4: get_symbol(&libs, b"FLA_Trmm_rlt_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_rlt_unb_var1: get_symbol(&libs, b"FLA_Trmm_rlt_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_rlt_unb_var2: get_symbol(&libs, b"FLA_Trmm_rlt_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_rlt_unb_var3: get_symbol(&libs, b"FLA_Trmm_rlt_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_rlt_unb_var4: get_symbol(&libs, b"FLA_Trmm_rlt_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_ruc_blk_var1: get_symbol(&libs, b"FLA_Trmm_ruc_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_ruc_blk_var2: get_symbol(&libs, b"FLA_Trmm_ruc_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_ruc_blk_var3: get_symbol(&libs, b"FLA_Trmm_ruc_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_ruc_blk_var4: get_symbol(&libs, b"FLA_Trmm_ruc_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_ruc_unb_var1: get_symbol(&libs, b"FLA_Trmm_ruc_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_ruc_unb_var2: get_symbol(&libs, b"FLA_Trmm_ruc_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_ruc_unb_var3: get_symbol(&libs, b"FLA_Trmm_ruc_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_ruc_unb_var4: get_symbol(&libs, b"FLA_Trmm_ruc_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_ruh_blk_var1: get_symbol(&libs, b"FLA_Trmm_ruh_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_ruh_blk_var2: get_symbol(&libs, b"FLA_Trmm_ruh_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_ruh_blk_var3: get_symbol(&libs, b"FLA_Trmm_ruh_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_ruh_blk_var4: get_symbol(&libs, b"FLA_Trmm_ruh_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_ruh_unb_var1: get_symbol(&libs, b"FLA_Trmm_ruh_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_ruh_unb_var2: get_symbol(&libs, b"FLA_Trmm_ruh_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_ruh_unb_var3: get_symbol(&libs, b"FLA_Trmm_ruh_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_ruh_unb_var4: get_symbol(&libs, b"FLA_Trmm_ruh_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_run_blk_var1: get_symbol(&libs, b"FLA_Trmm_run_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_run_blk_var2: get_symbol(&libs, b"FLA_Trmm_run_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_run_blk_var3: get_symbol(&libs, b"FLA_Trmm_run_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_run_blk_var4: get_symbol(&libs, b"FLA_Trmm_run_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_run_unb_var1: get_symbol(&libs, b"FLA_Trmm_run_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_run_unb_var2: get_symbol(&libs, b"FLA_Trmm_run_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_run_unb_var3: get_symbol(&libs, b"FLA_Trmm_run_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_run_unb_var4: get_symbol(&libs, b"FLA_Trmm_run_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_rut_blk_var1: get_symbol(&libs, b"FLA_Trmm_rut_blk_var1\0").map(|sym| *sym),
            FLA_Trmm_rut_blk_var2: get_symbol(&libs, b"FLA_Trmm_rut_blk_var2\0").map(|sym| *sym),
            FLA_Trmm_rut_blk_var3: get_symbol(&libs, b"FLA_Trmm_rut_blk_var3\0").map(|sym| *sym),
            FLA_Trmm_rut_blk_var4: get_symbol(&libs, b"FLA_Trmm_rut_blk_var4\0").map(|sym| *sym),
            FLA_Trmm_rut_unb_var1: get_symbol(&libs, b"FLA_Trmm_rut_unb_var1\0").map(|sym| *sym),
            FLA_Trmm_rut_unb_var2: get_symbol(&libs, b"FLA_Trmm_rut_unb_var2\0").map(|sym| *sym),
            FLA_Trmm_rut_unb_var3: get_symbol(&libs, b"FLA_Trmm_rut_unb_var3\0").map(|sym| *sym),
            FLA_Trmm_rut_unb_var4: get_symbol(&libs, b"FLA_Trmm_rut_unb_var4\0").map(|sym| *sym),
            FLA_Trmm_internal: get_symbol(&libs, b"FLA_Trmm_internal\0").map(|sym| *sym),
            FLA_Trmm_llc: get_symbol(&libs, b"FLA_Trmm_llc\0").map(|sym| *sym),
            FLA_Trmm_llh: get_symbol(&libs, b"FLA_Trmm_llh\0").map(|sym| *sym),
            FLA_Trmm_lln: get_symbol(&libs, b"FLA_Trmm_lln\0").map(|sym| *sym),
            FLA_Trmm_llt: get_symbol(&libs, b"FLA_Trmm_llt\0").map(|sym| *sym),
            FLA_Trmm_luc: get_symbol(&libs, b"FLA_Trmm_luc\0").map(|sym| *sym),
            FLA_Trmm_luh: get_symbol(&libs, b"FLA_Trmm_luh\0").map(|sym| *sym),
            FLA_Trmm_lun: get_symbol(&libs, b"FLA_Trmm_lun\0").map(|sym| *sym),
            FLA_Trmm_lut: get_symbol(&libs, b"FLA_Trmm_lut\0").map(|sym| *sym),
            FLA_Trmm_rlc: get_symbol(&libs, b"FLA_Trmm_rlc\0").map(|sym| *sym),
            FLA_Trmm_rlh: get_symbol(&libs, b"FLA_Trmm_rlh\0").map(|sym| *sym),
            FLA_Trmm_rln: get_symbol(&libs, b"FLA_Trmm_rln\0").map(|sym| *sym),
            FLA_Trmm_rlt: get_symbol(&libs, b"FLA_Trmm_rlt\0").map(|sym| *sym),
            FLA_Trmm_ruc: get_symbol(&libs, b"FLA_Trmm_ruc\0").map(|sym| *sym),
            FLA_Trmm_ruh: get_symbol(&libs, b"FLA_Trmm_ruh\0").map(|sym| *sym),
            FLA_Trmm_run: get_symbol(&libs, b"FLA_Trmm_run\0").map(|sym| *sym),
            FLA_Trmm_rut: get_symbol(&libs, b"FLA_Trmm_rut\0").map(|sym| *sym),
            FLA_Trsm_llc_blk_var1: get_symbol(&libs, b"FLA_Trsm_llc_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_llc_blk_var2: get_symbol(&libs, b"FLA_Trsm_llc_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_llc_blk_var3: get_symbol(&libs, b"FLA_Trsm_llc_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_llc_blk_var4: get_symbol(&libs, b"FLA_Trsm_llc_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_llc_unb_var1: get_symbol(&libs, b"FLA_Trsm_llc_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_llc_unb_var2: get_symbol(&libs, b"FLA_Trsm_llc_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_llc_unb_var3: get_symbol(&libs, b"FLA_Trsm_llc_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_llc_unb_var4: get_symbol(&libs, b"FLA_Trsm_llc_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_llh_blk_var1: get_symbol(&libs, b"FLA_Trsm_llh_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_llh_blk_var2: get_symbol(&libs, b"FLA_Trsm_llh_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_llh_blk_var3: get_symbol(&libs, b"FLA_Trsm_llh_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_llh_blk_var4: get_symbol(&libs, b"FLA_Trsm_llh_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_llh_unb_var1: get_symbol(&libs, b"FLA_Trsm_llh_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_llh_unb_var2: get_symbol(&libs, b"FLA_Trsm_llh_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_llh_unb_var3: get_symbol(&libs, b"FLA_Trsm_llh_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_llh_unb_var4: get_symbol(&libs, b"FLA_Trsm_llh_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_lln_blk_var1: get_symbol(&libs, b"FLA_Trsm_lln_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_lln_blk_var2: get_symbol(&libs, b"FLA_Trsm_lln_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_lln_blk_var3: get_symbol(&libs, b"FLA_Trsm_lln_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_lln_blk_var4: get_symbol(&libs, b"FLA_Trsm_lln_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_lln_unb_var1: get_symbol(&libs, b"FLA_Trsm_lln_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_lln_unb_var2: get_symbol(&libs, b"FLA_Trsm_lln_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_lln_unb_var3: get_symbol(&libs, b"FLA_Trsm_lln_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_lln_unb_var4: get_symbol(&libs, b"FLA_Trsm_lln_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_llt_blk_var1: get_symbol(&libs, b"FLA_Trsm_llt_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_llt_blk_var2: get_symbol(&libs, b"FLA_Trsm_llt_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_llt_blk_var3: get_symbol(&libs, b"FLA_Trsm_llt_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_llt_blk_var4: get_symbol(&libs, b"FLA_Trsm_llt_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_llt_unb_var1: get_symbol(&libs, b"FLA_Trsm_llt_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_llt_unb_var2: get_symbol(&libs, b"FLA_Trsm_llt_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_llt_unb_var3: get_symbol(&libs, b"FLA_Trsm_llt_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_llt_unb_var4: get_symbol(&libs, b"FLA_Trsm_llt_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_luc_blk_var1: get_symbol(&libs, b"FLA_Trsm_luc_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_luc_blk_var2: get_symbol(&libs, b"FLA_Trsm_luc_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_luc_blk_var3: get_symbol(&libs, b"FLA_Trsm_luc_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_luc_blk_var4: get_symbol(&libs, b"FLA_Trsm_luc_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_luc_unb_var1: get_symbol(&libs, b"FLA_Trsm_luc_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_luc_unb_var2: get_symbol(&libs, b"FLA_Trsm_luc_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_luc_unb_var3: get_symbol(&libs, b"FLA_Trsm_luc_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_luc_unb_var4: get_symbol(&libs, b"FLA_Trsm_luc_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_luh_blk_var1: get_symbol(&libs, b"FLA_Trsm_luh_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_luh_blk_var2: get_symbol(&libs, b"FLA_Trsm_luh_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_luh_blk_var3: get_symbol(&libs, b"FLA_Trsm_luh_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_luh_blk_var4: get_symbol(&libs, b"FLA_Trsm_luh_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_luh_unb_var1: get_symbol(&libs, b"FLA_Trsm_luh_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_luh_unb_var2: get_symbol(&libs, b"FLA_Trsm_luh_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_luh_unb_var3: get_symbol(&libs, b"FLA_Trsm_luh_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_luh_unb_var4: get_symbol(&libs, b"FLA_Trsm_luh_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_lun_blk_var1: get_symbol(&libs, b"FLA_Trsm_lun_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_lun_blk_var2: get_symbol(&libs, b"FLA_Trsm_lun_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_lun_blk_var3: get_symbol(&libs, b"FLA_Trsm_lun_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_lun_blk_var4: get_symbol(&libs, b"FLA_Trsm_lun_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_lun_unb_var1: get_symbol(&libs, b"FLA_Trsm_lun_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_lun_unb_var2: get_symbol(&libs, b"FLA_Trsm_lun_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_lun_unb_var3: get_symbol(&libs, b"FLA_Trsm_lun_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_lun_unb_var4: get_symbol(&libs, b"FLA_Trsm_lun_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_lut_blk_var1: get_symbol(&libs, b"FLA_Trsm_lut_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_lut_blk_var2: get_symbol(&libs, b"FLA_Trsm_lut_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_lut_blk_var3: get_symbol(&libs, b"FLA_Trsm_lut_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_lut_blk_var4: get_symbol(&libs, b"FLA_Trsm_lut_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_lut_unb_var1: get_symbol(&libs, b"FLA_Trsm_lut_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_lut_unb_var2: get_symbol(&libs, b"FLA_Trsm_lut_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_lut_unb_var3: get_symbol(&libs, b"FLA_Trsm_lut_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_lut_unb_var4: get_symbol(&libs, b"FLA_Trsm_lut_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_rlc_blk_var1: get_symbol(&libs, b"FLA_Trsm_rlc_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_rlc_blk_var2: get_symbol(&libs, b"FLA_Trsm_rlc_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_rlc_blk_var3: get_symbol(&libs, b"FLA_Trsm_rlc_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_rlc_blk_var4: get_symbol(&libs, b"FLA_Trsm_rlc_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_rlc_unb_var1: get_symbol(&libs, b"FLA_Trsm_rlc_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_rlc_unb_var2: get_symbol(&libs, b"FLA_Trsm_rlc_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_rlc_unb_var3: get_symbol(&libs, b"FLA_Trsm_rlc_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_rlc_unb_var4: get_symbol(&libs, b"FLA_Trsm_rlc_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_rlh_blk_var1: get_symbol(&libs, b"FLA_Trsm_rlh_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_rlh_blk_var2: get_symbol(&libs, b"FLA_Trsm_rlh_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_rlh_blk_var3: get_symbol(&libs, b"FLA_Trsm_rlh_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_rlh_blk_var4: get_symbol(&libs, b"FLA_Trsm_rlh_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_rlh_unb_var1: get_symbol(&libs, b"FLA_Trsm_rlh_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_rlh_unb_var2: get_symbol(&libs, b"FLA_Trsm_rlh_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_rlh_unb_var3: get_symbol(&libs, b"FLA_Trsm_rlh_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_rlh_unb_var4: get_symbol(&libs, b"FLA_Trsm_rlh_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_rln_blk_var1: get_symbol(&libs, b"FLA_Trsm_rln_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_rln_blk_var2: get_symbol(&libs, b"FLA_Trsm_rln_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_rln_blk_var3: get_symbol(&libs, b"FLA_Trsm_rln_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_rln_blk_var4: get_symbol(&libs, b"FLA_Trsm_rln_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_rln_unb_var1: get_symbol(&libs, b"FLA_Trsm_rln_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_rln_unb_var2: get_symbol(&libs, b"FLA_Trsm_rln_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_rln_unb_var3: get_symbol(&libs, b"FLA_Trsm_rln_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_rln_unb_var4: get_symbol(&libs, b"FLA_Trsm_rln_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_rlt_blk_var1: get_symbol(&libs, b"FLA_Trsm_rlt_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_rlt_blk_var2: get_symbol(&libs, b"FLA_Trsm_rlt_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_rlt_blk_var3: get_symbol(&libs, b"FLA_Trsm_rlt_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_rlt_blk_var4: get_symbol(&libs, b"FLA_Trsm_rlt_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_rlt_unb_var1: get_symbol(&libs, b"FLA_Trsm_rlt_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_rlt_unb_var2: get_symbol(&libs, b"FLA_Trsm_rlt_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_rlt_unb_var3: get_symbol(&libs, b"FLA_Trsm_rlt_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_rlt_unb_var4: get_symbol(&libs, b"FLA_Trsm_rlt_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_ruc_blk_var1: get_symbol(&libs, b"FLA_Trsm_ruc_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_ruc_blk_var2: get_symbol(&libs, b"FLA_Trsm_ruc_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_ruc_blk_var3: get_symbol(&libs, b"FLA_Trsm_ruc_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_ruc_blk_var4: get_symbol(&libs, b"FLA_Trsm_ruc_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_ruc_unb_var1: get_symbol(&libs, b"FLA_Trsm_ruc_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_ruc_unb_var2: get_symbol(&libs, b"FLA_Trsm_ruc_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_ruc_unb_var3: get_symbol(&libs, b"FLA_Trsm_ruc_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_ruc_unb_var4: get_symbol(&libs, b"FLA_Trsm_ruc_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_ruh_blk_var1: get_symbol(&libs, b"FLA_Trsm_ruh_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_ruh_blk_var2: get_symbol(&libs, b"FLA_Trsm_ruh_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_ruh_blk_var3: get_symbol(&libs, b"FLA_Trsm_ruh_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_ruh_blk_var4: get_symbol(&libs, b"FLA_Trsm_ruh_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_ruh_unb_var1: get_symbol(&libs, b"FLA_Trsm_ruh_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_ruh_unb_var2: get_symbol(&libs, b"FLA_Trsm_ruh_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_ruh_unb_var3: get_symbol(&libs, b"FLA_Trsm_ruh_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_ruh_unb_var4: get_symbol(&libs, b"FLA_Trsm_ruh_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_run_blk_var1: get_symbol(&libs, b"FLA_Trsm_run_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_run_blk_var2: get_symbol(&libs, b"FLA_Trsm_run_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_run_blk_var3: get_symbol(&libs, b"FLA_Trsm_run_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_run_blk_var4: get_symbol(&libs, b"FLA_Trsm_run_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_run_unb_var1: get_symbol(&libs, b"FLA_Trsm_run_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_run_unb_var2: get_symbol(&libs, b"FLA_Trsm_run_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_run_unb_var3: get_symbol(&libs, b"FLA_Trsm_run_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_run_unb_var4: get_symbol(&libs, b"FLA_Trsm_run_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_rut_blk_var1: get_symbol(&libs, b"FLA_Trsm_rut_blk_var1\0").map(|sym| *sym),
            FLA_Trsm_rut_blk_var2: get_symbol(&libs, b"FLA_Trsm_rut_blk_var2\0").map(|sym| *sym),
            FLA_Trsm_rut_blk_var3: get_symbol(&libs, b"FLA_Trsm_rut_blk_var3\0").map(|sym| *sym),
            FLA_Trsm_rut_blk_var4: get_symbol(&libs, b"FLA_Trsm_rut_blk_var4\0").map(|sym| *sym),
            FLA_Trsm_rut_unb_var1: get_symbol(&libs, b"FLA_Trsm_rut_unb_var1\0").map(|sym| *sym),
            FLA_Trsm_rut_unb_var2: get_symbol(&libs, b"FLA_Trsm_rut_unb_var2\0").map(|sym| *sym),
            FLA_Trsm_rut_unb_var3: get_symbol(&libs, b"FLA_Trsm_rut_unb_var3\0").map(|sym| *sym),
            FLA_Trsm_rut_unb_var4: get_symbol(&libs, b"FLA_Trsm_rut_unb_var4\0").map(|sym| *sym),
            FLA_Trsm_internal: get_symbol(&libs, b"FLA_Trsm_internal\0").map(|sym| *sym),
            FLA_Trsm_llc: get_symbol(&libs, b"FLA_Trsm_llc\0").map(|sym| *sym),
            FLA_Trsm_llh: get_symbol(&libs, b"FLA_Trsm_llh\0").map(|sym| *sym),
            FLA_Trsm_lln: get_symbol(&libs, b"FLA_Trsm_lln\0").map(|sym| *sym),
            FLA_Trsm_llt: get_symbol(&libs, b"FLA_Trsm_llt\0").map(|sym| *sym),
            FLA_Trsm_luc: get_symbol(&libs, b"FLA_Trsm_luc\0").map(|sym| *sym),
            FLA_Trsm_luh: get_symbol(&libs, b"FLA_Trsm_luh\0").map(|sym| *sym),
            FLA_Trsm_lun: get_symbol(&libs, b"FLA_Trsm_lun\0").map(|sym| *sym),
            FLA_Trsm_lut: get_symbol(&libs, b"FLA_Trsm_lut\0").map(|sym| *sym),
            FLA_Trsm_rlc: get_symbol(&libs, b"FLA_Trsm_rlc\0").map(|sym| *sym),
            FLA_Trsm_rlh: get_symbol(&libs, b"FLA_Trsm_rlh\0").map(|sym| *sym),
            FLA_Trsm_rln: get_symbol(&libs, b"FLA_Trsm_rln\0").map(|sym| *sym),
            FLA_Trsm_rlt: get_symbol(&libs, b"FLA_Trsm_rlt\0").map(|sym| *sym),
            FLA_Trsm_ruc: get_symbol(&libs, b"FLA_Trsm_ruc\0").map(|sym| *sym),
            FLA_Trsm_ruh: get_symbol(&libs, b"FLA_Trsm_ruh\0").map(|sym| *sym),
            FLA_Trsm_run: get_symbol(&libs, b"FLA_Trsm_run\0").map(|sym| *sym),
            FLA_Trsm_rut: get_symbol(&libs, b"FLA_Trsm_rut\0").map(|sym| *sym),
            FLA_Chol_l_blk_var1: get_symbol(&libs, b"FLA_Chol_l_blk_var1\0").map(|sym| *sym),
            FLA_Chol_l_blk_var2: get_symbol(&libs, b"FLA_Chol_l_blk_var2\0").map(|sym| *sym),
            FLA_Chol_l_blk_var3: get_symbol(&libs, b"FLA_Chol_l_blk_var3\0").map(|sym| *sym),
            FLA_Chol_l_unb_var1: get_symbol(&libs, b"FLA_Chol_l_unb_var1\0").map(|sym| *sym),
            FLA_Chol_l_unb_var2: get_symbol(&libs, b"FLA_Chol_l_unb_var2\0").map(|sym| *sym),
            FLA_Chol_l_unb_var3: get_symbol(&libs, b"FLA_Chol_l_unb_var3\0").map(|sym| *sym),
            FLA_Chol_l_opt_var1: get_symbol(&libs, b"FLA_Chol_l_opt_var1\0").map(|sym| *sym),
            FLA_Chol_l_ops_var1: get_symbol(&libs, b"FLA_Chol_l_ops_var1\0").map(|sym| *sym),
            FLA_Chol_l_opd_var1: get_symbol(&libs, b"FLA_Chol_l_opd_var1\0").map(|sym| *sym),
            FLA_Chol_l_opc_var1: get_symbol(&libs, b"FLA_Chol_l_opc_var1\0").map(|sym| *sym),
            FLA_Chol_l_opz_var1: get_symbol(&libs, b"FLA_Chol_l_opz_var1\0").map(|sym| *sym),
            FLA_Chol_l_opt_var2: get_symbol(&libs, b"FLA_Chol_l_opt_var2\0").map(|sym| *sym),
            FLA_Chol_l_ops_var2: get_symbol(&libs, b"FLA_Chol_l_ops_var2\0").map(|sym| *sym),
            FLA_Chol_l_opd_var2: get_symbol(&libs, b"FLA_Chol_l_opd_var2\0").map(|sym| *sym),
            FLA_Chol_l_opc_var2: get_symbol(&libs, b"FLA_Chol_l_opc_var2\0").map(|sym| *sym),
            FLA_Chol_l_opz_var2: get_symbol(&libs, b"FLA_Chol_l_opz_var2\0").map(|sym| *sym),
            FLA_Chol_l_opt_var3: get_symbol(&libs, b"FLA_Chol_l_opt_var3\0").map(|sym| *sym),
            FLA_Chol_l_ops_var3: get_symbol(&libs, b"FLA_Chol_l_ops_var3\0").map(|sym| *sym),
            FLA_Chol_l_opd_var3: get_symbol(&libs, b"FLA_Chol_l_opd_var3\0").map(|sym| *sym),
            FLA_Chol_l_opc_var3: get_symbol(&libs, b"FLA_Chol_l_opc_var3\0").map(|sym| *sym),
            FLA_Chol_l_opz_var3: get_symbol(&libs, b"FLA_Chol_l_opz_var3\0").map(|sym| *sym),
            FLA_Chol_u_blk_var1: get_symbol(&libs, b"FLA_Chol_u_blk_var1\0").map(|sym| *sym),
            FLA_Chol_u_blk_var2: get_symbol(&libs, b"FLA_Chol_u_blk_var2\0").map(|sym| *sym),
            FLA_Chol_u_blk_var3: get_symbol(&libs, b"FLA_Chol_u_blk_var3\0").map(|sym| *sym),
            FLA_Chol_u_unb_var1: get_symbol(&libs, b"FLA_Chol_u_unb_var1\0").map(|sym| *sym),
            FLA_Chol_u_unb_var2: get_symbol(&libs, b"FLA_Chol_u_unb_var2\0").map(|sym| *sym),
            FLA_Chol_u_unb_var3: get_symbol(&libs, b"FLA_Chol_u_unb_var3\0").map(|sym| *sym),
            FLA_Chol_u_opt_var1: get_symbol(&libs, b"FLA_Chol_u_opt_var1\0").map(|sym| *sym),
            FLA_Chol_u_ops_var1: get_symbol(&libs, b"FLA_Chol_u_ops_var1\0").map(|sym| *sym),
            FLA_Chol_u_opd_var1: get_symbol(&libs, b"FLA_Chol_u_opd_var1\0").map(|sym| *sym),
            FLA_Chol_u_opc_var1: get_symbol(&libs, b"FLA_Chol_u_opc_var1\0").map(|sym| *sym),
            FLA_Chol_u_opz_var1: get_symbol(&libs, b"FLA_Chol_u_opz_var1\0").map(|sym| *sym),
            FLA_Chol_u_opt_var2: get_symbol(&libs, b"FLA_Chol_u_opt_var2\0").map(|sym| *sym),
            FLA_Chol_u_ops_var2: get_symbol(&libs, b"FLA_Chol_u_ops_var2\0").map(|sym| *sym),
            FLA_Chol_u_opd_var2: get_symbol(&libs, b"FLA_Chol_u_opd_var2\0").map(|sym| *sym),
            FLA_Chol_u_opc_var2: get_symbol(&libs, b"FLA_Chol_u_opc_var2\0").map(|sym| *sym),
            FLA_Chol_u_opz_var2: get_symbol(&libs, b"FLA_Chol_u_opz_var2\0").map(|sym| *sym),
            FLA_Chol_u_opt_var3: get_symbol(&libs, b"FLA_Chol_u_opt_var3\0").map(|sym| *sym),
            FLA_Chol_u_ops_var3: get_symbol(&libs, b"FLA_Chol_u_ops_var3\0").map(|sym| *sym),
            FLA_Chol_u_opd_var3: get_symbol(&libs, b"FLA_Chol_u_opd_var3\0").map(|sym| *sym),
            FLA_Chol_u_opc_var3: get_symbol(&libs, b"FLA_Chol_u_opc_var3\0").map(|sym| *sym),
            FLA_Chol_u_opz_var3: get_symbol(&libs, b"FLA_Chol_u_opz_var3\0").map(|sym| *sym),
            FLA_Chol_internal: get_symbol(&libs, b"FLA_Chol_internal\0").map(|sym| *sym),
            FLA_Chol_l: get_symbol(&libs, b"FLA_Chol_l\0").map(|sym| *sym),
            FLA_Chol_u: get_symbol(&libs, b"FLA_Chol_u\0").map(|sym| *sym),
            FLA_Chol_solve: get_symbol(&libs, b"FLA_Chol_solve\0").map(|sym| *sym),
            FLASH_Chol_solve: get_symbol(&libs, b"FLASH_Chol_solve\0").map(|sym| *sym),
            FLA_LU_nopiv_blk_var1: get_symbol(&libs, b"FLA_LU_nopiv_blk_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_blk_var2: get_symbol(&libs, b"FLA_LU_nopiv_blk_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_blk_var3: get_symbol(&libs, b"FLA_LU_nopiv_blk_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_blk_var4: get_symbol(&libs, b"FLA_LU_nopiv_blk_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_blk_var5: get_symbol(&libs, b"FLA_LU_nopiv_blk_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_unb_var1: get_symbol(&libs, b"FLA_LU_nopiv_unb_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_unb_var2: get_symbol(&libs, b"FLA_LU_nopiv_unb_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_unb_var3: get_symbol(&libs, b"FLA_LU_nopiv_unb_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_unb_var4: get_symbol(&libs, b"FLA_LU_nopiv_unb_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_unb_var5: get_symbol(&libs, b"FLA_LU_nopiv_unb_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_opt_var1: get_symbol(&libs, b"FLA_LU_nopiv_opt_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_ops_var1: get_symbol(&libs, b"FLA_LU_nopiv_ops_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_opd_var1: get_symbol(&libs, b"FLA_LU_nopiv_opd_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_opc_var1: get_symbol(&libs, b"FLA_LU_nopiv_opc_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_opz_var1: get_symbol(&libs, b"FLA_LU_nopiv_opz_var1\0").map(|sym| *sym),
            FLA_LU_nopiv_opt_var2: get_symbol(&libs, b"FLA_LU_nopiv_opt_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_ops_var2: get_symbol(&libs, b"FLA_LU_nopiv_ops_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_opd_var2: get_symbol(&libs, b"FLA_LU_nopiv_opd_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_opc_var2: get_symbol(&libs, b"FLA_LU_nopiv_opc_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_opz_var2: get_symbol(&libs, b"FLA_LU_nopiv_opz_var2\0").map(|sym| *sym),
            FLA_LU_nopiv_opt_var3: get_symbol(&libs, b"FLA_LU_nopiv_opt_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_ops_var3: get_symbol(&libs, b"FLA_LU_nopiv_ops_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_opd_var3: get_symbol(&libs, b"FLA_LU_nopiv_opd_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_opc_var3: get_symbol(&libs, b"FLA_LU_nopiv_opc_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_opz_var3: get_symbol(&libs, b"FLA_LU_nopiv_opz_var3\0").map(|sym| *sym),
            FLA_LU_nopiv_opt_var4: get_symbol(&libs, b"FLA_LU_nopiv_opt_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_ops_var4: get_symbol(&libs, b"FLA_LU_nopiv_ops_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_opd_var4: get_symbol(&libs, b"FLA_LU_nopiv_opd_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_opc_var4: get_symbol(&libs, b"FLA_LU_nopiv_opc_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_opz_var4: get_symbol(&libs, b"FLA_LU_nopiv_opz_var4\0").map(|sym| *sym),
            FLA_LU_nopiv_opt_var5: get_symbol(&libs, b"FLA_LU_nopiv_opt_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_ops_var5: get_symbol(&libs, b"FLA_LU_nopiv_ops_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_opd_var5: get_symbol(&libs, b"FLA_LU_nopiv_opd_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_opc_var5: get_symbol(&libs, b"FLA_LU_nopiv_opc_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_opz_var5: get_symbol(&libs, b"FLA_LU_nopiv_opz_var5\0").map(|sym| *sym),
            FLA_LU_nopiv_internal: get_symbol(&libs, b"FLA_LU_nopiv_internal\0").map(|sym| *sym),
            FLA_LU_nopiv_solve: get_symbol(&libs, b"FLA_LU_nopiv_solve\0").map(|sym| *sym),
            FLASH_LU_nopiv_solve: get_symbol(&libs, b"FLASH_LU_nopiv_solve\0").map(|sym| *sym),
            FLA_LU_piv_blk_var3: get_symbol(&libs, b"FLA_LU_piv_blk_var3\0").map(|sym| *sym),
            FLA_LU_piv_blk_var4: get_symbol(&libs, b"FLA_LU_piv_blk_var4\0").map(|sym| *sym),
            FLA_LU_piv_blk_var5: get_symbol(&libs, b"FLA_LU_piv_blk_var5\0").map(|sym| *sym),
            FLA_LU_piv_unb_var3: get_symbol(&libs, b"FLA_LU_piv_unb_var3\0").map(|sym| *sym),
            FLA_LU_piv_unb_var3b: get_symbol(&libs, b"FLA_LU_piv_unb_var3b\0").map(|sym| *sym),
            FLA_LU_piv_unb_var4: get_symbol(&libs, b"FLA_LU_piv_unb_var4\0").map(|sym| *sym),
            FLA_LU_piv_unb_var5: get_symbol(&libs, b"FLA_LU_piv_unb_var5\0").map(|sym| *sym),
            FLA_LU_piv_opt_var3: get_symbol(&libs, b"FLA_LU_piv_opt_var3\0").map(|sym| *sym),
            FLA_LU_piv_ops_var3: get_symbol(&libs, b"FLA_LU_piv_ops_var3\0").map(|sym| *sym),
            FLA_LU_piv_opd_var3: get_symbol(&libs, b"FLA_LU_piv_opd_var3\0").map(|sym| *sym),
            FLA_LU_piv_opc_var3: get_symbol(&libs, b"FLA_LU_piv_opc_var3\0").map(|sym| *sym),
            FLA_LU_piv_opz_var3: get_symbol(&libs, b"FLA_LU_piv_opz_var3\0").map(|sym| *sym),
            FLA_LU_piv_opt_var4: get_symbol(&libs, b"FLA_LU_piv_opt_var4\0").map(|sym| *sym),
            FLA_LU_piv_ops_var4: get_symbol(&libs, b"FLA_LU_piv_ops_var4\0").map(|sym| *sym),
            FLA_LU_piv_opd_var4: get_symbol(&libs, b"FLA_LU_piv_opd_var4\0").map(|sym| *sym),
            FLA_LU_piv_opc_var4: get_symbol(&libs, b"FLA_LU_piv_opc_var4\0").map(|sym| *sym),
            FLA_LU_piv_opz_var4: get_symbol(&libs, b"FLA_LU_piv_opz_var4\0").map(|sym| *sym),
            FLA_LU_piv_opt_var5: get_symbol(&libs, b"FLA_LU_piv_opt_var5\0").map(|sym| *sym),
            FLA_LU_piv_ops_var5: get_symbol(&libs, b"FLA_LU_piv_ops_var5\0").map(|sym| *sym),
            FLA_LU_piv_opd_var5: get_symbol(&libs, b"FLA_LU_piv_opd_var5\0").map(|sym| *sym),
            FLA_LU_piv_opc_var5: get_symbol(&libs, b"FLA_LU_piv_opc_var5\0").map(|sym| *sym),
            FLA_LU_piv_opz_var5: get_symbol(&libs, b"FLA_LU_piv_opz_var5\0").map(|sym| *sym),
            FLA_LU_piv_internal: get_symbol(&libs, b"FLA_LU_piv_internal\0").map(|sym| *sym),
            FLA_LU_piv_solve: get_symbol(&libs, b"FLA_LU_piv_solve\0").map(|sym| *sym),
            FLASH_LU_piv_solve: get_symbol(&libs, b"FLASH_LU_piv_solve\0").map(|sym| *sym),
            FLA_SA_Apply_pivots: get_symbol(&libs, b"FLA_SA_Apply_pivots\0").map(|sym| *sym),
            FLA_SA_LU_blk: get_symbol(&libs, b"FLA_SA_LU_blk\0").map(|sym| *sym),
            FLA_SA_LU_unb: get_symbol(&libs, b"FLA_SA_LU_unb\0").map(|sym| *sym),
            FLA_SA_FS_blk: get_symbol(&libs, b"FLA_SA_FS_blk\0").map(|sym| *sym),
            FLASH_LU_incpiv_var1: get_symbol(&libs, b"FLASH_LU_incpiv_var1\0").map(|sym| *sym),
            FLASH_LU_incpiv_var2: get_symbol(&libs, b"FLASH_LU_incpiv_var2\0").map(|sym| *sym),
            FLASH_Trsm_piv: get_symbol(&libs, b"FLASH_Trsm_piv\0").map(|sym| *sym),
            FLASH_SA_LU: get_symbol(&libs, b"FLASH_SA_LU\0").map(|sym| *sym),
            FLASH_SA_FS: get_symbol(&libs, b"FLASH_SA_FS\0").map(|sym| *sym),
            FLASH_FS_incpiv_aux1: get_symbol(&libs, b"FLASH_FS_incpiv_aux1\0").map(|sym| *sym),
            FLASH_FS_incpiv_aux2: get_symbol(&libs, b"FLASH_FS_incpiv_aux2\0").map(|sym| *sym),
            FLASH_LU_incpiv_create_hier_matrices: get_symbol(
                &libs,
                b"FLASH_LU_incpiv_create_hier_matrices\0",
            )
            .map(|sym| *sym),
            FLASH_LU_incpiv_determine_alg_blocksize: get_symbol(
                &libs,
                b"FLASH_LU_incpiv_determine_alg_blocksize\0",
            )
            .map(|sym| *sym),
            FLASH_LU_incpiv_noopt: get_symbol(&libs, b"FLASH_LU_incpiv_noopt\0").map(|sym| *sym),
            FLASH_LU_incpiv_opt1: get_symbol(&libs, b"FLASH_LU_incpiv_opt1\0").map(|sym| *sym),
            FLASH_LU_incpiv_solve: get_symbol(&libs, b"FLASH_LU_incpiv_solve\0").map(|sym| *sym),
            FLA_QR_UT_unb_var1: get_symbol(&libs, b"FLA_QR_UT_unb_var1\0").map(|sym| *sym),
            FLA_QR_UT_blk_var1: get_symbol(&libs, b"FLA_QR_UT_blk_var1\0").map(|sym| *sym),
            FLA_QR_UT_opt_var1: get_symbol(&libs, b"FLA_QR_UT_opt_var1\0").map(|sym| *sym),
            FLA_QR_UT_ops_var1: get_symbol(&libs, b"FLA_QR_UT_ops_var1\0").map(|sym| *sym),
            FLA_QR_UT_opd_var1: get_symbol(&libs, b"FLA_QR_UT_opd_var1\0").map(|sym| *sym),
            FLA_QR_UT_opc_var1: get_symbol(&libs, b"FLA_QR_UT_opc_var1\0").map(|sym| *sym),
            FLA_QR_UT_opz_var1: get_symbol(&libs, b"FLA_QR_UT_opz_var1\0").map(|sym| *sym),
            FLA_QR_UT_unb_var2: get_symbol(&libs, b"FLA_QR_UT_unb_var2\0").map(|sym| *sym),
            FLA_QR_UT_blk_var2: get_symbol(&libs, b"FLA_QR_UT_blk_var2\0").map(|sym| *sym),
            FLA_QR_UT_opt_var2: get_symbol(&libs, b"FLA_QR_UT_opt_var2\0").map(|sym| *sym),
            FLA_QR_UT_ops_var2: get_symbol(&libs, b"FLA_QR_UT_ops_var2\0").map(|sym| *sym),
            FLA_QR_UT_opd_var2: get_symbol(&libs, b"FLA_QR_UT_opd_var2\0").map(|sym| *sym),
            FLA_QR_UT_opc_var2: get_symbol(&libs, b"FLA_QR_UT_opc_var2\0").map(|sym| *sym),
            FLA_QR_UT_opz_var2: get_symbol(&libs, b"FLA_QR_UT_opz_var2\0").map(|sym| *sym),
            FLA_QR_UT_blk_var3: get_symbol(&libs, b"FLA_QR_UT_blk_var3\0").map(|sym| *sym),
            FLA_QR_UT_internal: get_symbol(&libs, b"FLA_QR_UT_internal\0").map(|sym| *sym),
            FLA_QR_UT_copy_internal: get_symbol(&libs, b"FLA_QR_UT_copy_internal\0")
                .map(|sym| *sym),
            FLA_QR_UT_create_T: get_symbol(&libs, b"FLA_QR_UT_create_T\0").map(|sym| *sym),
            FLA_QR_UT_recover_tau: get_symbol(&libs, b"FLA_QR_UT_recover_tau\0").map(|sym| *sym),
            FLA_QR_UT_solve: get_symbol(&libs, b"FLA_QR_UT_solve\0").map(|sym| *sym),
            FLASH_QR_UT: get_symbol(&libs, b"FLASH_QR_UT\0").map(|sym| *sym),
            FLASH_QR_UT_create_hier_matrices: get_symbol(
                &libs,
                b"FLASH_QR_UT_create_hier_matrices\0",
            )
            .map(|sym| *sym),
            FLASH_QR_UT_solve: get_symbol(&libs, b"FLASH_QR_UT_solve\0").map(|sym| *sym),
            FLA_QR_UT_form_Q: get_symbol(&libs, b"FLA_QR_UT_form_Q\0").map(|sym| *sym),
            FLA_QR_UT_form_Q_blk_var1: get_symbol(&libs, b"FLA_QR_UT_form_Q_blk_var1\0")
                .map(|sym| *sym),
            FLA_QR_UT_form_Q_opt_var1: get_symbol(&libs, b"FLA_QR_UT_form_Q_opt_var1\0")
                .map(|sym| *sym),
            FLA_QR_UT_form_Q_ops_var1: get_symbol(&libs, b"FLA_QR_UT_form_Q_ops_var1\0")
                .map(|sym| *sym),
            FLA_QR_UT_form_Q_opd_var1: get_symbol(&libs, b"FLA_QR_UT_form_Q_opd_var1\0")
                .map(|sym| *sym),
            FLA_QR_UT_form_Q_opc_var1: get_symbol(&libs, b"FLA_QR_UT_form_Q_opc_var1\0")
                .map(|sym| *sym),
            FLA_QR_UT_form_Q_opz_var1: get_symbol(&libs, b"FLA_QR_UT_form_Q_opz_var1\0")
                .map(|sym| *sym),
            FLA_QR_UT_piv_unb_var1: get_symbol(&libs, b"FLA_QR_UT_piv_unb_var1\0").map(|sym| *sym),
            FLA_QR_UT_piv_blk_var1: get_symbol(&libs, b"FLA_QR_UT_piv_blk_var1\0").map(|sym| *sym),
            FLA_QR_UT_piv_unb_var2: get_symbol(&libs, b"FLA_QR_UT_piv_unb_var2\0").map(|sym| *sym),
            FLA_QR_UT_piv_blk_var2: get_symbol(&libs, b"FLA_QR_UT_piv_blk_var2\0").map(|sym| *sym),
            FLA_Apply_H2_UT_piv_row: get_symbol(&libs, b"FLA_Apply_H2_UT_piv_row\0")
                .map(|sym| *sym),
            FLA_QR_UT_piv_internal: get_symbol(&libs, b"FLA_QR_UT_piv_internal\0").map(|sym| *sym),
            FLA_QR_UT_piv_colnorm: get_symbol(&libs, b"FLA_QR_UT_piv_colnorm\0").map(|sym| *sym),
            FLA_QR_UT_piv_check: get_symbol(&libs, b"FLA_QR_UT_piv_check\0").map(|sym| *sym),
            FLA_QR_UT_piv_internal_check: get_symbol(&libs, b"FLA_QR_UT_piv_internal_check\0")
                .map(|sym| *sym),
            FLA_QR_UT_piv_colnorm_check: get_symbol(&libs, b"FLA_QR_UT_piv_colnorm_check\0")
                .map(|sym| *sym),
            FLA_QR2_UT_blk_var1: get_symbol(&libs, b"FLA_QR2_UT_blk_var1\0").map(|sym| *sym),
            FLA_QR2_UT_blk_var2: get_symbol(&libs, b"FLA_QR2_UT_blk_var2\0").map(|sym| *sym),
            FLA_QR2_UT_unb_var1: get_symbol(&libs, b"FLA_QR2_UT_unb_var1\0").map(|sym| *sym),
            FLA_QR2_UT_opt_var1: get_symbol(&libs, b"FLA_QR2_UT_opt_var1\0").map(|sym| *sym),
            FLA_QR2_UT_ops_var1: get_symbol(&libs, b"FLA_QR2_UT_ops_var1\0").map(|sym| *sym),
            FLA_QR2_UT_opd_var1: get_symbol(&libs, b"FLA_QR2_UT_opd_var1\0").map(|sym| *sym),
            FLA_QR2_UT_opc_var1: get_symbol(&libs, b"FLA_QR2_UT_opc_var1\0").map(|sym| *sym),
            FLA_QR2_UT_opz_var1: get_symbol(&libs, b"FLA_QR2_UT_opz_var1\0").map(|sym| *sym),
            FLASH_QR2_UT: get_symbol(&libs, b"FLASH_QR2_UT\0").map(|sym| *sym),
            FLA_QR2_UT_internal: get_symbol(&libs, b"FLA_QR2_UT_internal\0").map(|sym| *sym),
            FLASH_QR_UT_inc: get_symbol(&libs, b"FLASH_QR_UT_inc\0").map(|sym| *sym),
            FLASH_QR_UT_inc_noopt: get_symbol(&libs, b"FLASH_QR_UT_inc_noopt\0").map(|sym| *sym),
            FLASH_QR_UT_inc_opt1: get_symbol(&libs, b"FLASH_QR_UT_inc_opt1\0").map(|sym| *sym),
            FLA_QR_UT_inc_blk_var1: get_symbol(&libs, b"FLA_QR_UT_inc_blk_var1\0").map(|sym| *sym),
            FLA_QR_UT_inc_blk_var2: get_symbol(&libs, b"FLA_QR_UT_inc_blk_var2\0").map(|sym| *sym),
            FLASH_QR_UT_inc_create_hier_matrices: get_symbol(
                &libs,
                b"FLASH_QR_UT_inc_create_hier_matrices\0",
            )
            .map(|sym| *sym),
            FLASH_QR_UT_inc_determine_alg_blocksize: get_symbol(
                &libs,
                b"FLASH_QR_UT_inc_determine_alg_blocksize\0",
            )
            .map(|sym| *sym),
            FLASH_QR_UT_inc_solve: get_symbol(&libs, b"FLASH_QR_UT_inc_solve\0").map(|sym| *sym),
            FLA_LQ_UT_unb_var1: get_symbol(&libs, b"FLA_LQ_UT_unb_var1\0").map(|sym| *sym),
            FLA_LQ_UT_blk_var1: get_symbol(&libs, b"FLA_LQ_UT_blk_var1\0").map(|sym| *sym),
            FLA_LQ_UT_opt_var1: get_symbol(&libs, b"FLA_LQ_UT_opt_var1\0").map(|sym| *sym),
            FLA_LQ_UT_ops_var1: get_symbol(&libs, b"FLA_LQ_UT_ops_var1\0").map(|sym| *sym),
            FLA_LQ_UT_opd_var1: get_symbol(&libs, b"FLA_LQ_UT_opd_var1\0").map(|sym| *sym),
            FLA_LQ_UT_opc_var1: get_symbol(&libs, b"FLA_LQ_UT_opc_var1\0").map(|sym| *sym),
            FLA_LQ_UT_opz_var1: get_symbol(&libs, b"FLA_LQ_UT_opz_var1\0").map(|sym| *sym),
            FLA_LQ_UT_unb_var2: get_symbol(&libs, b"FLA_LQ_UT_unb_var2\0").map(|sym| *sym),
            FLA_LQ_UT_blk_var2: get_symbol(&libs, b"FLA_LQ_UT_blk_var2\0").map(|sym| *sym),
            FLA_LQ_UT_opt_var2: get_symbol(&libs, b"FLA_LQ_UT_opt_var2\0").map(|sym| *sym),
            FLA_LQ_UT_ops_var2: get_symbol(&libs, b"FLA_LQ_UT_ops_var2\0").map(|sym| *sym),
            FLA_LQ_UT_opd_var2: get_symbol(&libs, b"FLA_LQ_UT_opd_var2\0").map(|sym| *sym),
            FLA_LQ_UT_opc_var2: get_symbol(&libs, b"FLA_LQ_UT_opc_var2\0").map(|sym| *sym),
            FLA_LQ_UT_opz_var2: get_symbol(&libs, b"FLA_LQ_UT_opz_var2\0").map(|sym| *sym),
            FLA_LQ_UT_blk_var3: get_symbol(&libs, b"FLA_LQ_UT_blk_var3\0").map(|sym| *sym),
            FLA_LQ_UT_internal: get_symbol(&libs, b"FLA_LQ_UT_internal\0").map(|sym| *sym),
            FLA_LQ_UT_create_T: get_symbol(&libs, b"FLA_LQ_UT_create_T\0").map(|sym| *sym),
            FLA_LQ_UT_recover_tau: get_symbol(&libs, b"FLA_LQ_UT_recover_tau\0").map(|sym| *sym),
            FLA_LQ_UT_solve: get_symbol(&libs, b"FLA_LQ_UT_solve\0").map(|sym| *sym),
            FLASH_LQ_UT: get_symbol(&libs, b"FLASH_LQ_UT\0").map(|sym| *sym),
            FLASH_LQ_UT_create_hier_matrices: get_symbol(
                &libs,
                b"FLASH_LQ_UT_create_hier_matrices\0",
            )
            .map(|sym| *sym),
            FLASH_LQ_UT_solve: get_symbol(&libs, b"FLASH_LQ_UT_solve\0").map(|sym| *sym),
            FLA_LQ_UT_form_Q: get_symbol(&libs, b"FLA_LQ_UT_form_Q\0").map(|sym| *sym),
            FLA_CAQR2_UT_blk_var1: get_symbol(&libs, b"FLA_CAQR2_UT_blk_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_blk_var2: get_symbol(&libs, b"FLA_CAQR2_UT_blk_var2\0").map(|sym| *sym),
            FLA_CAQR2_UT_unb_var1: get_symbol(&libs, b"FLA_CAQR2_UT_unb_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_opt_var1: get_symbol(&libs, b"FLA_CAQR2_UT_opt_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_ops_var1: get_symbol(&libs, b"FLA_CAQR2_UT_ops_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_opd_var1: get_symbol(&libs, b"FLA_CAQR2_UT_opd_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_opc_var1: get_symbol(&libs, b"FLA_CAQR2_UT_opc_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_opz_var1: get_symbol(&libs, b"FLA_CAQR2_UT_opz_var1\0").map(|sym| *sym),
            FLA_CAQR2_UT_internal: get_symbol(&libs, b"FLA_CAQR2_UT_internal\0").map(|sym| *sym),
            FLASH_CAQR_UT_inc: get_symbol(&libs, b"FLASH_CAQR_UT_inc\0").map(|sym| *sym),
            FLASH_CAQR_UT_inc_noopt: get_symbol(&libs, b"FLASH_CAQR_UT_inc_noopt\0")
                .map(|sym| *sym),
            FLASH_CAQR_UT_inc_create_hier_matrices: get_symbol(
                &libs,
                b"FLASH_CAQR_UT_inc_create_hier_matrices\0",
            )
            .map(|sym| *sym),
            FLASH_CAQR_UT_inc_determine_alg_blocksize: get_symbol(
                &libs,
                b"FLASH_CAQR_UT_inc_determine_alg_blocksize\0",
            )
            .map(|sym| *sym),
            FLASH_CAQR_UT_inc_adjust_views: get_symbol(&libs, b"FLASH_CAQR_UT_inc_adjust_views\0")
                .map(|sym| *sym),
            FLA_CAQR_UT_inc_init_structure: get_symbol(&libs, b"FLA_CAQR_UT_inc_init_structure\0")
                .map(|sym| *sym),
            FLA_CAQR_UT_inc_compute_blocks_per_part: get_symbol(
                &libs,
                b"FLA_CAQR_UT_inc_compute_blocks_per_part\0",
            )
            .map(|sym| *sym),
            FLA_CAQR_UT_inc_factorize_panels: get_symbol(
                &libs,
                b"FLA_CAQR_UT_inc_factorize_panels\0",
            )
            .map(|sym| *sym),
            FLA_CAQR_UT_inc_copy_triangles: get_symbol(&libs, b"FLA_CAQR_UT_inc_copy_triangles\0")
                .map(|sym| *sym),
            FLA_CAQR_UT_inc_blk_var1: get_symbol(&libs, b"FLA_CAQR_UT_inc_blk_var1\0")
                .map(|sym| *sym),
            FLASH_CAQR_UT_inc_solve: get_symbol(&libs, b"FLASH_CAQR_UT_inc_solve\0")
                .map(|sym| *sym),
            FLA_Hevd_ln_unb_var1: get_symbol(&libs, b"FLA_Hevd_ln_unb_var1\0").map(|sym| *sym),
            FLA_Hevd_lv_unb_var1: get_symbol(&libs, b"FLA_Hevd_lv_unb_var1\0").map(|sym| *sym),
            FLA_Hevd_lv_unb_var2: get_symbol(&libs, b"FLA_Hevd_lv_unb_var2\0").map(|sym| *sym),
            FLA_Hevd_compute_scaling: get_symbol(&libs, b"FLA_Hevd_compute_scaling\0")
                .map(|sym| *sym),
            FLA_Hevd: get_symbol(&libs, b"FLA_Hevd\0").map(|sym| *sym),
            FLA_Tevd_iteracc_n_ops_var1: get_symbol(&libs, b"FLA_Tevd_iteracc_n_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_iteracc_n_opd_var1: get_symbol(&libs, b"FLA_Tevd_iteracc_n_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_n_opt_var1: get_symbol(&libs, b"FLA_Tevd_eigval_n_opt_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_n_ops_var1: get_symbol(&libs, b"FLA_Tevd_eigval_n_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_n_opd_var1: get_symbol(&libs, b"FLA_Tevd_eigval_n_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_francis_n_opt_var1: get_symbol(&libs, b"FLA_Tevd_francis_n_opt_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_francis_n_ops_var1: get_symbol(&libs, b"FLA_Tevd_francis_n_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_francis_n_opd_var1: get_symbol(&libs, b"FLA_Tevd_francis_n_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_find_submatrix_ops: get_symbol(&libs, b"FLA_Tevd_find_submatrix_ops\0")
                .map(|sym| *sym),
            FLA_Tevd_find_submatrix_opd: get_symbol(&libs, b"FLA_Tevd_find_submatrix_opd\0")
                .map(|sym| *sym),
            FLA_Norm1_tridiag: get_symbol(&libs, b"FLA_Norm1_tridiag\0").map(|sym| *sym),
            FLA_Norm1_tridiag_ops: get_symbol(&libs, b"FLA_Norm1_tridiag_ops\0").map(|sym| *sym),
            FLA_Norm1_tridiag_opd: get_symbol(&libs, b"FLA_Norm1_tridiag_opd\0").map(|sym| *sym),
            FLA_Tevd_n_opt_var1: get_symbol(&libs, b"FLA_Tevd_n_opt_var1\0").map(|sym| *sym),
            FLA_Tevd_n_ops_var1: get_symbol(&libs, b"FLA_Tevd_n_ops_var1\0").map(|sym| *sym),
            FLA_Tevd_n_opd_var1: get_symbol(&libs, b"FLA_Tevd_n_opd_var1\0").map(|sym| *sym),
            FLA_Tevd_n_opc_var1: get_symbol(&libs, b"FLA_Tevd_n_opc_var1\0").map(|sym| *sym),
            FLA_Tevd_n_opz_var1: get_symbol(&libs, b"FLA_Tevd_n_opz_var1\0").map(|sym| *sym),
            FLA_Tevd_iteracc_v_ops_var1: get_symbol(&libs, b"FLA_Tevd_iteracc_v_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_iteracc_v_opd_var1: get_symbol(&libs, b"FLA_Tevd_iteracc_v_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_iteracc_v_ops_var3: get_symbol(&libs, b"FLA_Tevd_iteracc_v_ops_var3\0")
                .map(|sym| *sym),
            FLA_Tevd_iteracc_v_opd_var3: get_symbol(&libs, b"FLA_Tevd_iteracc_v_opd_var3\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_v_opt_var1: get_symbol(&libs, b"FLA_Tevd_eigval_v_opt_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_v_ops_var1: get_symbol(&libs, b"FLA_Tevd_eigval_v_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_v_opd_var1: get_symbol(&libs, b"FLA_Tevd_eigval_v_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_v_ops_var3: get_symbol(&libs, b"FLA_Tevd_eigval_v_ops_var3\0")
                .map(|sym| *sym),
            FLA_Tevd_eigval_v_opd_var3: get_symbol(&libs, b"FLA_Tevd_eigval_v_opd_var3\0")
                .map(|sym| *sym),
            FLA_Tevd_francis_v_opt_var1: get_symbol(&libs, b"FLA_Tevd_francis_v_opt_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_francis_v_ops_var1: get_symbol(&libs, b"FLA_Tevd_francis_v_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_francis_v_opd_var1: get_symbol(&libs, b"FLA_Tevd_francis_v_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tevd_compute_scaling_ops: get_symbol(&libs, b"FLA_Tevd_compute_scaling_ops\0")
                .map(|sym| *sym),
            FLA_Tevd_compute_scaling_opd: get_symbol(&libs, b"FLA_Tevd_compute_scaling_opd\0")
                .map(|sym| *sym),
            FLA_Tevd_find_perfshift_ops: get_symbol(&libs, b"FLA_Tevd_find_perfshift_ops\0")
                .map(|sym| *sym),
            FLA_Tevd_find_perfshift_opd: get_symbol(&libs, b"FLA_Tevd_find_perfshift_opd\0")
                .map(|sym| *sym),
            FLA_Tevd_v_opt_var1: get_symbol(&libs, b"FLA_Tevd_v_opt_var1\0").map(|sym| *sym),
            FLA_Tevd_v_ops_var1: get_symbol(&libs, b"FLA_Tevd_v_ops_var1\0").map(|sym| *sym),
            FLA_Tevd_v_opd_var1: get_symbol(&libs, b"FLA_Tevd_v_opd_var1\0").map(|sym| *sym),
            FLA_Tevd_v_opc_var1: get_symbol(&libs, b"FLA_Tevd_v_opc_var1\0").map(|sym| *sym),
            FLA_Tevd_v_opz_var1: get_symbol(&libs, b"FLA_Tevd_v_opz_var1\0").map(|sym| *sym),
            FLA_Tevd_v_opt_var2: get_symbol(&libs, b"FLA_Tevd_v_opt_var2\0").map(|sym| *sym),
            FLA_Tevd_v_ops_var2: get_symbol(&libs, b"FLA_Tevd_v_ops_var2\0").map(|sym| *sym),
            FLA_Tevd_v_opd_var2: get_symbol(&libs, b"FLA_Tevd_v_opd_var2\0").map(|sym| *sym),
            FLA_Tevd_v_opc_var2: get_symbol(&libs, b"FLA_Tevd_v_opc_var2\0").map(|sym| *sym),
            FLA_Tevd_v_opz_var2: get_symbol(&libs, b"FLA_Tevd_v_opz_var2\0").map(|sym| *sym),
            FLA_Tevd: get_symbol(&libs, b"FLA_Tevd\0").map(|sym| *sym),
            FLA_Svd_ext_u_unb_var1: get_symbol(&libs, b"FLA_Svd_ext_u_unb_var1\0").map(|sym| *sym),
            FLA_Svd_uv_unb_var1: get_symbol(&libs, b"FLA_Svd_uv_unb_var1\0").map(|sym| *sym),
            FLA_Svd_uv_unb_var2: get_symbol(&libs, b"FLA_Svd_uv_unb_var2\0").map(|sym| *sym),
            FLA_Svd_compute_scaling: get_symbol(&libs, b"FLA_Svd_compute_scaling\0")
                .map(|sym| *sym),
            FLA_Svd: get_symbol(&libs, b"FLA_Svd\0").map(|sym| *sym),
            FLA_Svd_ext: get_symbol(&libs, b"FLA_Svd_ext\0").map(|sym| *sym),
            FLA_Bsvd_iteracc_v_ops_var1: get_symbol(&libs, b"FLA_Bsvd_iteracc_v_ops_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_iteracc_v_opd_var1: get_symbol(&libs, b"FLA_Bsvd_iteracc_v_opd_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_sinval_v_opt_var1: get_symbol(&libs, b"FLA_Bsvd_sinval_v_opt_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_sinval_v_ops_var1: get_symbol(&libs, b"FLA_Bsvd_sinval_v_ops_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_sinval_v_opd_var1: get_symbol(&libs, b"FLA_Bsvd_sinval_v_opd_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_francis_v_opt_var1: get_symbol(&libs, b"FLA_Bsvd_francis_v_opt_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_francis_v_ops_var1: get_symbol(&libs, b"FLA_Bsvd_francis_v_ops_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_francis_v_opd_var1: get_symbol(&libs, b"FLA_Bsvd_francis_v_opd_var1\0")
                .map(|sym| *sym),
            FLA_Bsvd_compute_shift: get_symbol(&libs, b"FLA_Bsvd_compute_shift\0").map(|sym| *sym),
            FLA_Bsvd_compute_shift_ops: get_symbol(&libs, b"FLA_Bsvd_compute_shift_ops\0")
                .map(|sym| *sym),
            FLA_Bsvd_compute_shift_opd: get_symbol(&libs, b"FLA_Bsvd_compute_shift_opd\0")
                .map(|sym| *sym),
            FLA_Bsvd_compute_tol_thresh: get_symbol(&libs, b"FLA_Bsvd_compute_tol_thresh\0")
                .map(|sym| *sym),
            FLA_Bsvd_compute_tol_thresh_ops: get_symbol(
                &libs,
                b"FLA_Bsvd_compute_tol_thresh_ops\0",
            )
            .map(|sym| *sym),
            FLA_Bsvd_compute_tol_thresh_opd: get_symbol(
                &libs,
                b"FLA_Bsvd_compute_tol_thresh_opd\0",
            )
            .map(|sym| *sym),
            FLA_Bsvd_find_converged: get_symbol(&libs, b"FLA_Bsvd_find_converged\0")
                .map(|sym| *sym),
            FLA_Bsvd_find_converged_ops: get_symbol(&libs, b"FLA_Bsvd_find_converged_ops\0")
                .map(|sym| *sym),
            FLA_Bsvd_find_converged_opd: get_symbol(&libs, b"FLA_Bsvd_find_converged_opd\0")
                .map(|sym| *sym),
            FLA_Bsvd_find_max_min: get_symbol(&libs, b"FLA_Bsvd_find_max_min\0").map(|sym| *sym),
            FLA_Bsvd_find_max_min_ops: get_symbol(&libs, b"FLA_Bsvd_find_max_min_ops\0")
                .map(|sym| *sym),
            FLA_Bsvd_find_max_min_opd: get_symbol(&libs, b"FLA_Bsvd_find_max_min_opd\0")
                .map(|sym| *sym),
            FLA_Bsvd_find_submatrix_ops: get_symbol(&libs, b"FLA_Bsvd_find_submatrix_ops\0")
                .map(|sym| *sym),
            FLA_Bsvd_find_submatrix_opd: get_symbol(&libs, b"FLA_Bsvd_find_submatrix_opd\0")
                .map(|sym| *sym),
            FLA_Bsvd_v_opt_var1: get_symbol(&libs, b"FLA_Bsvd_v_opt_var1\0").map(|sym| *sym),
            FLA_Bsvd_v_ops_var1: get_symbol(&libs, b"FLA_Bsvd_v_ops_var1\0").map(|sym| *sym),
            FLA_Bsvd_v_opd_var1: get_symbol(&libs, b"FLA_Bsvd_v_opd_var1\0").map(|sym| *sym),
            FLA_Bsvd_v_opc_var1: get_symbol(&libs, b"FLA_Bsvd_v_opc_var1\0").map(|sym| *sym),
            FLA_Bsvd_v_opz_var1: get_symbol(&libs, b"FLA_Bsvd_v_opz_var1\0").map(|sym| *sym),
            FLA_Bsvd_v_opt_var2: get_symbol(&libs, b"FLA_Bsvd_v_opt_var2\0").map(|sym| *sym),
            FLA_Bsvd_v_ops_var2: get_symbol(&libs, b"FLA_Bsvd_v_ops_var2\0").map(|sym| *sym),
            FLA_Bsvd_v_opd_var2: get_symbol(&libs, b"FLA_Bsvd_v_opd_var2\0").map(|sym| *sym),
            FLA_Bsvd_v_opc_var2: get_symbol(&libs, b"FLA_Bsvd_v_opc_var2\0").map(|sym| *sym),
            FLA_Bsvd_v_opz_var2: get_symbol(&libs, b"FLA_Bsvd_v_opz_var2\0").map(|sym| *sym),
            FLA_Bsvd_ext_opt_var1: get_symbol(&libs, b"FLA_Bsvd_ext_opt_var1\0").map(|sym| *sym),
            FLA_Bsvd_ext_ops_var1: get_symbol(&libs, b"FLA_Bsvd_ext_ops_var1\0").map(|sym| *sym),
            FLA_Bsvd_ext_opd_var1: get_symbol(&libs, b"FLA_Bsvd_ext_opd_var1\0").map(|sym| *sym),
            FLA_Bsvd_ext_opc_var1: get_symbol(&libs, b"FLA_Bsvd_ext_opc_var1\0").map(|sym| *sym),
            FLA_Bsvd_ext_opz_var1: get_symbol(&libs, b"FLA_Bsvd_ext_opz_var1\0").map(|sym| *sym),
            FLA_Bsvd_create_workspace: get_symbol(&libs, b"FLA_Bsvd_create_workspace\0")
                .map(|sym| *sym),
            FLA_Bsvd: get_symbol(&libs, b"FLA_Bsvd\0").map(|sym| *sym),
            FLA_Bsvd_ext: get_symbol(&libs, b"FLA_Bsvd_ext\0").map(|sym| *sym),
            FLA_Trinv_ln_blk_var1: get_symbol(&libs, b"FLA_Trinv_ln_blk_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_blk_var2: get_symbol(&libs, b"FLA_Trinv_ln_blk_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_blk_var3: get_symbol(&libs, b"FLA_Trinv_ln_blk_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_blk_var4: get_symbol(&libs, b"FLA_Trinv_ln_blk_var4\0").map(|sym| *sym),
            FLA_Trinv_ln_unb_var1: get_symbol(&libs, b"FLA_Trinv_ln_unb_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_unb_var2: get_symbol(&libs, b"FLA_Trinv_ln_unb_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_unb_var3: get_symbol(&libs, b"FLA_Trinv_ln_unb_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_unb_var4: get_symbol(&libs, b"FLA_Trinv_ln_unb_var4\0").map(|sym| *sym),
            FLA_Trinv_ln_opt_var1: get_symbol(&libs, b"FLA_Trinv_ln_opt_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_ops_var1: get_symbol(&libs, b"FLA_Trinv_ln_ops_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_opd_var1: get_symbol(&libs, b"FLA_Trinv_ln_opd_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_opc_var1: get_symbol(&libs, b"FLA_Trinv_ln_opc_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_opz_var1: get_symbol(&libs, b"FLA_Trinv_ln_opz_var1\0").map(|sym| *sym),
            FLA_Trinv_ln_opt_var2: get_symbol(&libs, b"FLA_Trinv_ln_opt_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_ops_var2: get_symbol(&libs, b"FLA_Trinv_ln_ops_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_opd_var2: get_symbol(&libs, b"FLA_Trinv_ln_opd_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_opc_var2: get_symbol(&libs, b"FLA_Trinv_ln_opc_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_opz_var2: get_symbol(&libs, b"FLA_Trinv_ln_opz_var2\0").map(|sym| *sym),
            FLA_Trinv_ln_opt_var3: get_symbol(&libs, b"FLA_Trinv_ln_opt_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_ops_var3: get_symbol(&libs, b"FLA_Trinv_ln_ops_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_opd_var3: get_symbol(&libs, b"FLA_Trinv_ln_opd_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_opc_var3: get_symbol(&libs, b"FLA_Trinv_ln_opc_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_opz_var3: get_symbol(&libs, b"FLA_Trinv_ln_opz_var3\0").map(|sym| *sym),
            FLA_Trinv_ln_opt_var4: get_symbol(&libs, b"FLA_Trinv_ln_opt_var4\0").map(|sym| *sym),
            FLA_Trinv_ln_ops_var4: get_symbol(&libs, b"FLA_Trinv_ln_ops_var4\0").map(|sym| *sym),
            FLA_Trinv_ln_opd_var4: get_symbol(&libs, b"FLA_Trinv_ln_opd_var4\0").map(|sym| *sym),
            FLA_Trinv_ln_opc_var4: get_symbol(&libs, b"FLA_Trinv_ln_opc_var4\0").map(|sym| *sym),
            FLA_Trinv_ln_opz_var4: get_symbol(&libs, b"FLA_Trinv_ln_opz_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_blk_var1: get_symbol(&libs, b"FLA_Trinv_lu_blk_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_blk_var2: get_symbol(&libs, b"FLA_Trinv_lu_blk_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_blk_var3: get_symbol(&libs, b"FLA_Trinv_lu_blk_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_blk_var4: get_symbol(&libs, b"FLA_Trinv_lu_blk_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_unb_var1: get_symbol(&libs, b"FLA_Trinv_lu_unb_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_unb_var2: get_symbol(&libs, b"FLA_Trinv_lu_unb_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_unb_var3: get_symbol(&libs, b"FLA_Trinv_lu_unb_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_unb_var4: get_symbol(&libs, b"FLA_Trinv_lu_unb_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_opt_var1: get_symbol(&libs, b"FLA_Trinv_lu_opt_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_ops_var1: get_symbol(&libs, b"FLA_Trinv_lu_ops_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_opd_var1: get_symbol(&libs, b"FLA_Trinv_lu_opd_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_opc_var1: get_symbol(&libs, b"FLA_Trinv_lu_opc_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_opz_var1: get_symbol(&libs, b"FLA_Trinv_lu_opz_var1\0").map(|sym| *sym),
            FLA_Trinv_lu_opt_var2: get_symbol(&libs, b"FLA_Trinv_lu_opt_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_ops_var2: get_symbol(&libs, b"FLA_Trinv_lu_ops_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_opd_var2: get_symbol(&libs, b"FLA_Trinv_lu_opd_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_opc_var2: get_symbol(&libs, b"FLA_Trinv_lu_opc_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_opz_var2: get_symbol(&libs, b"FLA_Trinv_lu_opz_var2\0").map(|sym| *sym),
            FLA_Trinv_lu_opt_var3: get_symbol(&libs, b"FLA_Trinv_lu_opt_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_ops_var3: get_symbol(&libs, b"FLA_Trinv_lu_ops_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_opd_var3: get_symbol(&libs, b"FLA_Trinv_lu_opd_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_opc_var3: get_symbol(&libs, b"FLA_Trinv_lu_opc_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_opz_var3: get_symbol(&libs, b"FLA_Trinv_lu_opz_var3\0").map(|sym| *sym),
            FLA_Trinv_lu_opt_var4: get_symbol(&libs, b"FLA_Trinv_lu_opt_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_ops_var4: get_symbol(&libs, b"FLA_Trinv_lu_ops_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_opd_var4: get_symbol(&libs, b"FLA_Trinv_lu_opd_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_opc_var4: get_symbol(&libs, b"FLA_Trinv_lu_opc_var4\0").map(|sym| *sym),
            FLA_Trinv_lu_opz_var4: get_symbol(&libs, b"FLA_Trinv_lu_opz_var4\0").map(|sym| *sym),
            FLA_Trinv_un_blk_var1: get_symbol(&libs, b"FLA_Trinv_un_blk_var1\0").map(|sym| *sym),
            FLA_Trinv_un_blk_var2: get_symbol(&libs, b"FLA_Trinv_un_blk_var2\0").map(|sym| *sym),
            FLA_Trinv_un_blk_var3: get_symbol(&libs, b"FLA_Trinv_un_blk_var3\0").map(|sym| *sym),
            FLA_Trinv_un_blk_var4: get_symbol(&libs, b"FLA_Trinv_un_blk_var4\0").map(|sym| *sym),
            FLA_Trinv_un_unb_var1: get_symbol(&libs, b"FLA_Trinv_un_unb_var1\0").map(|sym| *sym),
            FLA_Trinv_un_unb_var2: get_symbol(&libs, b"FLA_Trinv_un_unb_var2\0").map(|sym| *sym),
            FLA_Trinv_un_unb_var3: get_symbol(&libs, b"FLA_Trinv_un_unb_var3\0").map(|sym| *sym),
            FLA_Trinv_un_unb_var4: get_symbol(&libs, b"FLA_Trinv_un_unb_var4\0").map(|sym| *sym),
            FLA_Trinv_un_opt_var1: get_symbol(&libs, b"FLA_Trinv_un_opt_var1\0").map(|sym| *sym),
            FLA_Trinv_un_ops_var1: get_symbol(&libs, b"FLA_Trinv_un_ops_var1\0").map(|sym| *sym),
            FLA_Trinv_un_opd_var1: get_symbol(&libs, b"FLA_Trinv_un_opd_var1\0").map(|sym| *sym),
            FLA_Trinv_un_opc_var1: get_symbol(&libs, b"FLA_Trinv_un_opc_var1\0").map(|sym| *sym),
            FLA_Trinv_un_opz_var1: get_symbol(&libs, b"FLA_Trinv_un_opz_var1\0").map(|sym| *sym),
            FLA_Trinv_un_opt_var2: get_symbol(&libs, b"FLA_Trinv_un_opt_var2\0").map(|sym| *sym),
            FLA_Trinv_un_ops_var2: get_symbol(&libs, b"FLA_Trinv_un_ops_var2\0").map(|sym| *sym),
            FLA_Trinv_un_opd_var2: get_symbol(&libs, b"FLA_Trinv_un_opd_var2\0").map(|sym| *sym),
            FLA_Trinv_un_opc_var2: get_symbol(&libs, b"FLA_Trinv_un_opc_var2\0").map(|sym| *sym),
            FLA_Trinv_un_opz_var2: get_symbol(&libs, b"FLA_Trinv_un_opz_var2\0").map(|sym| *sym),
            FLA_Trinv_un_opt_var3: get_symbol(&libs, b"FLA_Trinv_un_opt_var3\0").map(|sym| *sym),
            FLA_Trinv_un_ops_var3: get_symbol(&libs, b"FLA_Trinv_un_ops_var3\0").map(|sym| *sym),
            FLA_Trinv_un_opd_var3: get_symbol(&libs, b"FLA_Trinv_un_opd_var3\0").map(|sym| *sym),
            FLA_Trinv_un_opc_var3: get_symbol(&libs, b"FLA_Trinv_un_opc_var3\0").map(|sym| *sym),
            FLA_Trinv_un_opz_var3: get_symbol(&libs, b"FLA_Trinv_un_opz_var3\0").map(|sym| *sym),
            FLA_Trinv_un_opt_var4: get_symbol(&libs, b"FLA_Trinv_un_opt_var4\0").map(|sym| *sym),
            FLA_Trinv_un_ops_var4: get_symbol(&libs, b"FLA_Trinv_un_ops_var4\0").map(|sym| *sym),
            FLA_Trinv_un_opd_var4: get_symbol(&libs, b"FLA_Trinv_un_opd_var4\0").map(|sym| *sym),
            FLA_Trinv_un_opc_var4: get_symbol(&libs, b"FLA_Trinv_un_opc_var4\0").map(|sym| *sym),
            FLA_Trinv_un_opz_var4: get_symbol(&libs, b"FLA_Trinv_un_opz_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_blk_var1: get_symbol(&libs, b"FLA_Trinv_uu_blk_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_blk_var2: get_symbol(&libs, b"FLA_Trinv_uu_blk_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_blk_var3: get_symbol(&libs, b"FLA_Trinv_uu_blk_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_blk_var4: get_symbol(&libs, b"FLA_Trinv_uu_blk_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_unb_var1: get_symbol(&libs, b"FLA_Trinv_uu_unb_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_unb_var2: get_symbol(&libs, b"FLA_Trinv_uu_unb_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_unb_var3: get_symbol(&libs, b"FLA_Trinv_uu_unb_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_unb_var4: get_symbol(&libs, b"FLA_Trinv_uu_unb_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_opt_var1: get_symbol(&libs, b"FLA_Trinv_uu_opt_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_ops_var1: get_symbol(&libs, b"FLA_Trinv_uu_ops_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_opd_var1: get_symbol(&libs, b"FLA_Trinv_uu_opd_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_opc_var1: get_symbol(&libs, b"FLA_Trinv_uu_opc_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_opz_var1: get_symbol(&libs, b"FLA_Trinv_uu_opz_var1\0").map(|sym| *sym),
            FLA_Trinv_uu_opt_var2: get_symbol(&libs, b"FLA_Trinv_uu_opt_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_ops_var2: get_symbol(&libs, b"FLA_Trinv_uu_ops_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_opd_var2: get_symbol(&libs, b"FLA_Trinv_uu_opd_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_opc_var2: get_symbol(&libs, b"FLA_Trinv_uu_opc_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_opz_var2: get_symbol(&libs, b"FLA_Trinv_uu_opz_var2\0").map(|sym| *sym),
            FLA_Trinv_uu_opt_var3: get_symbol(&libs, b"FLA_Trinv_uu_opt_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_ops_var3: get_symbol(&libs, b"FLA_Trinv_uu_ops_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_opd_var3: get_symbol(&libs, b"FLA_Trinv_uu_opd_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_opc_var3: get_symbol(&libs, b"FLA_Trinv_uu_opc_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_opz_var3: get_symbol(&libs, b"FLA_Trinv_uu_opz_var3\0").map(|sym| *sym),
            FLA_Trinv_uu_opt_var4: get_symbol(&libs, b"FLA_Trinv_uu_opt_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_ops_var4: get_symbol(&libs, b"FLA_Trinv_uu_ops_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_opd_var4: get_symbol(&libs, b"FLA_Trinv_uu_opd_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_opc_var4: get_symbol(&libs, b"FLA_Trinv_uu_opc_var4\0").map(|sym| *sym),
            FLA_Trinv_uu_opz_var4: get_symbol(&libs, b"FLA_Trinv_uu_opz_var4\0").map(|sym| *sym),
            FLA_Trinv_internal: get_symbol(&libs, b"FLA_Trinv_internal\0").map(|sym| *sym),
            FLA_Trinv_ln: get_symbol(&libs, b"FLA_Trinv_ln\0").map(|sym| *sym),
            FLA_Trinv_lu: get_symbol(&libs, b"FLA_Trinv_lu\0").map(|sym| *sym),
            FLA_Trinv_un: get_symbol(&libs, b"FLA_Trinv_un\0").map(|sym| *sym),
            FLA_Trinv_uu: get_symbol(&libs, b"FLA_Trinv_uu\0").map(|sym| *sym),
            FLA_SPDinv_internal: get_symbol(&libs, b"FLA_SPDinv_internal\0").map(|sym| *sym),
            FLA_Hess_UT_blk_var1: get_symbol(&libs, b"FLA_Hess_UT_blk_var1\0").map(|sym| *sym),
            FLA_Hess_UT_unb_var1: get_symbol(&libs, b"FLA_Hess_UT_unb_var1\0").map(|sym| *sym),
            FLA_Hess_UT_step_unb_var1: get_symbol(&libs, b"FLA_Hess_UT_step_unb_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_blk_var2: get_symbol(&libs, b"FLA_Hess_UT_blk_var2\0").map(|sym| *sym),
            FLA_Hess_UT_blf_var2: get_symbol(&libs, b"FLA_Hess_UT_blf_var2\0").map(|sym| *sym),
            FLA_Hess_UT_unb_var2: get_symbol(&libs, b"FLA_Hess_UT_unb_var2\0").map(|sym| *sym),
            FLA_Hess_UT_step_unb_var2: get_symbol(&libs, b"FLA_Hess_UT_step_unb_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_blk_var3: get_symbol(&libs, b"FLA_Hess_UT_blk_var3\0").map(|sym| *sym),
            FLA_Hess_UT_blf_var3: get_symbol(&libs, b"FLA_Hess_UT_blf_var3\0").map(|sym| *sym),
            FLA_Hess_UT_unb_var3: get_symbol(&libs, b"FLA_Hess_UT_unb_var3\0").map(|sym| *sym),
            FLA_Hess_UT_step_unb_var3: get_symbol(&libs, b"FLA_Hess_UT_step_unb_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_blk_var4: get_symbol(&libs, b"FLA_Hess_UT_blk_var4\0").map(|sym| *sym),
            FLA_Hess_UT_blf_var4: get_symbol(&libs, b"FLA_Hess_UT_blf_var4\0").map(|sym| *sym),
            FLA_Hess_UT_unb_var4: get_symbol(&libs, b"FLA_Hess_UT_unb_var4\0").map(|sym| *sym),
            FLA_Hess_UT_step_unb_var4: get_symbol(&libs, b"FLA_Hess_UT_step_unb_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_blk_var5: get_symbol(&libs, b"FLA_Hess_UT_blk_var5\0").map(|sym| *sym),
            FLA_Hess_UT_unb_var5: get_symbol(&libs, b"FLA_Hess_UT_unb_var5\0").map(|sym| *sym),
            FLA_Hess_UT_step_unb_var5: get_symbol(&libs, b"FLA_Hess_UT_step_unb_var5\0")
                .map(|sym| *sym),
            FLA_Hess_UT_opt_var1: get_symbol(&libs, b"FLA_Hess_UT_opt_var1\0").map(|sym| *sym),
            FLA_Hess_UT_step_opt_var1: get_symbol(&libs, b"FLA_Hess_UT_step_opt_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ops_var1: get_symbol(&libs, b"FLA_Hess_UT_step_ops_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opd_var1: get_symbol(&libs, b"FLA_Hess_UT_step_opd_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opc_var1: get_symbol(&libs, b"FLA_Hess_UT_step_opc_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opz_var1: get_symbol(&libs, b"FLA_Hess_UT_step_opz_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_opt_var2: get_symbol(&libs, b"FLA_Hess_UT_opt_var2\0").map(|sym| *sym),
            FLA_Hess_UT_step_opt_var2: get_symbol(&libs, b"FLA_Hess_UT_step_opt_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ops_var2: get_symbol(&libs, b"FLA_Hess_UT_step_ops_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opd_var2: get_symbol(&libs, b"FLA_Hess_UT_step_opd_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opc_var2: get_symbol(&libs, b"FLA_Hess_UT_step_opc_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opz_var2: get_symbol(&libs, b"FLA_Hess_UT_step_opz_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_opt_var3: get_symbol(&libs, b"FLA_Hess_UT_opt_var3\0").map(|sym| *sym),
            FLA_Hess_UT_step_opt_var3: get_symbol(&libs, b"FLA_Hess_UT_step_opt_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ops_var3: get_symbol(&libs, b"FLA_Hess_UT_step_ops_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opd_var3: get_symbol(&libs, b"FLA_Hess_UT_step_opd_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opc_var3: get_symbol(&libs, b"FLA_Hess_UT_step_opc_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opz_var3: get_symbol(&libs, b"FLA_Hess_UT_step_opz_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_opt_var4: get_symbol(&libs, b"FLA_Hess_UT_opt_var4\0").map(|sym| *sym),
            FLA_Hess_UT_step_opt_var4: get_symbol(&libs, b"FLA_Hess_UT_step_opt_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ops_var4: get_symbol(&libs, b"FLA_Hess_UT_step_ops_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opd_var4: get_symbol(&libs, b"FLA_Hess_UT_step_opd_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opc_var4: get_symbol(&libs, b"FLA_Hess_UT_step_opc_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opz_var4: get_symbol(&libs, b"FLA_Hess_UT_step_opz_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_opt_var5: get_symbol(&libs, b"FLA_Hess_UT_opt_var5\0").map(|sym| *sym),
            FLA_Hess_UT_step_opt_var5: get_symbol(&libs, b"FLA_Hess_UT_step_opt_var5\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ops_var5: get_symbol(&libs, b"FLA_Hess_UT_step_ops_var5\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opd_var5: get_symbol(&libs, b"FLA_Hess_UT_step_opd_var5\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opc_var5: get_symbol(&libs, b"FLA_Hess_UT_step_opc_var5\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_opz_var5: get_symbol(&libs, b"FLA_Hess_UT_step_opz_var5\0")
                .map(|sym| *sym),
            FLA_Hess_UT_ofu_var1: get_symbol(&libs, b"FLA_Hess_UT_ofu_var1\0").map(|sym| *sym),
            FLA_Hess_UT_step_ofu_var1: get_symbol(&libs, b"FLA_Hess_UT_step_ofu_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofs_var1: get_symbol(&libs, b"FLA_Hess_UT_step_ofs_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofd_var1: get_symbol(&libs, b"FLA_Hess_UT_step_ofd_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofc_var1: get_symbol(&libs, b"FLA_Hess_UT_step_ofc_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofz_var1: get_symbol(&libs, b"FLA_Hess_UT_step_ofz_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_ofu_var2: get_symbol(&libs, b"FLA_Hess_UT_ofu_var2\0").map(|sym| *sym),
            FLA_Hess_UT_step_ofu_var2: get_symbol(&libs, b"FLA_Hess_UT_step_ofu_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofs_var2: get_symbol(&libs, b"FLA_Hess_UT_step_ofs_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofd_var2: get_symbol(&libs, b"FLA_Hess_UT_step_ofd_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofc_var2: get_symbol(&libs, b"FLA_Hess_UT_step_ofc_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofz_var2: get_symbol(&libs, b"FLA_Hess_UT_step_ofz_var2\0")
                .map(|sym| *sym),
            FLA_Hess_UT_ofu_var3: get_symbol(&libs, b"FLA_Hess_UT_ofu_var3\0").map(|sym| *sym),
            FLA_Hess_UT_step_ofu_var3: get_symbol(&libs, b"FLA_Hess_UT_step_ofu_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofs_var3: get_symbol(&libs, b"FLA_Hess_UT_step_ofs_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofd_var3: get_symbol(&libs, b"FLA_Hess_UT_step_ofd_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofc_var3: get_symbol(&libs, b"FLA_Hess_UT_step_ofc_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofz_var3: get_symbol(&libs, b"FLA_Hess_UT_step_ofz_var3\0")
                .map(|sym| *sym),
            FLA_Hess_UT_ofu_var4: get_symbol(&libs, b"FLA_Hess_UT_ofu_var4\0").map(|sym| *sym),
            FLA_Hess_UT_step_ofu_var4: get_symbol(&libs, b"FLA_Hess_UT_step_ofu_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofs_var4: get_symbol(&libs, b"FLA_Hess_UT_step_ofs_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofd_var4: get_symbol(&libs, b"FLA_Hess_UT_step_ofd_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofc_var4: get_symbol(&libs, b"FLA_Hess_UT_step_ofc_var4\0")
                .map(|sym| *sym),
            FLA_Hess_UT_step_ofz_var4: get_symbol(&libs, b"FLA_Hess_UT_step_ofz_var4\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Ax_ops_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Ax_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Ax_opd_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Ax_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Ax_opc_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Ax_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Ax_opz_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Ax_opz_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Ax_ops_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Ax_ops_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Ax_opd_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Ax_opd_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Ax_opc_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Ax_opc_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Ax_opz_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Ax_opz_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Uhu_Yhu_Zhu_ops_var1: get_symbol(&libs, b"FLA_Fused_Uhu_Yhu_Zhu_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Uhu_Yhu_Zhu_opd_var1: get_symbol(&libs, b"FLA_Fused_Uhu_Yhu_Zhu_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Uhu_Yhu_Zhu_opc_var1: get_symbol(&libs, b"FLA_Fused_Uhu_Yhu_Zhu_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Uhu_Yhu_Zhu_opz_var1: get_symbol(&libs, b"FLA_Fused_Uhu_Yhu_Zhu_opz_var1\0")
                .map(|sym| *sym),
            FLA_Hess_UT_internal: get_symbol(&libs, b"FLA_Hess_UT_internal\0").map(|sym| *sym),
            FLA_Hess_UT_create_T: get_symbol(&libs, b"FLA_Hess_UT_create_T\0").map(|sym| *sym),
            FLA_Hess_UT_recover_tau: get_symbol(&libs, b"FLA_Hess_UT_recover_tau\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_blk_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_blk_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_unb_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_unb_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_unb_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_unb_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_blk_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_blk_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_blf_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_blf_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_unb_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_unb_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_unb_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_unb_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_blk_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_blk_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_blf_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_blf_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_unb_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_unb_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_unb_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_unb_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_opt_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_opt_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opt_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opt_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ops_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ops_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opd_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opd_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opc_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opc_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opz_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opz_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_opt_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_opt_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opt_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opt_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ops_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ops_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opd_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opd_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opc_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opc_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opz_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opz_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_opt_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_opt_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opt_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opt_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ops_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ops_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opd_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opd_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opc_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opc_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_opz_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_opz_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_ofu_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_ofu_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofu_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofu_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofs_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofs_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofd_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofd_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofc_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofc_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofz_var1: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofz_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_ofu_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_ofu_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofu_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofu_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofs_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofs_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofd_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofd_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofc_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofc_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofz_var2: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofz_var2\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_ofu_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_ofu_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofu_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofu_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofs_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofs_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofd_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofd_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofc_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofc_var3\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_step_ofz_var3: get_symbol(&libs, b"FLA_Tridiag_UT_l_step_ofz_var3\0")
                .map(|sym| *sym),
            FLA_Fused_Her2_Ax_l_opt_var1: get_symbol(&libs, b"FLA_Fused_Her2_Ax_l_opt_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Her2_Ax_l_ops_var1: get_symbol(&libs, b"FLA_Fused_Her2_Ax_l_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Her2_Ax_l_opd_var1: get_symbol(&libs, b"FLA_Fused_Her2_Ax_l_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Her2_Ax_l_opc_var1: get_symbol(&libs, b"FLA_Fused_Her2_Ax_l_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Her2_Ax_l_opz_var1: get_symbol(&libs, b"FLA_Fused_Her2_Ax_l_opz_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UZhu_ZUhu_opt_var1: get_symbol(&libs, b"FLA_Fused_UZhu_ZUhu_opt_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UZhu_ZUhu_ops_var1: get_symbol(&libs, b"FLA_Fused_UZhu_ZUhu_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UZhu_ZUhu_opd_var1: get_symbol(&libs, b"FLA_Fused_UZhu_ZUhu_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UZhu_ZUhu_opc_var1: get_symbol(&libs, b"FLA_Fused_UZhu_ZUhu_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UZhu_ZUhu_opz_var1: get_symbol(&libs, b"FLA_Fused_UZhu_ZUhu_opz_var1\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT: get_symbol(&libs, b"FLA_Tridiag_UT\0").map(|sym| *sym),
            FLA_Tridiag_UT_internal: get_symbol(&libs, b"FLA_Tridiag_UT_internal\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l: get_symbol(&libs, b"FLA_Tridiag_UT_l\0").map(|sym| *sym),
            FLA_Tridiag_UT_u: get_symbol(&libs, b"FLA_Tridiag_UT_u\0").map(|sym| *sym),
            FLA_Tridiag_UT_create_T: get_symbol(&libs, b"FLA_Tridiag_UT_create_T\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_recover_tau: get_symbol(&libs, b"FLA_Tridiag_UT_recover_tau\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_scale_diagonals: get_symbol(&libs, b"FLA_Tridiag_UT_scale_diagonals\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_extract_diagonals: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_extract_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_extract_real_diagonals: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_extract_real_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_realify: get_symbol(&libs, b"FLA_Tridiag_UT_realify\0").map(|sym| *sym),
            FLA_Tridiag_UT_l_realify_unb: get_symbol(&libs, b"FLA_Tridiag_UT_l_realify_unb\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_l_realify_opt: get_symbol(&libs, b"FLA_Tridiag_UT_l_realify_opt\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_u_realify_unb: get_symbol(&libs, b"FLA_Tridiag_UT_u_realify_unb\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_u_realify_opt: get_symbol(&libs, b"FLA_Tridiag_UT_u_realify_opt\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_realify_subdiagonal: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_realify_subdiagonal\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_realify_subdiagonal_opt: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_realify_subdiagonal_opt\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U\0").map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_l_ops: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_l_ops\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_u_ops: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_u_ops\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_l_opd: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_l_opd\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_u_opd: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_u_opd\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_l_opc: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_l_opc\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_u_opc: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_u_opc\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_l_opz: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_l_opz\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_shift_U_u_opz: get_symbol(&libs, b"FLA_Tridiag_UT_shift_U_u_opz\0")
                .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q: get_symbol(&libs, b"FLA_Tridiag_UT_form_Q\0").map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_l_blk_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_l_blk_var1\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_u_blk_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_u_blk_var1\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_l_opt_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_l_opt_var1\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_l_ops_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_l_ops_var1\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_l_opd_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_l_opd_var1\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_l_opc_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_l_opc_var1\0",
            )
            .map(|sym| *sym),
            FLA_Tridiag_UT_form_Q_l_opz_var1: get_symbol(
                &libs,
                b"FLA_Tridiag_UT_form_Q_l_opz_var1\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_u_unb_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_unb_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blk_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_blk_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_unb_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_unb_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_unb_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_unb_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blk_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_blk_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blf_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_blf_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_unb_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_unb_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_unb_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_unb_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blk_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_blk_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blf_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_blf_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_unb_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_unb_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_unb_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_unb_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blk_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_blk_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blf_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_blf_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_unb_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_unb_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_unb_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_unb_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_blk_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_blk_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_unb_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_unb_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_opt_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_opt_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opt_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opt_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ops_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ops_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opd_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opd_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opc_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opc_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opz_var1: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opz_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_opt_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_opt_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opt_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opt_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ops_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ops_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opd_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opd_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opc_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opc_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opz_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opz_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_opt_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_opt_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opt_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opt_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ops_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ops_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opd_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opd_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opc_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opc_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opz_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opz_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_opt_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_opt_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opt_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opt_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ops_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ops_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opd_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opd_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opc_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opc_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opz_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opz_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_opt_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_opt_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opt_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opt_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ops_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ops_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opd_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opd_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opc_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opc_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_opz_var5: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_opz_var5\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_ofu_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_ofu_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofu_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofu_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofs_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofs_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofd_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofd_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofc_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofc_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofz_var2: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofz_var2\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_ofu_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_ofu_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofu_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofu_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofs_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofs_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofd_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofd_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofc_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofc_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofz_var3: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofz_var3\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_ofu_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_ofu_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofu_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofu_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofs_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofs_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofd_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofd_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofc_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofc_var4\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_step_ofz_var4: get_symbol(&libs, b"FLA_Bidiag_UT_u_step_ofz_var4\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_opt_var1: get_symbol(&libs, b"FLA_Fused_Gerc2_opt_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_ops_var1: get_symbol(&libs, b"FLA_Fused_Gerc2_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_opd_var1: get_symbol(&libs, b"FLA_Fused_Gerc2_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_opc_var1: get_symbol(&libs, b"FLA_Fused_Gerc2_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_opz_var1: get_symbol(&libs, b"FLA_Fused_Gerc2_opz_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Axpy_Ax_opt_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Axpy_Ax_opt_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Axpy_Ax_ops_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Axpy_Ax_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Axpy_Ax_opd_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Axpy_Ax_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Axpy_Ax_opc_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Axpy_Ax_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Ahx_Axpy_Ax_opz_var1: get_symbol(&libs, b"FLA_Fused_Ahx_Axpy_Ax_opz_var1\0")
                .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Axpy_Ax_opt_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Axpy_Ax_opt_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Axpy_Ax_ops_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Axpy_Ax_ops_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Axpy_Ax_opd_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Axpy_Ax_opd_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Axpy_Ax_opc_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Axpy_Ax_opc_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_Gerc2_Ahx_Axpy_Ax_opz_var1: get_symbol(
                &libs,
                b"FLA_Fused_Gerc2_Ahx_Axpy_Ax_opz_var1\0",
            )
            .map(|sym| *sym),
            FLA_Fused_UYx_ZVx_opt_var1: get_symbol(&libs, b"FLA_Fused_UYx_ZVx_opt_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UYx_ZVx_ops_var1: get_symbol(&libs, b"FLA_Fused_UYx_ZVx_ops_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UYx_ZVx_opd_var1: get_symbol(&libs, b"FLA_Fused_UYx_ZVx_opd_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UYx_ZVx_opc_var1: get_symbol(&libs, b"FLA_Fused_UYx_ZVx_opc_var1\0")
                .map(|sym| *sym),
            FLA_Fused_UYx_ZVx_opz_var1: get_symbol(&libs, b"FLA_Fused_UYx_ZVx_opz_var1\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT: get_symbol(&libs, b"FLA_Bidiag_UT\0").map(|sym| *sym),
            FLA_Bidiag_UT_internal: get_symbol(&libs, b"FLA_Bidiag_UT_internal\0").map(|sym| *sym),
            FLA_Bidiag_UT_l: get_symbol(&libs, b"FLA_Bidiag_UT_l\0").map(|sym| *sym),
            FLA_Bidiag_UT_u: get_symbol(&libs, b"FLA_Bidiag_UT_u\0").map(|sym| *sym),
            FLA_Bidiag_UT_create_T: get_symbol(&libs, b"FLA_Bidiag_UT_create_T\0").map(|sym| *sym),
            FLA_Bidiag_UT_recover_tau: get_symbol(&libs, b"FLA_Bidiag_UT_recover_tau\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_extract_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_extract_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_u_extract_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_u_extract_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_l_extract_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_l_extract_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_extract_real_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_extract_real_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_u_extract_real_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_u_extract_real_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_l_extract_real_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_l_extract_real_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_scale_diagonals: get_symbol(&libs, b"FLA_Bidiag_UT_scale_diagonals\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_scale_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_u_scale_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_l_scale_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_l_scale_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_realify: get_symbol(&libs, b"FLA_Bidiag_UT_realify\0").map(|sym| *sym),
            FLA_Bidiag_UT_l_realify_unb: get_symbol(&libs, b"FLA_Bidiag_UT_l_realify_unb\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_l_realify_opt: get_symbol(&libs, b"FLA_Bidiag_UT_l_realify_opt\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_realify_unb: get_symbol(&libs, b"FLA_Bidiag_UT_u_realify_unb\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_u_realify_opt: get_symbol(&libs, b"FLA_Bidiag_UT_u_realify_opt\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_realify_diagonals: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_realify_diagonals\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_realify_diagonals_opt: get_symbol(
                &libs,
                b"FLA_Bidiag_UT_realify_diagonals_opt\0",
            )
            .map(|sym| *sym),
            FLA_Bidiag_UT_form_U: get_symbol(&libs, b"FLA_Bidiag_UT_form_U\0").map(|sym| *sym),
            FLA_Bidiag_UT_form_V: get_symbol(&libs, b"FLA_Bidiag_UT_form_V\0").map(|sym| *sym),
            FLA_Bidiag_UT_form_U_ext: get_symbol(&libs, b"FLA_Bidiag_UT_form_U_ext\0")
                .map(|sym| *sym),
            FLA_Bidiag_UT_form_V_ext: get_symbol(&libs, b"FLA_Bidiag_UT_form_V_ext\0")
                .map(|sym| *sym),
            FLA_Lyap_n_unb_var1: get_symbol(&libs, b"FLA_Lyap_n_unb_var1\0").map(|sym| *sym),
            FLA_Lyap_n_unb_var2: get_symbol(&libs, b"FLA_Lyap_n_unb_var2\0").map(|sym| *sym),
            FLA_Lyap_n_unb_var3: get_symbol(&libs, b"FLA_Lyap_n_unb_var3\0").map(|sym| *sym),
            FLA_Lyap_n_unb_var4: get_symbol(&libs, b"FLA_Lyap_n_unb_var4\0").map(|sym| *sym),
            FLA_Lyap_n_blk_var1: get_symbol(&libs, b"FLA_Lyap_n_blk_var1\0").map(|sym| *sym),
            FLA_Lyap_n_blk_var2: get_symbol(&libs, b"FLA_Lyap_n_blk_var2\0").map(|sym| *sym),
            FLA_Lyap_n_blk_var3: get_symbol(&libs, b"FLA_Lyap_n_blk_var3\0").map(|sym| *sym),
            FLA_Lyap_n_blk_var4: get_symbol(&libs, b"FLA_Lyap_n_blk_var4\0").map(|sym| *sym),
            FLA_Lyap_n_opt_var1: get_symbol(&libs, b"FLA_Lyap_n_opt_var1\0").map(|sym| *sym),
            FLA_Lyap_n_ops_var1: get_symbol(&libs, b"FLA_Lyap_n_ops_var1\0").map(|sym| *sym),
            FLA_Lyap_n_opd_var1: get_symbol(&libs, b"FLA_Lyap_n_opd_var1\0").map(|sym| *sym),
            FLA_Lyap_n_opc_var1: get_symbol(&libs, b"FLA_Lyap_n_opc_var1\0").map(|sym| *sym),
            FLA_Lyap_n_opz_var1: get_symbol(&libs, b"FLA_Lyap_n_opz_var1\0").map(|sym| *sym),
            FLA_Lyap_n_opt_var2: get_symbol(&libs, b"FLA_Lyap_n_opt_var2\0").map(|sym| *sym),
            FLA_Lyap_n_ops_var2: get_symbol(&libs, b"FLA_Lyap_n_ops_var2\0").map(|sym| *sym),
            FLA_Lyap_n_opd_var2: get_symbol(&libs, b"FLA_Lyap_n_opd_var2\0").map(|sym| *sym),
            FLA_Lyap_n_opc_var2: get_symbol(&libs, b"FLA_Lyap_n_opc_var2\0").map(|sym| *sym),
            FLA_Lyap_n_opz_var2: get_symbol(&libs, b"FLA_Lyap_n_opz_var2\0").map(|sym| *sym),
            FLA_Lyap_n_opt_var3: get_symbol(&libs, b"FLA_Lyap_n_opt_var3\0").map(|sym| *sym),
            FLA_Lyap_n_ops_var3: get_symbol(&libs, b"FLA_Lyap_n_ops_var3\0").map(|sym| *sym),
            FLA_Lyap_n_opd_var3: get_symbol(&libs, b"FLA_Lyap_n_opd_var3\0").map(|sym| *sym),
            FLA_Lyap_n_opc_var3: get_symbol(&libs, b"FLA_Lyap_n_opc_var3\0").map(|sym| *sym),
            FLA_Lyap_n_opz_var3: get_symbol(&libs, b"FLA_Lyap_n_opz_var3\0").map(|sym| *sym),
            FLA_Lyap_n_opt_var4: get_symbol(&libs, b"FLA_Lyap_n_opt_var4\0").map(|sym| *sym),
            FLA_Lyap_n_ops_var4: get_symbol(&libs, b"FLA_Lyap_n_ops_var4\0").map(|sym| *sym),
            FLA_Lyap_n_opd_var4: get_symbol(&libs, b"FLA_Lyap_n_opd_var4\0").map(|sym| *sym),
            FLA_Lyap_n_opc_var4: get_symbol(&libs, b"FLA_Lyap_n_opc_var4\0").map(|sym| *sym),
            FLA_Lyap_n_opz_var4: get_symbol(&libs, b"FLA_Lyap_n_opz_var4\0").map(|sym| *sym),
            FLA_Lyap_h_unb_var1: get_symbol(&libs, b"FLA_Lyap_h_unb_var1\0").map(|sym| *sym),
            FLA_Lyap_h_unb_var2: get_symbol(&libs, b"FLA_Lyap_h_unb_var2\0").map(|sym| *sym),
            FLA_Lyap_h_unb_var3: get_symbol(&libs, b"FLA_Lyap_h_unb_var3\0").map(|sym| *sym),
            FLA_Lyap_h_unb_var4: get_symbol(&libs, b"FLA_Lyap_h_unb_var4\0").map(|sym| *sym),
            FLA_Lyap_h_blk_var1: get_symbol(&libs, b"FLA_Lyap_h_blk_var1\0").map(|sym| *sym),
            FLA_Lyap_h_blk_var2: get_symbol(&libs, b"FLA_Lyap_h_blk_var2\0").map(|sym| *sym),
            FLA_Lyap_h_blk_var3: get_symbol(&libs, b"FLA_Lyap_h_blk_var3\0").map(|sym| *sym),
            FLA_Lyap_h_blk_var4: get_symbol(&libs, b"FLA_Lyap_h_blk_var4\0").map(|sym| *sym),
            FLA_Lyap_h_opt_var1: get_symbol(&libs, b"FLA_Lyap_h_opt_var1\0").map(|sym| *sym),
            FLA_Lyap_h_ops_var1: get_symbol(&libs, b"FLA_Lyap_h_ops_var1\0").map(|sym| *sym),
            FLA_Lyap_h_opd_var1: get_symbol(&libs, b"FLA_Lyap_h_opd_var1\0").map(|sym| *sym),
            FLA_Lyap_h_opc_var1: get_symbol(&libs, b"FLA_Lyap_h_opc_var1\0").map(|sym| *sym),
            FLA_Lyap_h_opz_var1: get_symbol(&libs, b"FLA_Lyap_h_opz_var1\0").map(|sym| *sym),
            FLA_Lyap_h_opt_var2: get_symbol(&libs, b"FLA_Lyap_h_opt_var2\0").map(|sym| *sym),
            FLA_Lyap_h_ops_var2: get_symbol(&libs, b"FLA_Lyap_h_ops_var2\0").map(|sym| *sym),
            FLA_Lyap_h_opd_var2: get_symbol(&libs, b"FLA_Lyap_h_opd_var2\0").map(|sym| *sym),
            FLA_Lyap_h_opc_var2: get_symbol(&libs, b"FLA_Lyap_h_opc_var2\0").map(|sym| *sym),
            FLA_Lyap_h_opz_var2: get_symbol(&libs, b"FLA_Lyap_h_opz_var2\0").map(|sym| *sym),
            FLA_Lyap_h_opt_var3: get_symbol(&libs, b"FLA_Lyap_h_opt_var3\0").map(|sym| *sym),
            FLA_Lyap_h_ops_var3: get_symbol(&libs, b"FLA_Lyap_h_ops_var3\0").map(|sym| *sym),
            FLA_Lyap_h_opd_var3: get_symbol(&libs, b"FLA_Lyap_h_opd_var3\0").map(|sym| *sym),
            FLA_Lyap_h_opc_var3: get_symbol(&libs, b"FLA_Lyap_h_opc_var3\0").map(|sym| *sym),
            FLA_Lyap_h_opz_var3: get_symbol(&libs, b"FLA_Lyap_h_opz_var3\0").map(|sym| *sym),
            FLA_Lyap_h_opt_var4: get_symbol(&libs, b"FLA_Lyap_h_opt_var4\0").map(|sym| *sym),
            FLA_Lyap_h_ops_var4: get_symbol(&libs, b"FLA_Lyap_h_ops_var4\0").map(|sym| *sym),
            FLA_Lyap_h_opd_var4: get_symbol(&libs, b"FLA_Lyap_h_opd_var4\0").map(|sym| *sym),
            FLA_Lyap_h_opc_var4: get_symbol(&libs, b"FLA_Lyap_h_opc_var4\0").map(|sym| *sym),
            FLA_Lyap_h_opz_var4: get_symbol(&libs, b"FLA_Lyap_h_opz_var4\0").map(|sym| *sym),
            FLASH_Lyap: get_symbol(&libs, b"FLASH_Lyap\0").map(|sym| *sym),
            FLA_Lyap: get_symbol(&libs, b"FLA_Lyap\0").map(|sym| *sym),
            FLA_Lyap_internal: get_symbol(&libs, b"FLA_Lyap_internal\0").map(|sym| *sym),
            FLA_Lyap_n: get_symbol(&libs, b"FLA_Lyap_n\0").map(|sym| *sym),
            FLA_Lyap_h: get_symbol(&libs, b"FLA_Lyap_h\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var1: get_symbol(&libs, b"FLA_Sylv_nn_blk_var1\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var2: get_symbol(&libs, b"FLA_Sylv_nn_blk_var2\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var3: get_symbol(&libs, b"FLA_Sylv_nn_blk_var3\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var4: get_symbol(&libs, b"FLA_Sylv_nn_blk_var4\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var5: get_symbol(&libs, b"FLA_Sylv_nn_blk_var5\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var6: get_symbol(&libs, b"FLA_Sylv_nn_blk_var6\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var7: get_symbol(&libs, b"FLA_Sylv_nn_blk_var7\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var8: get_symbol(&libs, b"FLA_Sylv_nn_blk_var8\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var9: get_symbol(&libs, b"FLA_Sylv_nn_blk_var9\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var10: get_symbol(&libs, b"FLA_Sylv_nn_blk_var10\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var11: get_symbol(&libs, b"FLA_Sylv_nn_blk_var11\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var12: get_symbol(&libs, b"FLA_Sylv_nn_blk_var12\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var13: get_symbol(&libs, b"FLA_Sylv_nn_blk_var13\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var14: get_symbol(&libs, b"FLA_Sylv_nn_blk_var14\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var15: get_symbol(&libs, b"FLA_Sylv_nn_blk_var15\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var16: get_symbol(&libs, b"FLA_Sylv_nn_blk_var16\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var17: get_symbol(&libs, b"FLA_Sylv_nn_blk_var17\0").map(|sym| *sym),
            FLA_Sylv_nn_blk_var18: get_symbol(&libs, b"FLA_Sylv_nn_blk_var18\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var1: get_symbol(&libs, b"FLA_Sylv_nn_opt_var1\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var2: get_symbol(&libs, b"FLA_Sylv_nn_opt_var2\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var3: get_symbol(&libs, b"FLA_Sylv_nn_opt_var3\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var4: get_symbol(&libs, b"FLA_Sylv_nn_opt_var4\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var5: get_symbol(&libs, b"FLA_Sylv_nn_opt_var5\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var6: get_symbol(&libs, b"FLA_Sylv_nn_opt_var6\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var7: get_symbol(&libs, b"FLA_Sylv_nn_opt_var7\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var8: get_symbol(&libs, b"FLA_Sylv_nn_opt_var8\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var9: get_symbol(&libs, b"FLA_Sylv_nn_opt_var9\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var10: get_symbol(&libs, b"FLA_Sylv_nn_opt_var10\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var11: get_symbol(&libs, b"FLA_Sylv_nn_opt_var11\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var12: get_symbol(&libs, b"FLA_Sylv_nn_opt_var12\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var13: get_symbol(&libs, b"FLA_Sylv_nn_opt_var13\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var14: get_symbol(&libs, b"FLA_Sylv_nn_opt_var14\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var15: get_symbol(&libs, b"FLA_Sylv_nn_opt_var15\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var16: get_symbol(&libs, b"FLA_Sylv_nn_opt_var16\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var17: get_symbol(&libs, b"FLA_Sylv_nn_opt_var17\0").map(|sym| *sym),
            FLA_Sylv_nn_opt_var18: get_symbol(&libs, b"FLA_Sylv_nn_opt_var18\0").map(|sym| *sym),
            FLA_Sylv_nn_ops_var1: get_symbol(&libs, b"FLA_Sylv_nn_ops_var1\0").map(|sym| *sym),
            FLA_Sylv_nn_opd_var1: get_symbol(&libs, b"FLA_Sylv_nn_opd_var1\0").map(|sym| *sym),
            FLA_Sylv_nn_opc_var1: get_symbol(&libs, b"FLA_Sylv_nn_opc_var1\0").map(|sym| *sym),
            FLA_Sylv_nn_opz_var1: get_symbol(&libs, b"FLA_Sylv_nn_opz_var1\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var1: get_symbol(&libs, b"FLA_Sylv_nh_blk_var1\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var2: get_symbol(&libs, b"FLA_Sylv_nh_blk_var2\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var3: get_symbol(&libs, b"FLA_Sylv_nh_blk_var3\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var4: get_symbol(&libs, b"FLA_Sylv_nh_blk_var4\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var5: get_symbol(&libs, b"FLA_Sylv_nh_blk_var5\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var6: get_symbol(&libs, b"FLA_Sylv_nh_blk_var6\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var7: get_symbol(&libs, b"FLA_Sylv_nh_blk_var7\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var8: get_symbol(&libs, b"FLA_Sylv_nh_blk_var8\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var9: get_symbol(&libs, b"FLA_Sylv_nh_blk_var9\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var10: get_symbol(&libs, b"FLA_Sylv_nh_blk_var10\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var11: get_symbol(&libs, b"FLA_Sylv_nh_blk_var11\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var12: get_symbol(&libs, b"FLA_Sylv_nh_blk_var12\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var13: get_symbol(&libs, b"FLA_Sylv_nh_blk_var13\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var14: get_symbol(&libs, b"FLA_Sylv_nh_blk_var14\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var15: get_symbol(&libs, b"FLA_Sylv_nh_blk_var15\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var16: get_symbol(&libs, b"FLA_Sylv_nh_blk_var16\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var17: get_symbol(&libs, b"FLA_Sylv_nh_blk_var17\0").map(|sym| *sym),
            FLA_Sylv_nh_blk_var18: get_symbol(&libs, b"FLA_Sylv_nh_blk_var18\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var1: get_symbol(&libs, b"FLA_Sylv_nh_opt_var1\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var2: get_symbol(&libs, b"FLA_Sylv_nh_opt_var2\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var3: get_symbol(&libs, b"FLA_Sylv_nh_opt_var3\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var4: get_symbol(&libs, b"FLA_Sylv_nh_opt_var4\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var5: get_symbol(&libs, b"FLA_Sylv_nh_opt_var5\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var6: get_symbol(&libs, b"FLA_Sylv_nh_opt_var6\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var7: get_symbol(&libs, b"FLA_Sylv_nh_opt_var7\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var8: get_symbol(&libs, b"FLA_Sylv_nh_opt_var8\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var9: get_symbol(&libs, b"FLA_Sylv_nh_opt_var9\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var10: get_symbol(&libs, b"FLA_Sylv_nh_opt_var10\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var11: get_symbol(&libs, b"FLA_Sylv_nh_opt_var11\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var12: get_symbol(&libs, b"FLA_Sylv_nh_opt_var12\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var13: get_symbol(&libs, b"FLA_Sylv_nh_opt_var13\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var14: get_symbol(&libs, b"FLA_Sylv_nh_opt_var14\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var15: get_symbol(&libs, b"FLA_Sylv_nh_opt_var15\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var16: get_symbol(&libs, b"FLA_Sylv_nh_opt_var16\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var17: get_symbol(&libs, b"FLA_Sylv_nh_opt_var17\0").map(|sym| *sym),
            FLA_Sylv_nh_opt_var18: get_symbol(&libs, b"FLA_Sylv_nh_opt_var18\0").map(|sym| *sym),
            FLA_Sylv_nh_ops_var1: get_symbol(&libs, b"FLA_Sylv_nh_ops_var1\0").map(|sym| *sym),
            FLA_Sylv_nh_opd_var1: get_symbol(&libs, b"FLA_Sylv_nh_opd_var1\0").map(|sym| *sym),
            FLA_Sylv_nh_opc_var1: get_symbol(&libs, b"FLA_Sylv_nh_opc_var1\0").map(|sym| *sym),
            FLA_Sylv_nh_opz_var1: get_symbol(&libs, b"FLA_Sylv_nh_opz_var1\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var1: get_symbol(&libs, b"FLA_Sylv_hn_blk_var1\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var2: get_symbol(&libs, b"FLA_Sylv_hn_blk_var2\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var3: get_symbol(&libs, b"FLA_Sylv_hn_blk_var3\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var4: get_symbol(&libs, b"FLA_Sylv_hn_blk_var4\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var5: get_symbol(&libs, b"FLA_Sylv_hn_blk_var5\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var6: get_symbol(&libs, b"FLA_Sylv_hn_blk_var6\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var7: get_symbol(&libs, b"FLA_Sylv_hn_blk_var7\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var8: get_symbol(&libs, b"FLA_Sylv_hn_blk_var8\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var9: get_symbol(&libs, b"FLA_Sylv_hn_blk_var9\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var10: get_symbol(&libs, b"FLA_Sylv_hn_blk_var10\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var11: get_symbol(&libs, b"FLA_Sylv_hn_blk_var11\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var12: get_symbol(&libs, b"FLA_Sylv_hn_blk_var12\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var13: get_symbol(&libs, b"FLA_Sylv_hn_blk_var13\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var14: get_symbol(&libs, b"FLA_Sylv_hn_blk_var14\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var15: get_symbol(&libs, b"FLA_Sylv_hn_blk_var15\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var16: get_symbol(&libs, b"FLA_Sylv_hn_blk_var16\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var17: get_symbol(&libs, b"FLA_Sylv_hn_blk_var17\0").map(|sym| *sym),
            FLA_Sylv_hn_blk_var18: get_symbol(&libs, b"FLA_Sylv_hn_blk_var18\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var1: get_symbol(&libs, b"FLA_Sylv_hn_opt_var1\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var2: get_symbol(&libs, b"FLA_Sylv_hn_opt_var2\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var3: get_symbol(&libs, b"FLA_Sylv_hn_opt_var3\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var4: get_symbol(&libs, b"FLA_Sylv_hn_opt_var4\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var5: get_symbol(&libs, b"FLA_Sylv_hn_opt_var5\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var6: get_symbol(&libs, b"FLA_Sylv_hn_opt_var6\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var7: get_symbol(&libs, b"FLA_Sylv_hn_opt_var7\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var8: get_symbol(&libs, b"FLA_Sylv_hn_opt_var8\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var9: get_symbol(&libs, b"FLA_Sylv_hn_opt_var9\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var10: get_symbol(&libs, b"FLA_Sylv_hn_opt_var10\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var11: get_symbol(&libs, b"FLA_Sylv_hn_opt_var11\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var12: get_symbol(&libs, b"FLA_Sylv_hn_opt_var12\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var13: get_symbol(&libs, b"FLA_Sylv_hn_opt_var13\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var14: get_symbol(&libs, b"FLA_Sylv_hn_opt_var14\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var15: get_symbol(&libs, b"FLA_Sylv_hn_opt_var15\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var16: get_symbol(&libs, b"FLA_Sylv_hn_opt_var16\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var17: get_symbol(&libs, b"FLA_Sylv_hn_opt_var17\0").map(|sym| *sym),
            FLA_Sylv_hn_opt_var18: get_symbol(&libs, b"FLA_Sylv_hn_opt_var18\0").map(|sym| *sym),
            FLA_Sylv_hn_ops_var1: get_symbol(&libs, b"FLA_Sylv_hn_ops_var1\0").map(|sym| *sym),
            FLA_Sylv_hn_opd_var1: get_symbol(&libs, b"FLA_Sylv_hn_opd_var1\0").map(|sym| *sym),
            FLA_Sylv_hn_opc_var1: get_symbol(&libs, b"FLA_Sylv_hn_opc_var1\0").map(|sym| *sym),
            FLA_Sylv_hn_opz_var1: get_symbol(&libs, b"FLA_Sylv_hn_opz_var1\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var1: get_symbol(&libs, b"FLA_Sylv_hh_blk_var1\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var2: get_symbol(&libs, b"FLA_Sylv_hh_blk_var2\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var3: get_symbol(&libs, b"FLA_Sylv_hh_blk_var3\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var4: get_symbol(&libs, b"FLA_Sylv_hh_blk_var4\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var5: get_symbol(&libs, b"FLA_Sylv_hh_blk_var5\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var6: get_symbol(&libs, b"FLA_Sylv_hh_blk_var6\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var7: get_symbol(&libs, b"FLA_Sylv_hh_blk_var7\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var8: get_symbol(&libs, b"FLA_Sylv_hh_blk_var8\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var9: get_symbol(&libs, b"FLA_Sylv_hh_blk_var9\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var10: get_symbol(&libs, b"FLA_Sylv_hh_blk_var10\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var11: get_symbol(&libs, b"FLA_Sylv_hh_blk_var11\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var12: get_symbol(&libs, b"FLA_Sylv_hh_blk_var12\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var13: get_symbol(&libs, b"FLA_Sylv_hh_blk_var13\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var14: get_symbol(&libs, b"FLA_Sylv_hh_blk_var14\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var15: get_symbol(&libs, b"FLA_Sylv_hh_blk_var15\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var16: get_symbol(&libs, b"FLA_Sylv_hh_blk_var16\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var17: get_symbol(&libs, b"FLA_Sylv_hh_blk_var17\0").map(|sym| *sym),
            FLA_Sylv_hh_blk_var18: get_symbol(&libs, b"FLA_Sylv_hh_blk_var18\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var1: get_symbol(&libs, b"FLA_Sylv_hh_opt_var1\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var2: get_symbol(&libs, b"FLA_Sylv_hh_opt_var2\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var3: get_symbol(&libs, b"FLA_Sylv_hh_opt_var3\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var4: get_symbol(&libs, b"FLA_Sylv_hh_opt_var4\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var5: get_symbol(&libs, b"FLA_Sylv_hh_opt_var5\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var6: get_symbol(&libs, b"FLA_Sylv_hh_opt_var6\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var7: get_symbol(&libs, b"FLA_Sylv_hh_opt_var7\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var8: get_symbol(&libs, b"FLA_Sylv_hh_opt_var8\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var9: get_symbol(&libs, b"FLA_Sylv_hh_opt_var9\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var10: get_symbol(&libs, b"FLA_Sylv_hh_opt_var10\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var11: get_symbol(&libs, b"FLA_Sylv_hh_opt_var11\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var12: get_symbol(&libs, b"FLA_Sylv_hh_opt_var12\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var13: get_symbol(&libs, b"FLA_Sylv_hh_opt_var13\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var14: get_symbol(&libs, b"FLA_Sylv_hh_opt_var14\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var15: get_symbol(&libs, b"FLA_Sylv_hh_opt_var15\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var16: get_symbol(&libs, b"FLA_Sylv_hh_opt_var16\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var17: get_symbol(&libs, b"FLA_Sylv_hh_opt_var17\0").map(|sym| *sym),
            FLA_Sylv_hh_opt_var18: get_symbol(&libs, b"FLA_Sylv_hh_opt_var18\0").map(|sym| *sym),
            FLA_Sylv_hh_ops_var1: get_symbol(&libs, b"FLA_Sylv_hh_ops_var1\0").map(|sym| *sym),
            FLA_Sylv_hh_opd_var1: get_symbol(&libs, b"FLA_Sylv_hh_opd_var1\0").map(|sym| *sym),
            FLA_Sylv_hh_opc_var1: get_symbol(&libs, b"FLA_Sylv_hh_opc_var1\0").map(|sym| *sym),
            FLA_Sylv_hh_opz_var1: get_symbol(&libs, b"FLA_Sylv_hh_opz_var1\0").map(|sym| *sym),
            FLA_Sylv_internal: get_symbol(&libs, b"FLA_Sylv_internal\0").map(|sym| *sym),
            FLA_Sylv_nn: get_symbol(&libs, b"FLA_Sylv_nn\0").map(|sym| *sym),
            FLA_Sylv_nh: get_symbol(&libs, b"FLA_Sylv_nh\0").map(|sym| *sym),
            FLA_Sylv_hn: get_symbol(&libs, b"FLA_Sylv_hn\0").map(|sym| *sym),
            FLA_Sylv_hh: get_symbol(&libs, b"FLA_Sylv_hh\0").map(|sym| *sym),
            FLA_Ttmm_l_blk_var1: get_symbol(&libs, b"FLA_Ttmm_l_blk_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_blk_var2: get_symbol(&libs, b"FLA_Ttmm_l_blk_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_blk_var3: get_symbol(&libs, b"FLA_Ttmm_l_blk_var3\0").map(|sym| *sym),
            FLA_Ttmm_l_unb_var1: get_symbol(&libs, b"FLA_Ttmm_l_unb_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_unb_var2: get_symbol(&libs, b"FLA_Ttmm_l_unb_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_unb_var3: get_symbol(&libs, b"FLA_Ttmm_l_unb_var3\0").map(|sym| *sym),
            FLA_Ttmm_l_opt_var1: get_symbol(&libs, b"FLA_Ttmm_l_opt_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_ops_var1: get_symbol(&libs, b"FLA_Ttmm_l_ops_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_opd_var1: get_symbol(&libs, b"FLA_Ttmm_l_opd_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_opc_var1: get_symbol(&libs, b"FLA_Ttmm_l_opc_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_opz_var1: get_symbol(&libs, b"FLA_Ttmm_l_opz_var1\0").map(|sym| *sym),
            FLA_Ttmm_l_opt_var2: get_symbol(&libs, b"FLA_Ttmm_l_opt_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_ops_var2: get_symbol(&libs, b"FLA_Ttmm_l_ops_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_opd_var2: get_symbol(&libs, b"FLA_Ttmm_l_opd_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_opc_var2: get_symbol(&libs, b"FLA_Ttmm_l_opc_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_opz_var2: get_symbol(&libs, b"FLA_Ttmm_l_opz_var2\0").map(|sym| *sym),
            FLA_Ttmm_l_opt_var3: get_symbol(&libs, b"FLA_Ttmm_l_opt_var3\0").map(|sym| *sym),
            FLA_Ttmm_l_ops_var3: get_symbol(&libs, b"FLA_Ttmm_l_ops_var3\0").map(|sym| *sym),
            FLA_Ttmm_l_opd_var3: get_symbol(&libs, b"FLA_Ttmm_l_opd_var3\0").map(|sym| *sym),
            FLA_Ttmm_l_opc_var3: get_symbol(&libs, b"FLA_Ttmm_l_opc_var3\0").map(|sym| *sym),
            FLA_Ttmm_l_opz_var3: get_symbol(&libs, b"FLA_Ttmm_l_opz_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_blk_var1: get_symbol(&libs, b"FLA_Ttmm_u_blk_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_blk_var2: get_symbol(&libs, b"FLA_Ttmm_u_blk_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_blk_var3: get_symbol(&libs, b"FLA_Ttmm_u_blk_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_unb_var1: get_symbol(&libs, b"FLA_Ttmm_u_unb_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_unb_var2: get_symbol(&libs, b"FLA_Ttmm_u_unb_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_unb_var3: get_symbol(&libs, b"FLA_Ttmm_u_unb_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_opt_var1: get_symbol(&libs, b"FLA_Ttmm_u_opt_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_ops_var1: get_symbol(&libs, b"FLA_Ttmm_u_ops_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_opd_var1: get_symbol(&libs, b"FLA_Ttmm_u_opd_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_opc_var1: get_symbol(&libs, b"FLA_Ttmm_u_opc_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_opz_var1: get_symbol(&libs, b"FLA_Ttmm_u_opz_var1\0").map(|sym| *sym),
            FLA_Ttmm_u_opt_var2: get_symbol(&libs, b"FLA_Ttmm_u_opt_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_ops_var2: get_symbol(&libs, b"FLA_Ttmm_u_ops_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_opd_var2: get_symbol(&libs, b"FLA_Ttmm_u_opd_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_opc_var2: get_symbol(&libs, b"FLA_Ttmm_u_opc_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_opz_var2: get_symbol(&libs, b"FLA_Ttmm_u_opz_var2\0").map(|sym| *sym),
            FLA_Ttmm_u_opt_var3: get_symbol(&libs, b"FLA_Ttmm_u_opt_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_ops_var3: get_symbol(&libs, b"FLA_Ttmm_u_ops_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_opd_var3: get_symbol(&libs, b"FLA_Ttmm_u_opd_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_opc_var3: get_symbol(&libs, b"FLA_Ttmm_u_opc_var3\0").map(|sym| *sym),
            FLA_Ttmm_u_opz_var3: get_symbol(&libs, b"FLA_Ttmm_u_opz_var3\0").map(|sym| *sym),
            FLA_Ttmm_internal: get_symbol(&libs, b"FLA_Ttmm_internal\0").map(|sym| *sym),
            FLA_Ttmm_l: get_symbol(&libs, b"FLA_Ttmm_l\0").map(|sym| *sym),
            FLA_Ttmm_u: get_symbol(&libs, b"FLA_Ttmm_u\0").map(|sym| *sym),
            FLA_UDdate_UT_blk_var1: get_symbol(&libs, b"FLA_UDdate_UT_blk_var1\0").map(|sym| *sym),
            FLA_UDdate_UT_blk_var2: get_symbol(&libs, b"FLA_UDdate_UT_blk_var2\0").map(|sym| *sym),
            FLA_UDdate_UT_unb_var1: get_symbol(&libs, b"FLA_UDdate_UT_unb_var1\0").map(|sym| *sym),
            FLA_UDdate_UT_opt_var1: get_symbol(&libs, b"FLA_UDdate_UT_opt_var1\0").map(|sym| *sym),
            FLA_UDdate_UT_ops_var1: get_symbol(&libs, b"FLA_UDdate_UT_ops_var1\0").map(|sym| *sym),
            FLA_UDdate_UT_opd_var1: get_symbol(&libs, b"FLA_UDdate_UT_opd_var1\0").map(|sym| *sym),
            FLA_UDdate_UT_opc_var1: get_symbol(&libs, b"FLA_UDdate_UT_opc_var1\0").map(|sym| *sym),
            FLA_UDdate_UT_opz_var1: get_symbol(&libs, b"FLA_UDdate_UT_opz_var1\0").map(|sym| *sym),
            FLA_UDdate_UT: get_symbol(&libs, b"FLA_UDdate_UT\0").map(|sym| *sym),
            FLA_UDdate_UT_internal: get_symbol(&libs, b"FLA_UDdate_UT_internal\0").map(|sym| *sym),
            FLA_UDdate_UT_create_T: get_symbol(&libs, b"FLA_UDdate_UT_create_T\0").map(|sym| *sym),
            FLA_UDdate_UT_update_rhs: get_symbol(&libs, b"FLA_UDdate_UT_update_rhs\0")
                .map(|sym| *sym),
            FLA_UDdate_UT_solve: get_symbol(&libs, b"FLA_UDdate_UT_solve\0").map(|sym| *sym),
            FLASH_UDdate_UT_inc: get_symbol(&libs, b"FLASH_UDdate_UT_inc\0").map(|sym| *sym),
            FLA_UDdate_UT_inc_blk_var1: get_symbol(&libs, b"FLA_UDdate_UT_inc_blk_var1\0")
                .map(|sym| *sym),
            FLASH_UDdate_UT_inc_create_hier_matrices: get_symbol(
                &libs,
                b"FLASH_UDdate_UT_inc_create_hier_matrices\0",
            )
            .map(|sym| *sym),
            FLASH_UDdate_UT_inc_determine_alg_blocksize: get_symbol(
                &libs,
                b"FLASH_UDdate_UT_inc_determine_alg_blocksize\0",
            )
            .map(|sym| *sym),
            FLASH_UDdate_UT_inc_update_rhs: get_symbol(&libs, b"FLASH_UDdate_UT_inc_update_rhs\0")
                .map(|sym| *sym),
            FLASH_UDdate_UT_inc_solve: get_symbol(&libs, b"FLASH_UDdate_UT_inc_solve\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_unb_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fc_unb_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_blk_var2: get_symbol(&libs, b"FLA_Accum_T_UT_fc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_opt_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fc_opt_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_ops_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fc_ops_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_opd_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fc_opd_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_opc_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fc_opc_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fc_opz_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fc_opz_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_unb_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fr_unb_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_blk_var2: get_symbol(&libs, b"FLA_Accum_T_UT_fr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_opt_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fr_opt_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_ops_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fr_ops_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_opd_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fr_opd_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_opc_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fr_opc_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_fr_opz_var1: get_symbol(&libs, b"FLA_Accum_T_UT_fr_opz_var1\0")
                .map(|sym| *sym),
            FLA_Accum_T_UT_internal: get_symbol(&libs, b"FLA_Accum_T_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_G_lf_opt_var1: get_symbol(&libs, b"FLA_Apply_G_lf_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_lf_blk_var3: get_symbol(&libs, b"FLA_Apply_G_lf_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_lb_opt_var1: get_symbol(&libs, b"FLA_Apply_G_lb_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_lb_ops_var1: get_symbol(&libs, b"FLA_Apply_G_lb_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_lb_opd_var1: get_symbol(&libs, b"FLA_Apply_G_lb_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_lb_opc_var1: get_symbol(&libs, b"FLA_Apply_G_lb_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_lb_opz_var1: get_symbol(&libs, b"FLA_Apply_G_lb_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var1: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var1: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var1: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var1: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var1: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var1: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var1: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var1: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var1: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var1: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var1: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var1: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var1: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var1: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var1: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var2: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var2: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var2: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var2: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var2: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var2: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var2: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var2: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var2: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var2: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var2: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var2: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var2: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var2: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var2: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var2\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var3: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var3: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var3: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var3: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var3: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var3: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var3: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var3: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var3: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var3: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var3: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var3: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var3: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var3: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var3: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var4: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var4: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var4: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var4: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var4: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var4: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var4: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var4: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var4: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var4: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var4: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var4: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var4: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var4: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var4: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var4\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var5: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var5: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var5: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var5: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var5: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var5: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var5: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var5: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var5: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var5: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var5: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var5: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var5: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var5: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var5: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var5\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var6: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var6: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var6: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var6: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var6: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var6: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var6: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var6: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var6: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var6: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var6: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var6: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var6: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var6: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var6: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var6\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var7: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var7: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var7: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var7: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var7: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var7: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var7: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var7: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var7: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var7: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var7: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var7: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var7: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var7: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var7: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var7\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var8: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var8: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var8: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var8: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var8: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var8: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var8: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var8: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var8: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var8: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var8: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var8: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var8: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var8: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var8: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var8\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opt_var9: get_symbol(&libs, b"FLA_Apply_G_rf_opt_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ops_var9: get_symbol(&libs, b"FLA_Apply_G_rf_ops_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opd_var9: get_symbol(&libs, b"FLA_Apply_G_rf_opd_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opc_var9: get_symbol(&libs, b"FLA_Apply_G_rf_opc_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_opz_var9: get_symbol(&libs, b"FLA_Apply_G_rf_opz_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var9: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var9: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var9: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var9: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var9: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var9: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var9: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var9: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var9: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var9: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var9\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var3b: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var3b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var5b: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var5b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var6b: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var6b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var8b: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var8b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bhs_var3: get_symbol(&libs, b"FLA_Apply_G_rf_bhs_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bhd_var3: get_symbol(&libs, b"FLA_Apply_G_rf_bhd_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bhc_var3: get_symbol(&libs, b"FLA_Apply_G_rf_bhc_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bhz_var3: get_symbol(&libs, b"FLA_Apply_G_rf_bhz_var3\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asm_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_asm_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_ass_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_ass_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asd_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_asd_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asc_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_asc_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_asz_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_asz_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blk_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_blk_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bls_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_bls_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_bld_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_bld_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blc_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_blc_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rf_blz_var9b: get_symbol(&libs, b"FLA_Apply_G_rf_blz_var9b\0")
                .map(|sym| *sym),
            FLA_Apply_G_rb_opt_var1: get_symbol(&libs, b"FLA_Apply_G_rb_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rb_ops_var1: get_symbol(&libs, b"FLA_Apply_G_rb_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rb_opd_var1: get_symbol(&libs, b"FLA_Apply_G_rb_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rb_opc_var1: get_symbol(&libs, b"FLA_Apply_G_rb_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G_rb_opz_var1: get_symbol(&libs, b"FLA_Apply_G_rb_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_G: get_symbol(&libs, b"FLA_Apply_G\0").map(|sym| *sym),
            FLA_Apply_G_internal: get_symbol(&libs, b"FLA_Apply_G_internal\0").map(|sym| *sym),
            FLA_Givens2: get_symbol(&libs, b"FLA_Givens2\0").map(|sym| *sym),
            FLA_Givens2_ops: get_symbol(&libs, b"FLA_Givens2_ops\0").map(|sym| *sym),
            FLA_Givens2_opd: get_symbol(&libs, b"FLA_Givens2_opd\0").map(|sym| *sym),
            FLA_Apply_GTG: get_symbol(&libs, b"FLA_Apply_GTG\0").map(|sym| *sym),
            FLA_Apply_GTG_ops: get_symbol(&libs, b"FLA_Apply_GTG_ops\0").map(|sym| *sym),
            FLA_Apply_GTG_opd: get_symbol(&libs, b"FLA_Apply_GTG_opd\0").map(|sym| *sym),
            FLA_Apply_H2_UT_l_unb_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_l_unb_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_l_opt_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_l_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_l_ops_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_l_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_l_opd_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_l_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_l_opc_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_l_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_l_opz_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_l_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_r_unb_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_r_unb_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_r_opt_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_r_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_r_ops_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_r_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_r_opd_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_r_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_r_opc_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_r_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_r_opz_var1: get_symbol(&libs, b"FLA_Apply_H2_UT_r_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_H2_UT_internal: get_symbol(&libs, b"FLA_Apply_H2_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_l_unb_var1: get_symbol(&libs, b"FLA_Apply_HUD_UT_l_unb_var1\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_l_opt_var1: get_symbol(&libs, b"FLA_Apply_HUD_UT_l_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_l_ops_var1: get_symbol(&libs, b"FLA_Apply_HUD_UT_l_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_l_opd_var1: get_symbol(&libs, b"FLA_Apply_HUD_UT_l_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_l_opc_var1: get_symbol(&libs, b"FLA_Apply_HUD_UT_l_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_l_opz_var1: get_symbol(&libs, b"FLA_Apply_HUD_UT_l_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_HUD_UT_internal: get_symbol(&libs, b"FLA_Apply_HUD_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnbr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhfr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lhbr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhbr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rhfr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnbr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfr_blk_var1: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfr_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfr_blk_var2: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfr_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_rnfr_blk_var3: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfr_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_internal: get_symbol(&libs, b"FLA_Apply_Q_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_lnfc: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lnfr: get_symbol(&libs, b"FLA_Apply_Q_UT_lnfr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lnbc: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lnbr: get_symbol(&libs, b"FLA_Apply_Q_UT_lnbr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lhfc: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lhfr: get_symbol(&libs, b"FLA_Apply_Q_UT_lhfr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lhbc: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_lhbr: get_symbol(&libs, b"FLA_Apply_Q_UT_lhbr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rhbc: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rhbr: get_symbol(&libs, b"FLA_Apply_Q_UT_rhbr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rhfc: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rhfr: get_symbol(&libs, b"FLA_Apply_Q_UT_rhfr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rnbc: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rnbr: get_symbol(&libs, b"FLA_Apply_Q_UT_rnbr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rnfc: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfc\0").map(|sym| *sym),
            FLA_Apply_Q_UT_rnfr: get_symbol(&libs, b"FLA_Apply_Q_UT_rnfr\0").map(|sym| *sym),
            FLA_Apply_Q_UT_create_workspace: get_symbol(
                &libs,
                b"FLA_Apply_Q_UT_create_workspace\0",
            )
            .map(|sym| *sym),
            FLA_Apply_Q_UT_create_workspace_side: get_symbol(
                &libs,
                b"FLA_Apply_Q_UT_create_workspace_side\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_Q_UT: get_symbol(&libs, b"FLASH_Apply_Q_UT\0").map(|sym| *sym),
            FLASH_Apply_Q_UT_create_workspace: get_symbol(
                &libs,
                b"FLASH_Apply_Q_UT_create_workspace\0",
            )
            .map(|sym| *sym),
            FLA_Apply_Q2_UT_lhfc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q2_UT_lhfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_lhfc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q2_UT_lhfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_lhfc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q2_UT_lhfc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_lnfc_blk_var1: get_symbol(&libs, b"FLA_Apply_Q2_UT_lnfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_lnfc_blk_var2: get_symbol(&libs, b"FLA_Apply_Q2_UT_lnfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_lnfc_blk_var3: get_symbol(&libs, b"FLA_Apply_Q2_UT_lnfc_blk_var3\0")
                .map(|sym| *sym),
            FLASH_Apply_Q2_UT: get_symbol(&libs, b"FLASH_Apply_Q2_UT\0").map(|sym| *sym),
            FLA_Apply_Q2_UT_internal: get_symbol(&libs, b"FLA_Apply_Q2_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_Q2_UT_lhfc: get_symbol(&libs, b"FLA_Apply_Q2_UT_lhfc\0").map(|sym| *sym),
            FLA_Apply_Q2_UT_lnfc: get_symbol(&libs, b"FLA_Apply_Q2_UT_lnfc\0").map(|sym| *sym),
            FLA_Apply_CAQ2_UT_lhfc_blk_var1: get_symbol(
                &libs,
                b"FLA_Apply_CAQ2_UT_lhfc_blk_var1\0",
            )
            .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_lhfc_blk_var2: get_symbol(
                &libs,
                b"FLA_Apply_CAQ2_UT_lhfc_blk_var2\0",
            )
            .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_lhfc_blk_var3: get_symbol(
                &libs,
                b"FLA_Apply_CAQ2_UT_lhfc_blk_var3\0",
            )
            .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_internal: get_symbol(&libs, b"FLA_Apply_CAQ2_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ2_UT_lhfc: get_symbol(&libs, b"FLA_Apply_CAQ2_UT_lhfc\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_lhfc_blk_var1: get_symbol(&libs, b"FLA_Apply_QUD_UT_lhfc_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT_lhfc_blk_var2: get_symbol(&libs, b"FLA_Apply_QUD_UT_lhfc_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT_lhfc_blk_var3: get_symbol(&libs, b"FLA_Apply_QUD_UT_lhfc_blk_var3\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT: get_symbol(&libs, b"FLA_Apply_QUD_UT\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_internal: get_symbol(&libs, b"FLA_Apply_QUD_UT_internal\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT_lhfc: get_symbol(&libs, b"FLA_Apply_QUD_UT_lhfc\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_create_workspace: get_symbol(
                &libs,
                b"FLA_Apply_QUD_UT_create_workspace\0",
            )
            .map(|sym| *sym),
            FLA_Apply_Q_UT_inc_lhfc_blk_var1: get_symbol(
                &libs,
                b"FLA_Apply_Q_UT_inc_lhfc_blk_var1\0",
            )
            .map(|sym| *sym),
            FLA_Apply_Q_UT_inc_lnfc_blk_var1: get_symbol(
                &libs,
                b"FLA_Apply_Q_UT_inc_lnfc_blk_var1\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_Q_UT_inc: get_symbol(&libs, b"FLASH_Apply_Q_UT_inc\0").map(|sym| *sym),
            FLASH_Apply_Q_UT_inc_create_workspace: get_symbol(
                &libs,
                b"FLASH_Apply_Q_UT_inc_create_workspace\0",
            )
            .map(|sym| *sym),
            FLA_Apply_Q_UT_inc_internal: get_symbol(&libs, b"FLA_Apply_Q_UT_inc_internal\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_inc_lhfc: get_symbol(&libs, b"FLA_Apply_Q_UT_inc_lhfc\0")
                .map(|sym| *sym),
            FLA_Apply_Q_UT_inc_lnfc: get_symbol(&libs, b"FLA_Apply_Q_UT_inc_lnfc\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ_UT_inc_lhfc_blk_var1: get_symbol(
                &libs,
                b"FLA_Apply_CAQ_UT_inc_lhfc_blk_var1\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_CAQ_UT_inc: get_symbol(&libs, b"FLASH_Apply_CAQ_UT_inc\0").map(|sym| *sym),
            FLA_Apply_CAQ_UT_inc_apply_panels: get_symbol(
                &libs,
                b"FLA_Apply_CAQ_UT_inc_apply_panels\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_CAQ_UT_inc_create_workspace: get_symbol(
                &libs,
                b"FLASH_Apply_CAQ_UT_inc_create_workspace\0",
            )
            .map(|sym| *sym),
            FLA_Apply_CAQ_UT_inc_internal: get_symbol(&libs, b"FLA_Apply_CAQ_UT_inc_internal\0")
                .map(|sym| *sym),
            FLA_Apply_CAQ_UT_inc_lhfc: get_symbol(&libs, b"FLA_Apply_CAQ_UT_inc_lhfc\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT_inc_lhfc_blk_var1: get_symbol(
                &libs,
                b"FLA_Apply_QUD_UT_inc_lhfc_blk_var1\0",
            )
            .map(|sym| *sym),
            FLASH_Apply_QUD_UT_inc: get_symbol(&libs, b"FLASH_Apply_QUD_UT_inc\0").map(|sym| *sym),
            FLA_Apply_QUD_UT_inc_internal: get_symbol(&libs, b"FLA_Apply_QUD_UT_inc_internal\0")
                .map(|sym| *sym),
            FLA_Apply_QUD_UT_inc_lhfc: get_symbol(&libs, b"FLA_Apply_QUD_UT_inc_lhfc\0")
                .map(|sym| *sym),
            FLASH_Apply_QUD_UT_inc_create_workspace: get_symbol(
                &libs,
                b"FLASH_Apply_QUD_UT_inc_create_workspace\0",
            )
            .map(|sym| *sym),
            FLA_Apply_pivots_ln_blk_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_blk_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_blk_var2: get_symbol(&libs, b"FLA_Apply_pivots_ln_blk_var2\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_opt_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_opi_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_opi_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_ops_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_opd_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_opc_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln_opz_var1: get_symbol(&libs, b"FLA_Apply_pivots_ln_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_lt_opt_var1: get_symbol(&libs, b"FLA_Apply_pivots_lt_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_rn_opt_var1: get_symbol(&libs, b"FLA_Apply_pivots_rn_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_rn_ops_var1: get_symbol(&libs, b"FLA_Apply_pivots_rn_ops_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_rn_opd_var1: get_symbol(&libs, b"FLA_Apply_pivots_rn_opd_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_rn_opc_var1: get_symbol(&libs, b"FLA_Apply_pivots_rn_opc_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_rn_opz_var1: get_symbol(&libs, b"FLA_Apply_pivots_rn_opz_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_rt_opt_var1: get_symbol(&libs, b"FLA_Apply_pivots_rt_opt_var1\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_internal: get_symbol(&libs, b"FLA_Apply_pivots_internal\0")
                .map(|sym| *sym),
            FLA_Apply_pivots_ln: get_symbol(&libs, b"FLA_Apply_pivots_ln\0").map(|sym| *sym),
            FLA_Apply_pivots_lt: get_symbol(&libs, b"FLA_Apply_pivots_lt\0").map(|sym| *sym),
            FLA_Apply_pivots_rn: get_symbol(&libs, b"FLA_Apply_pivots_rn\0").map(|sym| *sym),
            FLA_Apply_pivots_rt: get_symbol(&libs, b"FLA_Apply_pivots_rt\0").map(|sym| *sym),
            FLA_Eig_gest_il_blk_var1: get_symbol(&libs, b"FLA_Eig_gest_il_blk_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_blk_var2: get_symbol(&libs, b"FLA_Eig_gest_il_blk_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_blk_var3: get_symbol(&libs, b"FLA_Eig_gest_il_blk_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_blk_var4: get_symbol(&libs, b"FLA_Eig_gest_il_blk_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_blk_var5: get_symbol(&libs, b"FLA_Eig_gest_il_blk_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_unb_var1: get_symbol(&libs, b"FLA_Eig_gest_il_unb_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_unb_var2: get_symbol(&libs, b"FLA_Eig_gest_il_unb_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_unb_var3: get_symbol(&libs, b"FLA_Eig_gest_il_unb_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_unb_var4: get_symbol(&libs, b"FLA_Eig_gest_il_unb_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_unb_var5: get_symbol(&libs, b"FLA_Eig_gest_il_unb_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opt_var1: get_symbol(&libs, b"FLA_Eig_gest_il_opt_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_ops_var1: get_symbol(&libs, b"FLA_Eig_gest_il_ops_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opd_var1: get_symbol(&libs, b"FLA_Eig_gest_il_opd_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opc_var1: get_symbol(&libs, b"FLA_Eig_gest_il_opc_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opz_var1: get_symbol(&libs, b"FLA_Eig_gest_il_opz_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opt_var2: get_symbol(&libs, b"FLA_Eig_gest_il_opt_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_ops_var2: get_symbol(&libs, b"FLA_Eig_gest_il_ops_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opd_var2: get_symbol(&libs, b"FLA_Eig_gest_il_opd_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opc_var2: get_symbol(&libs, b"FLA_Eig_gest_il_opc_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opz_var2: get_symbol(&libs, b"FLA_Eig_gest_il_opz_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opt_var3: get_symbol(&libs, b"FLA_Eig_gest_il_opt_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_ops_var3: get_symbol(&libs, b"FLA_Eig_gest_il_ops_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opd_var3: get_symbol(&libs, b"FLA_Eig_gest_il_opd_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opc_var3: get_symbol(&libs, b"FLA_Eig_gest_il_opc_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opz_var3: get_symbol(&libs, b"FLA_Eig_gest_il_opz_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opt_var4: get_symbol(&libs, b"FLA_Eig_gest_il_opt_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_ops_var4: get_symbol(&libs, b"FLA_Eig_gest_il_ops_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opd_var4: get_symbol(&libs, b"FLA_Eig_gest_il_opd_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opc_var4: get_symbol(&libs, b"FLA_Eig_gest_il_opc_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opz_var4: get_symbol(&libs, b"FLA_Eig_gest_il_opz_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opt_var5: get_symbol(&libs, b"FLA_Eig_gest_il_opt_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_ops_var5: get_symbol(&libs, b"FLA_Eig_gest_il_ops_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opd_var5: get_symbol(&libs, b"FLA_Eig_gest_il_opd_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opc_var5: get_symbol(&libs, b"FLA_Eig_gest_il_opc_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_il_opz_var5: get_symbol(&libs, b"FLA_Eig_gest_il_opz_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_blk_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_blk_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_blk_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_blk_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_blk_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_blk_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_blk_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_blk_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_blk_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_blk_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_unb_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_unb_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_unb_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_unb_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_unb_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_unb_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_unb_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_unb_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_unb_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_unb_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opt_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_opt_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_ops_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_ops_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opd_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_opd_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opc_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_opc_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opz_var1: get_symbol(&libs, b"FLA_Eig_gest_iu_opz_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opt_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_opt_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_ops_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_ops_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opd_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_opd_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opc_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_opc_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opz_var2: get_symbol(&libs, b"FLA_Eig_gest_iu_opz_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opt_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_opt_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_ops_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_ops_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opd_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_opd_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opc_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_opc_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opz_var3: get_symbol(&libs, b"FLA_Eig_gest_iu_opz_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opt_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_opt_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_ops_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_ops_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opd_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_opd_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opc_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_opc_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opz_var4: get_symbol(&libs, b"FLA_Eig_gest_iu_opz_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opt_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_opt_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_ops_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_ops_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opd_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_opd_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opc_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_opc_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_iu_opz_var5: get_symbol(&libs, b"FLA_Eig_gest_iu_opz_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_blk_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_blk_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_blk_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_blk_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_blk_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_blk_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_blk_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_blk_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_blk_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_blk_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_unb_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_unb_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_unb_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_unb_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_unb_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_unb_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_unb_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_unb_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_unb_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_unb_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opt_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_opt_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_ops_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_ops_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opd_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_opd_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opc_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_opc_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opz_var1: get_symbol(&libs, b"FLA_Eig_gest_nl_opz_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opt_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_opt_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_ops_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_ops_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opd_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_opd_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opc_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_opc_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opz_var2: get_symbol(&libs, b"FLA_Eig_gest_nl_opz_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opt_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_opt_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_ops_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_ops_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opd_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_opd_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opc_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_opc_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opz_var3: get_symbol(&libs, b"FLA_Eig_gest_nl_opz_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opt_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_opt_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_ops_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_ops_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opd_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_opd_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opc_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_opc_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opz_var4: get_symbol(&libs, b"FLA_Eig_gest_nl_opz_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opt_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_opt_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_ops_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_ops_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opd_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_opd_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opc_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_opc_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nl_opz_var5: get_symbol(&libs, b"FLA_Eig_gest_nl_opz_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_blk_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_blk_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_blk_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_blk_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_blk_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_blk_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_blk_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_blk_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_blk_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_blk_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_unb_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_unb_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_unb_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_unb_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_unb_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_unb_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_unb_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_unb_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_unb_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_unb_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opt_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_opt_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_ops_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_ops_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opd_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_opd_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opc_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_opc_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opz_var1: get_symbol(&libs, b"FLA_Eig_gest_nu_opz_var1\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opt_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_opt_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_ops_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_ops_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opd_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_opd_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opc_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_opc_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opz_var2: get_symbol(&libs, b"FLA_Eig_gest_nu_opz_var2\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opt_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_opt_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_ops_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_ops_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opd_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_opd_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opc_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_opc_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opz_var3: get_symbol(&libs, b"FLA_Eig_gest_nu_opz_var3\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opt_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_opt_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_ops_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_ops_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opd_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_opd_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opc_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_opc_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opz_var4: get_symbol(&libs, b"FLA_Eig_gest_nu_opz_var4\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opt_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_opt_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_ops_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_ops_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opd_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_opd_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opc_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_opc_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_nu_opz_var5: get_symbol(&libs, b"FLA_Eig_gest_nu_opz_var5\0")
                .map(|sym| *sym),
            FLA_Eig_gest_internal: get_symbol(&libs, b"FLA_Eig_gest_internal\0").map(|sym| *sym),
            FLA_Eig_gest_il: get_symbol(&libs, b"FLA_Eig_gest_il\0").map(|sym| *sym),
            FLA_Eig_gest_iu: get_symbol(&libs, b"FLA_Eig_gest_iu\0").map(|sym| *sym),
            FLA_Eig_gest_nl: get_symbol(&libs, b"FLA_Eig_gest_nl\0").map(|sym| *sym),
            FLA_Eig_gest_nu: get_symbol(&libs, b"FLA_Eig_gest_nu\0").map(|sym| *sym),
            FLASH_Obj_blocksizes_check: get_symbol(&libs, b"FLASH_Obj_blocksizes_check\0")
                .map(|sym| *sym),
            FLASH_Obj_create_helper_check: get_symbol(&libs, b"FLASH_Obj_create_helper_check\0")
                .map(|sym| *sym),
            FLASH_Obj_create_hierarchy_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_hierarchy_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_conf_to_check: get_symbol(&libs, b"FLASH_Obj_create_conf_to_check\0")
                .map(|sym| *sym),
            FLASH_Obj_create_hier_conf_to_flat_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_conf_to_flat_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_hier_conf_to_flat_ext_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_conf_to_flat_ext_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_flat_conf_to_hier_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_flat_conf_to_hier_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_hier_copy_of_flat_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_copy_of_flat_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_hier_copy_of_flat_ext_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_copy_of_flat_ext_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_flat_copy_of_hier_check: get_symbol(
                &libs,
                b"FLASH_Obj_create_flat_copy_of_hier_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_free_check: get_symbol(&libs, b"FLASH_Obj_free_check\0").map(|sym| *sym),
            FLASH_Obj_free_without_buffer_check: get_symbol(
                &libs,
                b"FLASH_Obj_free_without_buffer_check\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_free_hierarchy_check: get_symbol(&libs, b"FLASH_Obj_free_hierarchy_check\0")
                .map(|sym| *sym),
            FLASH_Obj_attach_buffer_check: get_symbol(&libs, b"FLASH_Obj_attach_buffer_check\0")
                .map(|sym| *sym),
            FLASH_Obj_attach_buffer_hierarchy_check: get_symbol(
                &libs,
                b"FLASH_Obj_attach_buffer_hierarchy_check\0",
            )
            .map(|sym| *sym),
            FLASH_Part_create_2x1: get_symbol(&libs, b"FLASH_Part_create_2x1\0").map(|sym| *sym),
            FLASH_Part_create_1x2: get_symbol(&libs, b"FLASH_Part_create_1x2\0").map(|sym| *sym),
            FLASH_Part_create_2x2: get_symbol(&libs, b"FLASH_Part_create_2x2\0").map(|sym| *sym),
            FLASH_Part_free_2x1: get_symbol(&libs, b"FLASH_Part_free_2x1\0").map(|sym| *sym),
            FLASH_Part_free_1x2: get_symbol(&libs, b"FLASH_Part_free_1x2\0").map(|sym| *sym),
            FLASH_Part_free_2x2: get_symbol(&libs, b"FLASH_Part_free_2x2\0").map(|sym| *sym),
            FLASH_Obj_adjust_views: get_symbol(&libs, b"FLASH_Obj_adjust_views\0").map(|sym| *sym),
            FLASH_Obj_adjust_views_hierarchy: get_symbol(
                &libs,
                b"FLASH_Obj_adjust_views_hierarchy\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_scalar_length: get_symbol(&libs, b"FLASH_Obj_scalar_length\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_width: get_symbol(&libs, b"FLASH_Obj_scalar_width\0").map(|sym| *sym),
            FLASH_Obj_scalar_min_dim: get_symbol(&libs, b"FLASH_Obj_scalar_min_dim\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_max_dim: get_symbol(&libs, b"FLASH_Obj_scalar_max_dim\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_vector_dim: get_symbol(&libs, b"FLASH_Obj_scalar_vector_dim\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_row_offset: get_symbol(&libs, b"FLASH_Obj_scalar_row_offset\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_col_offset: get_symbol(&libs, b"FLASH_Obj_scalar_col_offset\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_length_tl: get_symbol(&libs, b"FLASH_Obj_scalar_length_tl\0")
                .map(|sym| *sym),
            FLASH_Obj_scalar_width_tl: get_symbol(&libs, b"FLASH_Obj_scalar_width_tl\0")
                .map(|sym| *sym),
            FLASH_Obj_base_scalar_length: get_symbol(&libs, b"FLASH_Obj_base_scalar_length\0")
                .map(|sym| *sym),
            FLASH_Obj_base_scalar_width: get_symbol(&libs, b"FLASH_Obj_base_scalar_width\0")
                .map(|sym| *sym),
            FLASH_Obj_show: get_symbol(&libs, b"FLASH_Obj_show\0").map(|sym| *sym),
            FLASH_Obj_show_hierarchy: get_symbol(&libs, b"FLASH_Obj_show_hierarchy\0")
                .map(|sym| *sym),
            FLASH_Axpy_buffer_to_hier: get_symbol(&libs, b"FLASH_Axpy_buffer_to_hier\0")
                .map(|sym| *sym),
            FLASH_Axpy_hier_to_buffer: get_symbol(&libs, b"FLASH_Axpy_hier_to_buffer\0")
                .map(|sym| *sym),
            FLASH_Axpy_flat_to_hier: get_symbol(&libs, b"FLASH_Axpy_flat_to_hier\0")
                .map(|sym| *sym),
            FLASH_Axpy_hier_to_flat: get_symbol(&libs, b"FLASH_Axpy_hier_to_flat\0")
                .map(|sym| *sym),
            FLASH_Axpy_hierarchy: get_symbol(&libs, b"FLASH_Axpy_hierarchy\0").map(|sym| *sym),
            FLASH_Copy_buffer_to_hier: get_symbol(&libs, b"FLASH_Copy_buffer_to_hier\0")
                .map(|sym| *sym),
            FLASH_Copy_hier_to_buffer: get_symbol(&libs, b"FLASH_Copy_hier_to_buffer\0")
                .map(|sym| *sym),
            FLASH_Copy_flat_to_hier: get_symbol(&libs, b"FLASH_Copy_flat_to_hier\0")
                .map(|sym| *sym),
            FLASH_Copy_hier_to_flat: get_symbol(&libs, b"FLASH_Copy_hier_to_flat\0")
                .map(|sym| *sym),
            FLASH_Copy_hierarchy: get_symbol(&libs, b"FLASH_Copy_hierarchy\0").map(|sym| *sym),
            FLASH_Obj_datatype: get_symbol(&libs, b"FLASH_Obj_datatype\0").map(|sym| *sym),
            FLASH_Obj_depth: get_symbol(&libs, b"FLASH_Obj_depth\0").map(|sym| *sym),
            FLASH_Obj_blocksizes: get_symbol(&libs, b"FLASH_Obj_blocksizes\0").map(|sym| *sym),
            FLASH_Obj_create: get_symbol(&libs, b"FLASH_Obj_create\0").map(|sym| *sym),
            FLASH_Obj_create_ext: get_symbol(&libs, b"FLASH_Obj_create_ext\0").map(|sym| *sym),
            FLASH_Obj_create_without_buffer: get_symbol(
                &libs,
                b"FLASH_Obj_create_without_buffer\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_without_buffer_ext: get_symbol(
                &libs,
                b"FLASH_Obj_create_without_buffer_ext\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_helper: get_symbol(&libs, b"FLASH_Obj_create_helper\0")
                .map(|sym| *sym),
            FLASH_Obj_create_hierarchy: get_symbol(&libs, b"FLASH_Obj_create_hierarchy\0")
                .map(|sym| *sym),
            FLASH_Obj_create_conf_to: get_symbol(&libs, b"FLASH_Obj_create_conf_to\0")
                .map(|sym| *sym),
            FLASH_Obj_create_hier_conf_to_flat: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_conf_to_flat\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_hier_conf_to_flat_ext: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_conf_to_flat_ext\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_flat_conf_to_hier: get_symbol(
                &libs,
                b"FLASH_Obj_create_flat_conf_to_hier\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_copy_of: get_symbol(&libs, b"FLASH_Obj_create_copy_of\0")
                .map(|sym| *sym),
            FLASH_Obj_create_hier_copy_of_flat: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_copy_of_flat\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_hier_copy_of_flat_ext: get_symbol(
                &libs,
                b"FLASH_Obj_create_hier_copy_of_flat_ext\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_create_flat_copy_of_hier: get_symbol(
                &libs,
                b"FLASH_Obj_create_flat_copy_of_hier\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_free: get_symbol(&libs, b"FLASH_Obj_free\0").map(|sym| *sym),
            FLASH_Obj_free_hierarchy: get_symbol(&libs, b"FLASH_Obj_free_hierarchy\0")
                .map(|sym| *sym),
            FLASH_Obj_free_without_buffer: get_symbol(&libs, b"FLASH_Obj_free_without_buffer\0")
                .map(|sym| *sym),
            FLASH_Obj_attach_buffer: get_symbol(&libs, b"FLASH_Obj_attach_buffer\0")
                .map(|sym| *sym),
            FLASH_Obj_attach_buffer_hierarchy: get_symbol(
                &libs,
                b"FLASH_Obj_attach_buffer_hierarchy\0",
            )
            .map(|sym| *sym),
            FLASH_Obj_flatten: get_symbol(&libs, b"FLASH_Obj_flatten\0").map(|sym| *sym),
            FLASH_Obj_hierarchify: get_symbol(&libs, b"FLASH_Obj_hierarchify\0").map(|sym| *sym),
            FLASH_Obj_extract_buffer: get_symbol(&libs, b"FLASH_Obj_extract_buffer\0")
                .map(|sym| *sym),
            FLASH_print_struct: get_symbol(&libs, b"FLASH_print_struct\0").map(|sym| *sym),
            FLASH_print_struct_helper: get_symbol(&libs, b"FLASH_print_struct_helper\0")
                .map(|sym| *sym),
            FLASH_Max_elemwise_diff: get_symbol(&libs, b"FLASH_Max_elemwise_diff\0")
                .map(|sym| *sym),
            FLASH_Random_matrix: get_symbol(&libs, b"FLASH_Random_matrix\0").map(|sym| *sym),
            FLASH_Random_spd_matrix: get_symbol(&libs, b"FLASH_Random_spd_matrix\0")
                .map(|sym| *sym),
            FLASH_Norm1: get_symbol(&libs, b"FLASH_Norm1\0").map(|sym| *sym),
            FLASH_Obj_shift_diagonal: get_symbol(&libs, b"FLASH_Obj_shift_diagonal\0")
                .map(|sym| *sym),
            FLASH_Set: get_symbol(&libs, b"FLASH_Set\0").map(|sym| *sym),
            FLASH_Obj_create_diag_panel: get_symbol(&libs, b"FLASH_Obj_create_diag_panel\0")
                .map(|sym| *sym),
            FLASH_LU_find_zero_on_diagonal: get_symbol(&libs, b"FLASH_LU_find_zero_on_diagonal\0")
                .map(|sym| *sym),
            FLASH_Triangularize: get_symbol(&libs, b"FLASH_Triangularize\0").map(|sym| *sym),
            FLASH_Hermitianize: get_symbol(&libs, b"FLASH_Hermitianize\0").map(|sym| *sym),
            FLASH_LU_find_zero_on_diagonal_check: get_symbol(
                &libs,
                b"FLASH_LU_find_zero_on_diagonal_check\0",
            )
            .map(|sym| *sym),
            FLASH_Axpy: get_symbol(&libs, b"FLASH_Axpy\0").map(|sym| *sym),
            FLASH_Axpyt: get_symbol(&libs, b"FLASH_Axpyt\0").map(|sym| *sym),
            FLASH_Copy: get_symbol(&libs, b"FLASH_Copy\0").map(|sym| *sym),
            FLASH_Copyt: get_symbol(&libs, b"FLASH_Copyt\0").map(|sym| *sym),
            FLASH_Scal: get_symbol(&libs, b"FLASH_Scal\0").map(|sym| *sym),
            FLASH_Scalr: get_symbol(&libs, b"FLASH_Scalr\0").map(|sym| *sym),
            FLASH_Gemv: get_symbol(&libs, b"FLASH_Gemv\0").map(|sym| *sym),
            FLASH_Trsv: get_symbol(&libs, b"FLASH_Trsv\0").map(|sym| *sym),
            FLASH_Gemm: get_symbol(&libs, b"FLASH_Gemm\0").map(|sym| *sym),
            FLASH_Hemm: get_symbol(&libs, b"FLASH_Hemm\0").map(|sym| *sym),
            FLASH_Herk: get_symbol(&libs, b"FLASH_Herk\0").map(|sym| *sym),
            FLASH_Her2k: get_symbol(&libs, b"FLASH_Her2k\0").map(|sym| *sym),
            FLASH_Symm: get_symbol(&libs, b"FLASH_Symm\0").map(|sym| *sym),
            FLASH_Syrk: get_symbol(&libs, b"FLASH_Syrk\0").map(|sym| *sym),
            FLASH_Syr2k: get_symbol(&libs, b"FLASH_Syr2k\0").map(|sym| *sym),
            FLASH_Trmm: get_symbol(&libs, b"FLASH_Trmm\0").map(|sym| *sym),
            FLASH_Trsm: get_symbol(&libs, b"FLASH_Trsm\0").map(|sym| *sym),
            FLASH_Chol: get_symbol(&libs, b"FLASH_Chol\0").map(|sym| *sym),
            FLASH_LU_nopiv: get_symbol(&libs, b"FLASH_LU_nopiv\0").map(|sym| *sym),
            FLASH_LU_piv: get_symbol(&libs, b"FLASH_LU_piv\0").map(|sym| *sym),
            FLASH_LU_incpiv: get_symbol(&libs, b"FLASH_LU_incpiv\0").map(|sym| *sym),
            FLASH_FS_incpiv: get_symbol(&libs, b"FLASH_FS_incpiv\0").map(|sym| *sym),
            FLASH_Trinv: get_symbol(&libs, b"FLASH_Trinv\0").map(|sym| *sym),
            FLASH_Ttmm: get_symbol(&libs, b"FLASH_Ttmm\0").map(|sym| *sym),
            FLASH_SPDinv: get_symbol(&libs, b"FLASH_SPDinv\0").map(|sym| *sym),
            FLASH_Sylv: get_symbol(&libs, b"FLASH_Sylv\0").map(|sym| *sym),
            FLASH_Apply_pivots: get_symbol(&libs, b"FLASH_Apply_pivots\0").map(|sym| *sym),
            FLASH_Eig_gest: get_symbol(&libs, b"FLASH_Eig_gest\0").map(|sym| *sym),
            FLASH_LQ_UT_inv: get_symbol(&libs, b"FLASH_LQ_UT_inv\0").map(|sym| *sym),
            FLASH_LQ2_UT: get_symbol(&libs, b"FLASH_LQ2_UT\0").map(|sym| *sym),
            FLASH_Queue_begin: get_symbol(&libs, b"FLASH_Queue_begin\0").map(|sym| *sym),
            FLASH_Queue_end: get_symbol(&libs, b"FLASH_Queue_end\0").map(|sym| *sym),
            FLASH_Queue_stack_depth: get_symbol(&libs, b"FLASH_Queue_stack_depth\0")
                .map(|sym| *sym),
            FLASH_Queue_enable: get_symbol(&libs, b"FLASH_Queue_enable\0").map(|sym| *sym),
            FLASH_Queue_disable: get_symbol(&libs, b"FLASH_Queue_disable\0").map(|sym| *sym),
            FLASH_Queue_get_enabled: get_symbol(&libs, b"FLASH_Queue_get_enabled\0")
                .map(|sym| *sym),
            FLASH_Queue_set_num_threads: get_symbol(&libs, b"FLASH_Queue_set_num_threads\0")
                .map(|sym| *sym),
            FLASH_Queue_get_num_threads: get_symbol(&libs, b"FLASH_Queue_get_num_threads\0")
                .map(|sym| *sym),
            spotrf_: get_symbol(&libs, b"spotrf_\0").map(|sym| *sym),
            dpotrf_: get_symbol(&libs, b"dpotrf_\0").map(|sym| *sym),
            cpotrf_: get_symbol(&libs, b"cpotrf_\0").map(|sym| *sym),
            zpotrf_: get_symbol(&libs, b"zpotrf_\0").map(|sym| *sym),
            spotf2_: get_symbol(&libs, b"spotf2_\0").map(|sym| *sym),
            dpotf2_: get_symbol(&libs, b"dpotf2_\0").map(|sym| *sym),
            cpotf2_: get_symbol(&libs, b"cpotf2_\0").map(|sym| *sym),
            zpotf2_: get_symbol(&libs, b"zpotf2_\0").map(|sym| *sym),
            sgetrf_: get_symbol(&libs, b"sgetrf_\0").map(|sym| *sym),
            dgetrf_: get_symbol(&libs, b"dgetrf_\0").map(|sym| *sym),
            cgetrf_: get_symbol(&libs, b"cgetrf_\0").map(|sym| *sym),
            zgetrf_: get_symbol(&libs, b"zgetrf_\0").map(|sym| *sym),
            sgetf2_: get_symbol(&libs, b"sgetf2_\0").map(|sym| *sym),
            dgetf2_: get_symbol(&libs, b"dgetf2_\0").map(|sym| *sym),
            cgetf2_: get_symbol(&libs, b"cgetf2_\0").map(|sym| *sym),
            zgetf2_: get_symbol(&libs, b"zgetf2_\0").map(|sym| *sym),
            sgeqrf_: get_symbol(&libs, b"sgeqrf_\0").map(|sym| *sym),
            dgeqrf_: get_symbol(&libs, b"dgeqrf_\0").map(|sym| *sym),
            cgeqrf_: get_symbol(&libs, b"cgeqrf_\0").map(|sym| *sym),
            zgeqrf_: get_symbol(&libs, b"zgeqrf_\0").map(|sym| *sym),
            sgeqr2_: get_symbol(&libs, b"sgeqr2_\0").map(|sym| *sym),
            dgeqr2_: get_symbol(&libs, b"dgeqr2_\0").map(|sym| *sym),
            cgeqr2_: get_symbol(&libs, b"cgeqr2_\0").map(|sym| *sym),
            zgeqr2_: get_symbol(&libs, b"zgeqr2_\0").map(|sym| *sym),
            sgeqpf_: get_symbol(&libs, b"sgeqpf_\0").map(|sym| *sym),
            dgeqpf_: get_symbol(&libs, b"dgeqpf_\0").map(|sym| *sym),
            cgeqpf_: get_symbol(&libs, b"cgeqpf_\0").map(|sym| *sym),
            zgeqpf_: get_symbol(&libs, b"zgeqpf_\0").map(|sym| *sym),
            sgeqp3_: get_symbol(&libs, b"sgeqp3_\0").map(|sym| *sym),
            dgeqp3_: get_symbol(&libs, b"dgeqp3_\0").map(|sym| *sym),
            cgeqp3_: get_symbol(&libs, b"cgeqp3_\0").map(|sym| *sym),
            zgeqp3_: get_symbol(&libs, b"zgeqp3_\0").map(|sym| *sym),
            sgelqf_: get_symbol(&libs, b"sgelqf_\0").map(|sym| *sym),
            dgelqf_: get_symbol(&libs, b"dgelqf_\0").map(|sym| *sym),
            cgelqf_: get_symbol(&libs, b"cgelqf_\0").map(|sym| *sym),
            zgelqf_: get_symbol(&libs, b"zgelqf_\0").map(|sym| *sym),
            sgelq2_: get_symbol(&libs, b"sgelq2_\0").map(|sym| *sym),
            dgelq2_: get_symbol(&libs, b"dgelq2_\0").map(|sym| *sym),
            cgelq2_: get_symbol(&libs, b"cgelq2_\0").map(|sym| *sym),
            zgelq2_: get_symbol(&libs, b"zgelq2_\0").map(|sym| *sym),
            sgelsd_: get_symbol(&libs, b"sgelsd_\0").map(|sym| *sym),
            dgelsd_: get_symbol(&libs, b"dgelsd_\0").map(|sym| *sym),
            cgelsd_: get_symbol(&libs, b"cgelsd_\0").map(|sym| *sym),
            zgelsd_: get_symbol(&libs, b"zgelsd_\0").map(|sym| *sym),
            sgelss_: get_symbol(&libs, b"sgelss_\0").map(|sym| *sym),
            dgelss_: get_symbol(&libs, b"dgelss_\0").map(|sym| *sym),
            cgelss_: get_symbol(&libs, b"cgelss_\0").map(|sym| *sym),
            zgelss_: get_symbol(&libs, b"zgelss_\0").map(|sym| *sym),
            slauum_: get_symbol(&libs, b"slauum_\0").map(|sym| *sym),
            dlauum_: get_symbol(&libs, b"dlauum_\0").map(|sym| *sym),
            clauum_: get_symbol(&libs, b"clauum_\0").map(|sym| *sym),
            zlauum_: get_symbol(&libs, b"zlauum_\0").map(|sym| *sym),
            slauu2_: get_symbol(&libs, b"slauu2_\0").map(|sym| *sym),
            dlauu2_: get_symbol(&libs, b"dlauu2_\0").map(|sym| *sym),
            clauu2_: get_symbol(&libs, b"clauu2_\0").map(|sym| *sym),
            zlauu2_: get_symbol(&libs, b"zlauu2_\0").map(|sym| *sym),
            spotri_: get_symbol(&libs, b"spotri_\0").map(|sym| *sym),
            dpotri_: get_symbol(&libs, b"dpotri_\0").map(|sym| *sym),
            cpotri_: get_symbol(&libs, b"cpotri_\0").map(|sym| *sym),
            zpotri_: get_symbol(&libs, b"zpotri_\0").map(|sym| *sym),
            strtri_: get_symbol(&libs, b"strtri_\0").map(|sym| *sym),
            dtrtri_: get_symbol(&libs, b"dtrtri_\0").map(|sym| *sym),
            ctrtri_: get_symbol(&libs, b"ctrtri_\0").map(|sym| *sym),
            ztrtri_: get_symbol(&libs, b"ztrtri_\0").map(|sym| *sym),
            strti2_: get_symbol(&libs, b"strti2_\0").map(|sym| *sym),
            dtrti2_: get_symbol(&libs, b"dtrti2_\0").map(|sym| *sym),
            ctrti2_: get_symbol(&libs, b"ctrti2_\0").map(|sym| *sym),
            ztrti2_: get_symbol(&libs, b"ztrti2_\0").map(|sym| *sym),
            strsyl_: get_symbol(&libs, b"strsyl_\0").map(|sym| *sym),
            dtrsyl_: get_symbol(&libs, b"dtrsyl_\0").map(|sym| *sym),
            ctrsyl_: get_symbol(&libs, b"ctrsyl_\0").map(|sym| *sym),
            ztrsyl_: get_symbol(&libs, b"ztrsyl_\0").map(|sym| *sym),
            sgehrd_: get_symbol(&libs, b"sgehrd_\0").map(|sym| *sym),
            dgehrd_: get_symbol(&libs, b"dgehrd_\0").map(|sym| *sym),
            cgehrd_: get_symbol(&libs, b"cgehrd_\0").map(|sym| *sym),
            zgehrd_: get_symbol(&libs, b"zgehrd_\0").map(|sym| *sym),
            sgehd2_: get_symbol(&libs, b"sgehd2_\0").map(|sym| *sym),
            dgehd2_: get_symbol(&libs, b"dgehd2_\0").map(|sym| *sym),
            cgehd2_: get_symbol(&libs, b"cgehd2_\0").map(|sym| *sym),
            zgehd2_: get_symbol(&libs, b"zgehd2_\0").map(|sym| *sym),
            ssytrd_: get_symbol(&libs, b"ssytrd_\0").map(|sym| *sym),
            dsytrd_: get_symbol(&libs, b"dsytrd_\0").map(|sym| *sym),
            chetrd_: get_symbol(&libs, b"chetrd_\0").map(|sym| *sym),
            zhetrd_: get_symbol(&libs, b"zhetrd_\0").map(|sym| *sym),
            ssytd2_: get_symbol(&libs, b"ssytd2_\0").map(|sym| *sym),
            dsytd2_: get_symbol(&libs, b"dsytd2_\0").map(|sym| *sym),
            chetd2_: get_symbol(&libs, b"chetd2_\0").map(|sym| *sym),
            zhetd2_: get_symbol(&libs, b"zhetd2_\0").map(|sym| *sym),
            sgebrd_: get_symbol(&libs, b"sgebrd_\0").map(|sym| *sym),
            dgebrd_: get_symbol(&libs, b"dgebrd_\0").map(|sym| *sym),
            cgebrd_: get_symbol(&libs, b"cgebrd_\0").map(|sym| *sym),
            zgebrd_: get_symbol(&libs, b"zgebrd_\0").map(|sym| *sym),
            sgebd2_: get_symbol(&libs, b"sgebd2_\0").map(|sym| *sym),
            dgebd2_: get_symbol(&libs, b"dgebd2_\0").map(|sym| *sym),
            cgebd2_: get_symbol(&libs, b"cgebd2_\0").map(|sym| *sym),
            zgebd2_: get_symbol(&libs, b"zgebd2_\0").map(|sym| *sym),
            ssygst_: get_symbol(&libs, b"ssygst_\0").map(|sym| *sym),
            dsygst_: get_symbol(&libs, b"dsygst_\0").map(|sym| *sym),
            chegst_: get_symbol(&libs, b"chegst_\0").map(|sym| *sym),
            zhegst_: get_symbol(&libs, b"zhegst_\0").map(|sym| *sym),
            ssygs2_: get_symbol(&libs, b"ssygs2_\0").map(|sym| *sym),
            dsygs2_: get_symbol(&libs, b"dsygs2_\0").map(|sym| *sym),
            chegs2_: get_symbol(&libs, b"chegs2_\0").map(|sym| *sym),
            zhegs2_: get_symbol(&libs, b"zhegs2_\0").map(|sym| *sym),
            slarft_: get_symbol(&libs, b"slarft_\0").map(|sym| *sym),
            dlarft_: get_symbol(&libs, b"dlarft_\0").map(|sym| *sym),
            clarft_: get_symbol(&libs, b"clarft_\0").map(|sym| *sym),
            zlarft_: get_symbol(&libs, b"zlarft_\0").map(|sym| *sym),
            slarfg_: get_symbol(&libs, b"slarfg_\0").map(|sym| *sym),
            dlarfg_: get_symbol(&libs, b"dlarfg_\0").map(|sym| *sym),
            clarfg_: get_symbol(&libs, b"clarfg_\0").map(|sym| *sym),
            zlarfg_: get_symbol(&libs, b"zlarfg_\0").map(|sym| *sym),
            slarfgp_: get_symbol(&libs, b"slarfgp_\0").map(|sym| *sym),
            dlarfgp_: get_symbol(&libs, b"dlarfgp_\0").map(|sym| *sym),
            clarfgp_: get_symbol(&libs, b"clarfgp_\0").map(|sym| *sym),
            zlarfgp_: get_symbol(&libs, b"zlarfgp_\0").map(|sym| *sym),
            sorgqr_: get_symbol(&libs, b"sorgqr_\0").map(|sym| *sym),
            dorgqr_: get_symbol(&libs, b"dorgqr_\0").map(|sym| *sym),
            cungqr_: get_symbol(&libs, b"cungqr_\0").map(|sym| *sym),
            zungqr_: get_symbol(&libs, b"zungqr_\0").map(|sym| *sym),
            sormqr_: get_symbol(&libs, b"sormqr_\0").map(|sym| *sym),
            dormqr_: get_symbol(&libs, b"dormqr_\0").map(|sym| *sym),
            cunmqr_: get_symbol(&libs, b"cunmqr_\0").map(|sym| *sym),
            zunmqr_: get_symbol(&libs, b"zunmqr_\0").map(|sym| *sym),
            sorm2r_: get_symbol(&libs, b"sorm2r_\0").map(|sym| *sym),
            dorm2r_: get_symbol(&libs, b"dorm2r_\0").map(|sym| *sym),
            cunm2r_: get_symbol(&libs, b"cunm2r_\0").map(|sym| *sym),
            zunm2r_: get_symbol(&libs, b"zunm2r_\0").map(|sym| *sym),
            sorglq_: get_symbol(&libs, b"sorglq_\0").map(|sym| *sym),
            dorglq_: get_symbol(&libs, b"dorglq_\0").map(|sym| *sym),
            cunglq_: get_symbol(&libs, b"cunglq_\0").map(|sym| *sym),
            zunglq_: get_symbol(&libs, b"zunglq_\0").map(|sym| *sym),
            sormlq_: get_symbol(&libs, b"sormlq_\0").map(|sym| *sym),
            dormlq_: get_symbol(&libs, b"dormlq_\0").map(|sym| *sym),
            cunmlq_: get_symbol(&libs, b"cunmlq_\0").map(|sym| *sym),
            zunmlq_: get_symbol(&libs, b"zunmlq_\0").map(|sym| *sym),
            sorml2_: get_symbol(&libs, b"sorml2_\0").map(|sym| *sym),
            dorml2_: get_symbol(&libs, b"dorml2_\0").map(|sym| *sym),
            cunml2_: get_symbol(&libs, b"cunml2_\0").map(|sym| *sym),
            zunml2_: get_symbol(&libs, b"zunml2_\0").map(|sym| *sym),
            sorgtr_: get_symbol(&libs, b"sorgtr_\0").map(|sym| *sym),
            dorgtr_: get_symbol(&libs, b"dorgtr_\0").map(|sym| *sym),
            cungtr_: get_symbol(&libs, b"cungtr_\0").map(|sym| *sym),
            zungtr_: get_symbol(&libs, b"zungtr_\0").map(|sym| *sym),
            sormtr_: get_symbol(&libs, b"sormtr_\0").map(|sym| *sym),
            dormtr_: get_symbol(&libs, b"dormtr_\0").map(|sym| *sym),
            cunmtr_: get_symbol(&libs, b"cunmtr_\0").map(|sym| *sym),
            zunmtr_: get_symbol(&libs, b"zunmtr_\0").map(|sym| *sym),
            sorgbr_: get_symbol(&libs, b"sorgbr_\0").map(|sym| *sym),
            dorgbr_: get_symbol(&libs, b"dorgbr_\0").map(|sym| *sym),
            cungbr_: get_symbol(&libs, b"cungbr_\0").map(|sym| *sym),
            zungbr_: get_symbol(&libs, b"zungbr_\0").map(|sym| *sym),
            sormbr_: get_symbol(&libs, b"sormbr_\0").map(|sym| *sym),
            dormbr_: get_symbol(&libs, b"dormbr_\0").map(|sym| *sym),
            cunmbr_: get_symbol(&libs, b"cunmbr_\0").map(|sym| *sym),
            zunmbr_: get_symbol(&libs, b"zunmbr_\0").map(|sym| *sym),
            ssteqr_: get_symbol(&libs, b"ssteqr_\0").map(|sym| *sym),
            dsteqr_: get_symbol(&libs, b"dsteqr_\0").map(|sym| *sym),
            csteqr_: get_symbol(&libs, b"csteqr_\0").map(|sym| *sym),
            zsteqr_: get_symbol(&libs, b"zsteqr_\0").map(|sym| *sym),
            sstedc_: get_symbol(&libs, b"sstedc_\0").map(|sym| *sym),
            dstedc_: get_symbol(&libs, b"dstedc_\0").map(|sym| *sym),
            cstedc_: get_symbol(&libs, b"cstedc_\0").map(|sym| *sym),
            zstedc_: get_symbol(&libs, b"zstedc_\0").map(|sym| *sym),
            sstemr_: get_symbol(&libs, b"sstemr_\0").map(|sym| *sym),
            dstemr_: get_symbol(&libs, b"dstemr_\0").map(|sym| *sym),
            cstemr_: get_symbol(&libs, b"cstemr_\0").map(|sym| *sym),
            zstemr_: get_symbol(&libs, b"zstemr_\0").map(|sym| *sym),
            ssyev_: get_symbol(&libs, b"ssyev_\0").map(|sym| *sym),
            dsyev_: get_symbol(&libs, b"dsyev_\0").map(|sym| *sym),
            cheev_: get_symbol(&libs, b"cheev_\0").map(|sym| *sym),
            zheev_: get_symbol(&libs, b"zheev_\0").map(|sym| *sym),
            ssyevd_: get_symbol(&libs, b"ssyevd_\0").map(|sym| *sym),
            dsyevd_: get_symbol(&libs, b"dsyevd_\0").map(|sym| *sym),
            cheevd_: get_symbol(&libs, b"cheevd_\0").map(|sym| *sym),
            zheevd_: get_symbol(&libs, b"zheevd_\0").map(|sym| *sym),
            ssyevr_: get_symbol(&libs, b"ssyevr_\0").map(|sym| *sym),
            dsyevr_: get_symbol(&libs, b"dsyevr_\0").map(|sym| *sym),
            cheevr_: get_symbol(&libs, b"cheevr_\0").map(|sym| *sym),
            zheevr_: get_symbol(&libs, b"zheevr_\0").map(|sym| *sym),
            sbdsqr_: get_symbol(&libs, b"sbdsqr_\0").map(|sym| *sym),
            dbdsqr_: get_symbol(&libs, b"dbdsqr_\0").map(|sym| *sym),
            cbdsqr_: get_symbol(&libs, b"cbdsqr_\0").map(|sym| *sym),
            zbdsqr_: get_symbol(&libs, b"zbdsqr_\0").map(|sym| *sym),
            sbdsdc_: get_symbol(&libs, b"sbdsdc_\0").map(|sym| *sym),
            dbdsdc_: get_symbol(&libs, b"dbdsdc_\0").map(|sym| *sym),
            sgesvd_: get_symbol(&libs, b"sgesvd_\0").map(|sym| *sym),
            dgesvd_: get_symbol(&libs, b"dgesvd_\0").map(|sym| *sym),
            cgesvd_: get_symbol(&libs, b"cgesvd_\0").map(|sym| *sym),
            zgesvd_: get_symbol(&libs, b"zgesvd_\0").map(|sym| *sym),
            sgesdd_: get_symbol(&libs, b"sgesdd_\0").map(|sym| *sym),
            dgesdd_: get_symbol(&libs, b"dgesdd_\0").map(|sym| *sym),
            cgesdd_: get_symbol(&libs, b"cgesdd_\0").map(|sym| *sym),
            zgesdd_: get_symbol(&libs, b"zgesdd_\0").map(|sym| *sym),
            slaswp_: get_symbol(&libs, b"slaswp_\0").map(|sym| *sym),
            dlaswp_: get_symbol(&libs, b"dlaswp_\0").map(|sym| *sym),
            claswp_: get_symbol(&libs, b"claswp_\0").map(|sym| *sym),
            zlaswp_: get_symbol(&libs, b"zlaswp_\0").map(|sym| *sym),
            slaset_: get_symbol(&libs, b"slaset_\0").map(|sym| *sym),
            dlaset_: get_symbol(&libs, b"dlaset_\0").map(|sym| *sym),
            claset_: get_symbol(&libs, b"claset_\0").map(|sym| *sym),
            zlaset_: get_symbol(&libs, b"zlaset_\0").map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
