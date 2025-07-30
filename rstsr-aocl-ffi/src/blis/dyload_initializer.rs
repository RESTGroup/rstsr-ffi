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
            bli_pthread_create: get_symbol(&libs, b"bli_pthread_create\0").map(|sym| *sym),
            bli_pthread_join: get_symbol(&libs, b"bli_pthread_join\0").map(|sym| *sym),
            bli_pthread_mutex_init: get_symbol(&libs, b"bli_pthread_mutex_init\0").map(|sym| *sym),
            bli_pthread_mutex_destroy: get_symbol(&libs, b"bli_pthread_mutex_destroy\0")
                .map(|sym| *sym),
            bli_pthread_mutex_lock: get_symbol(&libs, b"bli_pthread_mutex_lock\0").map(|sym| *sym),
            bli_pthread_mutex_trylock: get_symbol(&libs, b"bli_pthread_mutex_trylock\0")
                .map(|sym| *sym),
            bli_pthread_mutex_unlock: get_symbol(&libs, b"bli_pthread_mutex_unlock\0")
                .map(|sym| *sym),
            bli_pthread_cond_init: get_symbol(&libs, b"bli_pthread_cond_init\0").map(|sym| *sym),
            bli_pthread_cond_destroy: get_symbol(&libs, b"bli_pthread_cond_destroy\0")
                .map(|sym| *sym),
            bli_pthread_cond_wait: get_symbol(&libs, b"bli_pthread_cond_wait\0").map(|sym| *sym),
            bli_pthread_cond_broadcast: get_symbol(&libs, b"bli_pthread_cond_broadcast\0")
                .map(|sym| *sym),
            bli_pthread_once: get_symbol(&libs, b"bli_pthread_once\0").map(|sym| *sym),
            bli_pthread_barrier_init: get_symbol(&libs, b"bli_pthread_barrier_init\0")
                .map(|sym| *sym),
            bli_pthread_barrier_destroy: get_symbol(&libs, b"bli_pthread_barrier_destroy\0")
                .map(|sym| *sym),
            bli_pthread_barrier_wait: get_symbol(&libs, b"bli_pthread_barrier_wait\0")
                .map(|sym| *sym),
            bli_thrcomm_create: get_symbol(&libs, b"bli_thrcomm_create\0").map(|sym| *sym),
            bli_thrcomm_free: get_symbol(&libs, b"bli_thrcomm_free\0").map(|sym| *sym),
            bli_thrcomm_init: get_symbol(&libs, b"bli_thrcomm_init\0").map(|sym| *sym),
            bli_thrcomm_cleanup: get_symbol(&libs, b"bli_thrcomm_cleanup\0").map(|sym| *sym),
            bli_thrcomm_barrier: get_symbol(&libs, b"bli_thrcomm_barrier\0").map(|sym| *sym),
            bli_thrcomm_bcast: get_symbol(&libs, b"bli_thrcomm_bcast\0").map(|sym| *sym),
            bli_thrcomm_barrier_atomic: get_symbol(&libs, b"bli_thrcomm_barrier_atomic\0")
                .map(|sym| *sym),
            bli_thrinfo_create: get_symbol(&libs, b"bli_thrinfo_create\0").map(|sym| *sym),
            bli_thrinfo_init: get_symbol(&libs, b"bli_thrinfo_init\0").map(|sym| *sym),
            bli_thrinfo_init_single: get_symbol(&libs, b"bli_thrinfo_init_single\0")
                .map(|sym| *sym),
            bli_thrinfo_free: get_symbol(&libs, b"bli_thrinfo_free\0").map(|sym| *sym),
            bli_thrinfo_grow: get_symbol(&libs, b"bli_thrinfo_grow\0").map(|sym| *sym),
            bli_thrinfo_rgrow: get_symbol(&libs, b"bli_thrinfo_rgrow\0").map(|sym| *sym),
            bli_thrinfo_create_for_cntl: get_symbol(&libs, b"bli_thrinfo_create_for_cntl\0")
                .map(|sym| *sym),
            bli_thrinfo_rgrow_prenode: get_symbol(&libs, b"bli_thrinfo_rgrow_prenode\0")
                .map(|sym| *sym),
            bli_thrinfo_create_for_cntl_prenode: get_symbol(
                &libs,
                b"bli_thrinfo_create_for_cntl_prenode\0",
            )
            .map(|sym| *sym),
            bli_thrinfo_sup_grow: get_symbol(&libs, b"bli_thrinfo_sup_grow\0").map(|sym| *sym),
            bli_thrinfo_sup_rgrow: get_symbol(&libs, b"bli_thrinfo_sup_rgrow\0").map(|sym| *sym),
            bli_thrinfo_sup_create_for_cntl: get_symbol(
                &libs,
                b"bli_thrinfo_sup_create_for_cntl\0",
            )
            .map(|sym| *sym),
            bli_packm_thrinfo_init: get_symbol(&libs, b"bli_packm_thrinfo_init\0").map(|sym| *sym),
            bli_packm_thrinfo_init_single: get_symbol(&libs, b"bli_packm_thrinfo_init_single\0")
                .map(|sym| *sym),
            bli_l3_thrinfo_init: get_symbol(&libs, b"bli_l3_thrinfo_init\0").map(|sym| *sym),
            bli_l3_thrinfo_init_single: get_symbol(&libs, b"bli_l3_thrinfo_init_single\0")
                .map(|sym| *sym),
            bli_l3_thrinfo_free: get_symbol(&libs, b"bli_l3_thrinfo_free\0").map(|sym| *sym),
            bli_l3_sup_thrinfo_free: get_symbol(&libs, b"bli_l3_sup_thrinfo_free\0")
                .map(|sym| *sym),
            bli_l3_thrinfo_create_root: get_symbol(&libs, b"bli_l3_thrinfo_create_root\0")
                .map(|sym| *sym),
            bli_l3_sup_thrinfo_create_root: get_symbol(&libs, b"bli_l3_sup_thrinfo_create_root\0")
                .map(|sym| *sym),
            bli_l3_sup_thrinfo_update_root: get_symbol(&libs, b"bli_l3_sup_thrinfo_update_root\0")
                .map(|sym| *sym),
            bli_l3_thrinfo_print_gemm_paths: get_symbol(
                &libs,
                b"bli_l3_thrinfo_print_gemm_paths\0",
            )
            .map(|sym| *sym),
            bli_l3_thrinfo_print_trsm_paths: get_symbol(
                &libs,
                b"bli_l3_thrinfo_print_trsm_paths\0",
            )
            .map(|sym| *sym),
            bli_l3_thrinfo_free_paths: get_symbol(&libs, b"bli_l3_thrinfo_free_paths\0")
                .map(|sym| *sym),
            bli_l3_thread_decorator: get_symbol(&libs, b"bli_l3_thread_decorator\0")
                .map(|sym| *sym),
            bli_l3_thread_decorator_thread_check: get_symbol(
                &libs,
                b"bli_l3_thread_decorator_thread_check\0",
            )
            .map(|sym| *sym),
            bli_l3_sup_thread_decorator: get_symbol(&libs, b"bli_l3_sup_thread_decorator\0")
                .map(|sym| *sym),
            bli_pack_full_thread_decorator: get_symbol(&libs, b"bli_pack_full_thread_decorator\0")
                .map(|sym| *sym),
            bli_l3_compute_thread_decorator: get_symbol(
                &libs,
                b"bli_l3_compute_thread_decorator\0",
            )
            .map(|sym| *sym),
            bli_thread_init: get_symbol(&libs, b"bli_thread_init\0").map(|sym| *sym),
            bli_thread_init_tl: get_symbol(&libs, b"bli_thread_init_tl\0").map(|sym| *sym),
            bli_thread_finalize: get_symbol(&libs, b"bli_thread_finalize\0").map(|sym| *sym),
            bli_thread_finalize_tl: get_symbol(&libs, b"bli_thread_finalize_tl\0").map(|sym| *sym),
            bli_thread_range_sub: get_symbol(&libs, b"bli_thread_range_sub\0").map(|sym| *sym),
            bli_thread_range_mdim: get_symbol(&libs, b"bli_thread_range_mdim\0").map(|sym| *sym),
            bli_thread_range_ndim: get_symbol(&libs, b"bli_thread_range_ndim\0").map(|sym| *sym),
            bli_thread_range_l2r: get_symbol(&libs, b"bli_thread_range_l2r\0").map(|sym| *sym),
            bli_thread_range_r2l: get_symbol(&libs, b"bli_thread_range_r2l\0").map(|sym| *sym),
            bli_thread_range_t2b: get_symbol(&libs, b"bli_thread_range_t2b\0").map(|sym| *sym),
            bli_thread_range_b2t: get_symbol(&libs, b"bli_thread_range_b2t\0").map(|sym| *sym),
            bli_thread_range_weighted_l2r: get_symbol(&libs, b"bli_thread_range_weighted_l2r\0")
                .map(|sym| *sym),
            bli_thread_range_weighted_r2l: get_symbol(&libs, b"bli_thread_range_weighted_r2l\0")
                .map(|sym| *sym),
            bli_thread_range_weighted_t2b: get_symbol(&libs, b"bli_thread_range_weighted_t2b\0")
                .map(|sym| *sym),
            bli_thread_range_weighted_b2t: get_symbol(&libs, b"bli_thread_range_weighted_b2t\0")
                .map(|sym| *sym),
            bli_thread_range_width_l: get_symbol(&libs, b"bli_thread_range_width_l\0")
                .map(|sym| *sym),
            bli_find_area_trap_l: get_symbol(&libs, b"bli_find_area_trap_l\0").map(|sym| *sym),
            bli_thread_range_weighted_sub: get_symbol(&libs, b"bli_thread_range_weighted_sub\0")
                .map(|sym| *sym),
            bli_prime_factorization: get_symbol(&libs, b"bli_prime_factorization\0")
                .map(|sym| *sym),
            bli_next_prime_factor: get_symbol(&libs, b"bli_next_prime_factor\0").map(|sym| *sym),
            bli_is_prime: get_symbol(&libs, b"bli_is_prime\0").map(|sym| *sym),
            bli_thread_partition_2x2: get_symbol(&libs, b"bli_thread_partition_2x2\0")
                .map(|sym| *sym),
            bli_thread_partition_2x2_slow: get_symbol(&libs, b"bli_thread_partition_2x2_slow\0")
                .map(|sym| *sym),
            bli_thread_partition_2x2_fast: get_symbol(&libs, b"bli_thread_partition_2x2_fast\0")
                .map(|sym| *sym),
            bli_thread_vector_partition: get_symbol(&libs, b"bli_thread_vector_partition\0")
                .map(|sym| *sym),
            bli_normfv_thread_partition: get_symbol(&libs, b"bli_normfv_thread_partition\0")
                .map(|sym| *sym),
            bli_gcd: get_symbol(&libs, b"bli_gcd\0").map(|sym| *sym),
            bli_lcm: get_symbol(&libs, b"bli_lcm\0").map(|sym| *sym),
            bli_ipow: get_symbol(&libs, b"bli_ipow\0").map(|sym| *sym),
            bli_thread_get_jc_nt: get_symbol(&libs, b"bli_thread_get_jc_nt\0").map(|sym| *sym),
            bli_thread_get_pc_nt: get_symbol(&libs, b"bli_thread_get_pc_nt\0").map(|sym| *sym),
            bli_thread_get_ic_nt: get_symbol(&libs, b"bli_thread_get_ic_nt\0").map(|sym| *sym),
            bli_thread_get_jr_nt: get_symbol(&libs, b"bli_thread_get_jr_nt\0").map(|sym| *sym),
            bli_thread_get_ir_nt: get_symbol(&libs, b"bli_thread_get_ir_nt\0").map(|sym| *sym),
            bli_thread_get_num_threads: get_symbol(&libs, b"bli_thread_get_num_threads\0")
                .map(|sym| *sym),
            bli_thread_get_is_parallel: get_symbol(&libs, b"bli_thread_get_is_parallel\0")
                .map(|sym| *sym),
            bli_thread_set_ways: get_symbol(&libs, b"bli_thread_set_ways\0").map(|sym| *sym),
            bli_thread_set_num_threads: get_symbol(&libs, b"bli_thread_set_num_threads\0")
                .map(|sym| *sym),
            bli_thread_init_rntm_from_env: get_symbol(&libs, b"bli_thread_init_rntm_from_env\0")
                .map(|sym| *sym),
            bli_thread_init_rntm_from_global_rntm: get_symbol(
                &libs,
                b"bli_thread_init_rntm_from_global_rntm\0",
            )
            .map(|sym| *sym),
            bli_thread_update_rntm_from_env: get_symbol(
                &libs,
                b"bli_thread_update_rntm_from_env\0",
            )
            .map(|sym| *sym),
            bli_cntx_init_zen5: get_symbol(&libs, b"bli_cntx_init_zen5\0").map(|sym| *sym),
            bli_cntx_init_zen5_ref: get_symbol(&libs, b"bli_cntx_init_zen5_ref\0").map(|sym| *sym),
            bli_cntx_init_zen5_ind: get_symbol(&libs, b"bli_cntx_init_zen5_ind\0").map(|sym| *sym),
            bli_cntx_init_zen4: get_symbol(&libs, b"bli_cntx_init_zen4\0").map(|sym| *sym),
            bli_cntx_init_zen4_ref: get_symbol(&libs, b"bli_cntx_init_zen4_ref\0").map(|sym| *sym),
            bli_cntx_init_zen4_ind: get_symbol(&libs, b"bli_cntx_init_zen4_ind\0").map(|sym| *sym),
            bli_cntx_init_zen3: get_symbol(&libs, b"bli_cntx_init_zen3\0").map(|sym| *sym),
            bli_cntx_init_zen3_ref: get_symbol(&libs, b"bli_cntx_init_zen3_ref\0").map(|sym| *sym),
            bli_cntx_init_zen3_ind: get_symbol(&libs, b"bli_cntx_init_zen3_ind\0").map(|sym| *sym),
            bli_cntx_init_zen2: get_symbol(&libs, b"bli_cntx_init_zen2\0").map(|sym| *sym),
            bli_cntx_init_zen2_ref: get_symbol(&libs, b"bli_cntx_init_zen2_ref\0").map(|sym| *sym),
            bli_cntx_init_zen2_ind: get_symbol(&libs, b"bli_cntx_init_zen2_ind\0").map(|sym| *sym),
            bli_cntx_init_zen: get_symbol(&libs, b"bli_cntx_init_zen\0").map(|sym| *sym),
            bli_cntx_init_zen_ref: get_symbol(&libs, b"bli_cntx_init_zen_ref\0").map(|sym| *sym),
            bli_cntx_init_zen_ind: get_symbol(&libs, b"bli_cntx_init_zen_ind\0").map(|sym| *sym),
            bli_cntx_init_generic: get_symbol(&libs, b"bli_cntx_init_generic\0").map(|sym| *sym),
            bli_cntx_init_generic_ref: get_symbol(&libs, b"bli_cntx_init_generic_ref\0")
                .map(|sym| *sym),
            bli_cntx_init_generic_ind: get_symbol(&libs, b"bli_cntx_init_generic_ind\0")
                .map(|sym| *sym),
            bli_sgemm_skx_asm_32x12_l2: get_symbol(&libs, b"bli_sgemm_skx_asm_32x12_l2\0")
                .map(|sym| *sym),
            bli_sgemm_skx_asm_12x32_l2: get_symbol(&libs, b"bli_sgemm_skx_asm_12x32_l2\0")
                .map(|sym| *sym),
            bli_dgemm_skx_asm_16x12_l2: get_symbol(&libs, b"bli_dgemm_skx_asm_16x12_l2\0")
                .map(|sym| *sym),
            bli_dgemm_skx_asm_16x14: get_symbol(&libs, b"bli_dgemm_skx_asm_16x14\0")
                .map(|sym| *sym),
            bli_spackm_haswell_asm_6xk: get_symbol(&libs, b"bli_spackm_haswell_asm_6xk\0")
                .map(|sym| *sym),
            bli_spackm_haswell_asm_16xk: get_symbol(&libs, b"bli_spackm_haswell_asm_16xk\0")
                .map(|sym| *sym),
            bli_dpackm_haswell_asm_6xk: get_symbol(&libs, b"bli_dpackm_haswell_asm_6xk\0")
                .map(|sym| *sym),
            bli_dpackm_haswell_asm_8xk: get_symbol(&libs, b"bli_dpackm_haswell_asm_8xk\0")
                .map(|sym| *sym),
            bli_cpackm_haswell_asm_3xk: get_symbol(&libs, b"bli_cpackm_haswell_asm_3xk\0")
                .map(|sym| *sym),
            bli_cpackm_haswell_asm_8xk: get_symbol(&libs, b"bli_cpackm_haswell_asm_8xk\0")
                .map(|sym| *sym),
            bli_zpackm_haswell_asm_3xk: get_symbol(&libs, b"bli_zpackm_haswell_asm_3xk\0")
                .map(|sym| *sym),
            bli_zpackm_haswell_asm_4xk: get_symbol(&libs, b"bli_zpackm_haswell_asm_4xk\0")
                .map(|sym| *sym),
            bli_sgemm_haswell_asm_6x16: get_symbol(&libs, b"bli_sgemm_haswell_asm_6x16\0")
                .map(|sym| *sym),
            bli_dgemm_haswell_asm_6x8: get_symbol(&libs, b"bli_dgemm_haswell_asm_6x8\0")
                .map(|sym| *sym),
            bli_cgemm_haswell_asm_3x8: get_symbol(&libs, b"bli_cgemm_haswell_asm_3x8\0")
                .map(|sym| *sym),
            bli_zgemm_haswell_asm_3x4: get_symbol(&libs, b"bli_zgemm_haswell_asm_3x4\0")
                .map(|sym| *sym),
            bli_sgemm_haswell_asm_16x6: get_symbol(&libs, b"bli_sgemm_haswell_asm_16x6\0")
                .map(|sym| *sym),
            bli_dgemm_haswell_asm_8x6: get_symbol(&libs, b"bli_dgemm_haswell_asm_8x6\0")
                .map(|sym| *sym),
            bli_cgemm_haswell_asm_8x3: get_symbol(&libs, b"bli_cgemm_haswell_asm_8x3\0")
                .map(|sym| *sym),
            bli_zgemm_haswell_asm_4x3: get_symbol(&libs, b"bli_zgemm_haswell_asm_4x3\0")
                .map(|sym| *sym),
            bli_sgemmtrsm_l_haswell_asm_6x16: get_symbol(
                &libs,
                b"bli_sgemmtrsm_l_haswell_asm_6x16\0",
            )
            .map(|sym| *sym),
            bli_dgemmtrsm_l_haswell_asm_6x8: get_symbol(
                &libs,
                b"bli_dgemmtrsm_l_haswell_asm_6x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmtrsm_u_haswell_asm_6x16: get_symbol(
                &libs,
                b"bli_sgemmtrsm_u_haswell_asm_6x16\0",
            )
            .map(|sym| *sym),
            bli_dgemmtrsm_u_haswell_asm_6x8: get_symbol(
                &libs,
                b"bli_dgemmtrsm_u_haswell_asm_6x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_r_haswell_ref_6x1: get_symbol(&libs, b"bli_sgemmsup_r_haswell_ref_6x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_haswell_ref_5x1: get_symbol(&libs, b"bli_sgemmsup_r_haswell_ref_5x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_haswell_ref_4x1: get_symbol(&libs, b"bli_sgemmsup_r_haswell_ref_4x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_haswell_ref_3x1: get_symbol(&libs, b"bli_sgemmsup_r_haswell_ref_3x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_haswell_ref_2x1: get_symbol(&libs, b"bli_sgemmsup_r_haswell_ref_2x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_haswell_ref_1x1: get_symbol(&libs, b"bli_sgemmsup_r_haswell_ref_1x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x6: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x6\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x6: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x6\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x6: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x6\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x6: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x6\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x6: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x6\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x6: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x6\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x16m: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x16m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x12m: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x12m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x8m: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x8m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x6m: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x6m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x4m: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x4m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x2m: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x2m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_6x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_6x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_5x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_5x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_4x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_4x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_3x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_3x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_2x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_2x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_haswell_asm_1x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_haswell_asm_1x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x16: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x16\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x12: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x12\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x8: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x8\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x4: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x4\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_3x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_3x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x2: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x2\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x1: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x1\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_3x1: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_3x1\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x1: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x1\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x1: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x1\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x16m: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x16m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x12m: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x12m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x8m: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x8m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x4m: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x4m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x2m: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x2m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_6x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_6x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_3x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_3x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_2x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_2x16n\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_haswell_asm_1x16n: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_haswell_asm_1x16n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_r_haswell_ref_6x1: get_symbol(&libs, b"bli_dgemmsup_r_haswell_ref_6x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_r_haswell_ref_5x1: get_symbol(&libs, b"bli_dgemmsup_r_haswell_ref_5x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_r_haswell_ref_4x1: get_symbol(&libs, b"bli_dgemmsup_r_haswell_ref_4x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_r_haswell_ref_3x1: get_symbol(&libs, b"bli_dgemmsup_r_haswell_ref_3x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_r_haswell_ref_2x1: get_symbol(&libs, b"bli_dgemmsup_r_haswell_ref_2x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_r_haswell_ref_1x1: get_symbol(&libs, b"bli_dgemmsup_r_haswell_ref_1x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x7: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x7\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x7: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x7\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x7: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x7\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x7: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x7\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x7: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x7\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x6: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x6\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x6: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x6\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x6: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x6\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x6: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x6\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x6: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x6\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x6: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x6\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x5: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x5\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x5: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x5\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x5: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x5\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x5: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x5\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x5: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x5\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x3: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x3\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x3: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x3\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x3: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x3\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x3: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x3\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x3: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x3\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x6m: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x6m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x4m: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x4m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x2m: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x2m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_5x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_5x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_4x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_4x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_3x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_3x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_2x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_2x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_1x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_1x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_2x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_2x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_1x8: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_1x8\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_2x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_2x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_1x4: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_1x4\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_3x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_3x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_2x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_2x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_1x2: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_1x2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_3x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_3x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_2x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_2x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_1x1: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_1x1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_0x0_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_0x0_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_6x0_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_6x0_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_6x8_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_6x8_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_12x8_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_12x8_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_12x16_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_12x16_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_18x16_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_18x16_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_0x0_combined_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_0x0_combined_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_0x0_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_0x0_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_6x0_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_6x0_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_6x8_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_6x8_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_12x8_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_12x8_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_12x16_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_12x16_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_18x16_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_18x16_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_haswell_asm_6x8m_16x12_combined_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_haswell_asm_6x8m_16x12_combined_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_0x0_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_0x0_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_6x0_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_6x0_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_6x8_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_6x8_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_12x8_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_12x8_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_12x16_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_12x16_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_18x16_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_18x16_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_0x0_combined_U: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_0x0_combined_U\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_0x0_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_0x0_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_6x0_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_6x0_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_6x8_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_6x8_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_12x8_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_12x8_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_12x16_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_12x16_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_18x16_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_18x16_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m_16x12_combined_L: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m_16x12_combined_L\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8m: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x4m: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x4m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x2m: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x2m\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_6x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_6x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_3x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_3x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_2x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_2x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rd_haswell_asm_1x8n: get_symbol(
                &libs,
                b"bli_dgemmsup_rd_haswell_asm_1x8n\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x8m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x8m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x7m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x7m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x6m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x6m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x5m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x5m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x4m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x4m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x3m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x3m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x2m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x2m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen5_asm_24x1m: get_symbol(&libs, b"bli_dgemmsup_rv_zen5_asm_24x1m\0")
                .map(|sym| *sym),
            bli_cntx_gemmsup_thresh_is_met_zen5: get_symbol(
                &libs,
                b"bli_cntx_gemmsup_thresh_is_met_zen5\0",
            )
            .map(|sym| *sym),
            bli_dynamic_blkszs_zen5: get_symbol(&libs, b"bli_dynamic_blkszs_zen5\0")
                .map(|sym| *sym),
            bli_trsm_small_ZEN5: get_symbol(&libs, b"bli_trsm_small_ZEN5\0").map(|sym| *sym),
            bli_dtrsm_small_XAltB_XAuB_ZEN5: get_symbol(
                &libs,
                b"bli_dtrsm_small_XAltB_XAuB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_dtrsm_small_XAutB_XAlB_ZEN5: get_symbol(
                &libs,
                b"bli_dtrsm_small_XAutB_XAlB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_dtrsm_small_AltXB_AuXB_ZEN5: get_symbol(
                &libs,
                b"bli_dtrsm_small_AltXB_AuXB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_dtrsm_small_AutXB_AlXB_ZEN5: get_symbol(
                &libs,
                b"bli_dtrsm_small_AutXB_AlXB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_XAltB_XAuB_ZEN5: get_symbol(
                &libs,
                b"bli_ztrsm_small_XAltB_XAuB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_XAutB_XAlB_ZEN5: get_symbol(
                &libs,
                b"bli_ztrsm_small_XAutB_XAlB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_AltXB_AuXB_ZEN5: get_symbol(
                &libs,
                b"bli_ztrsm_small_AltXB_AuXB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_AutXB_AlXB_ZEN5: get_symbol(
                &libs,
                b"bli_ztrsm_small_AutXB_AlXB_ZEN5\0",
            )
            .map(|sym| *sym),
            bli_trsm_small_mt_ZEN5: get_symbol(&libs, b"bli_trsm_small_mt_ZEN5\0").map(|sym| *sym),
            bli_zgemmtiny_avx512_ukr_info: get_symbol(&libs, b"bli_zgemmtiny_avx512_ukr_info\0")
                .map(|sym| *sym),
            bli_daddv_zen_int_avx512: get_symbol(&libs, b"bli_daddv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_samaxv_zen_int_avx512: get_symbol(&libs, b"bli_samaxv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_damaxv_zen_int_avx512: get_symbol(&libs, b"bli_damaxv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_sscalv_zen_int_avx512: get_symbol(&libs, b"bli_sscalv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_dscalv_zen_int_avx512: get_symbol(&libs, b"bli_dscalv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_cscalv_zen_int_avx512: get_symbol(&libs, b"bli_cscalv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zscalv_zen_int_avx512: get_symbol(&libs, b"bli_zscalv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zdscalv_zen_int_avx512: get_symbol(&libs, b"bli_zdscalv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_ssetv_zen_int_avx512: get_symbol(&libs, b"bli_ssetv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_dsetv_zen_int_avx512: get_symbol(&libs, b"bli_dsetv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zsetv_zen_int_avx512: get_symbol(&libs, b"bli_zsetv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_sdotv_zen_int_avx512: get_symbol(&libs, b"bli_sdotv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_ddotv_zen_int_avx512: get_symbol(&libs, b"bli_ddotv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zdotv_zen_int_avx512: get_symbol(&libs, b"bli_zdotv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zdotv_zen4_asm_avx512: get_symbol(&libs, b"bli_zdotv_zen4_asm_avx512\0")
                .map(|sym| *sym),
            bli_saxpyv_zen_int_avx512: get_symbol(&libs, b"bli_saxpyv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_daxpyv_zen_int_avx512: get_symbol(&libs, b"bli_daxpyv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zaxpyv_zen_int_avx512: get_symbol(&libs, b"bli_zaxpyv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_daxpbyv_zen_int_avx512: get_symbol(&libs, b"bli_daxpbyv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zaxpyf_zen_int_2_avx512: get_symbol(&libs, b"bli_zaxpyf_zen_int_2_avx512\0")
                .map(|sym| *sym),
            bli_zaxpyf_zen_int_4_avx512: get_symbol(&libs, b"bli_zaxpyf_zen_int_4_avx512\0")
                .map(|sym| *sym),
            bli_zaxpyf_zen_int_8_avx512: get_symbol(&libs, b"bli_zaxpyf_zen_int_8_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int2_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int2_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int4_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int4_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int6_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int6_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int8_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int8_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int12_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int12_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int16_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int16_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int32_avx512: get_symbol(&libs, b"bli_daxpyf_zen_int32_avx512\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int32_avx512_mt: get_symbol(&libs, b"bli_daxpyf_zen_int32_avx512_mt\0")
                .map(|sym| *sym),
            bli_ddotxf_zen_int_avx512: get_symbol(&libs, b"bli_ddotxf_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_scopyv_zen4_asm_avx512: get_symbol(&libs, b"bli_scopyv_zen4_asm_avx512\0")
                .map(|sym| *sym),
            bli_dcopyv_zen4_asm_avx512: get_symbol(&libs, b"bli_dcopyv_zen4_asm_avx512\0")
                .map(|sym| *sym),
            bli_zcopyv_zen4_asm_avx512: get_symbol(&libs, b"bli_zcopyv_zen4_asm_avx512\0")
                .map(|sym| *sym),
            bli_dscal2v_zen_int_avx512: get_symbol(&libs, b"bli_dscal2v_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zdotxv_zen_int_avx512: get_symbol(&libs, b"bli_zdotxv_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_zdotxf_zen_int_8_avx512: get_symbol(&libs, b"bli_zdotxf_zen_int_8_avx512\0")
                .map(|sym| *sym),
            bli_zdotxf_zen_int_4_avx512: get_symbol(&libs, b"bli_zdotxf_zen_int_4_avx512\0")
                .map(|sym| *sym),
            bli_zdotxf_zen_int_2_avx512: get_symbol(&libs, b"bli_zdotxf_zen_int_2_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx8_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx8_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx7_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx7_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx6_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx6_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx5_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx5_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx4_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx4_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx3_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx3_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx2_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx2_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16mx1_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16mx1_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_32x8n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_32x8n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16x8n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16x8n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_8x8n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_8x8n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_m_leftx8n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_m_leftx8n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_32x4n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_32x4n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16x4n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16x4n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_8x4n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_8x4n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_m_leftx4n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_m_leftx4n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_32x3n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_32x3n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16x3n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16x3n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_8x3n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_8x3n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_m_leftx3n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_m_leftx3n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_32x2n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_32x2n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16x2n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16x2n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_8x2n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_8x2n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_m_leftx2n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_m_leftx2n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_32x1n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_32x1n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_16x1n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_16x1n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_8x1n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_8x1n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_n_zen_int_m_leftx1n_avx512: get_symbol(
                &libs,
                b"bli_dgemv_n_zen_int_m_leftx1n_avx512\0",
            )
            .map(|sym| *sym),
            bli_dgemv_t_zen_int_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx8_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx8_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx7_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx7_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx6_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx6_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx5_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx5_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx4_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx4_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx3_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx3_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx2_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx2_avx512\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx1_avx512: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx1_avx512\0")
                .map(|sym| *sym),
            bli_dgemmtrsm_l_zen_asm_16x14: get_symbol(&libs, b"bli_dgemmtrsm_l_zen_asm_16x14\0")
                .map(|sym| *sym),
            bli_dgemmtrsm_u_zen_asm_16x14: get_symbol(&libs, b"bli_dgemmtrsm_u_zen_asm_16x14\0")
                .map(|sym| *sym),
            bli_dgemmtrsm_l_zen4_asm_8x24: get_symbol(&libs, b"bli_dgemmtrsm_l_zen4_asm_8x24\0")
                .map(|sym| *sym),
            bli_dgemmtrsm_u_zen4_asm_8x24: get_symbol(&libs, b"bli_dgemmtrsm_u_zen4_asm_8x24\0")
                .map(|sym| *sym),
            bli_zgemmtrsm_l_zen4_asm_4x12: get_symbol(&libs, b"bli_zgemmtrsm_l_zen4_asm_4x12\0")
                .map(|sym| *sym),
            bli_zgemmtrsm_u_zen4_asm_4x12: get_symbol(&libs, b"bli_zgemmtrsm_u_zen4_asm_4x12\0")
                .map(|sym| *sym),
            bli_dpackm_zen4_asm_16xk: get_symbol(&libs, b"bli_dpackm_zen4_asm_16xk\0")
                .map(|sym| *sym),
            bli_dpackm_zen4_asm_8xk: get_symbol(&libs, b"bli_dpackm_zen4_asm_8xk\0")
                .map(|sym| *sym),
            bli_dpackm_zen4_asm_24xk: get_symbol(&libs, b"bli_dpackm_zen4_asm_24xk\0")
                .map(|sym| *sym),
            bli_dpackm_zen4_asm_32xk: get_symbol(&libs, b"bli_dpackm_zen4_asm_32xk\0")
                .map(|sym| *sym),
            bli_dpackm_32xk_zen4_ref: get_symbol(&libs, b"bli_dpackm_32xk_zen4_ref\0")
                .map(|sym| *sym),
            bli_zpackm_zen4_asm_12xk: get_symbol(&libs, b"bli_zpackm_zen4_asm_12xk\0")
                .map(|sym| *sym),
            bli_zpackm_zen4_asm_4xk: get_symbol(&libs, b"bli_zpackm_zen4_asm_4xk\0")
                .map(|sym| *sym),
            bli_dgemm_avx512_asm_8x24: get_symbol(&libs, b"bli_dgemm_avx512_asm_8x24\0")
                .map(|sym| *sym),
            bli_dgemm_zen4_asm_32x6: get_symbol(&libs, b"bli_dgemm_zen4_asm_32x6\0")
                .map(|sym| *sym),
            bli_zgemm_zen4_asm_12x4: get_symbol(&libs, b"bli_zgemm_zen4_asm_12x4\0")
                .map(|sym| *sym),
            bli_zgemm_zen4_asm_4x12: get_symbol(&libs, b"bli_zgemm_zen4_asm_4x12\0")
                .map(|sym| *sym),
            bli_dgemm_avx512_asm_8x24_macro_kernel: get_symbol(
                &libs,
                b"bli_dgemm_avx512_asm_8x24_macro_kernel\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x64m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x64m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x48m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x48m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x32m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x32m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x16m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x16m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x64m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x64m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x48m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x48m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x32m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x32m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x16m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x16m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x64m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x64m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x48m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x48m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x32m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x32m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x16m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x16m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x64m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x64m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x48m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x48m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x32m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x32m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x16m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x16m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x16_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x16_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x16_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x16_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x64m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_6x64m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x48m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_6x48m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x32m_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_6x32m_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_3x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_3x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_2x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x64n_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_6x64n_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_5x64_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_5x64_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_4x64_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_4x64_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_3x64_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_3x64_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x64_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_2x64_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x64_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_1x64_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_5x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_5x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_4x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_4x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_3x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_3x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_2x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x48_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_1x48_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_5x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_5x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_4x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_4x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_3x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_3x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_2x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x32_avx512: get_symbol(
                &libs,
                b"bli_sgemmsup_rd_zen_asm_1x32_avx512\0",
            )
            .map(|sym| *sym),
            bli_trsm_small_AVX512: get_symbol(&libs, b"bli_trsm_small_AVX512\0").map(|sym| *sym),
            bli_dtrsm_small_AutXB_AlXB_AVX512: get_symbol(
                &libs,
                b"bli_dtrsm_small_AutXB_AlXB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_dtrsm_small_XAltB_XAuB_AVX512: get_symbol(
                &libs,
                b"bli_dtrsm_small_XAltB_XAuB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_dtrsm_small_XAutB_XAlB_AVX512: get_symbol(
                &libs,
                b"bli_dtrsm_small_XAutB_XAlB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_dtrsm_small_AltXB_AuXB_AVX512: get_symbol(
                &libs,
                b"bli_dtrsm_small_AltXB_AuXB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_AutXB_AlXB_AVX512: get_symbol(
                &libs,
                b"bli_ztrsm_small_AutXB_AlXB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_XAltB_XAuB_AVX512: get_symbol(
                &libs,
                b"bli_ztrsm_small_XAltB_XAuB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_XAutB_XAlB_AVX512: get_symbol(
                &libs,
                b"bli_ztrsm_small_XAutB_XAlB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_ztrsm_small_AltXB_AuXB_AVX512: get_symbol(
                &libs,
                b"bli_ztrsm_small_AltXB_AuXB_AVX512\0",
            )
            .map(|sym| *sym),
            bli_trsm_small_mt_AVX512: get_symbol(&libs, b"bli_trsm_small_mt_AVX512\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x8m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x7m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x7m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x6m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x6m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x5m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x5m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x4m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x4m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x3m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x3m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x2m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x2m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x1m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x1m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x7m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x7m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x6m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x6m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x5m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x5m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x4m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x4m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x3m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x3m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x2m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x2m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x1m_new: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x1m_new\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x8\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x8: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x8\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x8: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x8\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x8m: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x8m\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x8m_lower: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_8x8m_lower\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x8m_upper: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_8x8m_upper\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_lower_0: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_lower_0\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_lower_1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_lower_1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_lower_2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_lower_2\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_upper_0: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_upper_0\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_upper_1: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_upper_1\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x8m_upper_2: get_symbol(
                &libs,
                b"bli_dgemmsup_rv_zen4_asm_24x8m_upper_2\0",
            )
            .map(|sym| *sym),
            bli_zgemmsup_rv_zen4_asm_4x4m: get_symbol(&libs, b"bli_zgemmsup_rv_zen4_asm_4x4m\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen4_asm_4x4m_lower: get_symbol(
                &libs,
                b"bli_zgemmsup_rv_zen4_asm_4x4m_lower\0",
            )
            .map(|sym| *sym),
            bli_zgemmsup_rv_zen4_asm_4x4m_upper: get_symbol(
                &libs,
                b"bli_zgemmsup_rv_zen4_asm_4x4m_upper\0",
            )
            .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x7: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x7\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x7: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x7\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x7: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x7\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x6: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x6\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x6: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x6\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x6: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x6\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x5: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x5\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x5: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x5\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x5: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x5\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x4: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x4\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x4: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x4\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x4: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x4\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x3: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x3\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x3: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x3\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x3: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x3\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x2: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x2\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x2: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x2\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x2: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x2\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_24x1: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_24x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_16x1: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_16x1\0")
                .map(|sym| *sym),
            bli_dgemmsup_rv_zen4_asm_8x1: get_symbol(&libs, b"bli_dgemmsup_rv_zen4_asm_8x1\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_12x4m: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_12x4m\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_12x3m: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_12x3m\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_12x2m: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_12x2m\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_12x1m: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_12x1m\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_8x4: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_8x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_8x3: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_8x3\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_8x2: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_8x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_8x1: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_8x1\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_4x4: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_4x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_4x3: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_4x3\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_4x2: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_4x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_4x1: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_4x1\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_2x4: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_2x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_2x3: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_2x3\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_2x2: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_2x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_cv_zen4_asm_2x1: get_symbol(&libs, b"bli_zgemmsup_cv_zen4_asm_2x1\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_12x4m: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_12x4m\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_12x2m: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_12x2m\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_8x4: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_8x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_8x2: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_8x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_4x4: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_4x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_4x2: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_4x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_2x4: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_2x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_cd_zen4_asm_2x2: get_symbol(&libs, b"bli_zgemmsup_cd_zen4_asm_2x2\0")
                .map(|sym| *sym),
            bli_dgemm_24x8_avx512_k1_nn: get_symbol(&libs, b"bli_dgemm_24x8_avx512_k1_nn\0")
                .map(|sym| *sym),
            bli_dgemm_tiny_24x8: get_symbol(&libs, b"bli_dgemm_tiny_24x8\0").map(|sym| *sym),
            bli_dnorm2fv_unb_var1_avx512: get_symbol(&libs, b"bli_dnorm2fv_unb_var1_avx512\0")
                .map(|sym| *sym),
            bli_zgemm_16x4_avx512_k1_nn: get_symbol(&libs, b"bli_zgemm_16x4_avx512_k1_nn\0")
                .map(|sym| *sym),
            bli_cntx_gemmsup_thresh_is_met_zen4: get_symbol(
                &libs,
                b"bli_cntx_gemmsup_thresh_is_met_zen4\0",
            )
            .map(|sym| *sym),
            bli_dynamic_blkszs_zen4: get_symbol(&libs, b"bli_dynamic_blkszs_zen4\0")
                .map(|sym| *sym),
            bli_zero_zmm: get_symbol(&libs, b"bli_zero_zmm\0").map(|sym| *sym),
            bli_dgemv_n_avx512: get_symbol(&libs, b"bli_dgemv_n_avx512\0").map(|sym| *sym),
            bli_saxpyf_zen_int_5: get_symbol(&libs, b"bli_saxpyf_zen_int_5\0").map(|sym| *sym),
            bli_daxpyf_zen_int_5: get_symbol(&libs, b"bli_daxpyf_zen_int_5\0").map(|sym| *sym),
            bli_zgemmtiny_avx2_ukr_info: get_symbol(&libs, b"bli_zgemmtiny_avx2_ukr_info\0")
                .map(|sym| *sym),
            bli_saddv_zen_int: get_symbol(&libs, b"bli_saddv_zen_int\0").map(|sym| *sym),
            bli_daddv_zen_int: get_symbol(&libs, b"bli_daddv_zen_int\0").map(|sym| *sym),
            bli_caddv_zen_int: get_symbol(&libs, b"bli_caddv_zen_int\0").map(|sym| *sym),
            bli_zaddv_zen_int: get_symbol(&libs, b"bli_zaddv_zen_int\0").map(|sym| *sym),
            bli_samaxv_zen_int: get_symbol(&libs, b"bli_samaxv_zen_int\0").map(|sym| *sym),
            bli_damaxv_zen_int: get_symbol(&libs, b"bli_damaxv_zen_int\0").map(|sym| *sym),
            bli_saxpbyv_zen_int: get_symbol(&libs, b"bli_saxpbyv_zen_int\0").map(|sym| *sym),
            bli_daxpbyv_zen_int: get_symbol(&libs, b"bli_daxpbyv_zen_int\0").map(|sym| *sym),
            bli_caxpbyv_zen_int: get_symbol(&libs, b"bli_caxpbyv_zen_int\0").map(|sym| *sym),
            bli_zaxpbyv_zen_int: get_symbol(&libs, b"bli_zaxpbyv_zen_int\0").map(|sym| *sym),
            bli_saxpbyv_zen_int10: get_symbol(&libs, b"bli_saxpbyv_zen_int10\0").map(|sym| *sym),
            bli_daxpbyv_zen_int10: get_symbol(&libs, b"bli_daxpbyv_zen_int10\0").map(|sym| *sym),
            bli_saxpyv_zen_int: get_symbol(&libs, b"bli_saxpyv_zen_int\0").map(|sym| *sym),
            bli_daxpyv_zen_int: get_symbol(&libs, b"bli_daxpyv_zen_int\0").map(|sym| *sym),
            bli_saxpyv_zen_int10: get_symbol(&libs, b"bli_saxpyv_zen_int10\0").map(|sym| *sym),
            bli_daxpyv_zen_int10: get_symbol(&libs, b"bli_daxpyv_zen_int10\0").map(|sym| *sym),
            bli_caxpyv_zen_int5: get_symbol(&libs, b"bli_caxpyv_zen_int5\0").map(|sym| *sym),
            bli_zaxpyv_zen_int5: get_symbol(&libs, b"bli_zaxpyv_zen_int5\0").map(|sym| *sym),
            bli_sdotv_zen_int: get_symbol(&libs, b"bli_sdotv_zen_int\0").map(|sym| *sym),
            bli_ddotv_zen_int: get_symbol(&libs, b"bli_ddotv_zen_int\0").map(|sym| *sym),
            bli_sdotv_zen_int10: get_symbol(&libs, b"bli_sdotv_zen_int10\0").map(|sym| *sym),
            bli_ddotv_zen_int10: get_symbol(&libs, b"bli_ddotv_zen_int10\0").map(|sym| *sym),
            bli_cdotv_zen_int5: get_symbol(&libs, b"bli_cdotv_zen_int5\0").map(|sym| *sym),
            bli_zdotv_zen_int5: get_symbol(&libs, b"bli_zdotv_zen_int5\0").map(|sym| *sym),
            bli_sdotxv_zen_int: get_symbol(&libs, b"bli_sdotxv_zen_int\0").map(|sym| *sym),
            bli_ddotxv_zen_int: get_symbol(&libs, b"bli_ddotxv_zen_int\0").map(|sym| *sym),
            bli_zdotxv_zen_int: get_symbol(&libs, b"bli_zdotxv_zen_int\0").map(|sym| *sym),
            bli_cdotxv_zen_int: get_symbol(&libs, b"bli_cdotxv_zen_int\0").map(|sym| *sym),
            bli_sscalv_zen_int: get_symbol(&libs, b"bli_sscalv_zen_int\0").map(|sym| *sym),
            bli_dscalv_zen_int: get_symbol(&libs, b"bli_dscalv_zen_int\0").map(|sym| *sym),
            bli_cscalv_zen_int: get_symbol(&libs, b"bli_cscalv_zen_int\0").map(|sym| *sym),
            bli_zscalv_zen_int: get_symbol(&libs, b"bli_zscalv_zen_int\0").map(|sym| *sym),
            bli_sscalv_zen_int10: get_symbol(&libs, b"bli_sscalv_zen_int10\0").map(|sym| *sym),
            bli_dscalv_zen_int10: get_symbol(&libs, b"bli_dscalv_zen_int10\0").map(|sym| *sym),
            bli_zdscalv_zen_int10: get_symbol(&libs, b"bli_zdscalv_zen_int10\0").map(|sym| *sym),
            bli_sswapv_zen_int8: get_symbol(&libs, b"bli_sswapv_zen_int8\0").map(|sym| *sym),
            bli_dswapv_zen_int8: get_symbol(&libs, b"bli_dswapv_zen_int8\0").map(|sym| *sym),
            bli_scopyv_zen_int: get_symbol(&libs, b"bli_scopyv_zen_int\0").map(|sym| *sym),
            bli_dcopyv_zen_int: get_symbol(&libs, b"bli_dcopyv_zen_int\0").map(|sym| *sym),
            bli_ccopyv_zen_int: get_symbol(&libs, b"bli_ccopyv_zen_int\0").map(|sym| *sym),
            bli_zcopyv_zen_int: get_symbol(&libs, b"bli_zcopyv_zen_int\0").map(|sym| *sym),
            bli_sscal2v_zen_int: get_symbol(&libs, b"bli_sscal2v_zen_int\0").map(|sym| *sym),
            bli_dscal2v_zen_int: get_symbol(&libs, b"bli_dscal2v_zen_int\0").map(|sym| *sym),
            bli_cscal2v_zen_int: get_symbol(&libs, b"bli_cscal2v_zen_int\0").map(|sym| *sym),
            bli_zscal2v_zen_int: get_symbol(&libs, b"bli_zscal2v_zen_int\0").map(|sym| *sym),
            bli_ssetv_zen_int: get_symbol(&libs, b"bli_ssetv_zen_int\0").map(|sym| *sym),
            bli_dsetv_zen_int: get_symbol(&libs, b"bli_dsetv_zen_int\0").map(|sym| *sym),
            bli_csetv_zen_int: get_symbol(&libs, b"bli_csetv_zen_int\0").map(|sym| *sym),
            bli_zsetv_zen_int: get_symbol(&libs, b"bli_zsetv_zen_int\0").map(|sym| *sym),
            bli_saxpyf_zen_int_8: get_symbol(&libs, b"bli_saxpyf_zen_int_8\0").map(|sym| *sym),
            bli_daxpyf_zen_int_8: get_symbol(&libs, b"bli_daxpyf_zen_int_8\0").map(|sym| *sym),
            bli_daxpyf_zen_int_16x4: get_symbol(&libs, b"bli_daxpyf_zen_int_16x4\0")
                .map(|sym| *sym),
            bli_daxpyf_zen_int_16x2: get_symbol(&libs, b"bli_daxpyf_zen_int_16x2\0")
                .map(|sym| *sym),
            bli_saxpyf_zen_int_6: get_symbol(&libs, b"bli_saxpyf_zen_int_6\0").map(|sym| *sym),
            bli_caxpyf_zen_int_5: get_symbol(&libs, b"bli_caxpyf_zen_int_5\0").map(|sym| *sym),
            bli_caxpyf_zen_int_4: get_symbol(&libs, b"bli_caxpyf_zen_int_4\0").map(|sym| *sym),
            bli_zaxpyf_zen_int_5: get_symbol(&libs, b"bli_zaxpyf_zen_int_5\0").map(|sym| *sym),
            bli_zaxpyf_zen_int_4: get_symbol(&libs, b"bli_zaxpyf_zen_int_4\0").map(|sym| *sym),
            bli_daxpy2v_zen_int: get_symbol(&libs, b"bli_daxpy2v_zen_int\0").map(|sym| *sym),
            bli_zaxpy2v_zen_int: get_symbol(&libs, b"bli_zaxpy2v_zen_int\0").map(|sym| *sym),
            bli_sdotxf_zen_int_8: get_symbol(&libs, b"bli_sdotxf_zen_int_8\0").map(|sym| *sym),
            bli_ddotxf_zen_int_8: get_symbol(&libs, b"bli_ddotxf_zen_int_8\0").map(|sym| *sym),
            bli_ddotxf_zen_int_4: get_symbol(&libs, b"bli_ddotxf_zen_int_4\0").map(|sym| *sym),
            bli_ddotxf_zen_int_2: get_symbol(&libs, b"bli_ddotxf_zen_int_2\0").map(|sym| *sym),
            bli_zdotxf_zen_int_6: get_symbol(&libs, b"bli_zdotxf_zen_int_6\0").map(|sym| *sym),
            bli_cdotxf_zen_int_6: get_symbol(&libs, b"bli_cdotxf_zen_int_6\0").map(|sym| *sym),
            bli_ddotxaxpyf_zen_int_8: get_symbol(&libs, b"bli_ddotxaxpyf_zen_int_8\0")
                .map(|sym| *sym),
            bli_cdotxaxpyf_zen_int_8: get_symbol(&libs, b"bli_cdotxaxpyf_zen_int_8\0")
                .map(|sym| *sym),
            bli_zdotxaxpyf_zen_int_8: get_symbol(&libs, b"bli_zdotxaxpyf_zen_int_8\0")
                .map(|sym| *sym),
            bli_dgemv_zen_ref_c: get_symbol(&libs, b"bli_dgemv_zen_ref_c\0").map(|sym| *sym),
            bli_cgemv_zen_int_4x4: get_symbol(&libs, b"bli_cgemv_zen_int_4x4\0").map(|sym| *sym),
            bli_zgemv_zen_int_4x4: get_symbol(&libs, b"bli_zgemv_zen_int_4x4\0").map(|sym| *sym),
            bli_dgemv_t_zen_int_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx7_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx7_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx6_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx6_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx5_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx5_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx4_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx4_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx3_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx3_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx2_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx2_avx2\0")
                .map(|sym| *sym),
            bli_dgemv_t_zen_int_mx1_avx2: get_symbol(&libs, b"bli_dgemv_t_zen_int_mx1_avx2\0")
                .map(|sym| *sym),
            bli_zher_zen_int_var1: get_symbol(&libs, b"bli_zher_zen_int_var1\0").map(|sym| *sym),
            bli_zher_zen_int_var2: get_symbol(&libs, b"bli_zher_zen_int_var2\0").map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x16: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_5x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x16: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_4x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x16: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_3x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x16: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_2x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x16: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_1x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x8: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x8: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_5x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x8: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_4x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x8: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_3x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x8: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_2x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x8: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_1x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x4: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x4: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_5x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x4: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_4x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x4: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_3x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x4: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_2x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x4: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_1x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x2: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x2: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_5x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x2: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_4x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x2: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_3x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x2: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_2x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x2: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_1x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_zen_ref_6x1: get_symbol(&libs, b"bli_sgemmsup_r_zen_ref_6x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_zen_ref_5x1: get_symbol(&libs, b"bli_sgemmsup_r_zen_ref_5x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_zen_ref_4x1: get_symbol(&libs, b"bli_sgemmsup_r_zen_ref_4x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_zen_ref_3x1: get_symbol(&libs, b"bli_sgemmsup_r_zen_ref_3x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_zen_ref_2x1: get_symbol(&libs, b"bli_sgemmsup_r_zen_ref_2x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_r_zen_ref_1x1: get_symbol(&libs, b"bli_sgemmsup_r_zen_ref_1x1\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x16m: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x16m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x8m: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x8m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x4m: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x4m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x2m: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x2m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x16m_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x16m_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x8m_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x8m_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x4m_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_6x4m_mask\0",
            )
            .map(|sym| *sym),
            bli_sbli_sgemmsup_rv_zen_asm_6x8m: get_symbol(
                &libs,
                b"bli_sbli_sgemmsup_rv_zen_asm_6x8m\0",
            )
            .map(|sym| *sym),
            bli_sbli_sgemmsup_rv_zen_asm_6x4m: get_symbol(
                &libs,
                b"bli_sbli_sgemmsup_rv_zen_asm_6x4m\0",
            )
            .map(|sym| *sym),
            bli_sbli_sgemmsup_rv_zen_asm_6x2m: get_symbol(
                &libs,
                b"bli_sbli_sgemmsup_rv_zen_asm_6x2m\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x16_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x16_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x16_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x16_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x16_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x16_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x16_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x16_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x16_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x16_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x8_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x8_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x8_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x8_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x8_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x8_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x8_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x8_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x8_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x8_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x4_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_1x4_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x4_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_2x4_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x4_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_3x4_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x4_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_4x4_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x4_mask: get_symbol(
                &libs,
                b"bli_sgemmsup_rv_zen_asm_5x4_mask\0",
            )
            .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_6x16n: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_6x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_5x16n: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_5x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_4x16n: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_4x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_3x16n: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_3x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_2x16n: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_2x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rv_zen_asm_1x16n: get_symbol(&libs, b"bli_sgemmsup_rv_zen_asm_1x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x8: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_2x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x16: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_2x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x8: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_1x8\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x16: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_1x16\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x4: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x4: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_2x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x4: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_1x4\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x2: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_3x2: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_3x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x2: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_2x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x2: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_1x2\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x16m: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x16m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x8m: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x8m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x4m: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x4m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x2m: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x2m\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_6x16n: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_6x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_3x16n: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_3x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_2x16n: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_2x16n\0")
                .map(|sym| *sym),
            bli_sgemmsup_rd_zen_asm_1x16n: get_symbol(&libs, b"bli_sgemmsup_rd_zen_asm_1x16n\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_3x8m: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_3x8m\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_3x4m: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_3x4m\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_3x2m: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_3x2m\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_2x8: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_2x8\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_1x8: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_1x8\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_2x4: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_2x4\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_1x4: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_1x4\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_2x2: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_2x2\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_1x2: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_1x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_3x4m: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_3x4m\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_3x2m: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_3x2m\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_2x4: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_2x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_1x4: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_1x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_2x2: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_2x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_1x2: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_1x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_3x4m: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_3x4m\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_3x2m: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_3x2m\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_2x4: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_2x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_1x4: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_1x4\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_2x2: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_2x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_1x2: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_1x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_3x4n: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_3x4n\0")
                .map(|sym| *sym),
            bli_zgemmsup_rd_zen_asm_2x4n: get_symbol(&libs, b"bli_zgemmsup_rd_zen_asm_2x4n\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_3x8n: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_3x8n\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_2x8n: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_2x8n\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_1x8n: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_1x8n\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_3x4: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_3x4\0")
                .map(|sym| *sym),
            bli_cgemmsup_rv_zen_asm_3x2: get_symbol(&libs, b"bli_cgemmsup_rv_zen_asm_3x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_3x4n: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_3x4n\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_2x4n: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_2x4n\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_1x4n: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_1x4n\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_3x2: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_3x2\0")
                .map(|sym| *sym),
            bli_zgemmsup_rv_zen_asm_3x1: get_symbol(&libs, b"bli_zgemmsup_rv_zen_asm_3x1\0")
                .map(|sym| *sym),
            bli_dgemm_tiny: get_symbol(&libs, b"bli_dgemm_tiny\0").map(|sym| *sym),
            bli_dgemm_tiny_6x8: get_symbol(&libs, b"bli_dgemm_tiny_6x8\0").map(|sym| *sym),
            bli_dgemm_small: get_symbol(&libs, b"bli_dgemm_small\0").map(|sym| *sym),
            bli_dgemm_small_At: get_symbol(&libs, b"bli_dgemm_small_At\0").map(|sym| *sym),
            bli_zgemm_small: get_symbol(&libs, b"bli_zgemm_small\0").map(|sym| *sym),
            bli_zgemm_small_At: get_symbol(&libs, b"bli_zgemm_small_At\0").map(|sym| *sym),
            bli_dgemm_8x6_avx2_k1_nn: get_symbol(&libs, b"bli_dgemm_8x6_avx2_k1_nn\0")
                .map(|sym| *sym),
            bli_zgemm_4x4_avx2_k1_nn: get_symbol(&libs, b"bli_zgemm_4x4_avx2_k1_nn\0")
                .map(|sym| *sym),
            bli_trsm_small: get_symbol(&libs, b"bli_trsm_small\0").map(|sym| *sym),
            bli_trsm_small_mt: get_symbol(&libs, b"bli_trsm_small_mt\0").map(|sym| *sym),
            bli_multi_sgemv_4x2: get_symbol(&libs, b"bli_multi_sgemv_4x2\0").map(|sym| *sym),
            bli_cntx_gemmtsup_thresh_is_met_zen: get_symbol(
                &libs,
                b"bli_cntx_gemmtsup_thresh_is_met_zen\0",
            )
            .map(|sym| *sym),
            bli_cntx_syrksup_thresh_is_met_zen: get_symbol(
                &libs,
                b"bli_cntx_syrksup_thresh_is_met_zen\0",
            )
            .map(|sym| *sym),
            bli_cntx_trsm_small_thresh_is_met_zen: get_symbol(
                &libs,
                b"bli_cntx_trsm_small_thresh_is_met_zen\0",
            )
            .map(|sym| *sym),
            bli_snorm2fv_unb_var1_avx2: get_symbol(&libs, b"bli_snorm2fv_unb_var1_avx2\0")
                .map(|sym| *sym),
            bli_dnorm2fv_unb_var1_avx2: get_symbol(&libs, b"bli_dnorm2fv_unb_var1_avx2\0")
                .map(|sym| *sym),
            bli_scnorm2fv_unb_var1_avx2: get_symbol(&libs, b"bli_scnorm2fv_unb_var1_avx2\0")
                .map(|sym| *sym),
            bli_dznorm2fv_unb_var1_avx2: get_symbol(&libs, b"bli_dznorm2fv_unb_var1_avx2\0")
                .map(|sym| *sym),
            bli_zgemm_zen_asm_2x6: get_symbol(&libs, b"bli_zgemm_zen_asm_2x6\0").map(|sym| *sym),
            bli_zgemmtrsm_l_zen_asm_2x6: get_symbol(&libs, b"bli_zgemmtrsm_l_zen_asm_2x6\0")
                .map(|sym| *sym),
            bli_zgemmtrsm_u_zen_asm_2x6: get_symbol(&libs, b"bli_zgemmtrsm_u_zen_asm_2x6\0")
                .map(|sym| *sym),
            bli_dgemv_zen_ref: get_symbol(&libs, b"bli_dgemv_zen_ref\0").map(|sym| *sym),
            bli_dgemv_n_avx2: get_symbol(&libs, b"bli_dgemv_n_avx2\0").map(|sym| *sym),
            bli_init: get_symbol(&libs, b"bli_init\0").map(|sym| *sym),
            bli_finalize: get_symbol(&libs, b"bli_finalize\0").map(|sym| *sym),
            bli_init_auto: get_symbol(&libs, b"bli_init_auto\0").map(|sym| *sym),
            bli_finalize_auto: get_symbol(&libs, b"bli_finalize_auto\0").map(|sym| *sym),
            bli_init_apis: get_symbol(&libs, b"bli_init_apis\0").map(|sym| *sym),
            bli_finalize_apis: get_symbol(&libs, b"bli_finalize_apis\0").map(|sym| *sym),
            bli_init_once: get_symbol(&libs, b"bli_init_once\0").map(|sym| *sym),
            bli_finalize_once: get_symbol(&libs, b"bli_finalize_once\0").map(|sym| *sym),
            bli_malloc_intl: get_symbol(&libs, b"bli_malloc_intl\0").map(|sym| *sym),
            bli_calloc_intl: get_symbol(&libs, b"bli_calloc_intl\0").map(|sym| *sym),
            bli_free_intl: get_symbol(&libs, b"bli_free_intl\0").map(|sym| *sym),
            bli_malloc_user: get_symbol(&libs, b"bli_malloc_user\0").map(|sym| *sym),
            bli_free_user: get_symbol(&libs, b"bli_free_user\0").map(|sym| *sym),
            bli_fmalloc_align: get_symbol(&libs, b"bli_fmalloc_align\0").map(|sym| *sym),
            bli_ffree_align: get_symbol(&libs, b"bli_ffree_align\0").map(|sym| *sym),
            bli_fmalloc_noalign: get_symbol(&libs, b"bli_fmalloc_noalign\0").map(|sym| *sym),
            bli_ffree_noalign: get_symbol(&libs, b"bli_ffree_noalign\0").map(|sym| *sym),
            bli_fmalloc_align_check: get_symbol(&libs, b"bli_fmalloc_align_check\0")
                .map(|sym| *sym),
            bli_fmalloc_post_check: get_symbol(&libs, b"bli_fmalloc_post_check\0").map(|sym| *sym),
            bli_const_init: get_symbol(&libs, b"bli_const_init\0").map(|sym| *sym),
            bli_const_finalize: get_symbol(&libs, b"bli_const_finalize\0").map(|sym| *sym),
            bli_obj_create_check: get_symbol(&libs, b"bli_obj_create_check\0").map(|sym| *sym),
            bli_obj_create_without_buffer_check: get_symbol(
                &libs,
                b"bli_obj_create_without_buffer_check\0",
            )
            .map(|sym| *sym),
            bli_obj_alloc_buffer_check: get_symbol(&libs, b"bli_obj_alloc_buffer_check\0")
                .map(|sym| *sym),
            bli_obj_attach_buffer_check: get_symbol(&libs, b"bli_obj_attach_buffer_check\0")
                .map(|sym| *sym),
            bli_obj_create_scalar_check: get_symbol(&libs, b"bli_obj_create_scalar_check\0")
                .map(|sym| *sym),
            bli_obj_free_check: get_symbol(&libs, b"bli_obj_free_check\0").map(|sym| *sym),
            bli_obj_create_const_check: get_symbol(&libs, b"bli_obj_create_const_check\0")
                .map(|sym| *sym),
            bli_obj_create_const_copy_of_check: get_symbol(
                &libs,
                b"bli_obj_create_const_copy_of_check\0",
            )
            .map(|sym| *sym),
            bli_dt_size_check: get_symbol(&libs, b"bli_dt_size_check\0").map(|sym| *sym),
            bli_dt_string_check: get_symbol(&libs, b"bli_dt_string_check\0").map(|sym| *sym),
            bli_dt_union_check: get_symbol(&libs, b"bli_dt_union_check\0").map(|sym| *sym),
            bli_obj_print_check: get_symbol(&libs, b"bli_obj_print_check\0").map(|sym| *sym),
            bli_obj_create: get_symbol(&libs, b"bli_obj_create\0").map(|sym| *sym),
            bli_obj_create_with_attached_buffer: get_symbol(
                &libs,
                b"bli_obj_create_with_attached_buffer\0",
            )
            .map(|sym| *sym),
            bli_obj_create_without_buffer: get_symbol(&libs, b"bli_obj_create_without_buffer\0")
                .map(|sym| *sym),
            bli_obj_alloc_buffer: get_symbol(&libs, b"bli_obj_alloc_buffer\0").map(|sym| *sym),
            bli_obj_attach_buffer: get_symbol(&libs, b"bli_obj_attach_buffer\0").map(|sym| *sym),
            bli_obj_create_1x1: get_symbol(&libs, b"bli_obj_create_1x1\0").map(|sym| *sym),
            bli_obj_create_1x1_with_attached_buffer: get_symbol(
                &libs,
                b"bli_obj_create_1x1_with_attached_buffer\0",
            )
            .map(|sym| *sym),
            bli_obj_create_conf_to: get_symbol(&libs, b"bli_obj_create_conf_to\0").map(|sym| *sym),
            bli_obj_free: get_symbol(&libs, b"bli_obj_free\0").map(|sym| *sym),
            bli_adjust_strides: get_symbol(&libs, b"bli_adjust_strides\0").map(|sym| *sym),
            bli_dt_size: get_symbol(&libs, b"bli_dt_size\0").map(|sym| *sym),
            bli_dt_string: get_symbol(&libs, b"bli_dt_string\0").map(|sym| *sym),
            bli_align_dim_to_mult: get_symbol(&libs, b"bli_align_dim_to_mult\0").map(|sym| *sym),
            bli_align_dim_to_size: get_symbol(&libs, b"bli_align_dim_to_size\0").map(|sym| *sym),
            bli_align_ptr_to_size: get_symbol(&libs, b"bli_align_ptr_to_size\0").map(|sym| *sym),
            bli_obj_print: get_symbol(&libs, b"bli_obj_print\0").map(|sym| *sym),
            bli_obj_scalar_init_detached: get_symbol(&libs, b"bli_obj_scalar_init_detached\0")
                .map(|sym| *sym),
            bli_obj_scalar_init_detached_copy_of: get_symbol(
                &libs,
                b"bli_obj_scalar_init_detached_copy_of\0",
            )
            .map(|sym| *sym),
            bli_obj_scalar_detach: get_symbol(&libs, b"bli_obj_scalar_detach\0").map(|sym| *sym),
            bli_obj_scalar_attach: get_symbol(&libs, b"bli_obj_scalar_attach\0").map(|sym| *sym),
            bli_obj_scalar_cast_to: get_symbol(&libs, b"bli_obj_scalar_cast_to\0").map(|sym| *sym),
            bli_obj_scalar_apply_scalar: get_symbol(&libs, b"bli_obj_scalar_apply_scalar\0")
                .map(|sym| *sym),
            bli_obj_scalar_reset: get_symbol(&libs, b"bli_obj_scalar_reset\0").map(|sym| *sym),
            bli_obj_scalar_has_nonzero_imag: get_symbol(
                &libs,
                b"bli_obj_scalar_has_nonzero_imag\0",
            )
            .map(|sym| *sym),
            bli_obj_scalar_equals: get_symbol(&libs, b"bli_obj_scalar_equals\0").map(|sym| *sym),
            bli_blksz_create_ed: get_symbol(&libs, b"bli_blksz_create_ed\0").map(|sym| *sym),
            bli_blksz_create: get_symbol(&libs, b"bli_blksz_create\0").map(|sym| *sym),
            bli_blksz_init_ed: get_symbol(&libs, b"bli_blksz_init_ed\0").map(|sym| *sym),
            bli_blksz_init: get_symbol(&libs, b"bli_blksz_init\0").map(|sym| *sym),
            bli_blksz_init_easy: get_symbol(&libs, b"bli_blksz_init_easy\0").map(|sym| *sym),
            bli_blksz_free: get_symbol(&libs, b"bli_blksz_free\0").map(|sym| *sym),
            bli_blksz_reduce_def_to: get_symbol(&libs, b"bli_blksz_reduce_def_to\0")
                .map(|sym| *sym),
            bli_blksz_reduce_max_to: get_symbol(&libs, b"bli_blksz_reduce_max_to\0")
                .map(|sym| *sym),
            bli_determine_blocksize: get_symbol(&libs, b"bli_determine_blocksize\0")
                .map(|sym| *sym),
            bli_determine_blocksize_f: get_symbol(&libs, b"bli_determine_blocksize_f\0")
                .map(|sym| *sym),
            bli_determine_blocksize_b: get_symbol(&libs, b"bli_determine_blocksize_b\0")
                .map(|sym| *sym),
            bli_determine_blocksize_f_sub: get_symbol(&libs, b"bli_determine_blocksize_f_sub\0")
                .map(|sym| *sym),
            bli_determine_blocksize_b_sub: get_symbol(&libs, b"bli_determine_blocksize_b_sub\0")
                .map(|sym| *sym),
            bli_func_create: get_symbol(&libs, b"bli_func_create\0").map(|sym| *sym),
            bli_func_init: get_symbol(&libs, b"bli_func_init\0").map(|sym| *sym),
            bli_func_init_null: get_symbol(&libs, b"bli_func_init_null\0").map(|sym| *sym),
            bli_func_free: get_symbol(&libs, b"bli_func_free\0").map(|sym| *sym),
            bli_func_is_null_dt: get_symbol(&libs, b"bli_func_is_null_dt\0").map(|sym| *sym),
            bli_func_is_null: get_symbol(&libs, b"bli_func_is_null\0").map(|sym| *sym),
            bli_mbool_create: get_symbol(&libs, b"bli_mbool_create\0").map(|sym| *sym),
            bli_mbool_init: get_symbol(&libs, b"bli_mbool_init\0").map(|sym| *sym),
            bli_mbool_free: get_symbol(&libs, b"bli_mbool_free\0").map(|sym| *sym),
            bli_cntx_clear: get_symbol(&libs, b"bli_cntx_clear\0").map(|sym| *sym),
            bli_cntx_set_blkszs: get_symbol(&libs, b"bli_cntx_set_blkszs\0").map(|sym| *sym),
            bli_cntx_set_trsm_blkszs: get_symbol(&libs, b"bli_cntx_set_trsm_blkszs\0")
                .map(|sym| *sym),
            bli_cntx_set_ind_blkszs: get_symbol(&libs, b"bli_cntx_set_ind_blkszs\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_nat_ukrs: get_symbol(&libs, b"bli_cntx_set_l3_nat_ukrs\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_vir_ukrs: get_symbol(&libs, b"bli_cntx_set_l3_vir_ukrs\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_sup_thresh: get_symbol(&libs, b"bli_cntx_set_l3_sup_thresh\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_sup_handlers: get_symbol(&libs, b"bli_cntx_set_l3_sup_handlers\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_sup_blkszs: get_symbol(&libs, b"bli_cntx_set_l3_sup_blkszs\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_sup_tri_blkszs: get_symbol(&libs, b"bli_cntx_set_l3_sup_tri_blkszs\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_sup_kers: get_symbol(&libs, b"bli_cntx_set_l3_sup_kers\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_sup_tri_kers: get_symbol(&libs, b"bli_cntx_set_l3_sup_tri_kers\0")
                .map(|sym| *sym),
            bli_cntx_set_l1f_kers: get_symbol(&libs, b"bli_cntx_set_l1f_kers\0").map(|sym| *sym),
            bli_cntx_set_l1v_kers: get_symbol(&libs, b"bli_cntx_set_l1v_kers\0").map(|sym| *sym),
            bli_cntx_set_packm_kers: get_symbol(&libs, b"bli_cntx_set_packm_kers\0")
                .map(|sym| *sym),
            bli_cntx_set_l3_thresh_funcs: get_symbol(&libs, b"bli_cntx_set_l3_thresh_funcs\0")
                .map(|sym| *sym),
            bli_cntx_print: get_symbol(&libs, b"bli_cntx_print\0").map(|sym| *sym),
            bli_rntm_init_from_global: get_symbol(&libs, b"bli_rntm_init_from_global\0")
                .map(|sym| *sym),
            bli_rntm_set_ways_for_op: get_symbol(&libs, b"bli_rntm_set_ways_for_op\0")
                .map(|sym| *sym),
            bli_rntm_set_ways_from_rntm: get_symbol(&libs, b"bli_rntm_set_ways_from_rntm\0")
                .map(|sym| *sym),
            bli_rntm_set_ways_from_rntm_sup: get_symbol(
                &libs,
                b"bli_rntm_set_ways_from_rntm_sup\0",
            )
            .map(|sym| *sym),
            bli_rntm_print: get_symbol(&libs, b"bli_rntm_print\0").map(|sym| *sym),
            bli_rntm_calc_num_threads_in: get_symbol(&libs, b"bli_rntm_calc_num_threads_in\0")
                .map(|sym| *sym),
            bli_nthreads_optimum: get_symbol(&libs, b"bli_nthreads_optimum\0").map(|sym| *sym),
            bli_smart_threading_sup: get_symbol(&libs, b"bli_smart_threading_sup\0")
                .map(|sym| *sym),
            aocl_dnormfv_dynamic: get_symbol(&libs, b"aocl_dnormfv_dynamic\0").map(|sym| *sym),
            aocl_znormfv_dynamic: get_symbol(&libs, b"aocl_znormfv_dynamic\0").map(|sym| *sym),
            bli_nthreads_l1: get_symbol(&libs, b"bli_nthreads_l1\0").map(|sym| *sym),
            bli_nthreads_l1f: get_symbol(&libs, b"bli_nthreads_l1f\0").map(|sym| *sym),
            bli_gks_init: get_symbol(&libs, b"bli_gks_init\0").map(|sym| *sym),
            bli_gks_init_once: get_symbol(&libs, b"bli_gks_init_once\0").map(|sym| *sym),
            bli_gks_finalize: get_symbol(&libs, b"bli_gks_finalize\0").map(|sym| *sym),
            bli_gks_init_index: get_symbol(&libs, b"bli_gks_init_index\0").map(|sym| *sym),
            bli_gks_lookup_nat_cntx: get_symbol(&libs, b"bli_gks_lookup_nat_cntx\0")
                .map(|sym| *sym),
            bli_gks_lookup_ind_cntx: get_symbol(&libs, b"bli_gks_lookup_ind_cntx\0")
                .map(|sym| *sym),
            bli_gks_lookup_id: get_symbol(&libs, b"bli_gks_lookup_id\0").map(|sym| *sym),
            bli_gks_register_cntx: get_symbol(&libs, b"bli_gks_register_cntx\0").map(|sym| *sym),
            bli_gks_query_cntx: get_symbol(&libs, b"bli_gks_query_cntx\0").map(|sym| *sym),
            bli_gks_query_nat_cntx: get_symbol(&libs, b"bli_gks_query_nat_cntx\0").map(|sym| *sym),
            bli_gks_query_cntx_noinit: get_symbol(&libs, b"bli_gks_query_cntx_noinit\0")
                .map(|sym| *sym),
            bli_gks_query_ind_cntx: get_symbol(&libs, b"bli_gks_query_ind_cntx\0").map(|sym| *sym),
            bli_gks_init_ref_cntx: get_symbol(&libs, b"bli_gks_init_ref_cntx\0").map(|sym| *sym),
            bli_gks_cntx_l3_nat_ukr_is_ref: get_symbol(&libs, b"bli_gks_cntx_l3_nat_ukr_is_ref\0")
                .map(|sym| *sym),
            bli_gks_l3_ukr_impl_string: get_symbol(&libs, b"bli_gks_l3_ukr_impl_string\0")
                .map(|sym| *sym),
            bli_gks_l3_ukr_impl_type: get_symbol(&libs, b"bli_gks_l3_ukr_impl_type\0")
                .map(|sym| *sym),
            bli_gemmind_find_avail: get_symbol(&libs, b"bli_gemmind_find_avail\0").map(|sym| *sym),
            bli_gemmtind_find_avail: get_symbol(&libs, b"bli_gemmtind_find_avail\0")
                .map(|sym| *sym),
            bli_hemmind_find_avail: get_symbol(&libs, b"bli_hemmind_find_avail\0").map(|sym| *sym),
            bli_herkind_find_avail: get_symbol(&libs, b"bli_herkind_find_avail\0").map(|sym| *sym),
            bli_her2kind_find_avail: get_symbol(&libs, b"bli_her2kind_find_avail\0")
                .map(|sym| *sym),
            bli_symmind_find_avail: get_symbol(&libs, b"bli_symmind_find_avail\0").map(|sym| *sym),
            bli_syrkind_find_avail: get_symbol(&libs, b"bli_syrkind_find_avail\0").map(|sym| *sym),
            bli_syr2kind_find_avail: get_symbol(&libs, b"bli_syr2kind_find_avail\0")
                .map(|sym| *sym),
            bli_trmm3ind_find_avail: get_symbol(&libs, b"bli_trmm3ind_find_avail\0")
                .map(|sym| *sym),
            bli_trmmind_find_avail: get_symbol(&libs, b"bli_trmmind_find_avail\0").map(|sym| *sym),
            bli_trsmind_find_avail: get_symbol(&libs, b"bli_trsmind_find_avail\0").map(|sym| *sym),
            bli_l3_ind_oper_find_avail: get_symbol(&libs, b"bli_l3_ind_oper_find_avail\0")
                .map(|sym| *sym),
            bli_l3_ind_set_enable_dt: get_symbol(&libs, b"bli_l3_ind_set_enable_dt\0")
                .map(|sym| *sym),
            bli_l3_ind_oper_enable_only: get_symbol(&libs, b"bli_l3_ind_oper_enable_only\0")
                .map(|sym| *sym),
            bli_l3_ind_oper_set_enable_all: get_symbol(&libs, b"bli_l3_ind_oper_set_enable_all\0")
                .map(|sym| *sym),
            bli_l3_ind_oper_set_enable: get_symbol(&libs, b"bli_l3_ind_oper_set_enable\0")
                .map(|sym| *sym),
            bli_l3_ind_oper_get_enable: get_symbol(&libs, b"bli_l3_ind_oper_get_enable\0")
                .map(|sym| *sym),
            bli_l3_ind_oper_is_impl: get_symbol(&libs, b"bli_l3_ind_oper_is_impl\0")
                .map(|sym| *sym),
            bli_ind_init: get_symbol(&libs, b"bli_ind_init\0").map(|sym| *sym),
            bli_ind_finalize: get_symbol(&libs, b"bli_ind_finalize\0").map(|sym| *sym),
            bli_ind_enable: get_symbol(&libs, b"bli_ind_enable\0").map(|sym| *sym),
            bli_ind_disable: get_symbol(&libs, b"bli_ind_disable\0").map(|sym| *sym),
            bli_ind_disable_all: get_symbol(&libs, b"bli_ind_disable_all\0").map(|sym| *sym),
            bli_ind_enable_dt: get_symbol(&libs, b"bli_ind_enable_dt\0").map(|sym| *sym),
            bli_ind_disable_dt: get_symbol(&libs, b"bli_ind_disable_dt\0").map(|sym| *sym),
            bli_ind_disable_all_dt: get_symbol(&libs, b"bli_ind_disable_all_dt\0").map(|sym| *sym),
            bli_ind_oper_enable_only: get_symbol(&libs, b"bli_ind_oper_enable_only\0")
                .map(|sym| *sym),
            bli_ind_oper_is_impl: get_symbol(&libs, b"bli_ind_oper_is_impl\0").map(|sym| *sym),
            bli_ind_oper_find_avail: get_symbol(&libs, b"bli_ind_oper_find_avail\0")
                .map(|sym| *sym),
            bli_ind_oper_get_avail_impl_string: get_symbol(
                &libs,
                b"bli_ind_oper_get_avail_impl_string\0",
            )
            .map(|sym| *sym),
            bli_ind_get_impl_string: get_symbol(&libs, b"bli_ind_get_impl_string\0")
                .map(|sym| *sym),
            bli_ind_map_cdt_to_index: get_symbol(&libs, b"bli_ind_map_cdt_to_index\0")
                .map(|sym| *sym),
            bli_pba_query: get_symbol(&libs, b"bli_pba_query\0").map(|sym| *sym),
            bli_pba_init: get_symbol(&libs, b"bli_pba_init\0").map(|sym| *sym),
            bli_pba_finalize: get_symbol(&libs, b"bli_pba_finalize\0").map(|sym| *sym),
            bli_pba_acquire_m: get_symbol(&libs, b"bli_pba_acquire_m\0").map(|sym| *sym),
            bli_pba_release: get_symbol(&libs, b"bli_pba_release\0").map(|sym| *sym),
            bli_pba_rntm_set_pba: get_symbol(&libs, b"bli_pba_rntm_set_pba\0").map(|sym| *sym),
            bli_pba_pool_size: get_symbol(&libs, b"bli_pba_pool_size\0").map(|sym| *sym),
            bli_pba_init_pools: get_symbol(&libs, b"bli_pba_init_pools\0").map(|sym| *sym),
            bli_pba_finalize_pools: get_symbol(&libs, b"bli_pba_finalize_pools\0").map(|sym| *sym),
            bli_pba_compute_pool_block_sizes: get_symbol(
                &libs,
                b"bli_pba_compute_pool_block_sizes\0",
            )
            .map(|sym| *sym),
            bli_pba_compute_pool_block_sizes_dt: get_symbol(
                &libs,
                b"bli_pba_compute_pool_block_sizes_dt\0",
            )
            .map(|sym| *sym),
            bli_pool_init: get_symbol(&libs, b"bli_pool_init\0").map(|sym| *sym),
            bli_pool_finalize: get_symbol(&libs, b"bli_pool_finalize\0").map(|sym| *sym),
            bli_pool_reinit: get_symbol(&libs, b"bli_pool_reinit\0").map(|sym| *sym),
            bli_pool_checkout_block: get_symbol(&libs, b"bli_pool_checkout_block\0")
                .map(|sym| *sym),
            bli_pool_checkin_block: get_symbol(&libs, b"bli_pool_checkin_block\0").map(|sym| *sym),
            bli_pool_grow: get_symbol(&libs, b"bli_pool_grow\0").map(|sym| *sym),
            bli_pool_shrink: get_symbol(&libs, b"bli_pool_shrink\0").map(|sym| *sym),
            bli_pool_alloc_block: get_symbol(&libs, b"bli_pool_alloc_block\0").map(|sym| *sym),
            bli_pool_free_block: get_symbol(&libs, b"bli_pool_free_block\0").map(|sym| *sym),
            bli_pool_print: get_symbol(&libs, b"bli_pool_print\0").map(|sym| *sym),
            bli_pblk_print: get_symbol(&libs, b"bli_pblk_print\0").map(|sym| *sym),
            bli_array_init: get_symbol(&libs, b"bli_array_init\0").map(|sym| *sym),
            bli_array_resize: get_symbol(&libs, b"bli_array_resize\0").map(|sym| *sym),
            bli_array_finalize: get_symbol(&libs, b"bli_array_finalize\0").map(|sym| *sym),
            bli_array_elem: get_symbol(&libs, b"bli_array_elem\0").map(|sym| *sym),
            bli_array_set_elem: get_symbol(&libs, b"bli_array_set_elem\0").map(|sym| *sym),
            bli_apool_init: get_symbol(&libs, b"bli_apool_init\0").map(|sym| *sym),
            bli_apool_finalize: get_symbol(&libs, b"bli_apool_finalize\0").map(|sym| *sym),
            bli_apool_checkout_array: get_symbol(&libs, b"bli_apool_checkout_array\0")
                .map(|sym| *sym),
            bli_apool_checkin_array: get_symbol(&libs, b"bli_apool_checkin_array\0")
                .map(|sym| *sym),
            bli_apool_array_elem: get_symbol(&libs, b"bli_apool_array_elem\0").map(|sym| *sym),
            bli_apool_grow: get_symbol(&libs, b"bli_apool_grow\0").map(|sym| *sym),
            bli_apool_alloc_block: get_symbol(&libs, b"bli_apool_alloc_block\0").map(|sym| *sym),
            bli_apool_free_block: get_symbol(&libs, b"bli_apool_free_block\0").map(|sym| *sym),
            bli_sba_query: get_symbol(&libs, b"bli_sba_query\0").map(|sym| *sym),
            bli_sba_init: get_symbol(&libs, b"bli_sba_init\0").map(|sym| *sym),
            bli_sba_finalize: get_symbol(&libs, b"bli_sba_finalize\0").map(|sym| *sym),
            bli_sba_checkout_array: get_symbol(&libs, b"bli_sba_checkout_array\0").map(|sym| *sym),
            bli_sba_checkin_array: get_symbol(&libs, b"bli_sba_checkin_array\0").map(|sym| *sym),
            bli_sba_rntm_set_pool: get_symbol(&libs, b"bli_sba_rntm_set_pool\0").map(|sym| *sym),
            bli_sba_acquire: get_symbol(&libs, b"bli_sba_acquire\0").map(|sym| *sym),
            bli_sba_release: get_symbol(&libs, b"bli_sba_release\0").map(|sym| *sym),
            bli_memsys_init: get_symbol(&libs, b"bli_memsys_init\0").map(|sym| *sym),
            bli_memsys_finalize: get_symbol(&libs, b"bli_memsys_finalize\0").map(|sym| *sym),
            bli_acquire_mpart_t2b_check: get_symbol(&libs, b"bli_acquire_mpart_t2b_check\0")
                .map(|sym| *sym),
            bli_acquire_mpart_l2r_check: get_symbol(&libs, b"bli_acquire_mpart_l2r_check\0")
                .map(|sym| *sym),
            bli_acquire_mpart_tl2br_check: get_symbol(&libs, b"bli_acquire_mpart_tl2br_check\0")
                .map(|sym| *sym),
            bli_acquire_mpart: get_symbol(&libs, b"bli_acquire_mpart\0").map(|sym| *sym),
            bli_acquire_mpart_t2b: get_symbol(&libs, b"bli_acquire_mpart_t2b\0").map(|sym| *sym),
            bli_acquire_mpart_b2t: get_symbol(&libs, b"bli_acquire_mpart_b2t\0").map(|sym| *sym),
            bli_acquire_mpart_l2r: get_symbol(&libs, b"bli_acquire_mpart_l2r\0").map(|sym| *sym),
            bli_acquire_mpart_r2l: get_symbol(&libs, b"bli_acquire_mpart_r2l\0").map(|sym| *sym),
            bli_acquire_mpart_tl2br: get_symbol(&libs, b"bli_acquire_mpart_tl2br\0")
                .map(|sym| *sym),
            bli_acquire_mpart_br2tl: get_symbol(&libs, b"bli_acquire_mpart_br2tl\0")
                .map(|sym| *sym),
            bli_acquire_mpart_mdim: get_symbol(&libs, b"bli_acquire_mpart_mdim\0").map(|sym| *sym),
            bli_acquire_mpart_ndim: get_symbol(&libs, b"bli_acquire_mpart_ndim\0").map(|sym| *sym),
            bli_acquire_mpart_mndim: get_symbol(&libs, b"bli_acquire_mpart_mndim\0")
                .map(|sym| *sym),
            bli_acquire_vpart_f2b: get_symbol(&libs, b"bli_acquire_vpart_f2b\0").map(|sym| *sym),
            bli_acquire_vpart_b2f: get_symbol(&libs, b"bli_acquire_vpart_b2f\0").map(|sym| *sym),
            bli_acquire_mij: get_symbol(&libs, b"bli_acquire_mij\0").map(|sym| *sym),
            bli_acquire_vi: get_symbol(&libs, b"bli_acquire_vi\0").map(|sym| *sym),
            bli_prune_unref_mparts: get_symbol(&libs, b"bli_prune_unref_mparts\0").map(|sym| *sym),
            bli_obj_equals: get_symbol(&libs, b"bli_obj_equals\0").map(|sym| *sym),
            bli_obj_imag_equals: get_symbol(&libs, b"bli_obj_imag_equals\0").map(|sym| *sym),
            bli_obj_imag_is_zero: get_symbol(&libs, b"bli_obj_imag_is_zero\0").map(|sym| *sym),
            bli_param_map_blis_to_netlib_side: get_symbol(
                &libs,
                b"bli_param_map_blis_to_netlib_side\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_netlib_uplo: get_symbol(
                &libs,
                b"bli_param_map_blis_to_netlib_uplo\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_netlib_trans: get_symbol(
                &libs,
                b"bli_param_map_blis_to_netlib_trans\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_netlib_diag: get_symbol(
                &libs,
                b"bli_param_map_blis_to_netlib_diag\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_netlib_machval: get_symbol(
                &libs,
                b"bli_param_map_blis_to_netlib_machval\0",
            )
            .map(|sym| *sym),
            bli_param_map_char_to_blis_side: get_symbol(
                &libs,
                b"bli_param_map_char_to_blis_side\0",
            )
            .map(|sym| *sym),
            bli_param_map_char_to_blis_uplo: get_symbol(
                &libs,
                b"bli_param_map_char_to_blis_uplo\0",
            )
            .map(|sym| *sym),
            bli_param_map_char_to_blis_trans: get_symbol(
                &libs,
                b"bli_param_map_char_to_blis_trans\0",
            )
            .map(|sym| *sym),
            bli_param_map_char_to_blis_conj: get_symbol(
                &libs,
                b"bli_param_map_char_to_blis_conj\0",
            )
            .map(|sym| *sym),
            bli_param_map_char_to_blis_diag: get_symbol(
                &libs,
                b"bli_param_map_char_to_blis_diag\0",
            )
            .map(|sym| *sym),
            bli_param_map_char_to_blis_dt: get_symbol(&libs, b"bli_param_map_char_to_blis_dt\0")
                .map(|sym| *sym),
            bli_param_map_blis_to_char_side: get_symbol(
                &libs,
                b"bli_param_map_blis_to_char_side\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_char_uplo: get_symbol(
                &libs,
                b"bli_param_map_blis_to_char_uplo\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_char_trans: get_symbol(
                &libs,
                b"bli_param_map_blis_to_char_trans\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_char_conj: get_symbol(
                &libs,
                b"bli_param_map_blis_to_char_conj\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_char_diag: get_symbol(
                &libs,
                b"bli_param_map_blis_to_char_diag\0",
            )
            .map(|sym| *sym),
            bli_param_map_blis_to_char_dt: get_symbol(&libs, b"bli_param_map_blis_to_char_dt\0")
                .map(|sym| *sym),
            bli_clock: get_symbol(&libs, b"bli_clock\0").map(|sym| *sym),
            bli_clock_min_diff: get_symbol(&libs, b"bli_clock_min_diff\0").map(|sym| *sym),
            bli_clock_helper: get_symbol(&libs, b"bli_clock_helper\0").map(|sym| *sym),
            bli_check_error_code_helper: get_symbol(&libs, b"bli_check_error_code_helper\0")
                .map(|sym| *sym),
            bli_check_valid_error_level: get_symbol(&libs, b"bli_check_valid_error_level\0")
                .map(|sym| *sym),
            bli_check_null_pointer: get_symbol(&libs, b"bli_check_null_pointer\0").map(|sym| *sym),
            bli_check_valid_side: get_symbol(&libs, b"bli_check_valid_side\0").map(|sym| *sym),
            bli_check_valid_uplo: get_symbol(&libs, b"bli_check_valid_uplo\0").map(|sym| *sym),
            bli_check_valid_trans: get_symbol(&libs, b"bli_check_valid_trans\0").map(|sym| *sym),
            bli_check_valid_diag: get_symbol(&libs, b"bli_check_valid_diag\0").map(|sym| *sym),
            bli_check_nonunit_diag: get_symbol(&libs, b"bli_check_nonunit_diag\0").map(|sym| *sym),
            bli_check_valid_datatype: get_symbol(&libs, b"bli_check_valid_datatype\0")
                .map(|sym| *sym),
            bli_check_object_valid_datatype: get_symbol(
                &libs,
                b"bli_check_object_valid_datatype\0",
            )
            .map(|sym| *sym),
            bli_check_noninteger_datatype: get_symbol(&libs, b"bli_check_noninteger_datatype\0")
                .map(|sym| *sym),
            bli_check_noninteger_object: get_symbol(&libs, b"bli_check_noninteger_object\0")
                .map(|sym| *sym),
            bli_check_nonconstant_datatype: get_symbol(&libs, b"bli_check_nonconstant_datatype\0")
                .map(|sym| *sym),
            bli_check_nonconstant_object: get_symbol(&libs, b"bli_check_nonconstant_object\0")
                .map(|sym| *sym),
            bli_check_floating_datatype: get_symbol(&libs, b"bli_check_floating_datatype\0")
                .map(|sym| *sym),
            bli_check_floating_object: get_symbol(&libs, b"bli_check_floating_object\0")
                .map(|sym| *sym),
            bli_check_real_datatype: get_symbol(&libs, b"bli_check_real_datatype\0")
                .map(|sym| *sym),
            bli_check_real_object: get_symbol(&libs, b"bli_check_real_object\0").map(|sym| *sym),
            bli_check_integer_datatype: get_symbol(&libs, b"bli_check_integer_datatype\0")
                .map(|sym| *sym),
            bli_check_integer_object: get_symbol(&libs, b"bli_check_integer_object\0")
                .map(|sym| *sym),
            bli_check_consistent_datatypes: get_symbol(&libs, b"bli_check_consistent_datatypes\0")
                .map(|sym| *sym),
            bli_check_consistent_object_datatypes: get_symbol(
                &libs,
                b"bli_check_consistent_object_datatypes\0",
            )
            .map(|sym| *sym),
            bli_check_datatype_real_proj_of: get_symbol(
                &libs,
                b"bli_check_datatype_real_proj_of\0",
            )
            .map(|sym| *sym),
            bli_check_object_real_proj_of: get_symbol(&libs, b"bli_check_object_real_proj_of\0")
                .map(|sym| *sym),
            bli_check_real_valued_object: get_symbol(&libs, b"bli_check_real_valued_object\0")
                .map(|sym| *sym),
            bli_check_consistent_precisions: get_symbol(
                &libs,
                b"bli_check_consistent_precisions\0",
            )
            .map(|sym| *sym),
            bli_check_consistent_object_precisions: get_symbol(
                &libs,
                b"bli_check_consistent_object_precisions\0",
            )
            .map(|sym| *sym),
            bli_check_conformal_dims: get_symbol(&libs, b"bli_check_conformal_dims\0")
                .map(|sym| *sym),
            bli_check_level3_dims: get_symbol(&libs, b"bli_check_level3_dims\0").map(|sym| *sym),
            bli_check_scalar_object: get_symbol(&libs, b"bli_check_scalar_object\0")
                .map(|sym| *sym),
            bli_check_vector_object: get_symbol(&libs, b"bli_check_vector_object\0")
                .map(|sym| *sym),
            bli_check_matrix_object: get_symbol(&libs, b"bli_check_matrix_object\0")
                .map(|sym| *sym),
            bli_check_equal_vector_lengths: get_symbol(&libs, b"bli_check_equal_vector_lengths\0")
                .map(|sym| *sym),
            bli_check_square_object: get_symbol(&libs, b"bli_check_square_object\0")
                .map(|sym| *sym),
            bli_check_object_length_equals: get_symbol(&libs, b"bli_check_object_length_equals\0")
                .map(|sym| *sym),
            bli_check_object_width_equals: get_symbol(&libs, b"bli_check_object_width_equals\0")
                .map(|sym| *sym),
            bli_check_vector_dim_equals: get_symbol(&libs, b"bli_check_vector_dim_equals\0")
                .map(|sym| *sym),
            bli_check_object_diag_offset_equals: get_symbol(
                &libs,
                b"bli_check_object_diag_offset_equals\0",
            )
            .map(|sym| *sym),
            bli_check_matrix_strides: get_symbol(&libs, b"bli_check_matrix_strides\0")
                .map(|sym| *sym),
            bli_check_general_object: get_symbol(&libs, b"bli_check_general_object\0")
                .map(|sym| *sym),
            bli_check_hermitian_object: get_symbol(&libs, b"bli_check_hermitian_object\0")
                .map(|sym| *sym),
            bli_check_symmetric_object: get_symbol(&libs, b"bli_check_symmetric_object\0")
                .map(|sym| *sym),
            bli_check_triangular_object: get_symbol(&libs, b"bli_check_triangular_object\0")
                .map(|sym| *sym),
            bli_check_object_struc: get_symbol(&libs, b"bli_check_object_struc\0").map(|sym| *sym),
            bli_check_upper_or_lower_object: get_symbol(
                &libs,
                b"bli_check_upper_or_lower_object\0",
            )
            .map(|sym| *sym),
            bli_check_valid_3x1_subpart: get_symbol(&libs, b"bli_check_valid_3x1_subpart\0")
                .map(|sym| *sym),
            bli_check_valid_1x3_subpart: get_symbol(&libs, b"bli_check_valid_1x3_subpart\0")
                .map(|sym| *sym),
            bli_check_valid_3x3_subpart: get_symbol(&libs, b"bli_check_valid_3x3_subpart\0")
                .map(|sym| *sym),
            bli_check_valid_cntl: get_symbol(&libs, b"bli_check_valid_cntl\0").map(|sym| *sym),
            bli_check_packm_schema_on_unpack: get_symbol(
                &libs,
                b"bli_check_packm_schema_on_unpack\0",
            )
            .map(|sym| *sym),
            bli_check_packv_schema_on_unpack: get_symbol(
                &libs,
                b"bli_check_packv_schema_on_unpack\0",
            )
            .map(|sym| *sym),
            bli_check_object_buffer: get_symbol(&libs, b"bli_check_object_buffer\0")
                .map(|sym| *sym),
            bli_check_valid_malloc_buf: get_symbol(&libs, b"bli_check_valid_malloc_buf\0")
                .map(|sym| *sym),
            bli_check_valid_packbuf: get_symbol(&libs, b"bli_check_valid_packbuf\0")
                .map(|sym| *sym),
            bli_check_if_exhausted_pool: get_symbol(&libs, b"bli_check_if_exhausted_pool\0")
                .map(|sym| *sym),
            bli_check_sufficient_stack_buf_size: get_symbol(
                &libs,
                b"bli_check_sufficient_stack_buf_size\0",
            )
            .map(|sym| *sym),
            bli_check_alignment_is_power_of_two: get_symbol(
                &libs,
                b"bli_check_alignment_is_power_of_two\0",
            )
            .map(|sym| *sym),
            bli_check_alignment_is_mult_of_ptr_size: get_symbol(
                &libs,
                b"bli_check_alignment_is_mult_of_ptr_size\0",
            )
            .map(|sym| *sym),
            bli_check_object_alias_of: get_symbol(&libs, b"bli_check_object_alias_of\0")
                .map(|sym| *sym),
            bli_check_valid_arch_id: get_symbol(&libs, b"bli_check_valid_arch_id\0")
                .map(|sym| *sym),
            bli_check_valid_model_id: get_symbol(&libs, b"bli_check_valid_model_id\0")
                .map(|sym| *sym),
            bli_check_initialized_gks_cntx: get_symbol(&libs, b"bli_check_initialized_gks_cntx\0")
                .map(|sym| *sym),
            bli_check_valid_mc_mod_mult: get_symbol(&libs, b"bli_check_valid_mc_mod_mult\0")
                .map(|sym| *sym),
            bli_check_valid_nc_mod_mult: get_symbol(&libs, b"bli_check_valid_nc_mod_mult\0")
                .map(|sym| *sym),
            bli_check_valid_kc_mod_mult: get_symbol(&libs, b"bli_check_valid_kc_mod_mult\0")
                .map(|sym| *sym),
            bli_error_checking_level: get_symbol(&libs, b"bli_error_checking_level\0")
                .map(|sym| *sym),
            bli_error_checking_level_set: get_symbol(&libs, b"bli_error_checking_level_set\0")
                .map(|sym| *sym),
            bli_error_checking_is_enabled: get_symbol(&libs, b"bli_error_checking_is_enabled\0")
                .map(|sym| *sym),
            bli_print_msg: get_symbol(&libs, b"bli_print_msg\0").map(|sym| *sym),
            bli_abort: get_symbol(&libs, b"bli_abort\0").map(|sym| *sym),
            bli_error_string_for_code: get_symbol(&libs, b"bli_error_string_for_code\0")
                .map(|sym| *sym),
            bli_lsame: get_symbol(&libs, b"bli_lsame\0").map(|sym| *sym),
            bli_slamch: get_symbol(&libs, b"bli_slamch\0").map(|sym| *sym),
            bli_dlamch: get_symbol(&libs, b"bli_dlamch\0").map(|sym| *sym),
            bli_machval: get_symbol(&libs, b"bli_machval\0").map(|sym| *sym),
            bli_smachval: get_symbol(&libs, b"bli_smachval\0").map(|sym| *sym),
            bli_dmachval: get_symbol(&libs, b"bli_dmachval\0").map(|sym| *sym),
            bli_cmachval: get_symbol(&libs, b"bli_cmachval\0").map(|sym| *sym),
            bli_zmachval: get_symbol(&libs, b"bli_zmachval\0").map(|sym| *sym),
            bli_getopt_init_state: get_symbol(&libs, b"bli_getopt_init_state\0").map(|sym| *sym),
            bli_getopt: get_symbol(&libs, b"bli_getopt\0").map(|sym| *sym),
            bli_cntl_create_node: get_symbol(&libs, b"bli_cntl_create_node\0").map(|sym| *sym),
            bli_cntl_free_node: get_symbol(&libs, b"bli_cntl_free_node\0").map(|sym| *sym),
            bli_cntl_clear_node: get_symbol(&libs, b"bli_cntl_clear_node\0").map(|sym| *sym),
            bli_cntl_free: get_symbol(&libs, b"bli_cntl_free\0").map(|sym| *sym),
            bli_cntl_free_w_thrinfo: get_symbol(&libs, b"bli_cntl_free_w_thrinfo\0")
                .map(|sym| *sym),
            bli_cntl_free_wo_thrinfo: get_symbol(&libs, b"bli_cntl_free_wo_thrinfo\0")
                .map(|sym| *sym),
            bli_cntl_copy: get_symbol(&libs, b"bli_cntl_copy\0").map(|sym| *sym),
            bli_cntl_mark_family: get_symbol(&libs, b"bli_cntl_mark_family\0").map(|sym| *sym),
            bli_cntl_calc_num_threads_in: get_symbol(&libs, b"bli_cntl_calc_num_threads_in\0")
                .map(|sym| *sym),
            bli_env_get_var: get_symbol(&libs, b"bli_env_get_var\0").map(|sym| *sym),
            bli_env_get_var_arch_type: get_symbol(&libs, b"bli_env_get_var_arch_type\0")
                .map(|sym| *sym),
            bli_env_get_var_model_type: get_symbol(&libs, b"bli_env_get_var_model_type\0")
                .map(|sym| *sym),
            bli_pack_init: get_symbol(&libs, b"bli_pack_init\0").map(|sym| *sym),
            bli_pack_finalize: get_symbol(&libs, b"bli_pack_finalize\0").map(|sym| *sym),
            bli_pack_get_pack_a: get_symbol(&libs, b"bli_pack_get_pack_a\0").map(|sym| *sym),
            bli_pack_get_pack_b: get_symbol(&libs, b"bli_pack_get_pack_b\0").map(|sym| *sym),
            bli_pack_set_pack_a: get_symbol(&libs, b"bli_pack_set_pack_a\0").map(|sym| *sym),
            bli_pack_set_pack_b: get_symbol(&libs, b"bli_pack_set_pack_b\0").map(|sym| *sym),
            bli_pack_init_rntm_from_env: get_symbol(&libs, b"bli_pack_init_rntm_from_env\0")
                .map(|sym| *sym),
            bli_info_get_version_str: get_symbol(&libs, b"bli_info_get_version_str\0")
                .map(|sym| *sym),
            bli_info_get_int_type_size_str: get_symbol(&libs, b"bli_info_get_int_type_size_str\0")
                .map(|sym| *sym),
            bli_info_get_int_type_size: get_symbol(&libs, b"bli_info_get_int_type_size\0")
                .map(|sym| *sym),
            bli_info_get_num_fp_types: get_symbol(&libs, b"bli_info_get_num_fp_types\0")
                .map(|sym| *sym),
            bli_info_get_max_type_size: get_symbol(&libs, b"bli_info_get_max_type_size\0")
                .map(|sym| *sym),
            bli_info_get_page_size: get_symbol(&libs, b"bli_info_get_page_size\0").map(|sym| *sym),
            bli_info_get_simd_num_registers: get_symbol(
                &libs,
                b"bli_info_get_simd_num_registers\0",
            )
            .map(|sym| *sym),
            bli_info_get_simd_size: get_symbol(&libs, b"bli_info_get_simd_size\0").map(|sym| *sym),
            bli_info_get_simd_align_size: get_symbol(&libs, b"bli_info_get_simd_align_size\0")
                .map(|sym| *sym),
            bli_info_get_stack_buf_max_size: get_symbol(
                &libs,
                b"bli_info_get_stack_buf_max_size\0",
            )
            .map(|sym| *sym),
            bli_info_get_stack_buf_align_size: get_symbol(
                &libs,
                b"bli_info_get_stack_buf_align_size\0",
            )
            .map(|sym| *sym),
            bli_info_get_heap_addr_align_size: get_symbol(
                &libs,
                b"bli_info_get_heap_addr_align_size\0",
            )
            .map(|sym| *sym),
            bli_info_get_heap_stride_align_size: get_symbol(
                &libs,
                b"bli_info_get_heap_stride_align_size\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_align_size_a: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_align_size_a\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_align_size_b: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_align_size_b\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_align_size_c: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_align_size_c\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_align_size_gen: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_align_size_gen\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_offset_size_a: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_offset_size_a\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_offset_size_b: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_offset_size_b\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_offset_size_c: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_offset_size_c\0",
            )
            .map(|sym| *sym),
            bli_info_get_pool_addr_offset_size_gen: get_symbol(
                &libs,
                b"bli_info_get_pool_addr_offset_size_gen\0",
            )
            .map(|sym| *sym),
            bli_info_get_enable_stay_auto_init: get_symbol(
                &libs,
                b"bli_info_get_enable_stay_auto_init\0",
            )
            .map(|sym| *sym),
            bli_info_get_enable_blas: get_symbol(&libs, b"bli_info_get_enable_blas\0")
                .map(|sym| *sym),
            bli_info_get_enable_cblas: get_symbol(&libs, b"bli_info_get_enable_cblas\0")
                .map(|sym| *sym),
            bli_info_get_blas_int_type_size: get_symbol(
                &libs,
                b"bli_info_get_blas_int_type_size\0",
            )
            .map(|sym| *sym),
            bli_info_get_enable_pba_pools: get_symbol(&libs, b"bli_info_get_enable_pba_pools\0")
                .map(|sym| *sym),
            bli_info_get_enable_sba_pools: get_symbol(&libs, b"bli_info_get_enable_sba_pools\0")
                .map(|sym| *sym),
            bli_info_get_enable_threading: get_symbol(&libs, b"bli_info_get_enable_threading\0")
                .map(|sym| *sym),
            bli_info_get_enable_openmp: get_symbol(&libs, b"bli_info_get_enable_openmp\0")
                .map(|sym| *sym),
            bli_info_get_enable_pthreads: get_symbol(&libs, b"bli_info_get_enable_pthreads\0")
                .map(|sym| *sym),
            bli_info_get_thread_part_jrir_slab: get_symbol(
                &libs,
                b"bli_info_get_thread_part_jrir_slab\0",
            )
            .map(|sym| *sym),
            bli_info_get_thread_part_jrir_rr: get_symbol(
                &libs,
                b"bli_info_get_thread_part_jrir_rr\0",
            )
            .map(|sym| *sym),
            bli_info_get_enable_memkind: get_symbol(&libs, b"bli_info_get_enable_memkind\0")
                .map(|sym| *sym),
            bli_info_get_enable_sandbox: get_symbol(&libs, b"bli_info_get_enable_sandbox\0")
                .map(|sym| *sym),
            bli_info_get_info_value: get_symbol(&libs, b"bli_info_get_info_value\0")
                .map(|sym| *sym),
            bli_info_get_gemm_ukr_impl_string: get_symbol(
                &libs,
                b"bli_info_get_gemm_ukr_impl_string\0",
            )
            .map(|sym| *sym),
            bli_info_get_gemmtrsm_l_ukr_impl_string: get_symbol(
                &libs,
                b"bli_info_get_gemmtrsm_l_ukr_impl_string\0",
            )
            .map(|sym| *sym),
            bli_info_get_gemmtrsm_u_ukr_impl_string: get_symbol(
                &libs,
                b"bli_info_get_gemmtrsm_u_ukr_impl_string\0",
            )
            .map(|sym| *sym),
            bli_info_get_trsm_l_ukr_impl_string: get_symbol(
                &libs,
                b"bli_info_get_trsm_l_ukr_impl_string\0",
            )
            .map(|sym| *sym),
            bli_info_get_trsm_u_ukr_impl_string: get_symbol(
                &libs,
                b"bli_info_get_trsm_u_ukr_impl_string\0",
            )
            .map(|sym| *sym),
            bli_info_get_gemm_impl_string: get_symbol(&libs, b"bli_info_get_gemm_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_hemm_impl_string: get_symbol(&libs, b"bli_info_get_hemm_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_herk_impl_string: get_symbol(&libs, b"bli_info_get_herk_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_her2k_impl_string: get_symbol(&libs, b"bli_info_get_her2k_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_symm_impl_string: get_symbol(&libs, b"bli_info_get_symm_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_syrk_impl_string: get_symbol(&libs, b"bli_info_get_syrk_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_syr2k_impl_string: get_symbol(&libs, b"bli_info_get_syr2k_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_trmm_impl_string: get_symbol(&libs, b"bli_info_get_trmm_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_trmm3_impl_string: get_symbol(&libs, b"bli_info_get_trmm3_impl_string\0")
                .map(|sym| *sym),
            bli_info_get_trsm_impl_string: get_symbol(&libs, b"bli_info_get_trsm_impl_string\0")
                .map(|sym| *sym),
            bli_arch_query_id: get_symbol(&libs, b"bli_arch_query_id\0").map(|sym| *sym),
            bli_aocl_enable_instruction_query: get_symbol(
                &libs,
                b"bli_aocl_enable_instruction_query\0",
            )
            .map(|sym| *sym),
            bli_arch_set_id_once: get_symbol(&libs, b"bli_arch_set_id_once\0").map(|sym| *sym),
            bli_arch_set_id: get_symbol(&libs, b"bli_arch_set_id\0").map(|sym| *sym),
            bli_arch_check_id_once: get_symbol(&libs, b"bli_arch_check_id_once\0").map(|sym| *sym),
            bli_arch_check_id: get_symbol(&libs, b"bli_arch_check_id\0").map(|sym| *sym),
            bli_arch_string: get_symbol(&libs, b"bli_arch_string\0").map(|sym| *sym),
            bli_arch_set_logging: get_symbol(&libs, b"bli_arch_set_logging\0").map(|sym| *sym),
            bli_arch_get_logging: get_symbol(&libs, b"bli_arch_get_logging\0").map(|sym| *sym),
            bli_arch_log: get_symbol(&libs, b"bli_arch_log\0").map(|sym| *sym),
            bli_model_query_id: get_symbol(&libs, b"bli_model_query_id\0").map(|sym| *sym),
            bli_init_model_query_id: get_symbol(&libs, b"bli_init_model_query_id\0")
                .map(|sym| *sym),
            bli_model_string: get_symbol(&libs, b"bli_model_string\0").map(|sym| *sym),
            bli_cpuid_query_id: get_symbol(&libs, b"bli_cpuid_query_id\0").map(|sym| *sym),
            bli_cpuid_query_model_id: get_symbol(&libs, b"bli_cpuid_query_model_id\0")
                .map(|sym| *sym),
            bli_cpuid_query_fp_datapath: get_symbol(&libs, b"bli_cpuid_query_fp_datapath\0")
                .map(|sym| *sym),
            bli_cpuid_query_l1d_cache_size: get_symbol(&libs, b"bli_cpuid_query_l1d_cache_size\0")
                .map(|sym| *sym),
            bli_cpuid_query_l1i_cache_size: get_symbol(&libs, b"bli_cpuid_query_l1i_cache_size\0")
                .map(|sym| *sym),
            bli_cpuid_query_l2_cache_size: get_symbol(&libs, b"bli_cpuid_query_l2_cache_size\0")
                .map(|sym| *sym),
            bli_cpuid_query_l3_cache_size: get_symbol(&libs, b"bli_cpuid_query_l3_cache_size\0")
                .map(|sym| *sym),
            bli_cpuid_is_skx: get_symbol(&libs, b"bli_cpuid_is_skx\0").map(|sym| *sym),
            bli_cpuid_is_knl: get_symbol(&libs, b"bli_cpuid_is_knl\0").map(|sym| *sym),
            bli_cpuid_is_haswell: get_symbol(&libs, b"bli_cpuid_is_haswell\0").map(|sym| *sym),
            bli_cpuid_is_sandybridge: get_symbol(&libs, b"bli_cpuid_is_sandybridge\0")
                .map(|sym| *sym),
            bli_cpuid_is_penryn: get_symbol(&libs, b"bli_cpuid_is_penryn\0").map(|sym| *sym),
            bli_cpuid_is_zen5: get_symbol(&libs, b"bli_cpuid_is_zen5\0").map(|sym| *sym),
            bli_cpuid_is_zen4: get_symbol(&libs, b"bli_cpuid_is_zen4\0").map(|sym| *sym),
            bli_cpuid_is_avx512_fallback: get_symbol(&libs, b"bli_cpuid_is_avx512_fallback\0")
                .map(|sym| *sym),
            bli_cpuid_is_zen3: get_symbol(&libs, b"bli_cpuid_is_zen3\0").map(|sym| *sym),
            bli_cpuid_is_zen2: get_symbol(&libs, b"bli_cpuid_is_zen2\0").map(|sym| *sym),
            bli_cpuid_is_zen: get_symbol(&libs, b"bli_cpuid_is_zen\0").map(|sym| *sym),
            bli_cpuid_is_excavator: get_symbol(&libs, b"bli_cpuid_is_excavator\0").map(|sym| *sym),
            bli_cpuid_is_steamroller: get_symbol(&libs, b"bli_cpuid_is_steamroller\0")
                .map(|sym| *sym),
            bli_cpuid_is_piledriver: get_symbol(&libs, b"bli_cpuid_is_piledriver\0")
                .map(|sym| *sym),
            bli_cpuid_is_bulldozer: get_symbol(&libs, b"bli_cpuid_is_bulldozer\0").map(|sym| *sym),
            bli_cpuid_get_zen5_cpuid_model: get_symbol(&libs, b"bli_cpuid_get_zen5_cpuid_model\0")
                .map(|sym| *sym),
            bli_cpuid_get_zen4_cpuid_model: get_symbol(&libs, b"bli_cpuid_get_zen4_cpuid_model\0")
                .map(|sym| *sym),
            bli_cpuid_get_zen3_cpuid_model: get_symbol(&libs, b"bli_cpuid_get_zen3_cpuid_model\0")
                .map(|sym| *sym),
            bli_cpuid_is_thunderx2: get_symbol(&libs, b"bli_cpuid_is_thunderx2\0").map(|sym| *sym),
            bli_cpuid_is_cortexa57: get_symbol(&libs, b"bli_cpuid_is_cortexa57\0").map(|sym| *sym),
            bli_cpuid_is_cortexa53: get_symbol(&libs, b"bli_cpuid_is_cortexa53\0").map(|sym| *sym),
            bli_cpuid_is_armsve: get_symbol(&libs, b"bli_cpuid_is_armsve\0").map(|sym| *sym),
            bli_cpuid_is_a64fx: get_symbol(&libs, b"bli_cpuid_is_a64fx\0").map(|sym| *sym),
            bli_cpuid_is_cortexa15: get_symbol(&libs, b"bli_cpuid_is_cortexa15\0").map(|sym| *sym),
            bli_cpuid_is_cortexa9: get_symbol(&libs, b"bli_cpuid_is_cortexa9\0").map(|sym| *sym),
            bli_cpuid_query: get_symbol(&libs, b"bli_cpuid_query\0").map(|sym| *sym),
            bli_cpuid_check_datapath: get_symbol(&libs, b"bli_cpuid_check_datapath\0")
                .map(|sym| *sym),
            bli_cpuid_check_cache: get_symbol(&libs, b"bli_cpuid_check_cache\0").map(|sym| *sym),
            get_cpu_name: get_symbol(&libs, b"get_cpu_name\0").map(|sym| *sym),
            vpu_count: get_symbol(&libs, b"vpu_count\0").map(|sym| *sym),
            bli_cpuid_is_avx2fma3_supported: get_symbol(
                &libs,
                b"bli_cpuid_is_avx2fma3_supported\0",
            )
            .map(|sym| *sym),
            bli_cpuid_is_avx512_supported: get_symbol(&libs, b"bli_cpuid_is_avx512_supported\0")
                .map(|sym| *sym),
            bli_cpuid_is_avx512vnni_supported: get_symbol(
                &libs,
                b"bli_cpuid_is_avx512vnni_supported\0",
            )
            .map(|sym| *sym),
            bli_cpuid_is_avx512bf16_supported: get_symbol(
                &libs,
                b"bli_cpuid_is_avx512bf16_supported\0",
            )
            .map(|sym| *sym),
            bli_cpuid_check_avx2fma3_support: get_symbol(
                &libs,
                b"bli_cpuid_check_avx2fma3_support\0",
            )
            .map(|sym| *sym),
            bli_cpuid_check_avx512_support: get_symbol(&libs, b"bli_cpuid_check_avx512_support\0")
                .map(|sym| *sym),
            bli_cpuid_check_avx512vnni_support: get_symbol(
                &libs,
                b"bli_cpuid_check_avx512vnni_support\0",
            )
            .map(|sym| *sym),
            bli_cpuid_check_avx512bf16_support: get_symbol(
                &libs,
                b"bli_cpuid_check_avx512bf16_support\0",
            )
            .map(|sym| *sym),
            bli_string_mkupper: get_symbol(&libs, b"bli_string_mkupper\0").map(|sym| *sym),
            bli_setijm: get_symbol(&libs, b"bli_setijm\0").map(|sym| *sym),
            bli_ssetijm: get_symbol(&libs, b"bli_ssetijm\0").map(|sym| *sym),
            bli_dsetijm: get_symbol(&libs, b"bli_dsetijm\0").map(|sym| *sym),
            bli_csetijm: get_symbol(&libs, b"bli_csetijm\0").map(|sym| *sym),
            bli_zsetijm: get_symbol(&libs, b"bli_zsetijm\0").map(|sym| *sym),
            bli_getijm: get_symbol(&libs, b"bli_getijm\0").map(|sym| *sym),
            bli_sgetijm: get_symbol(&libs, b"bli_sgetijm\0").map(|sym| *sym),
            bli_dgetijm: get_symbol(&libs, b"bli_dgetijm\0").map(|sym| *sym),
            bli_cgetijm: get_symbol(&libs, b"bli_cgetijm\0").map(|sym| *sym),
            bli_zgetijm: get_symbol(&libs, b"bli_zgetijm\0").map(|sym| *sym),
            bli_setijv: get_symbol(&libs, b"bli_setijv\0").map(|sym| *sym),
            bli_ssetijv: get_symbol(&libs, b"bli_ssetijv\0").map(|sym| *sym),
            bli_dsetijv: get_symbol(&libs, b"bli_dsetijv\0").map(|sym| *sym),
            bli_csetijv: get_symbol(&libs, b"bli_csetijv\0").map(|sym| *sym),
            bli_zsetijv: get_symbol(&libs, b"bli_zsetijv\0").map(|sym| *sym),
            bli_getijv: get_symbol(&libs, b"bli_getijv\0").map(|sym| *sym),
            bli_sgetijv: get_symbol(&libs, b"bli_sgetijv\0").map(|sym| *sym),
            bli_dgetijv: get_symbol(&libs, b"bli_dgetijv\0").map(|sym| *sym),
            bli_cgetijv: get_symbol(&libs, b"bli_cgetijv\0").map(|sym| *sym),
            bli_zgetijv: get_symbol(&libs, b"bli_zgetijv\0").map(|sym| *sym),
            bli_setrm: get_symbol(&libs, b"bli_setrm\0").map(|sym| *sym),
            bli_setrv: get_symbol(&libs, b"bli_setrv\0").map(|sym| *sym),
            bli_setim: get_symbol(&libs, b"bli_setim\0").map(|sym| *sym),
            bli_setiv: get_symbol(&libs, b"bli_setiv\0").map(|sym| *sym),
            bli_castm: get_symbol(&libs, b"bli_castm\0").map(|sym| *sym),
            bli_sscastm: get_symbol(&libs, b"bli_sscastm\0").map(|sym| *sym),
            bli_ddcastm: get_symbol(&libs, b"bli_ddcastm\0").map(|sym| *sym),
            bli_cccastm: get_symbol(&libs, b"bli_cccastm\0").map(|sym| *sym),
            bli_zzcastm: get_symbol(&libs, b"bli_zzcastm\0").map(|sym| *sym),
            bli_sdcastm: get_symbol(&libs, b"bli_sdcastm\0").map(|sym| *sym),
            bli_sccastm: get_symbol(&libs, b"bli_sccastm\0").map(|sym| *sym),
            bli_szcastm: get_symbol(&libs, b"bli_szcastm\0").map(|sym| *sym),
            bli_dscastm: get_symbol(&libs, b"bli_dscastm\0").map(|sym| *sym),
            bli_dccastm: get_symbol(&libs, b"bli_dccastm\0").map(|sym| *sym),
            bli_dzcastm: get_symbol(&libs, b"bli_dzcastm\0").map(|sym| *sym),
            bli_cscastm: get_symbol(&libs, b"bli_cscastm\0").map(|sym| *sym),
            bli_cdcastm: get_symbol(&libs, b"bli_cdcastm\0").map(|sym| *sym),
            bli_czcastm: get_symbol(&libs, b"bli_czcastm\0").map(|sym| *sym),
            bli_zscastm: get_symbol(&libs, b"bli_zscastm\0").map(|sym| *sym),
            bli_zdcastm: get_symbol(&libs, b"bli_zdcastm\0").map(|sym| *sym),
            bli_zccastm: get_symbol(&libs, b"bli_zccastm\0").map(|sym| *sym),
            bli_castm_check: get_symbol(&libs, b"bli_castm_check\0").map(|sym| *sym),
            bli_castnzm: get_symbol(&libs, b"bli_castnzm\0").map(|sym| *sym),
            bli_sscastnzm: get_symbol(&libs, b"bli_sscastnzm\0").map(|sym| *sym),
            bli_ddcastnzm: get_symbol(&libs, b"bli_ddcastnzm\0").map(|sym| *sym),
            bli_cccastnzm: get_symbol(&libs, b"bli_cccastnzm\0").map(|sym| *sym),
            bli_zzcastnzm: get_symbol(&libs, b"bli_zzcastnzm\0").map(|sym| *sym),
            bli_sdcastnzm: get_symbol(&libs, b"bli_sdcastnzm\0").map(|sym| *sym),
            bli_sccastnzm: get_symbol(&libs, b"bli_sccastnzm\0").map(|sym| *sym),
            bli_szcastnzm: get_symbol(&libs, b"bli_szcastnzm\0").map(|sym| *sym),
            bli_dscastnzm: get_symbol(&libs, b"bli_dscastnzm\0").map(|sym| *sym),
            bli_dccastnzm: get_symbol(&libs, b"bli_dccastnzm\0").map(|sym| *sym),
            bli_dzcastnzm: get_symbol(&libs, b"bli_dzcastnzm\0").map(|sym| *sym),
            bli_cscastnzm: get_symbol(&libs, b"bli_cscastnzm\0").map(|sym| *sym),
            bli_cdcastnzm: get_symbol(&libs, b"bli_cdcastnzm\0").map(|sym| *sym),
            bli_czcastnzm: get_symbol(&libs, b"bli_czcastnzm\0").map(|sym| *sym),
            bli_zscastnzm: get_symbol(&libs, b"bli_zscastnzm\0").map(|sym| *sym),
            bli_zdcastnzm: get_symbol(&libs, b"bli_zdcastnzm\0").map(|sym| *sym),
            bli_zccastnzm: get_symbol(&libs, b"bli_zccastnzm\0").map(|sym| *sym),
            bli_castnzm_check: get_symbol(&libs, b"bli_castnzm_check\0").map(|sym| *sym),
            bli_castv: get_symbol(&libs, b"bli_castv\0").map(|sym| *sym),
            bli_sscastv: get_symbol(&libs, b"bli_sscastv\0").map(|sym| *sym),
            bli_ddcastv: get_symbol(&libs, b"bli_ddcastv\0").map(|sym| *sym),
            bli_cccastv: get_symbol(&libs, b"bli_cccastv\0").map(|sym| *sym),
            bli_zzcastv: get_symbol(&libs, b"bli_zzcastv\0").map(|sym| *sym),
            bli_sdcastv: get_symbol(&libs, b"bli_sdcastv\0").map(|sym| *sym),
            bli_sccastv: get_symbol(&libs, b"bli_sccastv\0").map(|sym| *sym),
            bli_szcastv: get_symbol(&libs, b"bli_szcastv\0").map(|sym| *sym),
            bli_dscastv: get_symbol(&libs, b"bli_dscastv\0").map(|sym| *sym),
            bli_dccastv: get_symbol(&libs, b"bli_dccastv\0").map(|sym| *sym),
            bli_dzcastv: get_symbol(&libs, b"bli_dzcastv\0").map(|sym| *sym),
            bli_cscastv: get_symbol(&libs, b"bli_cscastv\0").map(|sym| *sym),
            bli_cdcastv: get_symbol(&libs, b"bli_cdcastv\0").map(|sym| *sym),
            bli_czcastv: get_symbol(&libs, b"bli_czcastv\0").map(|sym| *sym),
            bli_zscastv: get_symbol(&libs, b"bli_zscastv\0").map(|sym| *sym),
            bli_zdcastv: get_symbol(&libs, b"bli_zdcastv\0").map(|sym| *sym),
            bli_zccastv: get_symbol(&libs, b"bli_zccastv\0").map(|sym| *sym),
            bli_castv_check: get_symbol(&libs, b"bli_castv_check\0").map(|sym| *sym),
            bli_projm: get_symbol(&libs, b"bli_projm\0").map(|sym| *sym),
            bli_projm_check: get_symbol(&libs, b"bli_projm_check\0").map(|sym| *sym),
            bli_projv: get_symbol(&libs, b"bli_projv\0").map(|sym| *sym),
            bli_projv_check: get_symbol(&libs, b"bli_projv_check\0").map(|sym| *sym),
            bli_addsc_check: get_symbol(&libs, b"bli_addsc_check\0").map(|sym| *sym),
            bli_copysc_check: get_symbol(&libs, b"bli_copysc_check\0").map(|sym| *sym),
            bli_divsc_check: get_symbol(&libs, b"bli_divsc_check\0").map(|sym| *sym),
            bli_mulsc_check: get_symbol(&libs, b"bli_mulsc_check\0").map(|sym| *sym),
            bli_sqrtsc_check: get_symbol(&libs, b"bli_sqrtsc_check\0").map(|sym| *sym),
            bli_subsc_check: get_symbol(&libs, b"bli_subsc_check\0").map(|sym| *sym),
            bli_invertsc_check: get_symbol(&libs, b"bli_invertsc_check\0").map(|sym| *sym),
            bli_absqsc_check: get_symbol(&libs, b"bli_absqsc_check\0").map(|sym| *sym),
            bli_normfsc_check: get_symbol(&libs, b"bli_normfsc_check\0").map(|sym| *sym),
            bli_getsc_check: get_symbol(&libs, b"bli_getsc_check\0").map(|sym| *sym),
            bli_setsc_check: get_symbol(&libs, b"bli_setsc_check\0").map(|sym| *sym),
            bli_unzipsc_check: get_symbol(&libs, b"bli_unzipsc_check\0").map(|sym| *sym),
            bli_zipsc_check: get_symbol(&libs, b"bli_zipsc_check\0").map(|sym| *sym),
            bli_l0_xsc_check: get_symbol(&libs, b"bli_l0_xsc_check\0").map(|sym| *sym),
            bli_l0_xxsc_check: get_symbol(&libs, b"bli_l0_xxsc_check\0").map(|sym| *sym),
            bli_l0_xx2sc_check: get_symbol(&libs, b"bli_l0_xx2sc_check\0").map(|sym| *sym),
            bli_l0_xxbsc_check: get_symbol(&libs, b"bli_l0_xxbsc_check\0").map(|sym| *sym),
            bli_absqsc: get_symbol(&libs, b"bli_absqsc\0").map(|sym| *sym),
            bli_normfsc: get_symbol(&libs, b"bli_normfsc\0").map(|sym| *sym),
            bli_addsc: get_symbol(&libs, b"bli_addsc\0").map(|sym| *sym),
            bli_divsc: get_symbol(&libs, b"bli_divsc\0").map(|sym| *sym),
            bli_mulsc: get_symbol(&libs, b"bli_mulsc\0").map(|sym| *sym),
            bli_sqrtsc: get_symbol(&libs, b"bli_sqrtsc\0").map(|sym| *sym),
            bli_subsc: get_symbol(&libs, b"bli_subsc\0").map(|sym| *sym),
            bli_invertsc: get_symbol(&libs, b"bli_invertsc\0").map(|sym| *sym),
            bli_getsc: get_symbol(&libs, b"bli_getsc\0").map(|sym| *sym),
            bli_setsc: get_symbol(&libs, b"bli_setsc\0").map(|sym| *sym),
            bli_unzipsc: get_symbol(&libs, b"bli_unzipsc\0").map(|sym| *sym),
            bli_zipsc: get_symbol(&libs, b"bli_zipsc\0").map(|sym| *sym),
            bli_saddsc: get_symbol(&libs, b"bli_saddsc\0").map(|sym| *sym),
            bli_daddsc: get_symbol(&libs, b"bli_daddsc\0").map(|sym| *sym),
            bli_caddsc: get_symbol(&libs, b"bli_caddsc\0").map(|sym| *sym),
            bli_zaddsc: get_symbol(&libs, b"bli_zaddsc\0").map(|sym| *sym),
            bli_sdivsc: get_symbol(&libs, b"bli_sdivsc\0").map(|sym| *sym),
            bli_ddivsc: get_symbol(&libs, b"bli_ddivsc\0").map(|sym| *sym),
            bli_cdivsc: get_symbol(&libs, b"bli_cdivsc\0").map(|sym| *sym),
            bli_zdivsc: get_symbol(&libs, b"bli_zdivsc\0").map(|sym| *sym),
            bli_smulsc: get_symbol(&libs, b"bli_smulsc\0").map(|sym| *sym),
            bli_dmulsc: get_symbol(&libs, b"bli_dmulsc\0").map(|sym| *sym),
            bli_cmulsc: get_symbol(&libs, b"bli_cmulsc\0").map(|sym| *sym),
            bli_zmulsc: get_symbol(&libs, b"bli_zmulsc\0").map(|sym| *sym),
            bli_ssubsc: get_symbol(&libs, b"bli_ssubsc\0").map(|sym| *sym),
            bli_dsubsc: get_symbol(&libs, b"bli_dsubsc\0").map(|sym| *sym),
            bli_csubsc: get_symbol(&libs, b"bli_csubsc\0").map(|sym| *sym),
            bli_zsubsc: get_symbol(&libs, b"bli_zsubsc\0").map(|sym| *sym),
            bli_sinvertsc: get_symbol(&libs, b"bli_sinvertsc\0").map(|sym| *sym),
            bli_dinvertsc: get_symbol(&libs, b"bli_dinvertsc\0").map(|sym| *sym),
            bli_cinvertsc: get_symbol(&libs, b"bli_cinvertsc\0").map(|sym| *sym),
            bli_zinvertsc: get_symbol(&libs, b"bli_zinvertsc\0").map(|sym| *sym),
            bli_sabsqsc: get_symbol(&libs, b"bli_sabsqsc\0").map(|sym| *sym),
            bli_dabsqsc: get_symbol(&libs, b"bli_dabsqsc\0").map(|sym| *sym),
            bli_cabsqsc: get_symbol(&libs, b"bli_cabsqsc\0").map(|sym| *sym),
            bli_zabsqsc: get_symbol(&libs, b"bli_zabsqsc\0").map(|sym| *sym),
            bli_snormfsc: get_symbol(&libs, b"bli_snormfsc\0").map(|sym| *sym),
            bli_dnormfsc: get_symbol(&libs, b"bli_dnormfsc\0").map(|sym| *sym),
            bli_cnormfsc: get_symbol(&libs, b"bli_cnormfsc\0").map(|sym| *sym),
            bli_znormfsc: get_symbol(&libs, b"bli_znormfsc\0").map(|sym| *sym),
            bli_ssqrtsc: get_symbol(&libs, b"bli_ssqrtsc\0").map(|sym| *sym),
            bli_dsqrtsc: get_symbol(&libs, b"bli_dsqrtsc\0").map(|sym| *sym),
            bli_csqrtsc: get_symbol(&libs, b"bli_csqrtsc\0").map(|sym| *sym),
            bli_zsqrtsc: get_symbol(&libs, b"bli_zsqrtsc\0").map(|sym| *sym),
            bli_sgetsc: get_symbol(&libs, b"bli_sgetsc\0").map(|sym| *sym),
            bli_dgetsc: get_symbol(&libs, b"bli_dgetsc\0").map(|sym| *sym),
            bli_cgetsc: get_symbol(&libs, b"bli_cgetsc\0").map(|sym| *sym),
            bli_zgetsc: get_symbol(&libs, b"bli_zgetsc\0").map(|sym| *sym),
            bli_ssetsc: get_symbol(&libs, b"bli_ssetsc\0").map(|sym| *sym),
            bli_dsetsc: get_symbol(&libs, b"bli_dsetsc\0").map(|sym| *sym),
            bli_csetsc: get_symbol(&libs, b"bli_csetsc\0").map(|sym| *sym),
            bli_zsetsc: get_symbol(&libs, b"bli_zsetsc\0").map(|sym| *sym),
            bli_sunzipsc: get_symbol(&libs, b"bli_sunzipsc\0").map(|sym| *sym),
            bli_dunzipsc: get_symbol(&libs, b"bli_dunzipsc\0").map(|sym| *sym),
            bli_cunzipsc: get_symbol(&libs, b"bli_cunzipsc\0").map(|sym| *sym),
            bli_zunzipsc: get_symbol(&libs, b"bli_zunzipsc\0").map(|sym| *sym),
            bli_szipsc: get_symbol(&libs, b"bli_szipsc\0").map(|sym| *sym),
            bli_dzipsc: get_symbol(&libs, b"bli_dzipsc\0").map(|sym| *sym),
            bli_czipsc: get_symbol(&libs, b"bli_czipsc\0").map(|sym| *sym),
            bli_zzipsc: get_symbol(&libs, b"bli_zzipsc\0").map(|sym| *sym),
            bli_igetsc: get_symbol(&libs, b"bli_igetsc\0").map(|sym| *sym),
            bli_isetsc: get_symbol(&libs, b"bli_isetsc\0").map(|sym| *sym),
            bli_absqsc_qfp: get_symbol(&libs, b"bli_absqsc_qfp\0").map(|sym| *sym),
            bli_normfsc_qfp: get_symbol(&libs, b"bli_normfsc_qfp\0").map(|sym| *sym),
            bli_addsc_qfp: get_symbol(&libs, b"bli_addsc_qfp\0").map(|sym| *sym),
            bli_divsc_qfp: get_symbol(&libs, b"bli_divsc_qfp\0").map(|sym| *sym),
            bli_mulsc_qfp: get_symbol(&libs, b"bli_mulsc_qfp\0").map(|sym| *sym),
            bli_subsc_qfp: get_symbol(&libs, b"bli_subsc_qfp\0").map(|sym| *sym),
            bli_invertsc_qfp: get_symbol(&libs, b"bli_invertsc_qfp\0").map(|sym| *sym),
            bli_sqrtsc_qfp: get_symbol(&libs, b"bli_sqrtsc_qfp\0").map(|sym| *sym),
            bli_unzipsc_qfp: get_symbol(&libs, b"bli_unzipsc_qfp\0").map(|sym| *sym),
            bli_zipsc_qfp: get_symbol(&libs, b"bli_zipsc_qfp\0").map(|sym| *sym),
            bli_getsc_qfp: get_symbol(&libs, b"bli_getsc_qfp\0").map(|sym| *sym),
            bli_setsc_qfp: get_symbol(&libs, b"bli_setsc_qfp\0").map(|sym| *sym),
            bli_copysc: get_symbol(&libs, b"bli_copysc\0").map(|sym| *sym),
            bli_sscopysc: get_symbol(&libs, b"bli_sscopysc\0").map(|sym| *sym),
            bli_ddcopysc: get_symbol(&libs, b"bli_ddcopysc\0").map(|sym| *sym),
            bli_cccopysc: get_symbol(&libs, b"bli_cccopysc\0").map(|sym| *sym),
            bli_zzcopysc: get_symbol(&libs, b"bli_zzcopysc\0").map(|sym| *sym),
            bli_sccopysc: get_symbol(&libs, b"bli_sccopysc\0").map(|sym| *sym),
            bli_cscopysc: get_symbol(&libs, b"bli_cscopysc\0").map(|sym| *sym),
            bli_dzcopysc: get_symbol(&libs, b"bli_dzcopysc\0").map(|sym| *sym),
            bli_zdcopysc: get_symbol(&libs, b"bli_zdcopysc\0").map(|sym| *sym),
            bli_sdcopysc: get_symbol(&libs, b"bli_sdcopysc\0").map(|sym| *sym),
            bli_szcopysc: get_symbol(&libs, b"bli_szcopysc\0").map(|sym| *sym),
            bli_dscopysc: get_symbol(&libs, b"bli_dscopysc\0").map(|sym| *sym),
            bli_dccopysc: get_symbol(&libs, b"bli_dccopysc\0").map(|sym| *sym),
            bli_cdcopysc: get_symbol(&libs, b"bli_cdcopysc\0").map(|sym| *sym),
            bli_czcopysc: get_symbol(&libs, b"bli_czcopysc\0").map(|sym| *sym),
            bli_zscopysc: get_symbol(&libs, b"bli_zscopysc\0").map(|sym| *sym),
            bli_zccopysc: get_symbol(&libs, b"bli_zccopysc\0").map(|sym| *sym),
            bli_addv_check: get_symbol(&libs, b"bli_addv_check\0").map(|sym| *sym),
            bli_copyv_check: get_symbol(&libs, b"bli_copyv_check\0").map(|sym| *sym),
            bli_subv_check: get_symbol(&libs, b"bli_subv_check\0").map(|sym| *sym),
            bli_swapv_check: get_symbol(&libs, b"bli_swapv_check\0").map(|sym| *sym),
            bli_amaxv_check: get_symbol(&libs, b"bli_amaxv_check\0").map(|sym| *sym),
            bli_aminv_check: get_symbol(&libs, b"bli_aminv_check\0").map(|sym| *sym),
            bli_axpbyv_check: get_symbol(&libs, b"bli_axpbyv_check\0").map(|sym| *sym),
            bli_axpyv_check: get_symbol(&libs, b"bli_axpyv_check\0").map(|sym| *sym),
            bli_scal2v_check: get_symbol(&libs, b"bli_scal2v_check\0").map(|sym| *sym),
            bli_dotv_check: get_symbol(&libs, b"bli_dotv_check\0").map(|sym| *sym),
            bli_dotxv_check: get_symbol(&libs, b"bli_dotxv_check\0").map(|sym| *sym),
            bli_invertv_check: get_symbol(&libs, b"bli_invertv_check\0").map(|sym| *sym),
            bli_scalv_check: get_symbol(&libs, b"bli_scalv_check\0").map(|sym| *sym),
            bli_setv_check: get_symbol(&libs, b"bli_setv_check\0").map(|sym| *sym),
            bli_xpbyv_check: get_symbol(&libs, b"bli_xpbyv_check\0").map(|sym| *sym),
            bli_l1v_xy_check: get_symbol(&libs, b"bli_l1v_xy_check\0").map(|sym| *sym),
            bli_l1v_axy_check: get_symbol(&libs, b"bli_l1v_axy_check\0").map(|sym| *sym),
            bli_l1v_xby_check: get_symbol(&libs, b"bli_l1v_xby_check\0").map(|sym| *sym),
            bli_l1v_axby_check: get_symbol(&libs, b"bli_l1v_axby_check\0").map(|sym| *sym),
            bli_l1v_dot_check: get_symbol(&libs, b"bli_l1v_dot_check\0").map(|sym| *sym),
            bli_l1v_x_check: get_symbol(&libs, b"bli_l1v_x_check\0").map(|sym| *sym),
            bli_l1v_ax_check: get_symbol(&libs, b"bli_l1v_ax_check\0").map(|sym| *sym),
            bli_l1v_xi_check: get_symbol(&libs, b"bli_l1v_xi_check\0").map(|sym| *sym),
            bli_addv_ex: get_symbol(&libs, b"bli_addv_ex\0").map(|sym| *sym),
            bli_copyv_ex: get_symbol(&libs, b"bli_copyv_ex\0").map(|sym| *sym),
            bli_subv_ex: get_symbol(&libs, b"bli_subv_ex\0").map(|sym| *sym),
            bli_amaxv_ex: get_symbol(&libs, b"bli_amaxv_ex\0").map(|sym| *sym),
            bli_aminv_ex: get_symbol(&libs, b"bli_aminv_ex\0").map(|sym| *sym),
            bli_axpbyv_ex: get_symbol(&libs, b"bli_axpbyv_ex\0").map(|sym| *sym),
            bli_axpyv_ex: get_symbol(&libs, b"bli_axpyv_ex\0").map(|sym| *sym),
            bli_scal2v_ex: get_symbol(&libs, b"bli_scal2v_ex\0").map(|sym| *sym),
            bli_dotv_ex: get_symbol(&libs, b"bli_dotv_ex\0").map(|sym| *sym),
            bli_dotxv_ex: get_symbol(&libs, b"bli_dotxv_ex\0").map(|sym| *sym),
            bli_invertv_ex: get_symbol(&libs, b"bli_invertv_ex\0").map(|sym| *sym),
            bli_scalv_ex: get_symbol(&libs, b"bli_scalv_ex\0").map(|sym| *sym),
            bli_setv_ex: get_symbol(&libs, b"bli_setv_ex\0").map(|sym| *sym),
            bli_swapv_ex: get_symbol(&libs, b"bli_swapv_ex\0").map(|sym| *sym),
            bli_xpbyv_ex: get_symbol(&libs, b"bli_xpbyv_ex\0").map(|sym| *sym),
            bli_addv: get_symbol(&libs, b"bli_addv\0").map(|sym| *sym),
            bli_copyv: get_symbol(&libs, b"bli_copyv\0").map(|sym| *sym),
            bli_subv: get_symbol(&libs, b"bli_subv\0").map(|sym| *sym),
            bli_amaxv: get_symbol(&libs, b"bli_amaxv\0").map(|sym| *sym),
            bli_aminv: get_symbol(&libs, b"bli_aminv\0").map(|sym| *sym),
            bli_axpbyv: get_symbol(&libs, b"bli_axpbyv\0").map(|sym| *sym),
            bli_axpyv: get_symbol(&libs, b"bli_axpyv\0").map(|sym| *sym),
            bli_scal2v: get_symbol(&libs, b"bli_scal2v\0").map(|sym| *sym),
            bli_dotv: get_symbol(&libs, b"bli_dotv\0").map(|sym| *sym),
            bli_dotxv: get_symbol(&libs, b"bli_dotxv\0").map(|sym| *sym),
            bli_invertv: get_symbol(&libs, b"bli_invertv\0").map(|sym| *sym),
            bli_scalv: get_symbol(&libs, b"bli_scalv\0").map(|sym| *sym),
            bli_setv: get_symbol(&libs, b"bli_setv\0").map(|sym| *sym),
            bli_swapv: get_symbol(&libs, b"bli_swapv\0").map(|sym| *sym),
            bli_xpbyv: get_symbol(&libs, b"bli_xpbyv\0").map(|sym| *sym),
            bli_saddv_ex: get_symbol(&libs, b"bli_saddv_ex\0").map(|sym| *sym),
            bli_daddv_ex: get_symbol(&libs, b"bli_daddv_ex\0").map(|sym| *sym),
            bli_caddv_ex: get_symbol(&libs, b"bli_caddv_ex\0").map(|sym| *sym),
            bli_zaddv_ex: get_symbol(&libs, b"bli_zaddv_ex\0").map(|sym| *sym),
            bli_scopyv_ex: get_symbol(&libs, b"bli_scopyv_ex\0").map(|sym| *sym),
            bli_dcopyv_ex: get_symbol(&libs, b"bli_dcopyv_ex\0").map(|sym| *sym),
            bli_ccopyv_ex: get_symbol(&libs, b"bli_ccopyv_ex\0").map(|sym| *sym),
            bli_zcopyv_ex: get_symbol(&libs, b"bli_zcopyv_ex\0").map(|sym| *sym),
            bli_ssubv_ex: get_symbol(&libs, b"bli_ssubv_ex\0").map(|sym| *sym),
            bli_dsubv_ex: get_symbol(&libs, b"bli_dsubv_ex\0").map(|sym| *sym),
            bli_csubv_ex: get_symbol(&libs, b"bli_csubv_ex\0").map(|sym| *sym),
            bli_zsubv_ex: get_symbol(&libs, b"bli_zsubv_ex\0").map(|sym| *sym),
            bli_samaxv_ex: get_symbol(&libs, b"bli_samaxv_ex\0").map(|sym| *sym),
            bli_damaxv_ex: get_symbol(&libs, b"bli_damaxv_ex\0").map(|sym| *sym),
            bli_camaxv_ex: get_symbol(&libs, b"bli_camaxv_ex\0").map(|sym| *sym),
            bli_zamaxv_ex: get_symbol(&libs, b"bli_zamaxv_ex\0").map(|sym| *sym),
            bli_saminv_ex: get_symbol(&libs, b"bli_saminv_ex\0").map(|sym| *sym),
            bli_daminv_ex: get_symbol(&libs, b"bli_daminv_ex\0").map(|sym| *sym),
            bli_caminv_ex: get_symbol(&libs, b"bli_caminv_ex\0").map(|sym| *sym),
            bli_zaminv_ex: get_symbol(&libs, b"bli_zaminv_ex\0").map(|sym| *sym),
            bli_saxpbyv_ex: get_symbol(&libs, b"bli_saxpbyv_ex\0").map(|sym| *sym),
            bli_daxpbyv_ex: get_symbol(&libs, b"bli_daxpbyv_ex\0").map(|sym| *sym),
            bli_caxpbyv_ex: get_symbol(&libs, b"bli_caxpbyv_ex\0").map(|sym| *sym),
            bli_zaxpbyv_ex: get_symbol(&libs, b"bli_zaxpbyv_ex\0").map(|sym| *sym),
            bli_saxpyv_ex: get_symbol(&libs, b"bli_saxpyv_ex\0").map(|sym| *sym),
            bli_daxpyv_ex: get_symbol(&libs, b"bli_daxpyv_ex\0").map(|sym| *sym),
            bli_caxpyv_ex: get_symbol(&libs, b"bli_caxpyv_ex\0").map(|sym| *sym),
            bli_zaxpyv_ex: get_symbol(&libs, b"bli_zaxpyv_ex\0").map(|sym| *sym),
            bli_sscal2v_ex: get_symbol(&libs, b"bli_sscal2v_ex\0").map(|sym| *sym),
            bli_dscal2v_ex: get_symbol(&libs, b"bli_dscal2v_ex\0").map(|sym| *sym),
            bli_cscal2v_ex: get_symbol(&libs, b"bli_cscal2v_ex\0").map(|sym| *sym),
            bli_zscal2v_ex: get_symbol(&libs, b"bli_zscal2v_ex\0").map(|sym| *sym),
            bli_sdotv_ex: get_symbol(&libs, b"bli_sdotv_ex\0").map(|sym| *sym),
            bli_ddotv_ex: get_symbol(&libs, b"bli_ddotv_ex\0").map(|sym| *sym),
            bli_cdotv_ex: get_symbol(&libs, b"bli_cdotv_ex\0").map(|sym| *sym),
            bli_zdotv_ex: get_symbol(&libs, b"bli_zdotv_ex\0").map(|sym| *sym),
            bli_sdotxv_ex: get_symbol(&libs, b"bli_sdotxv_ex\0").map(|sym| *sym),
            bli_ddotxv_ex: get_symbol(&libs, b"bli_ddotxv_ex\0").map(|sym| *sym),
            bli_cdotxv_ex: get_symbol(&libs, b"bli_cdotxv_ex\0").map(|sym| *sym),
            bli_zdotxv_ex: get_symbol(&libs, b"bli_zdotxv_ex\0").map(|sym| *sym),
            bli_sinvertv_ex: get_symbol(&libs, b"bli_sinvertv_ex\0").map(|sym| *sym),
            bli_dinvertv_ex: get_symbol(&libs, b"bli_dinvertv_ex\0").map(|sym| *sym),
            bli_cinvertv_ex: get_symbol(&libs, b"bli_cinvertv_ex\0").map(|sym| *sym),
            bli_zinvertv_ex: get_symbol(&libs, b"bli_zinvertv_ex\0").map(|sym| *sym),
            bli_sscalv_ex: get_symbol(&libs, b"bli_sscalv_ex\0").map(|sym| *sym),
            bli_dscalv_ex: get_symbol(&libs, b"bli_dscalv_ex\0").map(|sym| *sym),
            bli_cscalv_ex: get_symbol(&libs, b"bli_cscalv_ex\0").map(|sym| *sym),
            bli_zscalv_ex: get_symbol(&libs, b"bli_zscalv_ex\0").map(|sym| *sym),
            bli_ssetv_ex: get_symbol(&libs, b"bli_ssetv_ex\0").map(|sym| *sym),
            bli_dsetv_ex: get_symbol(&libs, b"bli_dsetv_ex\0").map(|sym| *sym),
            bli_csetv_ex: get_symbol(&libs, b"bli_csetv_ex\0").map(|sym| *sym),
            bli_zsetv_ex: get_symbol(&libs, b"bli_zsetv_ex\0").map(|sym| *sym),
            bli_sswapv_ex: get_symbol(&libs, b"bli_sswapv_ex\0").map(|sym| *sym),
            bli_dswapv_ex: get_symbol(&libs, b"bli_dswapv_ex\0").map(|sym| *sym),
            bli_cswapv_ex: get_symbol(&libs, b"bli_cswapv_ex\0").map(|sym| *sym),
            bli_zswapv_ex: get_symbol(&libs, b"bli_zswapv_ex\0").map(|sym| *sym),
            bli_sxpbyv_ex: get_symbol(&libs, b"bli_sxpbyv_ex\0").map(|sym| *sym),
            bli_dxpbyv_ex: get_symbol(&libs, b"bli_dxpbyv_ex\0").map(|sym| *sym),
            bli_cxpbyv_ex: get_symbol(&libs, b"bli_cxpbyv_ex\0").map(|sym| *sym),
            bli_zxpbyv_ex: get_symbol(&libs, b"bli_zxpbyv_ex\0").map(|sym| *sym),
            bli_saddv: get_symbol(&libs, b"bli_saddv\0").map(|sym| *sym),
            bli_daddv: get_symbol(&libs, b"bli_daddv\0").map(|sym| *sym),
            bli_caddv: get_symbol(&libs, b"bli_caddv\0").map(|sym| *sym),
            bli_zaddv: get_symbol(&libs, b"bli_zaddv\0").map(|sym| *sym),
            bli_scopyv: get_symbol(&libs, b"bli_scopyv\0").map(|sym| *sym),
            bli_dcopyv: get_symbol(&libs, b"bli_dcopyv\0").map(|sym| *sym),
            bli_ccopyv: get_symbol(&libs, b"bli_ccopyv\0").map(|sym| *sym),
            bli_zcopyv: get_symbol(&libs, b"bli_zcopyv\0").map(|sym| *sym),
            bli_ssubv: get_symbol(&libs, b"bli_ssubv\0").map(|sym| *sym),
            bli_dsubv: get_symbol(&libs, b"bli_dsubv\0").map(|sym| *sym),
            bli_csubv: get_symbol(&libs, b"bli_csubv\0").map(|sym| *sym),
            bli_zsubv: get_symbol(&libs, b"bli_zsubv\0").map(|sym| *sym),
            bli_samaxv: get_symbol(&libs, b"bli_samaxv\0").map(|sym| *sym),
            bli_damaxv: get_symbol(&libs, b"bli_damaxv\0").map(|sym| *sym),
            bli_camaxv: get_symbol(&libs, b"bli_camaxv\0").map(|sym| *sym),
            bli_zamaxv: get_symbol(&libs, b"bli_zamaxv\0").map(|sym| *sym),
            bli_saminv: get_symbol(&libs, b"bli_saminv\0").map(|sym| *sym),
            bli_daminv: get_symbol(&libs, b"bli_daminv\0").map(|sym| *sym),
            bli_caminv: get_symbol(&libs, b"bli_caminv\0").map(|sym| *sym),
            bli_zaminv: get_symbol(&libs, b"bli_zaminv\0").map(|sym| *sym),
            bli_saxpbyv: get_symbol(&libs, b"bli_saxpbyv\0").map(|sym| *sym),
            bli_daxpbyv: get_symbol(&libs, b"bli_daxpbyv\0").map(|sym| *sym),
            bli_caxpbyv: get_symbol(&libs, b"bli_caxpbyv\0").map(|sym| *sym),
            bli_zaxpbyv: get_symbol(&libs, b"bli_zaxpbyv\0").map(|sym| *sym),
            bli_saxpyv: get_symbol(&libs, b"bli_saxpyv\0").map(|sym| *sym),
            bli_daxpyv: get_symbol(&libs, b"bli_daxpyv\0").map(|sym| *sym),
            bli_caxpyv: get_symbol(&libs, b"bli_caxpyv\0").map(|sym| *sym),
            bli_zaxpyv: get_symbol(&libs, b"bli_zaxpyv\0").map(|sym| *sym),
            bli_sscal2v: get_symbol(&libs, b"bli_sscal2v\0").map(|sym| *sym),
            bli_dscal2v: get_symbol(&libs, b"bli_dscal2v\0").map(|sym| *sym),
            bli_cscal2v: get_symbol(&libs, b"bli_cscal2v\0").map(|sym| *sym),
            bli_zscal2v: get_symbol(&libs, b"bli_zscal2v\0").map(|sym| *sym),
            bli_sdotv: get_symbol(&libs, b"bli_sdotv\0").map(|sym| *sym),
            bli_ddotv: get_symbol(&libs, b"bli_ddotv\0").map(|sym| *sym),
            bli_cdotv: get_symbol(&libs, b"bli_cdotv\0").map(|sym| *sym),
            bli_zdotv: get_symbol(&libs, b"bli_zdotv\0").map(|sym| *sym),
            bli_sdotxv: get_symbol(&libs, b"bli_sdotxv\0").map(|sym| *sym),
            bli_ddotxv: get_symbol(&libs, b"bli_ddotxv\0").map(|sym| *sym),
            bli_cdotxv: get_symbol(&libs, b"bli_cdotxv\0").map(|sym| *sym),
            bli_zdotxv: get_symbol(&libs, b"bli_zdotxv\0").map(|sym| *sym),
            bli_sinvertv: get_symbol(&libs, b"bli_sinvertv\0").map(|sym| *sym),
            bli_dinvertv: get_symbol(&libs, b"bli_dinvertv\0").map(|sym| *sym),
            bli_cinvertv: get_symbol(&libs, b"bli_cinvertv\0").map(|sym| *sym),
            bli_zinvertv: get_symbol(&libs, b"bli_zinvertv\0").map(|sym| *sym),
            bli_sscalv: get_symbol(&libs, b"bli_sscalv\0").map(|sym| *sym),
            bli_dscalv: get_symbol(&libs, b"bli_dscalv\0").map(|sym| *sym),
            bli_cscalv: get_symbol(&libs, b"bli_cscalv\0").map(|sym| *sym),
            bli_zscalv: get_symbol(&libs, b"bli_zscalv\0").map(|sym| *sym),
            bli_ssetv: get_symbol(&libs, b"bli_ssetv\0").map(|sym| *sym),
            bli_dsetv: get_symbol(&libs, b"bli_dsetv\0").map(|sym| *sym),
            bli_csetv: get_symbol(&libs, b"bli_csetv\0").map(|sym| *sym),
            bli_zsetv: get_symbol(&libs, b"bli_zsetv\0").map(|sym| *sym),
            bli_sswapv: get_symbol(&libs, b"bli_sswapv\0").map(|sym| *sym),
            bli_dswapv: get_symbol(&libs, b"bli_dswapv\0").map(|sym| *sym),
            bli_cswapv: get_symbol(&libs, b"bli_cswapv\0").map(|sym| *sym),
            bli_zswapv: get_symbol(&libs, b"bli_zswapv\0").map(|sym| *sym),
            bli_sxpbyv: get_symbol(&libs, b"bli_sxpbyv\0").map(|sym| *sym),
            bli_dxpbyv: get_symbol(&libs, b"bli_dxpbyv\0").map(|sym| *sym),
            bli_cxpbyv: get_symbol(&libs, b"bli_cxpbyv\0").map(|sym| *sym),
            bli_zxpbyv: get_symbol(&libs, b"bli_zxpbyv\0").map(|sym| *sym),
            bli_addv_ex_qfp: get_symbol(&libs, b"bli_addv_ex_qfp\0").map(|sym| *sym),
            bli_copyv_ex_qfp: get_symbol(&libs, b"bli_copyv_ex_qfp\0").map(|sym| *sym),
            bli_subv_ex_qfp: get_symbol(&libs, b"bli_subv_ex_qfp\0").map(|sym| *sym),
            bli_amaxv_ex_qfp: get_symbol(&libs, b"bli_amaxv_ex_qfp\0").map(|sym| *sym),
            bli_aminv_ex_qfp: get_symbol(&libs, b"bli_aminv_ex_qfp\0").map(|sym| *sym),
            bli_axpbyv_ex_qfp: get_symbol(&libs, b"bli_axpbyv_ex_qfp\0").map(|sym| *sym),
            bli_axpyv_ex_qfp: get_symbol(&libs, b"bli_axpyv_ex_qfp\0").map(|sym| *sym),
            bli_scal2v_ex_qfp: get_symbol(&libs, b"bli_scal2v_ex_qfp\0").map(|sym| *sym),
            bli_dotv_ex_qfp: get_symbol(&libs, b"bli_dotv_ex_qfp\0").map(|sym| *sym),
            bli_dotxv_ex_qfp: get_symbol(&libs, b"bli_dotxv_ex_qfp\0").map(|sym| *sym),
            bli_invertv_ex_qfp: get_symbol(&libs, b"bli_invertv_ex_qfp\0").map(|sym| *sym),
            bli_scalv_ex_qfp: get_symbol(&libs, b"bli_scalv_ex_qfp\0").map(|sym| *sym),
            bli_setv_ex_qfp: get_symbol(&libs, b"bli_setv_ex_qfp\0").map(|sym| *sym),
            bli_swapv_ex_qfp: get_symbol(&libs, b"bli_swapv_ex_qfp\0").map(|sym| *sym),
            bli_xpbyv_ex_qfp: get_symbol(&libs, b"bli_xpbyv_ex_qfp\0").map(|sym| *sym),
            bli_addd_check: get_symbol(&libs, b"bli_addd_check\0").map(|sym| *sym),
            bli_copyd_check: get_symbol(&libs, b"bli_copyd_check\0").map(|sym| *sym),
            bli_subd_check: get_symbol(&libs, b"bli_subd_check\0").map(|sym| *sym),
            bli_axpyd_check: get_symbol(&libs, b"bli_axpyd_check\0").map(|sym| *sym),
            bli_scal2d_check: get_symbol(&libs, b"bli_scal2d_check\0").map(|sym| *sym),
            bli_invertd_check: get_symbol(&libs, b"bli_invertd_check\0").map(|sym| *sym),
            bli_scald_check: get_symbol(&libs, b"bli_scald_check\0").map(|sym| *sym),
            bli_setd_check: get_symbol(&libs, b"bli_setd_check\0").map(|sym| *sym),
            bli_setid_check: get_symbol(&libs, b"bli_setid_check\0").map(|sym| *sym),
            bli_shiftd_check: get_symbol(&libs, b"bli_shiftd_check\0").map(|sym| *sym),
            bli_xpbyd_check: get_symbol(&libs, b"bli_xpbyd_check\0").map(|sym| *sym),
            bli_l1d_xy_check: get_symbol(&libs, b"bli_l1d_xy_check\0").map(|sym| *sym),
            bli_l1d_axy_check: get_symbol(&libs, b"bli_l1d_axy_check\0").map(|sym| *sym),
            bli_l1d_x_check: get_symbol(&libs, b"bli_l1d_x_check\0").map(|sym| *sym),
            bli_l1d_ax_check: get_symbol(&libs, b"bli_l1d_ax_check\0").map(|sym| *sym),
            bli_addd_ex: get_symbol(&libs, b"bli_addd_ex\0").map(|sym| *sym),
            bli_copyd_ex: get_symbol(&libs, b"bli_copyd_ex\0").map(|sym| *sym),
            bli_subd_ex: get_symbol(&libs, b"bli_subd_ex\0").map(|sym| *sym),
            bli_axpyd_ex: get_symbol(&libs, b"bli_axpyd_ex\0").map(|sym| *sym),
            bli_scal2d_ex: get_symbol(&libs, b"bli_scal2d_ex\0").map(|sym| *sym),
            bli_invertd_ex: get_symbol(&libs, b"bli_invertd_ex\0").map(|sym| *sym),
            bli_scald_ex: get_symbol(&libs, b"bli_scald_ex\0").map(|sym| *sym),
            bli_setd_ex: get_symbol(&libs, b"bli_setd_ex\0").map(|sym| *sym),
            bli_setid_ex: get_symbol(&libs, b"bli_setid_ex\0").map(|sym| *sym),
            bli_shiftd_ex: get_symbol(&libs, b"bli_shiftd_ex\0").map(|sym| *sym),
            bli_xpbyd_ex: get_symbol(&libs, b"bli_xpbyd_ex\0").map(|sym| *sym),
            bli_addd: get_symbol(&libs, b"bli_addd\0").map(|sym| *sym),
            bli_copyd: get_symbol(&libs, b"bli_copyd\0").map(|sym| *sym),
            bli_subd: get_symbol(&libs, b"bli_subd\0").map(|sym| *sym),
            bli_axpyd: get_symbol(&libs, b"bli_axpyd\0").map(|sym| *sym),
            bli_scal2d: get_symbol(&libs, b"bli_scal2d\0").map(|sym| *sym),
            bli_invertd: get_symbol(&libs, b"bli_invertd\0").map(|sym| *sym),
            bli_scald: get_symbol(&libs, b"bli_scald\0").map(|sym| *sym),
            bli_setd: get_symbol(&libs, b"bli_setd\0").map(|sym| *sym),
            bli_setid: get_symbol(&libs, b"bli_setid\0").map(|sym| *sym),
            bli_shiftd: get_symbol(&libs, b"bli_shiftd\0").map(|sym| *sym),
            bli_xpbyd: get_symbol(&libs, b"bli_xpbyd\0").map(|sym| *sym),
            bli_saddd_ex: get_symbol(&libs, b"bli_saddd_ex\0").map(|sym| *sym),
            bli_daddd_ex: get_symbol(&libs, b"bli_daddd_ex\0").map(|sym| *sym),
            bli_caddd_ex: get_symbol(&libs, b"bli_caddd_ex\0").map(|sym| *sym),
            bli_zaddd_ex: get_symbol(&libs, b"bli_zaddd_ex\0").map(|sym| *sym),
            bli_scopyd_ex: get_symbol(&libs, b"bli_scopyd_ex\0").map(|sym| *sym),
            bli_dcopyd_ex: get_symbol(&libs, b"bli_dcopyd_ex\0").map(|sym| *sym),
            bli_ccopyd_ex: get_symbol(&libs, b"bli_ccopyd_ex\0").map(|sym| *sym),
            bli_zcopyd_ex: get_symbol(&libs, b"bli_zcopyd_ex\0").map(|sym| *sym),
            bli_ssubd_ex: get_symbol(&libs, b"bli_ssubd_ex\0").map(|sym| *sym),
            bli_dsubd_ex: get_symbol(&libs, b"bli_dsubd_ex\0").map(|sym| *sym),
            bli_csubd_ex: get_symbol(&libs, b"bli_csubd_ex\0").map(|sym| *sym),
            bli_zsubd_ex: get_symbol(&libs, b"bli_zsubd_ex\0").map(|sym| *sym),
            bli_saxpyd_ex: get_symbol(&libs, b"bli_saxpyd_ex\0").map(|sym| *sym),
            bli_daxpyd_ex: get_symbol(&libs, b"bli_daxpyd_ex\0").map(|sym| *sym),
            bli_caxpyd_ex: get_symbol(&libs, b"bli_caxpyd_ex\0").map(|sym| *sym),
            bli_zaxpyd_ex: get_symbol(&libs, b"bli_zaxpyd_ex\0").map(|sym| *sym),
            bli_sscal2d_ex: get_symbol(&libs, b"bli_sscal2d_ex\0").map(|sym| *sym),
            bli_dscal2d_ex: get_symbol(&libs, b"bli_dscal2d_ex\0").map(|sym| *sym),
            bli_cscal2d_ex: get_symbol(&libs, b"bli_cscal2d_ex\0").map(|sym| *sym),
            bli_zscal2d_ex: get_symbol(&libs, b"bli_zscal2d_ex\0").map(|sym| *sym),
            bli_sinvertd_ex: get_symbol(&libs, b"bli_sinvertd_ex\0").map(|sym| *sym),
            bli_dinvertd_ex: get_symbol(&libs, b"bli_dinvertd_ex\0").map(|sym| *sym),
            bli_cinvertd_ex: get_symbol(&libs, b"bli_cinvertd_ex\0").map(|sym| *sym),
            bli_zinvertd_ex: get_symbol(&libs, b"bli_zinvertd_ex\0").map(|sym| *sym),
            bli_sscald_ex: get_symbol(&libs, b"bli_sscald_ex\0").map(|sym| *sym),
            bli_dscald_ex: get_symbol(&libs, b"bli_dscald_ex\0").map(|sym| *sym),
            bli_cscald_ex: get_symbol(&libs, b"bli_cscald_ex\0").map(|sym| *sym),
            bli_zscald_ex: get_symbol(&libs, b"bli_zscald_ex\0").map(|sym| *sym),
            bli_ssetd_ex: get_symbol(&libs, b"bli_ssetd_ex\0").map(|sym| *sym),
            bli_dsetd_ex: get_symbol(&libs, b"bli_dsetd_ex\0").map(|sym| *sym),
            bli_csetd_ex: get_symbol(&libs, b"bli_csetd_ex\0").map(|sym| *sym),
            bli_zsetd_ex: get_symbol(&libs, b"bli_zsetd_ex\0").map(|sym| *sym),
            bli_ssetid_ex: get_symbol(&libs, b"bli_ssetid_ex\0").map(|sym| *sym),
            bli_dsetid_ex: get_symbol(&libs, b"bli_dsetid_ex\0").map(|sym| *sym),
            bli_csetid_ex: get_symbol(&libs, b"bli_csetid_ex\0").map(|sym| *sym),
            bli_zsetid_ex: get_symbol(&libs, b"bli_zsetid_ex\0").map(|sym| *sym),
            bli_sshiftd_ex: get_symbol(&libs, b"bli_sshiftd_ex\0").map(|sym| *sym),
            bli_dshiftd_ex: get_symbol(&libs, b"bli_dshiftd_ex\0").map(|sym| *sym),
            bli_cshiftd_ex: get_symbol(&libs, b"bli_cshiftd_ex\0").map(|sym| *sym),
            bli_zshiftd_ex: get_symbol(&libs, b"bli_zshiftd_ex\0").map(|sym| *sym),
            bli_sxpbyd_ex: get_symbol(&libs, b"bli_sxpbyd_ex\0").map(|sym| *sym),
            bli_dxpbyd_ex: get_symbol(&libs, b"bli_dxpbyd_ex\0").map(|sym| *sym),
            bli_cxpbyd_ex: get_symbol(&libs, b"bli_cxpbyd_ex\0").map(|sym| *sym),
            bli_zxpbyd_ex: get_symbol(&libs, b"bli_zxpbyd_ex\0").map(|sym| *sym),
            bli_saddd: get_symbol(&libs, b"bli_saddd\0").map(|sym| *sym),
            bli_daddd: get_symbol(&libs, b"bli_daddd\0").map(|sym| *sym),
            bli_caddd: get_symbol(&libs, b"bli_caddd\0").map(|sym| *sym),
            bli_zaddd: get_symbol(&libs, b"bli_zaddd\0").map(|sym| *sym),
            bli_scopyd: get_symbol(&libs, b"bli_scopyd\0").map(|sym| *sym),
            bli_dcopyd: get_symbol(&libs, b"bli_dcopyd\0").map(|sym| *sym),
            bli_ccopyd: get_symbol(&libs, b"bli_ccopyd\0").map(|sym| *sym),
            bli_zcopyd: get_symbol(&libs, b"bli_zcopyd\0").map(|sym| *sym),
            bli_ssubd: get_symbol(&libs, b"bli_ssubd\0").map(|sym| *sym),
            bli_dsubd: get_symbol(&libs, b"bli_dsubd\0").map(|sym| *sym),
            bli_csubd: get_symbol(&libs, b"bli_csubd\0").map(|sym| *sym),
            bli_zsubd: get_symbol(&libs, b"bli_zsubd\0").map(|sym| *sym),
            bli_saxpyd: get_symbol(&libs, b"bli_saxpyd\0").map(|sym| *sym),
            bli_daxpyd: get_symbol(&libs, b"bli_daxpyd\0").map(|sym| *sym),
            bli_caxpyd: get_symbol(&libs, b"bli_caxpyd\0").map(|sym| *sym),
            bli_zaxpyd: get_symbol(&libs, b"bli_zaxpyd\0").map(|sym| *sym),
            bli_sscal2d: get_symbol(&libs, b"bli_sscal2d\0").map(|sym| *sym),
            bli_dscal2d: get_symbol(&libs, b"bli_dscal2d\0").map(|sym| *sym),
            bli_cscal2d: get_symbol(&libs, b"bli_cscal2d\0").map(|sym| *sym),
            bli_zscal2d: get_symbol(&libs, b"bli_zscal2d\0").map(|sym| *sym),
            bli_sinvertd: get_symbol(&libs, b"bli_sinvertd\0").map(|sym| *sym),
            bli_dinvertd: get_symbol(&libs, b"bli_dinvertd\0").map(|sym| *sym),
            bli_cinvertd: get_symbol(&libs, b"bli_cinvertd\0").map(|sym| *sym),
            bli_zinvertd: get_symbol(&libs, b"bli_zinvertd\0").map(|sym| *sym),
            bli_sscald: get_symbol(&libs, b"bli_sscald\0").map(|sym| *sym),
            bli_dscald: get_symbol(&libs, b"bli_dscald\0").map(|sym| *sym),
            bli_cscald: get_symbol(&libs, b"bli_cscald\0").map(|sym| *sym),
            bli_zscald: get_symbol(&libs, b"bli_zscald\0").map(|sym| *sym),
            bli_ssetd: get_symbol(&libs, b"bli_ssetd\0").map(|sym| *sym),
            bli_dsetd: get_symbol(&libs, b"bli_dsetd\0").map(|sym| *sym),
            bli_csetd: get_symbol(&libs, b"bli_csetd\0").map(|sym| *sym),
            bli_zsetd: get_symbol(&libs, b"bli_zsetd\0").map(|sym| *sym),
            bli_ssetid: get_symbol(&libs, b"bli_ssetid\0").map(|sym| *sym),
            bli_dsetid: get_symbol(&libs, b"bli_dsetid\0").map(|sym| *sym),
            bli_csetid: get_symbol(&libs, b"bli_csetid\0").map(|sym| *sym),
            bli_zsetid: get_symbol(&libs, b"bli_zsetid\0").map(|sym| *sym),
            bli_sshiftd: get_symbol(&libs, b"bli_sshiftd\0").map(|sym| *sym),
            bli_dshiftd: get_symbol(&libs, b"bli_dshiftd\0").map(|sym| *sym),
            bli_cshiftd: get_symbol(&libs, b"bli_cshiftd\0").map(|sym| *sym),
            bli_zshiftd: get_symbol(&libs, b"bli_zshiftd\0").map(|sym| *sym),
            bli_sxpbyd: get_symbol(&libs, b"bli_sxpbyd\0").map(|sym| *sym),
            bli_dxpbyd: get_symbol(&libs, b"bli_dxpbyd\0").map(|sym| *sym),
            bli_cxpbyd: get_symbol(&libs, b"bli_cxpbyd\0").map(|sym| *sym),
            bli_zxpbyd: get_symbol(&libs, b"bli_zxpbyd\0").map(|sym| *sym),
            bli_addd_ex_qfp: get_symbol(&libs, b"bli_addd_ex_qfp\0").map(|sym| *sym),
            bli_copyd_ex_qfp: get_symbol(&libs, b"bli_copyd_ex_qfp\0").map(|sym| *sym),
            bli_subd_ex_qfp: get_symbol(&libs, b"bli_subd_ex_qfp\0").map(|sym| *sym),
            bli_axpyd_ex_qfp: get_symbol(&libs, b"bli_axpyd_ex_qfp\0").map(|sym| *sym),
            bli_scal2d_ex_qfp: get_symbol(&libs, b"bli_scal2d_ex_qfp\0").map(|sym| *sym),
            bli_invertd_ex_qfp: get_symbol(&libs, b"bli_invertd_ex_qfp\0").map(|sym| *sym),
            bli_scald_ex_qfp: get_symbol(&libs, b"bli_scald_ex_qfp\0").map(|sym| *sym),
            bli_setd_ex_qfp: get_symbol(&libs, b"bli_setd_ex_qfp\0").map(|sym| *sym),
            bli_setid_ex_qfp: get_symbol(&libs, b"bli_setid_ex_qfp\0").map(|sym| *sym),
            bli_shiftd_ex_qfp: get_symbol(&libs, b"bli_shiftd_ex_qfp\0").map(|sym| *sym),
            bli_xpbyd_ex_qfp: get_symbol(&libs, b"bli_xpbyd_ex_qfp\0").map(|sym| *sym),
            bli_axpy2v_check: get_symbol(&libs, b"bli_axpy2v_check\0").map(|sym| *sym),
            bli_axpyf_check: get_symbol(&libs, b"bli_axpyf_check\0").map(|sym| *sym),
            bli_dotaxpyv_check: get_symbol(&libs, b"bli_dotaxpyv_check\0").map(|sym| *sym),
            bli_dotxaxpyf_check: get_symbol(&libs, b"bli_dotxaxpyf_check\0").map(|sym| *sym),
            bli_dotxf_check: get_symbol(&libs, b"bli_dotxf_check\0").map(|sym| *sym),
            bli_axpy2v_ex: get_symbol(&libs, b"bli_axpy2v_ex\0").map(|sym| *sym),
            bli_axpyf_ex: get_symbol(&libs, b"bli_axpyf_ex\0").map(|sym| *sym),
            bli_dotaxpyv_ex: get_symbol(&libs, b"bli_dotaxpyv_ex\0").map(|sym| *sym),
            bli_dotxaxpyf_ex: get_symbol(&libs, b"bli_dotxaxpyf_ex\0").map(|sym| *sym),
            bli_dotxf_ex: get_symbol(&libs, b"bli_dotxf_ex\0").map(|sym| *sym),
            bli_axpy2v: get_symbol(&libs, b"bli_axpy2v\0").map(|sym| *sym),
            bli_axpyf: get_symbol(&libs, b"bli_axpyf\0").map(|sym| *sym),
            bli_dotaxpyv: get_symbol(&libs, b"bli_dotaxpyv\0").map(|sym| *sym),
            bli_dotxaxpyf: get_symbol(&libs, b"bli_dotxaxpyf\0").map(|sym| *sym),
            bli_dotxf: get_symbol(&libs, b"bli_dotxf\0").map(|sym| *sym),
            bli_saxpy2v_ex: get_symbol(&libs, b"bli_saxpy2v_ex\0").map(|sym| *sym),
            bli_daxpy2v_ex: get_symbol(&libs, b"bli_daxpy2v_ex\0").map(|sym| *sym),
            bli_caxpy2v_ex: get_symbol(&libs, b"bli_caxpy2v_ex\0").map(|sym| *sym),
            bli_zaxpy2v_ex: get_symbol(&libs, b"bli_zaxpy2v_ex\0").map(|sym| *sym),
            bli_saxpyf_ex: get_symbol(&libs, b"bli_saxpyf_ex\0").map(|sym| *sym),
            bli_daxpyf_ex: get_symbol(&libs, b"bli_daxpyf_ex\0").map(|sym| *sym),
            bli_caxpyf_ex: get_symbol(&libs, b"bli_caxpyf_ex\0").map(|sym| *sym),
            bli_zaxpyf_ex: get_symbol(&libs, b"bli_zaxpyf_ex\0").map(|sym| *sym),
            bli_sdotaxpyv_ex: get_symbol(&libs, b"bli_sdotaxpyv_ex\0").map(|sym| *sym),
            bli_ddotaxpyv_ex: get_symbol(&libs, b"bli_ddotaxpyv_ex\0").map(|sym| *sym),
            bli_cdotaxpyv_ex: get_symbol(&libs, b"bli_cdotaxpyv_ex\0").map(|sym| *sym),
            bli_zdotaxpyv_ex: get_symbol(&libs, b"bli_zdotaxpyv_ex\0").map(|sym| *sym),
            bli_sdotxaxpyf_ex: get_symbol(&libs, b"bli_sdotxaxpyf_ex\0").map(|sym| *sym),
            bli_ddotxaxpyf_ex: get_symbol(&libs, b"bli_ddotxaxpyf_ex\0").map(|sym| *sym),
            bli_cdotxaxpyf_ex: get_symbol(&libs, b"bli_cdotxaxpyf_ex\0").map(|sym| *sym),
            bli_zdotxaxpyf_ex: get_symbol(&libs, b"bli_zdotxaxpyf_ex\0").map(|sym| *sym),
            bli_sdotxf_ex: get_symbol(&libs, b"bli_sdotxf_ex\0").map(|sym| *sym),
            bli_ddotxf_ex: get_symbol(&libs, b"bli_ddotxf_ex\0").map(|sym| *sym),
            bli_cdotxf_ex: get_symbol(&libs, b"bli_cdotxf_ex\0").map(|sym| *sym),
            bli_zdotxf_ex: get_symbol(&libs, b"bli_zdotxf_ex\0").map(|sym| *sym),
            bli_saxpy2v: get_symbol(&libs, b"bli_saxpy2v\0").map(|sym| *sym),
            bli_daxpy2v: get_symbol(&libs, b"bli_daxpy2v\0").map(|sym| *sym),
            bli_caxpy2v: get_symbol(&libs, b"bli_caxpy2v\0").map(|sym| *sym),
            bli_zaxpy2v: get_symbol(&libs, b"bli_zaxpy2v\0").map(|sym| *sym),
            bli_saxpyf: get_symbol(&libs, b"bli_saxpyf\0").map(|sym| *sym),
            bli_daxpyf: get_symbol(&libs, b"bli_daxpyf\0").map(|sym| *sym),
            bli_caxpyf: get_symbol(&libs, b"bli_caxpyf\0").map(|sym| *sym),
            bli_zaxpyf: get_symbol(&libs, b"bli_zaxpyf\0").map(|sym| *sym),
            bli_sdotaxpyv: get_symbol(&libs, b"bli_sdotaxpyv\0").map(|sym| *sym),
            bli_ddotaxpyv: get_symbol(&libs, b"bli_ddotaxpyv\0").map(|sym| *sym),
            bli_cdotaxpyv: get_symbol(&libs, b"bli_cdotaxpyv\0").map(|sym| *sym),
            bli_zdotaxpyv: get_symbol(&libs, b"bli_zdotaxpyv\0").map(|sym| *sym),
            bli_sdotxaxpyf: get_symbol(&libs, b"bli_sdotxaxpyf\0").map(|sym| *sym),
            bli_ddotxaxpyf: get_symbol(&libs, b"bli_ddotxaxpyf\0").map(|sym| *sym),
            bli_cdotxaxpyf: get_symbol(&libs, b"bli_cdotxaxpyf\0").map(|sym| *sym),
            bli_zdotxaxpyf: get_symbol(&libs, b"bli_zdotxaxpyf\0").map(|sym| *sym),
            bli_sdotxf: get_symbol(&libs, b"bli_sdotxf\0").map(|sym| *sym),
            bli_ddotxf: get_symbol(&libs, b"bli_ddotxf\0").map(|sym| *sym),
            bli_cdotxf: get_symbol(&libs, b"bli_cdotxf\0").map(|sym| *sym),
            bli_zdotxf: get_symbol(&libs, b"bli_zdotxf\0").map(|sym| *sym),
            bli_axpy2v_ex_qfp: get_symbol(&libs, b"bli_axpy2v_ex_qfp\0").map(|sym| *sym),
            bli_axpyf_ex_qfp: get_symbol(&libs, b"bli_axpyf_ex_qfp\0").map(|sym| *sym),
            bli_dotaxpyv_ex_qfp: get_symbol(&libs, b"bli_dotaxpyv_ex_qfp\0").map(|sym| *sym),
            bli_dotxaxpyf_ex_qfp: get_symbol(&libs, b"bli_dotxaxpyf_ex_qfp\0").map(|sym| *sym),
            bli_dotxf_ex_qfp: get_symbol(&libs, b"bli_dotxf_ex_qfp\0").map(|sym| *sym),
            bli_addm_check: get_symbol(&libs, b"bli_addm_check\0").map(|sym| *sym),
            bli_copym_check: get_symbol(&libs, b"bli_copym_check\0").map(|sym| *sym),
            bli_subm_check: get_symbol(&libs, b"bli_subm_check\0").map(|sym| *sym),
            bli_axpym_check: get_symbol(&libs, b"bli_axpym_check\0").map(|sym| *sym),
            bli_scal2m_check: get_symbol(&libs, b"bli_scal2m_check\0").map(|sym| *sym),
            bli_scalm_check: get_symbol(&libs, b"bli_scalm_check\0").map(|sym| *sym),
            bli_setm_check: get_symbol(&libs, b"bli_setm_check\0").map(|sym| *sym),
            bli_xpbym_check: get_symbol(&libs, b"bli_xpbym_check\0").map(|sym| *sym),
            bli_l1m_xy_check: get_symbol(&libs, b"bli_l1m_xy_check\0").map(|sym| *sym),
            bli_l1m_axy_check: get_symbol(&libs, b"bli_l1m_axy_check\0").map(|sym| *sym),
            bli_l1m_ax_check: get_symbol(&libs, b"bli_l1m_ax_check\0").map(|sym| *sym),
            bli_addm_ex: get_symbol(&libs, b"bli_addm_ex\0").map(|sym| *sym),
            bli_copym_ex: get_symbol(&libs, b"bli_copym_ex\0").map(|sym| *sym),
            bli_subm_ex: get_symbol(&libs, b"bli_subm_ex\0").map(|sym| *sym),
            bli_axpym_ex: get_symbol(&libs, b"bli_axpym_ex\0").map(|sym| *sym),
            bli_scal2m_ex: get_symbol(&libs, b"bli_scal2m_ex\0").map(|sym| *sym),
            bli_scalm_ex: get_symbol(&libs, b"bli_scalm_ex\0").map(|sym| *sym),
            bli_setm_ex: get_symbol(&libs, b"bli_setm_ex\0").map(|sym| *sym),
            bli_xpbym_ex: get_symbol(&libs, b"bli_xpbym_ex\0").map(|sym| *sym),
            bli_xpbym_md_ex: get_symbol(&libs, b"bli_xpbym_md_ex\0").map(|sym| *sym),
            bli_addm: get_symbol(&libs, b"bli_addm\0").map(|sym| *sym),
            bli_copym: get_symbol(&libs, b"bli_copym\0").map(|sym| *sym),
            bli_subm: get_symbol(&libs, b"bli_subm\0").map(|sym| *sym),
            bli_axpym: get_symbol(&libs, b"bli_axpym\0").map(|sym| *sym),
            bli_scal2m: get_symbol(&libs, b"bli_scal2m\0").map(|sym| *sym),
            bli_scalm: get_symbol(&libs, b"bli_scalm\0").map(|sym| *sym),
            bli_setm: get_symbol(&libs, b"bli_setm\0").map(|sym| *sym),
            bli_xpbym: get_symbol(&libs, b"bli_xpbym\0").map(|sym| *sym),
            bli_xpbym_md: get_symbol(&libs, b"bli_xpbym_md\0").map(|sym| *sym),
            bli_saddm_ex: get_symbol(&libs, b"bli_saddm_ex\0").map(|sym| *sym),
            bli_daddm_ex: get_symbol(&libs, b"bli_daddm_ex\0").map(|sym| *sym),
            bli_caddm_ex: get_symbol(&libs, b"bli_caddm_ex\0").map(|sym| *sym),
            bli_zaddm_ex: get_symbol(&libs, b"bli_zaddm_ex\0").map(|sym| *sym),
            bli_scopym_ex: get_symbol(&libs, b"bli_scopym_ex\0").map(|sym| *sym),
            bli_dcopym_ex: get_symbol(&libs, b"bli_dcopym_ex\0").map(|sym| *sym),
            bli_ccopym_ex: get_symbol(&libs, b"bli_ccopym_ex\0").map(|sym| *sym),
            bli_zcopym_ex: get_symbol(&libs, b"bli_zcopym_ex\0").map(|sym| *sym),
            bli_ssubm_ex: get_symbol(&libs, b"bli_ssubm_ex\0").map(|sym| *sym),
            bli_dsubm_ex: get_symbol(&libs, b"bli_dsubm_ex\0").map(|sym| *sym),
            bli_csubm_ex: get_symbol(&libs, b"bli_csubm_ex\0").map(|sym| *sym),
            bli_zsubm_ex: get_symbol(&libs, b"bli_zsubm_ex\0").map(|sym| *sym),
            bli_saxpym_ex: get_symbol(&libs, b"bli_saxpym_ex\0").map(|sym| *sym),
            bli_daxpym_ex: get_symbol(&libs, b"bli_daxpym_ex\0").map(|sym| *sym),
            bli_caxpym_ex: get_symbol(&libs, b"bli_caxpym_ex\0").map(|sym| *sym),
            bli_zaxpym_ex: get_symbol(&libs, b"bli_zaxpym_ex\0").map(|sym| *sym),
            bli_sscal2m_ex: get_symbol(&libs, b"bli_sscal2m_ex\0").map(|sym| *sym),
            bli_dscal2m_ex: get_symbol(&libs, b"bli_dscal2m_ex\0").map(|sym| *sym),
            bli_cscal2m_ex: get_symbol(&libs, b"bli_cscal2m_ex\0").map(|sym| *sym),
            bli_zscal2m_ex: get_symbol(&libs, b"bli_zscal2m_ex\0").map(|sym| *sym),
            bli_sscalm_ex: get_symbol(&libs, b"bli_sscalm_ex\0").map(|sym| *sym),
            bli_dscalm_ex: get_symbol(&libs, b"bli_dscalm_ex\0").map(|sym| *sym),
            bli_cscalm_ex: get_symbol(&libs, b"bli_cscalm_ex\0").map(|sym| *sym),
            bli_zscalm_ex: get_symbol(&libs, b"bli_zscalm_ex\0").map(|sym| *sym),
            bli_ssetm_ex: get_symbol(&libs, b"bli_ssetm_ex\0").map(|sym| *sym),
            bli_dsetm_ex: get_symbol(&libs, b"bli_dsetm_ex\0").map(|sym| *sym),
            bli_csetm_ex: get_symbol(&libs, b"bli_csetm_ex\0").map(|sym| *sym),
            bli_zsetm_ex: get_symbol(&libs, b"bli_zsetm_ex\0").map(|sym| *sym),
            bli_sxpbym_ex: get_symbol(&libs, b"bli_sxpbym_ex\0").map(|sym| *sym),
            bli_dxpbym_ex: get_symbol(&libs, b"bli_dxpbym_ex\0").map(|sym| *sym),
            bli_cxpbym_ex: get_symbol(&libs, b"bli_cxpbym_ex\0").map(|sym| *sym),
            bli_zxpbym_ex: get_symbol(&libs, b"bli_zxpbym_ex\0").map(|sym| *sym),
            bli_ssxpbym_md_ex: get_symbol(&libs, b"bli_ssxpbym_md_ex\0").map(|sym| *sym),
            bli_ddxpbym_md_ex: get_symbol(&libs, b"bli_ddxpbym_md_ex\0").map(|sym| *sym),
            bli_ccxpbym_md_ex: get_symbol(&libs, b"bli_ccxpbym_md_ex\0").map(|sym| *sym),
            bli_zzxpbym_md_ex: get_symbol(&libs, b"bli_zzxpbym_md_ex\0").map(|sym| *sym),
            bli_sdxpbym_md_ex: get_symbol(&libs, b"bli_sdxpbym_md_ex\0").map(|sym| *sym),
            bli_scxpbym_md_ex: get_symbol(&libs, b"bli_scxpbym_md_ex\0").map(|sym| *sym),
            bli_szxpbym_md_ex: get_symbol(&libs, b"bli_szxpbym_md_ex\0").map(|sym| *sym),
            bli_dsxpbym_md_ex: get_symbol(&libs, b"bli_dsxpbym_md_ex\0").map(|sym| *sym),
            bli_dcxpbym_md_ex: get_symbol(&libs, b"bli_dcxpbym_md_ex\0").map(|sym| *sym),
            bli_dzxpbym_md_ex: get_symbol(&libs, b"bli_dzxpbym_md_ex\0").map(|sym| *sym),
            bli_csxpbym_md_ex: get_symbol(&libs, b"bli_csxpbym_md_ex\0").map(|sym| *sym),
            bli_cdxpbym_md_ex: get_symbol(&libs, b"bli_cdxpbym_md_ex\0").map(|sym| *sym),
            bli_czxpbym_md_ex: get_symbol(&libs, b"bli_czxpbym_md_ex\0").map(|sym| *sym),
            bli_zsxpbym_md_ex: get_symbol(&libs, b"bli_zsxpbym_md_ex\0").map(|sym| *sym),
            bli_zdxpbym_md_ex: get_symbol(&libs, b"bli_zdxpbym_md_ex\0").map(|sym| *sym),
            bli_zcxpbym_md_ex: get_symbol(&libs, b"bli_zcxpbym_md_ex\0").map(|sym| *sym),
            bli_saddm: get_symbol(&libs, b"bli_saddm\0").map(|sym| *sym),
            bli_daddm: get_symbol(&libs, b"bli_daddm\0").map(|sym| *sym),
            bli_caddm: get_symbol(&libs, b"bli_caddm\0").map(|sym| *sym),
            bli_zaddm: get_symbol(&libs, b"bli_zaddm\0").map(|sym| *sym),
            bli_scopym: get_symbol(&libs, b"bli_scopym\0").map(|sym| *sym),
            bli_dcopym: get_symbol(&libs, b"bli_dcopym\0").map(|sym| *sym),
            bli_ccopym: get_symbol(&libs, b"bli_ccopym\0").map(|sym| *sym),
            bli_zcopym: get_symbol(&libs, b"bli_zcopym\0").map(|sym| *sym),
            bli_ssubm: get_symbol(&libs, b"bli_ssubm\0").map(|sym| *sym),
            bli_dsubm: get_symbol(&libs, b"bli_dsubm\0").map(|sym| *sym),
            bli_csubm: get_symbol(&libs, b"bli_csubm\0").map(|sym| *sym),
            bli_zsubm: get_symbol(&libs, b"bli_zsubm\0").map(|sym| *sym),
            bli_saxpym: get_symbol(&libs, b"bli_saxpym\0").map(|sym| *sym),
            bli_daxpym: get_symbol(&libs, b"bli_daxpym\0").map(|sym| *sym),
            bli_caxpym: get_symbol(&libs, b"bli_caxpym\0").map(|sym| *sym),
            bli_zaxpym: get_symbol(&libs, b"bli_zaxpym\0").map(|sym| *sym),
            bli_sscal2m: get_symbol(&libs, b"bli_sscal2m\0").map(|sym| *sym),
            bli_dscal2m: get_symbol(&libs, b"bli_dscal2m\0").map(|sym| *sym),
            bli_cscal2m: get_symbol(&libs, b"bli_cscal2m\0").map(|sym| *sym),
            bli_zscal2m: get_symbol(&libs, b"bli_zscal2m\0").map(|sym| *sym),
            bli_sscalm: get_symbol(&libs, b"bli_sscalm\0").map(|sym| *sym),
            bli_dscalm: get_symbol(&libs, b"bli_dscalm\0").map(|sym| *sym),
            bli_cscalm: get_symbol(&libs, b"bli_cscalm\0").map(|sym| *sym),
            bli_zscalm: get_symbol(&libs, b"bli_zscalm\0").map(|sym| *sym),
            bli_ssetm: get_symbol(&libs, b"bli_ssetm\0").map(|sym| *sym),
            bli_dsetm: get_symbol(&libs, b"bli_dsetm\0").map(|sym| *sym),
            bli_csetm: get_symbol(&libs, b"bli_csetm\0").map(|sym| *sym),
            bli_zsetm: get_symbol(&libs, b"bli_zsetm\0").map(|sym| *sym),
            bli_sxpbym: get_symbol(&libs, b"bli_sxpbym\0").map(|sym| *sym),
            bli_dxpbym: get_symbol(&libs, b"bli_dxpbym\0").map(|sym| *sym),
            bli_cxpbym: get_symbol(&libs, b"bli_cxpbym\0").map(|sym| *sym),
            bli_zxpbym: get_symbol(&libs, b"bli_zxpbym\0").map(|sym| *sym),
            bli_ssxpbym_md: get_symbol(&libs, b"bli_ssxpbym_md\0").map(|sym| *sym),
            bli_ddxpbym_md: get_symbol(&libs, b"bli_ddxpbym_md\0").map(|sym| *sym),
            bli_ccxpbym_md: get_symbol(&libs, b"bli_ccxpbym_md\0").map(|sym| *sym),
            bli_zzxpbym_md: get_symbol(&libs, b"bli_zzxpbym_md\0").map(|sym| *sym),
            bli_sdxpbym_md: get_symbol(&libs, b"bli_sdxpbym_md\0").map(|sym| *sym),
            bli_scxpbym_md: get_symbol(&libs, b"bli_scxpbym_md\0").map(|sym| *sym),
            bli_szxpbym_md: get_symbol(&libs, b"bli_szxpbym_md\0").map(|sym| *sym),
            bli_dsxpbym_md: get_symbol(&libs, b"bli_dsxpbym_md\0").map(|sym| *sym),
            bli_dcxpbym_md: get_symbol(&libs, b"bli_dcxpbym_md\0").map(|sym| *sym),
            bli_dzxpbym_md: get_symbol(&libs, b"bli_dzxpbym_md\0").map(|sym| *sym),
            bli_csxpbym_md: get_symbol(&libs, b"bli_csxpbym_md\0").map(|sym| *sym),
            bli_cdxpbym_md: get_symbol(&libs, b"bli_cdxpbym_md\0").map(|sym| *sym),
            bli_czxpbym_md: get_symbol(&libs, b"bli_czxpbym_md\0").map(|sym| *sym),
            bli_zsxpbym_md: get_symbol(&libs, b"bli_zsxpbym_md\0").map(|sym| *sym),
            bli_zdxpbym_md: get_symbol(&libs, b"bli_zdxpbym_md\0").map(|sym| *sym),
            bli_zcxpbym_md: get_symbol(&libs, b"bli_zcxpbym_md\0").map(|sym| *sym),
            bli_addm_ex_qfp: get_symbol(&libs, b"bli_addm_ex_qfp\0").map(|sym| *sym),
            bli_copym_ex_qfp: get_symbol(&libs, b"bli_copym_ex_qfp\0").map(|sym| *sym),
            bli_subm_ex_qfp: get_symbol(&libs, b"bli_subm_ex_qfp\0").map(|sym| *sym),
            bli_axpym_ex_qfp: get_symbol(&libs, b"bli_axpym_ex_qfp\0").map(|sym| *sym),
            bli_scal2m_ex_qfp: get_symbol(&libs, b"bli_scal2m_ex_qfp\0").map(|sym| *sym),
            bli_scalm_ex_qfp: get_symbol(&libs, b"bli_scalm_ex_qfp\0").map(|sym| *sym),
            bli_setm_ex_qfp: get_symbol(&libs, b"bli_setm_ex_qfp\0").map(|sym| *sym),
            bli_xpbym_ex_qfp: get_symbol(&libs, b"bli_xpbym_ex_qfp\0").map(|sym| *sym),
            bli_xpbym_md_ex_qfp2: get_symbol(&libs, b"bli_xpbym_md_ex_qfp2\0").map(|sym| *sym),
            bli_saddm_unb_var1: get_symbol(&libs, b"bli_saddm_unb_var1\0").map(|sym| *sym),
            bli_daddm_unb_var1: get_symbol(&libs, b"bli_daddm_unb_var1\0").map(|sym| *sym),
            bli_caddm_unb_var1: get_symbol(&libs, b"bli_caddm_unb_var1\0").map(|sym| *sym),
            bli_zaddm_unb_var1: get_symbol(&libs, b"bli_zaddm_unb_var1\0").map(|sym| *sym),
            bli_scopym_unb_var1: get_symbol(&libs, b"bli_scopym_unb_var1\0").map(|sym| *sym),
            bli_dcopym_unb_var1: get_symbol(&libs, b"bli_dcopym_unb_var1\0").map(|sym| *sym),
            bli_ccopym_unb_var1: get_symbol(&libs, b"bli_ccopym_unb_var1\0").map(|sym| *sym),
            bli_zcopym_unb_var1: get_symbol(&libs, b"bli_zcopym_unb_var1\0").map(|sym| *sym),
            bli_ssubm_unb_var1: get_symbol(&libs, b"bli_ssubm_unb_var1\0").map(|sym| *sym),
            bli_dsubm_unb_var1: get_symbol(&libs, b"bli_dsubm_unb_var1\0").map(|sym| *sym),
            bli_csubm_unb_var1: get_symbol(&libs, b"bli_csubm_unb_var1\0").map(|sym| *sym),
            bli_zsubm_unb_var1: get_symbol(&libs, b"bli_zsubm_unb_var1\0").map(|sym| *sym),
            bli_saxpym_unb_var1: get_symbol(&libs, b"bli_saxpym_unb_var1\0").map(|sym| *sym),
            bli_daxpym_unb_var1: get_symbol(&libs, b"bli_daxpym_unb_var1\0").map(|sym| *sym),
            bli_caxpym_unb_var1: get_symbol(&libs, b"bli_caxpym_unb_var1\0").map(|sym| *sym),
            bli_zaxpym_unb_var1: get_symbol(&libs, b"bli_zaxpym_unb_var1\0").map(|sym| *sym),
            bli_sscal2m_unb_var1: get_symbol(&libs, b"bli_sscal2m_unb_var1\0").map(|sym| *sym),
            bli_dscal2m_unb_var1: get_symbol(&libs, b"bli_dscal2m_unb_var1\0").map(|sym| *sym),
            bli_cscal2m_unb_var1: get_symbol(&libs, b"bli_cscal2m_unb_var1\0").map(|sym| *sym),
            bli_zscal2m_unb_var1: get_symbol(&libs, b"bli_zscal2m_unb_var1\0").map(|sym| *sym),
            bli_sscalm_unb_var1: get_symbol(&libs, b"bli_sscalm_unb_var1\0").map(|sym| *sym),
            bli_dscalm_unb_var1: get_symbol(&libs, b"bli_dscalm_unb_var1\0").map(|sym| *sym),
            bli_cscalm_unb_var1: get_symbol(&libs, b"bli_cscalm_unb_var1\0").map(|sym| *sym),
            bli_zscalm_unb_var1: get_symbol(&libs, b"bli_zscalm_unb_var1\0").map(|sym| *sym),
            bli_ssetm_unb_var1: get_symbol(&libs, b"bli_ssetm_unb_var1\0").map(|sym| *sym),
            bli_dsetm_unb_var1: get_symbol(&libs, b"bli_dsetm_unb_var1\0").map(|sym| *sym),
            bli_csetm_unb_var1: get_symbol(&libs, b"bli_csetm_unb_var1\0").map(|sym| *sym),
            bli_zsetm_unb_var1: get_symbol(&libs, b"bli_zsetm_unb_var1\0").map(|sym| *sym),
            bli_sxpbym_unb_var1: get_symbol(&libs, b"bli_sxpbym_unb_var1\0").map(|sym| *sym),
            bli_dxpbym_unb_var1: get_symbol(&libs, b"bli_dxpbym_unb_var1\0").map(|sym| *sym),
            bli_cxpbym_unb_var1: get_symbol(&libs, b"bli_cxpbym_unb_var1\0").map(|sym| *sym),
            bli_zxpbym_unb_var1: get_symbol(&libs, b"bli_zxpbym_unb_var1\0").map(|sym| *sym),
            bli_ssxpbym_md_unb_var1: get_symbol(&libs, b"bli_ssxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_ddxpbym_md_unb_var1: get_symbol(&libs, b"bli_ddxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_ccxpbym_md_unb_var1: get_symbol(&libs, b"bli_ccxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_zzxpbym_md_unb_var1: get_symbol(&libs, b"bli_zzxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_sdxpbym_md_unb_var1: get_symbol(&libs, b"bli_sdxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_scxpbym_md_unb_var1: get_symbol(&libs, b"bli_scxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_szxpbym_md_unb_var1: get_symbol(&libs, b"bli_szxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_dsxpbym_md_unb_var1: get_symbol(&libs, b"bli_dsxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_dcxpbym_md_unb_var1: get_symbol(&libs, b"bli_dcxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_dzxpbym_md_unb_var1: get_symbol(&libs, b"bli_dzxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_csxpbym_md_unb_var1: get_symbol(&libs, b"bli_csxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_cdxpbym_md_unb_var1: get_symbol(&libs, b"bli_cdxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_czxpbym_md_unb_var1: get_symbol(&libs, b"bli_czxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_zsxpbym_md_unb_var1: get_symbol(&libs, b"bli_zsxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_zdxpbym_md_unb_var1: get_symbol(&libs, b"bli_zdxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_zcxpbym_md_unb_var1: get_symbol(&libs, b"bli_zcxpbym_md_unb_var1\0")
                .map(|sym| *sym),
            bli_packm_cntl_create_node: get_symbol(&libs, b"bli_packm_cntl_create_node\0")
                .map(|sym| *sym),
            bli_packm_init_check: get_symbol(&libs, b"bli_packm_init_check\0").map(|sym| *sym),
            bli_packm_int_check: get_symbol(&libs, b"bli_packm_int_check\0").map(|sym| *sym),
            bli_packm_init: get_symbol(&libs, b"bli_packm_init\0").map(|sym| *sym),
            bli_packm_init_pack: get_symbol(&libs, b"bli_packm_init_pack\0").map(|sym| *sym),
            bli_packm_int: get_symbol(&libs, b"bli_packm_int\0").map(|sym| *sym),
            bli_packm_acquire_mpart_t2b: get_symbol(&libs, b"bli_packm_acquire_mpart_t2b\0")
                .map(|sym| *sym),
            bli_packm_acquire_mpart_l2r: get_symbol(&libs, b"bli_packm_acquire_mpart_l2r\0")
                .map(|sym| *sym),
            bli_packm_acquire_mpart_tl2br: get_symbol(&libs, b"bli_packm_acquire_mpart_tl2br\0")
                .map(|sym| *sym),
            bli_packm_offset_to_panel_for: get_symbol(&libs, b"bli_packm_offset_to_panel_for\0")
                .map(|sym| *sym),
            bli_packm_unb_var1: get_symbol(&libs, b"bli_packm_unb_var1\0").map(|sym| *sym),
            bli_packm_blk_var1: get_symbol(&libs, b"bli_packm_blk_var1\0").map(|sym| *sym),
            bli_spackm_unb_var1: get_symbol(&libs, b"bli_spackm_unb_var1\0").map(|sym| *sym),
            bli_dpackm_unb_var1: get_symbol(&libs, b"bli_dpackm_unb_var1\0").map(|sym| *sym),
            bli_cpackm_unb_var1: get_symbol(&libs, b"bli_cpackm_unb_var1\0").map(|sym| *sym),
            bli_zpackm_unb_var1: get_symbol(&libs, b"bli_zpackm_unb_var1\0").map(|sym| *sym),
            bli_spackm_blk_var1: get_symbol(&libs, b"bli_spackm_blk_var1\0").map(|sym| *sym),
            bli_dpackm_blk_var1: get_symbol(&libs, b"bli_dpackm_blk_var1\0").map(|sym| *sym),
            bli_cpackm_blk_var1: get_symbol(&libs, b"bli_cpackm_blk_var1\0").map(|sym| *sym),
            bli_zpackm_blk_var1: get_symbol(&libs, b"bli_zpackm_blk_var1\0").map(|sym| *sym),
            bli_spackm_struc_cxk: get_symbol(&libs, b"bli_spackm_struc_cxk\0").map(|sym| *sym),
            bli_dpackm_struc_cxk: get_symbol(&libs, b"bli_dpackm_struc_cxk\0").map(|sym| *sym),
            bli_cpackm_struc_cxk: get_symbol(&libs, b"bli_cpackm_struc_cxk\0").map(|sym| *sym),
            bli_zpackm_struc_cxk: get_symbol(&libs, b"bli_zpackm_struc_cxk\0").map(|sym| *sym),
            bli_spackm_herm_cxk: get_symbol(&libs, b"bli_spackm_herm_cxk\0").map(|sym| *sym),
            bli_dpackm_herm_cxk: get_symbol(&libs, b"bli_dpackm_herm_cxk\0").map(|sym| *sym),
            bli_cpackm_herm_cxk: get_symbol(&libs, b"bli_cpackm_herm_cxk\0").map(|sym| *sym),
            bli_zpackm_herm_cxk: get_symbol(&libs, b"bli_zpackm_herm_cxk\0").map(|sym| *sym),
            bli_spackm_tri_cxk: get_symbol(&libs, b"bli_spackm_tri_cxk\0").map(|sym| *sym),
            bli_dpackm_tri_cxk: get_symbol(&libs, b"bli_dpackm_tri_cxk\0").map(|sym| *sym),
            bli_cpackm_tri_cxk: get_symbol(&libs, b"bli_cpackm_tri_cxk\0").map(|sym| *sym),
            bli_zpackm_tri_cxk: get_symbol(&libs, b"bli_zpackm_tri_cxk\0").map(|sym| *sym),
            bli_cpackm_struc_cxk_1er: get_symbol(&libs, b"bli_cpackm_struc_cxk_1er\0")
                .map(|sym| *sym),
            bli_zpackm_struc_cxk_1er: get_symbol(&libs, b"bli_zpackm_struc_cxk_1er\0")
                .map(|sym| *sym),
            bli_cpackm_herm_cxk_1er: get_symbol(&libs, b"bli_cpackm_herm_cxk_1er\0")
                .map(|sym| *sym),
            bli_zpackm_herm_cxk_1er: get_symbol(&libs, b"bli_zpackm_herm_cxk_1er\0")
                .map(|sym| *sym),
            bli_cpackm_tri_cxk_1er: get_symbol(&libs, b"bli_cpackm_tri_cxk_1er\0").map(|sym| *sym),
            bli_zpackm_tri_cxk_1er: get_symbol(&libs, b"bli_zpackm_tri_cxk_1er\0").map(|sym| *sym),
            bli_spackm_cxk: get_symbol(&libs, b"bli_spackm_cxk\0").map(|sym| *sym),
            bli_dpackm_cxk: get_symbol(&libs, b"bli_dpackm_cxk\0").map(|sym| *sym),
            bli_cpackm_cxk: get_symbol(&libs, b"bli_cpackm_cxk\0").map(|sym| *sym),
            bli_zpackm_cxk: get_symbol(&libs, b"bli_zpackm_cxk\0").map(|sym| *sym),
            bli_cpackm_cxk_1er: get_symbol(&libs, b"bli_cpackm_cxk_1er\0").map(|sym| *sym),
            bli_zpackm_cxk_1er: get_symbol(&libs, b"bli_zpackm_cxk_1er\0").map(|sym| *sym),
            bli_pack_full_init: get_symbol(&libs, b"bli_pack_full_init\0").map(|sym| *sym),
            bli_spackm_full: get_symbol(&libs, b"bli_spackm_full\0").map(|sym| *sym),
            bli_dpackm_full: get_symbol(&libs, b"bli_dpackm_full\0").map(|sym| *sym),
            bli_packm_blk_var1_md: get_symbol(&libs, b"bli_packm_blk_var1_md\0").map(|sym| *sym),
            bli_sspackm_blk_var1_md: get_symbol(&libs, b"bli_sspackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_ddpackm_blk_var1_md: get_symbol(&libs, b"bli_ddpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_ccpackm_blk_var1_md: get_symbol(&libs, b"bli_ccpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_zzpackm_blk_var1_md: get_symbol(&libs, b"bli_zzpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_sdpackm_blk_var1_md: get_symbol(&libs, b"bli_sdpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_scpackm_blk_var1_md: get_symbol(&libs, b"bli_scpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_szpackm_blk_var1_md: get_symbol(&libs, b"bli_szpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_dspackm_blk_var1_md: get_symbol(&libs, b"bli_dspackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_dcpackm_blk_var1_md: get_symbol(&libs, b"bli_dcpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_dzpackm_blk_var1_md: get_symbol(&libs, b"bli_dzpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_cspackm_blk_var1_md: get_symbol(&libs, b"bli_cspackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_cdpackm_blk_var1_md: get_symbol(&libs, b"bli_cdpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_czpackm_blk_var1_md: get_symbol(&libs, b"bli_czpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_zspackm_blk_var1_md: get_symbol(&libs, b"bli_zspackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_zdpackm_blk_var1_md: get_symbol(&libs, b"bli_zdpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_zcpackm_blk_var1_md: get_symbol(&libs, b"bli_zcpackm_blk_var1_md\0")
                .map(|sym| *sym),
            bli_sspackm_struc_cxk_md: get_symbol(&libs, b"bli_sspackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_ddpackm_struc_cxk_md: get_symbol(&libs, b"bli_ddpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_ccpackm_struc_cxk_md: get_symbol(&libs, b"bli_ccpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_zzpackm_struc_cxk_md: get_symbol(&libs, b"bli_zzpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_sdpackm_struc_cxk_md: get_symbol(&libs, b"bli_sdpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_scpackm_struc_cxk_md: get_symbol(&libs, b"bli_scpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_szpackm_struc_cxk_md: get_symbol(&libs, b"bli_szpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_dspackm_struc_cxk_md: get_symbol(&libs, b"bli_dspackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_dcpackm_struc_cxk_md: get_symbol(&libs, b"bli_dcpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_dzpackm_struc_cxk_md: get_symbol(&libs, b"bli_dzpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_cspackm_struc_cxk_md: get_symbol(&libs, b"bli_cspackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_cdpackm_struc_cxk_md: get_symbol(&libs, b"bli_cdpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_czpackm_struc_cxk_md: get_symbol(&libs, b"bli_czpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_zspackm_struc_cxk_md: get_symbol(&libs, b"bli_zspackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_zdpackm_struc_cxk_md: get_symbol(&libs, b"bli_zdpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_zcpackm_struc_cxk_md: get_symbol(&libs, b"bli_zcpackm_struc_cxk_md\0")
                .map(|sym| *sym),
            bli_sspackm_cxk_1e_md: get_symbol(&libs, b"bli_sspackm_cxk_1e_md\0").map(|sym| *sym),
            bli_ddpackm_cxk_1e_md: get_symbol(&libs, b"bli_ddpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_ccpackm_cxk_1e_md: get_symbol(&libs, b"bli_ccpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_zzpackm_cxk_1e_md: get_symbol(&libs, b"bli_zzpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_sdpackm_cxk_1e_md: get_symbol(&libs, b"bli_sdpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_scpackm_cxk_1e_md: get_symbol(&libs, b"bli_scpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_szpackm_cxk_1e_md: get_symbol(&libs, b"bli_szpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_dspackm_cxk_1e_md: get_symbol(&libs, b"bli_dspackm_cxk_1e_md\0").map(|sym| *sym),
            bli_dcpackm_cxk_1e_md: get_symbol(&libs, b"bli_dcpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_dzpackm_cxk_1e_md: get_symbol(&libs, b"bli_dzpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_cspackm_cxk_1e_md: get_symbol(&libs, b"bli_cspackm_cxk_1e_md\0").map(|sym| *sym),
            bli_cdpackm_cxk_1e_md: get_symbol(&libs, b"bli_cdpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_czpackm_cxk_1e_md: get_symbol(&libs, b"bli_czpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_zspackm_cxk_1e_md: get_symbol(&libs, b"bli_zspackm_cxk_1e_md\0").map(|sym| *sym),
            bli_zdpackm_cxk_1e_md: get_symbol(&libs, b"bli_zdpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_zcpackm_cxk_1e_md: get_symbol(&libs, b"bli_zcpackm_cxk_1e_md\0").map(|sym| *sym),
            bli_sspackm_cxk_1r_md: get_symbol(&libs, b"bli_sspackm_cxk_1r_md\0").map(|sym| *sym),
            bli_ddpackm_cxk_1r_md: get_symbol(&libs, b"bli_ddpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_ccpackm_cxk_1r_md: get_symbol(&libs, b"bli_ccpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_zzpackm_cxk_1r_md: get_symbol(&libs, b"bli_zzpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_sdpackm_cxk_1r_md: get_symbol(&libs, b"bli_sdpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_scpackm_cxk_1r_md: get_symbol(&libs, b"bli_scpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_szpackm_cxk_1r_md: get_symbol(&libs, b"bli_szpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_dspackm_cxk_1r_md: get_symbol(&libs, b"bli_dspackm_cxk_1r_md\0").map(|sym| *sym),
            bli_dcpackm_cxk_1r_md: get_symbol(&libs, b"bli_dcpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_dzpackm_cxk_1r_md: get_symbol(&libs, b"bli_dzpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_cspackm_cxk_1r_md: get_symbol(&libs, b"bli_cspackm_cxk_1r_md\0").map(|sym| *sym),
            bli_cdpackm_cxk_1r_md: get_symbol(&libs, b"bli_cdpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_czpackm_cxk_1r_md: get_symbol(&libs, b"bli_czpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_zspackm_cxk_1r_md: get_symbol(&libs, b"bli_zspackm_cxk_1r_md\0").map(|sym| *sym),
            bli_zdpackm_cxk_1r_md: get_symbol(&libs, b"bli_zdpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_zcpackm_cxk_1r_md: get_symbol(&libs, b"bli_zcpackm_cxk_1r_md\0").map(|sym| *sym),
            bli_unpackm_cntl_create_node: get_symbol(&libs, b"bli_unpackm_cntl_create_node\0")
                .map(|sym| *sym),
            bli_unpackm_int_check: get_symbol(&libs, b"bli_unpackm_int_check\0").map(|sym| *sym),
            bli_unpackm_int: get_symbol(&libs, b"bli_unpackm_int\0").map(|sym| *sym),
            bli_unpackm_unb_var1: get_symbol(&libs, b"bli_unpackm_unb_var1\0").map(|sym| *sym),
            bli_sunpackm_unb_var1: get_symbol(&libs, b"bli_sunpackm_unb_var1\0").map(|sym| *sym),
            bli_dunpackm_unb_var1: get_symbol(&libs, b"bli_dunpackm_unb_var1\0").map(|sym| *sym),
            bli_cunpackm_unb_var1: get_symbol(&libs, b"bli_cunpackm_unb_var1\0").map(|sym| *sym),
            bli_zunpackm_unb_var1: get_symbol(&libs, b"bli_zunpackm_unb_var1\0").map(|sym| *sym),
            bli_unpackm_blk_var1: get_symbol(&libs, b"bli_unpackm_blk_var1\0").map(|sym| *sym),
            bli_sunpackm_blk_var1: get_symbol(&libs, b"bli_sunpackm_blk_var1\0").map(|sym| *sym),
            bli_dunpackm_blk_var1: get_symbol(&libs, b"bli_dunpackm_blk_var1\0").map(|sym| *sym),
            bli_cunpackm_blk_var1: get_symbol(&libs, b"bli_cunpackm_blk_var1\0").map(|sym| *sym),
            bli_zunpackm_blk_var1: get_symbol(&libs, b"bli_zunpackm_blk_var1\0").map(|sym| *sym),
            bli_sunpackm_cxk: get_symbol(&libs, b"bli_sunpackm_cxk\0").map(|sym| *sym),
            bli_dunpackm_cxk: get_symbol(&libs, b"bli_dunpackm_cxk\0").map(|sym| *sym),
            bli_cunpackm_cxk: get_symbol(&libs, b"bli_cunpackm_cxk\0").map(|sym| *sym),
            bli_zunpackm_cxk: get_symbol(&libs, b"bli_zunpackm_cxk\0").map(|sym| *sym),
            bli_gemv_check: get_symbol(&libs, b"bli_gemv_check\0").map(|sym| *sym),
            bli_hemv_check: get_symbol(&libs, b"bli_hemv_check\0").map(|sym| *sym),
            bli_symv_check: get_symbol(&libs, b"bli_symv_check\0").map(|sym| *sym),
            bli_ger_check: get_symbol(&libs, b"bli_ger_check\0").map(|sym| *sym),
            bli_her2_check: get_symbol(&libs, b"bli_her2_check\0").map(|sym| *sym),
            bli_syr2_check: get_symbol(&libs, b"bli_syr2_check\0").map(|sym| *sym),
            bli_her_check: get_symbol(&libs, b"bli_her_check\0").map(|sym| *sym),
            bli_syr_check: get_symbol(&libs, b"bli_syr_check\0").map(|sym| *sym),
            bli_trmv_check: get_symbol(&libs, b"bli_trmv_check\0").map(|sym| *sym),
            bli_trsv_check: get_symbol(&libs, b"bli_trsv_check\0").map(|sym| *sym),
            bli_xxmv_check: get_symbol(&libs, b"bli_xxmv_check\0").map(|sym| *sym),
            bli_xxr_check: get_symbol(&libs, b"bli_xxr_check\0").map(|sym| *sym),
            bli_gemv_ex: get_symbol(&libs, b"bli_gemv_ex\0").map(|sym| *sym),
            bli_hemv_ex: get_symbol(&libs, b"bli_hemv_ex\0").map(|sym| *sym),
            bli_symv_ex: get_symbol(&libs, b"bli_symv_ex\0").map(|sym| *sym),
            bli_ger_ex: get_symbol(&libs, b"bli_ger_ex\0").map(|sym| *sym),
            bli_her2_ex: get_symbol(&libs, b"bli_her2_ex\0").map(|sym| *sym),
            bli_syr2_ex: get_symbol(&libs, b"bli_syr2_ex\0").map(|sym| *sym),
            bli_her_ex: get_symbol(&libs, b"bli_her_ex\0").map(|sym| *sym),
            bli_syr_ex: get_symbol(&libs, b"bli_syr_ex\0").map(|sym| *sym),
            bli_trmv_ex: get_symbol(&libs, b"bli_trmv_ex\0").map(|sym| *sym),
            bli_trsv_ex: get_symbol(&libs, b"bli_trsv_ex\0").map(|sym| *sym),
            bli_gemv: get_symbol(&libs, b"bli_gemv\0").map(|sym| *sym),
            bli_hemv: get_symbol(&libs, b"bli_hemv\0").map(|sym| *sym),
            bli_symv: get_symbol(&libs, b"bli_symv\0").map(|sym| *sym),
            bli_ger: get_symbol(&libs, b"bli_ger\0").map(|sym| *sym),
            bli_her2: get_symbol(&libs, b"bli_her2\0").map(|sym| *sym),
            bli_syr2: get_symbol(&libs, b"bli_syr2\0").map(|sym| *sym),
            bli_her: get_symbol(&libs, b"bli_her\0").map(|sym| *sym),
            bli_syr: get_symbol(&libs, b"bli_syr\0").map(|sym| *sym),
            bli_trmv: get_symbol(&libs, b"bli_trmv\0").map(|sym| *sym),
            bli_trsv: get_symbol(&libs, b"bli_trsv\0").map(|sym| *sym),
            bli_sgemv_ex: get_symbol(&libs, b"bli_sgemv_ex\0").map(|sym| *sym),
            bli_dgemv_ex: get_symbol(&libs, b"bli_dgemv_ex\0").map(|sym| *sym),
            bli_cgemv_ex: get_symbol(&libs, b"bli_cgemv_ex\0").map(|sym| *sym),
            bli_zgemv_ex: get_symbol(&libs, b"bli_zgemv_ex\0").map(|sym| *sym),
            bli_sger_ex: get_symbol(&libs, b"bli_sger_ex\0").map(|sym| *sym),
            bli_dger_ex: get_symbol(&libs, b"bli_dger_ex\0").map(|sym| *sym),
            bli_cger_ex: get_symbol(&libs, b"bli_cger_ex\0").map(|sym| *sym),
            bli_zger_ex: get_symbol(&libs, b"bli_zger_ex\0").map(|sym| *sym),
            bli_shemv_ex: get_symbol(&libs, b"bli_shemv_ex\0").map(|sym| *sym),
            bli_dhemv_ex: get_symbol(&libs, b"bli_dhemv_ex\0").map(|sym| *sym),
            bli_chemv_ex: get_symbol(&libs, b"bli_chemv_ex\0").map(|sym| *sym),
            bli_zhemv_ex: get_symbol(&libs, b"bli_zhemv_ex\0").map(|sym| *sym),
            bli_ssymv_ex: get_symbol(&libs, b"bli_ssymv_ex\0").map(|sym| *sym),
            bli_dsymv_ex: get_symbol(&libs, b"bli_dsymv_ex\0").map(|sym| *sym),
            bli_csymv_ex: get_symbol(&libs, b"bli_csymv_ex\0").map(|sym| *sym),
            bli_zsymv_ex: get_symbol(&libs, b"bli_zsymv_ex\0").map(|sym| *sym),
            bli_sher_ex: get_symbol(&libs, b"bli_sher_ex\0").map(|sym| *sym),
            bli_dher_ex: get_symbol(&libs, b"bli_dher_ex\0").map(|sym| *sym),
            bli_cher_ex: get_symbol(&libs, b"bli_cher_ex\0").map(|sym| *sym),
            bli_zher_ex: get_symbol(&libs, b"bli_zher_ex\0").map(|sym| *sym),
            bli_ssyr_ex: get_symbol(&libs, b"bli_ssyr_ex\0").map(|sym| *sym),
            bli_dsyr_ex: get_symbol(&libs, b"bli_dsyr_ex\0").map(|sym| *sym),
            bli_csyr_ex: get_symbol(&libs, b"bli_csyr_ex\0").map(|sym| *sym),
            bli_zsyr_ex: get_symbol(&libs, b"bli_zsyr_ex\0").map(|sym| *sym),
            bli_sher2_ex: get_symbol(&libs, b"bli_sher2_ex\0").map(|sym| *sym),
            bli_dher2_ex: get_symbol(&libs, b"bli_dher2_ex\0").map(|sym| *sym),
            bli_cher2_ex: get_symbol(&libs, b"bli_cher2_ex\0").map(|sym| *sym),
            bli_zher2_ex: get_symbol(&libs, b"bli_zher2_ex\0").map(|sym| *sym),
            bli_ssyr2_ex: get_symbol(&libs, b"bli_ssyr2_ex\0").map(|sym| *sym),
            bli_dsyr2_ex: get_symbol(&libs, b"bli_dsyr2_ex\0").map(|sym| *sym),
            bli_csyr2_ex: get_symbol(&libs, b"bli_csyr2_ex\0").map(|sym| *sym),
            bli_zsyr2_ex: get_symbol(&libs, b"bli_zsyr2_ex\0").map(|sym| *sym),
            bli_strmv_ex: get_symbol(&libs, b"bli_strmv_ex\0").map(|sym| *sym),
            bli_dtrmv_ex: get_symbol(&libs, b"bli_dtrmv_ex\0").map(|sym| *sym),
            bli_ctrmv_ex: get_symbol(&libs, b"bli_ctrmv_ex\0").map(|sym| *sym),
            bli_ztrmv_ex: get_symbol(&libs, b"bli_ztrmv_ex\0").map(|sym| *sym),
            bli_strsv_ex: get_symbol(&libs, b"bli_strsv_ex\0").map(|sym| *sym),
            bli_dtrsv_ex: get_symbol(&libs, b"bli_dtrsv_ex\0").map(|sym| *sym),
            bli_ctrsv_ex: get_symbol(&libs, b"bli_ctrsv_ex\0").map(|sym| *sym),
            bli_ztrsv_ex: get_symbol(&libs, b"bli_ztrsv_ex\0").map(|sym| *sym),
            bli_sgemv: get_symbol(&libs, b"bli_sgemv\0").map(|sym| *sym),
            bli_dgemv: get_symbol(&libs, b"bli_dgemv\0").map(|sym| *sym),
            bli_cgemv: get_symbol(&libs, b"bli_cgemv\0").map(|sym| *sym),
            bli_zgemv: get_symbol(&libs, b"bli_zgemv\0").map(|sym| *sym),
            bli_sger: get_symbol(&libs, b"bli_sger\0").map(|sym| *sym),
            bli_dger: get_symbol(&libs, b"bli_dger\0").map(|sym| *sym),
            bli_cger: get_symbol(&libs, b"bli_cger\0").map(|sym| *sym),
            bli_zger: get_symbol(&libs, b"bli_zger\0").map(|sym| *sym),
            bli_shemv: get_symbol(&libs, b"bli_shemv\0").map(|sym| *sym),
            bli_dhemv: get_symbol(&libs, b"bli_dhemv\0").map(|sym| *sym),
            bli_chemv: get_symbol(&libs, b"bli_chemv\0").map(|sym| *sym),
            bli_zhemv: get_symbol(&libs, b"bli_zhemv\0").map(|sym| *sym),
            bli_ssymv: get_symbol(&libs, b"bli_ssymv\0").map(|sym| *sym),
            bli_dsymv: get_symbol(&libs, b"bli_dsymv\0").map(|sym| *sym),
            bli_csymv: get_symbol(&libs, b"bli_csymv\0").map(|sym| *sym),
            bli_zsymv: get_symbol(&libs, b"bli_zsymv\0").map(|sym| *sym),
            bli_sher: get_symbol(&libs, b"bli_sher\0").map(|sym| *sym),
            bli_dher: get_symbol(&libs, b"bli_dher\0").map(|sym| *sym),
            bli_cher: get_symbol(&libs, b"bli_cher\0").map(|sym| *sym),
            bli_zher: get_symbol(&libs, b"bli_zher\0").map(|sym| *sym),
            bli_ssyr: get_symbol(&libs, b"bli_ssyr\0").map(|sym| *sym),
            bli_dsyr: get_symbol(&libs, b"bli_dsyr\0").map(|sym| *sym),
            bli_csyr: get_symbol(&libs, b"bli_csyr\0").map(|sym| *sym),
            bli_zsyr: get_symbol(&libs, b"bli_zsyr\0").map(|sym| *sym),
            bli_sher2: get_symbol(&libs, b"bli_sher2\0").map(|sym| *sym),
            bli_dher2: get_symbol(&libs, b"bli_dher2\0").map(|sym| *sym),
            bli_cher2: get_symbol(&libs, b"bli_cher2\0").map(|sym| *sym),
            bli_zher2: get_symbol(&libs, b"bli_zher2\0").map(|sym| *sym),
            bli_ssyr2: get_symbol(&libs, b"bli_ssyr2\0").map(|sym| *sym),
            bli_dsyr2: get_symbol(&libs, b"bli_dsyr2\0").map(|sym| *sym),
            bli_csyr2: get_symbol(&libs, b"bli_csyr2\0").map(|sym| *sym),
            bli_zsyr2: get_symbol(&libs, b"bli_zsyr2\0").map(|sym| *sym),
            bli_strmv: get_symbol(&libs, b"bli_strmv\0").map(|sym| *sym),
            bli_dtrmv: get_symbol(&libs, b"bli_dtrmv\0").map(|sym| *sym),
            bli_ctrmv: get_symbol(&libs, b"bli_ctrmv\0").map(|sym| *sym),
            bli_ztrmv: get_symbol(&libs, b"bli_ztrmv\0").map(|sym| *sym),
            bli_strsv: get_symbol(&libs, b"bli_strsv\0").map(|sym| *sym),
            bli_dtrsv: get_symbol(&libs, b"bli_dtrsv\0").map(|sym| *sym),
            bli_ctrsv: get_symbol(&libs, b"bli_ctrsv\0").map(|sym| *sym),
            bli_ztrsv: get_symbol(&libs, b"bli_ztrsv\0").map(|sym| *sym),
            bli_gemv_ex_qfp: get_symbol(&libs, b"bli_gemv_ex_qfp\0").map(|sym| *sym),
            bli_ger_ex_qfp: get_symbol(&libs, b"bli_ger_ex_qfp\0").map(|sym| *sym),
            bli_hemv_ex_qfp: get_symbol(&libs, b"bli_hemv_ex_qfp\0").map(|sym| *sym),
            bli_symv_ex_qfp: get_symbol(&libs, b"bli_symv_ex_qfp\0").map(|sym| *sym),
            bli_her_ex_qfp: get_symbol(&libs, b"bli_her_ex_qfp\0").map(|sym| *sym),
            bli_syr_ex_qfp: get_symbol(&libs, b"bli_syr_ex_qfp\0").map(|sym| *sym),
            bli_her2_ex_qfp: get_symbol(&libs, b"bli_her2_ex_qfp\0").map(|sym| *sym),
            bli_syr2_ex_qfp: get_symbol(&libs, b"bli_syr2_ex_qfp\0").map(|sym| *sym),
            bli_trmv_ex_qfp: get_symbol(&libs, b"bli_trmv_ex_qfp\0").map(|sym| *sym),
            bli_trsv_ex_qfp: get_symbol(&libs, b"bli_trsv_ex_qfp\0").map(|sym| *sym),
            bli_gemv_unb_var1_qfp: get_symbol(&libs, b"bli_gemv_unb_var1_qfp\0").map(|sym| *sym),
            bli_gemv_unb_var2_qfp: get_symbol(&libs, b"bli_gemv_unb_var2_qfp\0").map(|sym| *sym),
            bli_gemv_unf_var1_qfp: get_symbol(&libs, b"bli_gemv_unf_var1_qfp\0").map(|sym| *sym),
            bli_gemv_unf_var2_qfp: get_symbol(&libs, b"bli_gemv_unf_var2_qfp\0").map(|sym| *sym),
            bli_ger_unb_var1_qfp: get_symbol(&libs, b"bli_ger_unb_var1_qfp\0").map(|sym| *sym),
            bli_ger_unb_var2_qfp: get_symbol(&libs, b"bli_ger_unb_var2_qfp\0").map(|sym| *sym),
            bli_hemv_unb_var1_qfp: get_symbol(&libs, b"bli_hemv_unb_var1_qfp\0").map(|sym| *sym),
            bli_hemv_unb_var2_qfp: get_symbol(&libs, b"bli_hemv_unb_var2_qfp\0").map(|sym| *sym),
            bli_hemv_unb_var3_qfp: get_symbol(&libs, b"bli_hemv_unb_var3_qfp\0").map(|sym| *sym),
            bli_hemv_unb_var4_qfp: get_symbol(&libs, b"bli_hemv_unb_var4_qfp\0").map(|sym| *sym),
            bli_hemv_unf_var1_qfp: get_symbol(&libs, b"bli_hemv_unf_var1_qfp\0").map(|sym| *sym),
            bli_hemv_unf_var3_qfp: get_symbol(&libs, b"bli_hemv_unf_var3_qfp\0").map(|sym| *sym),
            bli_hemv_unf_var1a_qfp: get_symbol(&libs, b"bli_hemv_unf_var1a_qfp\0").map(|sym| *sym),
            bli_hemv_unf_var3a_qfp: get_symbol(&libs, b"bli_hemv_unf_var3a_qfp\0").map(|sym| *sym),
            bli_her_unb_var1_qfp: get_symbol(&libs, b"bli_her_unb_var1_qfp\0").map(|sym| *sym),
            bli_her_unb_var2_qfp: get_symbol(&libs, b"bli_her_unb_var2_qfp\0").map(|sym| *sym),
            bli_her2_unb_var1_qfp: get_symbol(&libs, b"bli_her2_unb_var1_qfp\0").map(|sym| *sym),
            bli_her2_unb_var2_qfp: get_symbol(&libs, b"bli_her2_unb_var2_qfp\0").map(|sym| *sym),
            bli_her2_unb_var3_qfp: get_symbol(&libs, b"bli_her2_unb_var3_qfp\0").map(|sym| *sym),
            bli_her2_unb_var4_qfp: get_symbol(&libs, b"bli_her2_unb_var4_qfp\0").map(|sym| *sym),
            bli_her2_unf_var1_qfp: get_symbol(&libs, b"bli_her2_unf_var1_qfp\0").map(|sym| *sym),
            bli_her2_unf_var4_qfp: get_symbol(&libs, b"bli_her2_unf_var4_qfp\0").map(|sym| *sym),
            bli_trmv_unb_var1_qfp: get_symbol(&libs, b"bli_trmv_unb_var1_qfp\0").map(|sym| *sym),
            bli_trmv_unb_var2_qfp: get_symbol(&libs, b"bli_trmv_unb_var2_qfp\0").map(|sym| *sym),
            bli_trmv_unf_var1_qfp: get_symbol(&libs, b"bli_trmv_unf_var1_qfp\0").map(|sym| *sym),
            bli_trmv_unf_var2_qfp: get_symbol(&libs, b"bli_trmv_unf_var2_qfp\0").map(|sym| *sym),
            bli_trsv_unb_var1_qfp: get_symbol(&libs, b"bli_trsv_unb_var1_qfp\0").map(|sym| *sym),
            bli_trsv_unb_var2_qfp: get_symbol(&libs, b"bli_trsv_unb_var2_qfp\0").map(|sym| *sym),
            bli_trsv_unf_var1_qfp: get_symbol(&libs, b"bli_trsv_unf_var1_qfp\0").map(|sym| *sym),
            bli_trsv_unf_var2_qfp: get_symbol(&libs, b"bli_trsv_unf_var2_qfp\0").map(|sym| *sym),
            bli_gemv_blk_var1: get_symbol(&libs, b"bli_gemv_blk_var1\0").map(|sym| *sym),
            bli_gemv_blk_var2: get_symbol(&libs, b"bli_gemv_blk_var2\0").map(|sym| *sym),
            bli_gemv_unb_var1: get_symbol(&libs, b"bli_gemv_unb_var1\0").map(|sym| *sym),
            bli_gemv_unb_var2: get_symbol(&libs, b"bli_gemv_unb_var2\0").map(|sym| *sym),
            bli_gemv_unf_var1: get_symbol(&libs, b"bli_gemv_unf_var1\0").map(|sym| *sym),
            bli_gemv_unf_var2: get_symbol(&libs, b"bli_gemv_unf_var2\0").map(|sym| *sym),
            bli_sgemv_unb_var1: get_symbol(&libs, b"bli_sgemv_unb_var1\0").map(|sym| *sym),
            bli_dgemv_unb_var1: get_symbol(&libs, b"bli_dgemv_unb_var1\0").map(|sym| *sym),
            bli_cgemv_unb_var1: get_symbol(&libs, b"bli_cgemv_unb_var1\0").map(|sym| *sym),
            bli_zgemv_unb_var1: get_symbol(&libs, b"bli_zgemv_unb_var1\0").map(|sym| *sym),
            bli_sgemv_unb_var2: get_symbol(&libs, b"bli_sgemv_unb_var2\0").map(|sym| *sym),
            bli_dgemv_unb_var2: get_symbol(&libs, b"bli_dgemv_unb_var2\0").map(|sym| *sym),
            bli_cgemv_unb_var2: get_symbol(&libs, b"bli_cgemv_unb_var2\0").map(|sym| *sym),
            bli_zgemv_unb_var2: get_symbol(&libs, b"bli_zgemv_unb_var2\0").map(|sym| *sym),
            bli_sgemv_unf_var1: get_symbol(&libs, b"bli_sgemv_unf_var1\0").map(|sym| *sym),
            bli_dgemv_unf_var1: get_symbol(&libs, b"bli_dgemv_unf_var1\0").map(|sym| *sym),
            bli_cgemv_unf_var1: get_symbol(&libs, b"bli_cgemv_unf_var1\0").map(|sym| *sym),
            bli_zgemv_unf_var1: get_symbol(&libs, b"bli_zgemv_unf_var1\0").map(|sym| *sym),
            bli_sgemv_unf_var2: get_symbol(&libs, b"bli_sgemv_unf_var2\0").map(|sym| *sym),
            bli_dgemv_unf_var2: get_symbol(&libs, b"bli_dgemv_unf_var2\0").map(|sym| *sym),
            bli_cgemv_unf_var2: get_symbol(&libs, b"bli_cgemv_unf_var2\0").map(|sym| *sym),
            bli_zgemv_unf_var2: get_symbol(&libs, b"bli_zgemv_unf_var2\0").map(|sym| *sym),
            bli_ger_blk_var1: get_symbol(&libs, b"bli_ger_blk_var1\0").map(|sym| *sym),
            bli_ger_blk_var2: get_symbol(&libs, b"bli_ger_blk_var2\0").map(|sym| *sym),
            bli_ger_unb_var1: get_symbol(&libs, b"bli_ger_unb_var1\0").map(|sym| *sym),
            bli_ger_unb_var2: get_symbol(&libs, b"bli_ger_unb_var2\0").map(|sym| *sym),
            bli_sger_unb_var1: get_symbol(&libs, b"bli_sger_unb_var1\0").map(|sym| *sym),
            bli_dger_unb_var1: get_symbol(&libs, b"bli_dger_unb_var1\0").map(|sym| *sym),
            bli_cger_unb_var1: get_symbol(&libs, b"bli_cger_unb_var1\0").map(|sym| *sym),
            bli_zger_unb_var1: get_symbol(&libs, b"bli_zger_unb_var1\0").map(|sym| *sym),
            bli_sger_unb_var2: get_symbol(&libs, b"bli_sger_unb_var2\0").map(|sym| *sym),
            bli_dger_unb_var2: get_symbol(&libs, b"bli_dger_unb_var2\0").map(|sym| *sym),
            bli_cger_unb_var2: get_symbol(&libs, b"bli_cger_unb_var2\0").map(|sym| *sym),
            bli_zger_unb_var2: get_symbol(&libs, b"bli_zger_unb_var2\0").map(|sym| *sym),
            bli_hemv_blk_var1: get_symbol(&libs, b"bli_hemv_blk_var1\0").map(|sym| *sym),
            bli_hemv_blk_var2: get_symbol(&libs, b"bli_hemv_blk_var2\0").map(|sym| *sym),
            bli_hemv_blk_var3: get_symbol(&libs, b"bli_hemv_blk_var3\0").map(|sym| *sym),
            bli_hemv_blk_var4: get_symbol(&libs, b"bli_hemv_blk_var4\0").map(|sym| *sym),
            bli_hemv_unb_var1: get_symbol(&libs, b"bli_hemv_unb_var1\0").map(|sym| *sym),
            bli_hemv_unb_var2: get_symbol(&libs, b"bli_hemv_unb_var2\0").map(|sym| *sym),
            bli_hemv_unb_var3: get_symbol(&libs, b"bli_hemv_unb_var3\0").map(|sym| *sym),
            bli_hemv_unb_var4: get_symbol(&libs, b"bli_hemv_unb_var4\0").map(|sym| *sym),
            bli_hemv_unf_var1: get_symbol(&libs, b"bli_hemv_unf_var1\0").map(|sym| *sym),
            bli_hemv_unf_var3: get_symbol(&libs, b"bli_hemv_unf_var3\0").map(|sym| *sym),
            bli_hemv_unf_var1a: get_symbol(&libs, b"bli_hemv_unf_var1a\0").map(|sym| *sym),
            bli_hemv_unf_var3a: get_symbol(&libs, b"bli_hemv_unf_var3a\0").map(|sym| *sym),
            bli_shemv_unb_var1: get_symbol(&libs, b"bli_shemv_unb_var1\0").map(|sym| *sym),
            bli_dhemv_unb_var1: get_symbol(&libs, b"bli_dhemv_unb_var1\0").map(|sym| *sym),
            bli_chemv_unb_var1: get_symbol(&libs, b"bli_chemv_unb_var1\0").map(|sym| *sym),
            bli_zhemv_unb_var1: get_symbol(&libs, b"bli_zhemv_unb_var1\0").map(|sym| *sym),
            bli_shemv_unb_var2: get_symbol(&libs, b"bli_shemv_unb_var2\0").map(|sym| *sym),
            bli_dhemv_unb_var2: get_symbol(&libs, b"bli_dhemv_unb_var2\0").map(|sym| *sym),
            bli_chemv_unb_var2: get_symbol(&libs, b"bli_chemv_unb_var2\0").map(|sym| *sym),
            bli_zhemv_unb_var2: get_symbol(&libs, b"bli_zhemv_unb_var2\0").map(|sym| *sym),
            bli_shemv_unb_var3: get_symbol(&libs, b"bli_shemv_unb_var3\0").map(|sym| *sym),
            bli_dhemv_unb_var3: get_symbol(&libs, b"bli_dhemv_unb_var3\0").map(|sym| *sym),
            bli_chemv_unb_var3: get_symbol(&libs, b"bli_chemv_unb_var3\0").map(|sym| *sym),
            bli_zhemv_unb_var3: get_symbol(&libs, b"bli_zhemv_unb_var3\0").map(|sym| *sym),
            bli_shemv_unb_var4: get_symbol(&libs, b"bli_shemv_unb_var4\0").map(|sym| *sym),
            bli_dhemv_unb_var4: get_symbol(&libs, b"bli_dhemv_unb_var4\0").map(|sym| *sym),
            bli_chemv_unb_var4: get_symbol(&libs, b"bli_chemv_unb_var4\0").map(|sym| *sym),
            bli_zhemv_unb_var4: get_symbol(&libs, b"bli_zhemv_unb_var4\0").map(|sym| *sym),
            bli_shemv_unf_var1: get_symbol(&libs, b"bli_shemv_unf_var1\0").map(|sym| *sym),
            bli_dhemv_unf_var1: get_symbol(&libs, b"bli_dhemv_unf_var1\0").map(|sym| *sym),
            bli_chemv_unf_var1: get_symbol(&libs, b"bli_chemv_unf_var1\0").map(|sym| *sym),
            bli_zhemv_unf_var1: get_symbol(&libs, b"bli_zhemv_unf_var1\0").map(|sym| *sym),
            bli_shemv_unf_var3: get_symbol(&libs, b"bli_shemv_unf_var3\0").map(|sym| *sym),
            bli_dhemv_unf_var3: get_symbol(&libs, b"bli_dhemv_unf_var3\0").map(|sym| *sym),
            bli_chemv_unf_var3: get_symbol(&libs, b"bli_chemv_unf_var3\0").map(|sym| *sym),
            bli_zhemv_unf_var3: get_symbol(&libs, b"bli_zhemv_unf_var3\0").map(|sym| *sym),
            bli_shemv_unf_var1a: get_symbol(&libs, b"bli_shemv_unf_var1a\0").map(|sym| *sym),
            bli_dhemv_unf_var1a: get_symbol(&libs, b"bli_dhemv_unf_var1a\0").map(|sym| *sym),
            bli_chemv_unf_var1a: get_symbol(&libs, b"bli_chemv_unf_var1a\0").map(|sym| *sym),
            bli_zhemv_unf_var1a: get_symbol(&libs, b"bli_zhemv_unf_var1a\0").map(|sym| *sym),
            bli_shemv_unf_var3a: get_symbol(&libs, b"bli_shemv_unf_var3a\0").map(|sym| *sym),
            bli_dhemv_unf_var3a: get_symbol(&libs, b"bli_dhemv_unf_var3a\0").map(|sym| *sym),
            bli_chemv_unf_var3a: get_symbol(&libs, b"bli_chemv_unf_var3a\0").map(|sym| *sym),
            bli_zhemv_unf_var3a: get_symbol(&libs, b"bli_zhemv_unf_var3a\0").map(|sym| *sym),
            bli_her_blk_var1: get_symbol(&libs, b"bli_her_blk_var1\0").map(|sym| *sym),
            bli_her_blk_var2: get_symbol(&libs, b"bli_her_blk_var2\0").map(|sym| *sym),
            bli_her_unb_var1: get_symbol(&libs, b"bli_her_unb_var1\0").map(|sym| *sym),
            bli_her_unb_var2: get_symbol(&libs, b"bli_her_unb_var2\0").map(|sym| *sym),
            bli_sher_unb_var1: get_symbol(&libs, b"bli_sher_unb_var1\0").map(|sym| *sym),
            bli_dher_unb_var1: get_symbol(&libs, b"bli_dher_unb_var1\0").map(|sym| *sym),
            bli_cher_unb_var1: get_symbol(&libs, b"bli_cher_unb_var1\0").map(|sym| *sym),
            bli_zher_unb_var1: get_symbol(&libs, b"bli_zher_unb_var1\0").map(|sym| *sym),
            bli_sher_unb_var2: get_symbol(&libs, b"bli_sher_unb_var2\0").map(|sym| *sym),
            bli_dher_unb_var2: get_symbol(&libs, b"bli_dher_unb_var2\0").map(|sym| *sym),
            bli_cher_unb_var2: get_symbol(&libs, b"bli_cher_unb_var2\0").map(|sym| *sym),
            bli_zher_unb_var2: get_symbol(&libs, b"bli_zher_unb_var2\0").map(|sym| *sym),
            bli_her2_blk_var1: get_symbol(&libs, b"bli_her2_blk_var1\0").map(|sym| *sym),
            bli_her2_blk_var2: get_symbol(&libs, b"bli_her2_blk_var2\0").map(|sym| *sym),
            bli_her2_blk_var3: get_symbol(&libs, b"bli_her2_blk_var3\0").map(|sym| *sym),
            bli_her2_blk_var4: get_symbol(&libs, b"bli_her2_blk_var4\0").map(|sym| *sym),
            bli_her2_unb_var1: get_symbol(&libs, b"bli_her2_unb_var1\0").map(|sym| *sym),
            bli_her2_unb_var2: get_symbol(&libs, b"bli_her2_unb_var2\0").map(|sym| *sym),
            bli_her2_unb_var3: get_symbol(&libs, b"bli_her2_unb_var3\0").map(|sym| *sym),
            bli_her2_unb_var4: get_symbol(&libs, b"bli_her2_unb_var4\0").map(|sym| *sym),
            bli_her2_unf_var1: get_symbol(&libs, b"bli_her2_unf_var1\0").map(|sym| *sym),
            bli_her2_unf_var4: get_symbol(&libs, b"bli_her2_unf_var4\0").map(|sym| *sym),
            bli_sher2_unb_var1: get_symbol(&libs, b"bli_sher2_unb_var1\0").map(|sym| *sym),
            bli_dher2_unb_var1: get_symbol(&libs, b"bli_dher2_unb_var1\0").map(|sym| *sym),
            bli_cher2_unb_var1: get_symbol(&libs, b"bli_cher2_unb_var1\0").map(|sym| *sym),
            bli_zher2_unb_var1: get_symbol(&libs, b"bli_zher2_unb_var1\0").map(|sym| *sym),
            bli_sher2_unb_var2: get_symbol(&libs, b"bli_sher2_unb_var2\0").map(|sym| *sym),
            bli_dher2_unb_var2: get_symbol(&libs, b"bli_dher2_unb_var2\0").map(|sym| *sym),
            bli_cher2_unb_var2: get_symbol(&libs, b"bli_cher2_unb_var2\0").map(|sym| *sym),
            bli_zher2_unb_var2: get_symbol(&libs, b"bli_zher2_unb_var2\0").map(|sym| *sym),
            bli_sher2_unb_var3: get_symbol(&libs, b"bli_sher2_unb_var3\0").map(|sym| *sym),
            bli_dher2_unb_var3: get_symbol(&libs, b"bli_dher2_unb_var3\0").map(|sym| *sym),
            bli_cher2_unb_var3: get_symbol(&libs, b"bli_cher2_unb_var3\0").map(|sym| *sym),
            bli_zher2_unb_var3: get_symbol(&libs, b"bli_zher2_unb_var3\0").map(|sym| *sym),
            bli_sher2_unb_var4: get_symbol(&libs, b"bli_sher2_unb_var4\0").map(|sym| *sym),
            bli_dher2_unb_var4: get_symbol(&libs, b"bli_dher2_unb_var4\0").map(|sym| *sym),
            bli_cher2_unb_var4: get_symbol(&libs, b"bli_cher2_unb_var4\0").map(|sym| *sym),
            bli_zher2_unb_var4: get_symbol(&libs, b"bli_zher2_unb_var4\0").map(|sym| *sym),
            bli_sher2_unf_var1: get_symbol(&libs, b"bli_sher2_unf_var1\0").map(|sym| *sym),
            bli_dher2_unf_var1: get_symbol(&libs, b"bli_dher2_unf_var1\0").map(|sym| *sym),
            bli_cher2_unf_var1: get_symbol(&libs, b"bli_cher2_unf_var1\0").map(|sym| *sym),
            bli_zher2_unf_var1: get_symbol(&libs, b"bli_zher2_unf_var1\0").map(|sym| *sym),
            bli_sher2_unf_var4: get_symbol(&libs, b"bli_sher2_unf_var4\0").map(|sym| *sym),
            bli_dher2_unf_var4: get_symbol(&libs, b"bli_dher2_unf_var4\0").map(|sym| *sym),
            bli_cher2_unf_var4: get_symbol(&libs, b"bli_cher2_unf_var4\0").map(|sym| *sym),
            bli_zher2_unf_var4: get_symbol(&libs, b"bli_zher2_unf_var4\0").map(|sym| *sym),
            bli_trmv_l_blk_var1: get_symbol(&libs, b"bli_trmv_l_blk_var1\0").map(|sym| *sym),
            bli_trmv_l_blk_var2: get_symbol(&libs, b"bli_trmv_l_blk_var2\0").map(|sym| *sym),
            bli_trmv_u_blk_var1: get_symbol(&libs, b"bli_trmv_u_blk_var1\0").map(|sym| *sym),
            bli_trmv_u_blk_var2: get_symbol(&libs, b"bli_trmv_u_blk_var2\0").map(|sym| *sym),
            bli_trmv_unb_var1: get_symbol(&libs, b"bli_trmv_unb_var1\0").map(|sym| *sym),
            bli_trmv_unb_var2: get_symbol(&libs, b"bli_trmv_unb_var2\0").map(|sym| *sym),
            bli_trmv_unf_var1: get_symbol(&libs, b"bli_trmv_unf_var1\0").map(|sym| *sym),
            bli_trmv_unf_var2: get_symbol(&libs, b"bli_trmv_unf_var2\0").map(|sym| *sym),
            bli_strmv_unb_var1: get_symbol(&libs, b"bli_strmv_unb_var1\0").map(|sym| *sym),
            bli_dtrmv_unb_var1: get_symbol(&libs, b"bli_dtrmv_unb_var1\0").map(|sym| *sym),
            bli_ctrmv_unb_var1: get_symbol(&libs, b"bli_ctrmv_unb_var1\0").map(|sym| *sym),
            bli_ztrmv_unb_var1: get_symbol(&libs, b"bli_ztrmv_unb_var1\0").map(|sym| *sym),
            bli_strmv_unb_var2: get_symbol(&libs, b"bli_strmv_unb_var2\0").map(|sym| *sym),
            bli_dtrmv_unb_var2: get_symbol(&libs, b"bli_dtrmv_unb_var2\0").map(|sym| *sym),
            bli_ctrmv_unb_var2: get_symbol(&libs, b"bli_ctrmv_unb_var2\0").map(|sym| *sym),
            bli_ztrmv_unb_var2: get_symbol(&libs, b"bli_ztrmv_unb_var2\0").map(|sym| *sym),
            bli_strmv_unf_var1: get_symbol(&libs, b"bli_strmv_unf_var1\0").map(|sym| *sym),
            bli_dtrmv_unf_var1: get_symbol(&libs, b"bli_dtrmv_unf_var1\0").map(|sym| *sym),
            bli_ctrmv_unf_var1: get_symbol(&libs, b"bli_ctrmv_unf_var1\0").map(|sym| *sym),
            bli_ztrmv_unf_var1: get_symbol(&libs, b"bli_ztrmv_unf_var1\0").map(|sym| *sym),
            bli_strmv_unf_var2: get_symbol(&libs, b"bli_strmv_unf_var2\0").map(|sym| *sym),
            bli_dtrmv_unf_var2: get_symbol(&libs, b"bli_dtrmv_unf_var2\0").map(|sym| *sym),
            bli_ctrmv_unf_var2: get_symbol(&libs, b"bli_ctrmv_unf_var2\0").map(|sym| *sym),
            bli_ztrmv_unf_var2: get_symbol(&libs, b"bli_ztrmv_unf_var2\0").map(|sym| *sym),
            bli_trsv_l_blk_var1: get_symbol(&libs, b"bli_trsv_l_blk_var1\0").map(|sym| *sym),
            bli_trsv_l_blk_var2: get_symbol(&libs, b"bli_trsv_l_blk_var2\0").map(|sym| *sym),
            bli_trsv_u_blk_var1: get_symbol(&libs, b"bli_trsv_u_blk_var1\0").map(|sym| *sym),
            bli_trsv_u_blk_var2: get_symbol(&libs, b"bli_trsv_u_blk_var2\0").map(|sym| *sym),
            bli_trsv_unb_var1: get_symbol(&libs, b"bli_trsv_unb_var1\0").map(|sym| *sym),
            bli_trsv_unb_var2: get_symbol(&libs, b"bli_trsv_unb_var2\0").map(|sym| *sym),
            bli_trsv_unf_var1: get_symbol(&libs, b"bli_trsv_unf_var1\0").map(|sym| *sym),
            bli_trsv_unf_var2: get_symbol(&libs, b"bli_trsv_unf_var2\0").map(|sym| *sym),
            bli_strsv_unb_var1: get_symbol(&libs, b"bli_strsv_unb_var1\0").map(|sym| *sym),
            bli_dtrsv_unb_var1: get_symbol(&libs, b"bli_dtrsv_unb_var1\0").map(|sym| *sym),
            bli_ctrsv_unb_var1: get_symbol(&libs, b"bli_ctrsv_unb_var1\0").map(|sym| *sym),
            bli_ztrsv_unb_var1: get_symbol(&libs, b"bli_ztrsv_unb_var1\0").map(|sym| *sym),
            bli_strsv_unb_var2: get_symbol(&libs, b"bli_strsv_unb_var2\0").map(|sym| *sym),
            bli_dtrsv_unb_var2: get_symbol(&libs, b"bli_dtrsv_unb_var2\0").map(|sym| *sym),
            bli_ctrsv_unb_var2: get_symbol(&libs, b"bli_ctrsv_unb_var2\0").map(|sym| *sym),
            bli_ztrsv_unb_var2: get_symbol(&libs, b"bli_ztrsv_unb_var2\0").map(|sym| *sym),
            bli_strsv_unf_var1: get_symbol(&libs, b"bli_strsv_unf_var1\0").map(|sym| *sym),
            bli_dtrsv_unf_var1: get_symbol(&libs, b"bli_dtrsv_unf_var1\0").map(|sym| *sym),
            bli_ctrsv_unf_var1: get_symbol(&libs, b"bli_ctrsv_unf_var1\0").map(|sym| *sym),
            bli_ztrsv_unf_var1: get_symbol(&libs, b"bli_ztrsv_unf_var1\0").map(|sym| *sym),
            bli_strsv_unf_var2: get_symbol(&libs, b"bli_strsv_unf_var2\0").map(|sym| *sym),
            bli_dtrsv_unf_var2: get_symbol(&libs, b"bli_dtrsv_unf_var2\0").map(|sym| *sym),
            bli_ctrsv_unf_var2: get_symbol(&libs, b"bli_ctrsv_unf_var2\0").map(|sym| *sym),
            bli_ztrsv_unf_var2: get_symbol(&libs, b"bli_ztrsv_unf_var2\0").map(|sym| *sym),
            bli_l3_cntl_create_if: get_symbol(&libs, b"bli_l3_cntl_create_if\0").map(|sym| *sym),
            bli_l3_cntl_free: get_symbol(&libs, b"bli_l3_cntl_free\0").map(|sym| *sym),
            bli_gemm_check: get_symbol(&libs, b"bli_gemm_check\0").map(|sym| *sym),
            bli_gemmt_check: get_symbol(&libs, b"bli_gemmt_check\0").map(|sym| *sym),
            bli_her2k_check: get_symbol(&libs, b"bli_her2k_check\0").map(|sym| *sym),
            bli_syr2k_check: get_symbol(&libs, b"bli_syr2k_check\0").map(|sym| *sym),
            bli_hemm_check: get_symbol(&libs, b"bli_hemm_check\0").map(|sym| *sym),
            bli_symm_check: get_symbol(&libs, b"bli_symm_check\0").map(|sym| *sym),
            bli_trmm3_check: get_symbol(&libs, b"bli_trmm3_check\0").map(|sym| *sym),
            bli_herk_check: get_symbol(&libs, b"bli_herk_check\0").map(|sym| *sym),
            bli_syrk_check: get_symbol(&libs, b"bli_syrk_check\0").map(|sym| *sym),
            bli_trmm_check: get_symbol(&libs, b"bli_trmm_check\0").map(|sym| *sym),
            bli_trsm_check: get_symbol(&libs, b"bli_trsm_check\0").map(|sym| *sym),
            bli_gemm_basic_check: get_symbol(&libs, b"bli_gemm_basic_check\0").map(|sym| *sym),
            bli_gemmt_basic_check: get_symbol(&libs, b"bli_gemmt_basic_check\0").map(|sym| *sym),
            bli_hemm_basic_check: get_symbol(&libs, b"bli_hemm_basic_check\0").map(|sym| *sym),
            bli_herk_basic_check: get_symbol(&libs, b"bli_herk_basic_check\0").map(|sym| *sym),
            bli_her2k_basic_check: get_symbol(&libs, b"bli_her2k_basic_check\0").map(|sym| *sym),
            bli_l3_basic_check: get_symbol(&libs, b"bli_l3_basic_check\0").map(|sym| *sym),
            bli_l3_determine_kc: get_symbol(&libs, b"bli_l3_determine_kc\0").map(|sym| *sym),
            bli_gemm_determine_kc: get_symbol(&libs, b"bli_gemm_determine_kc\0").map(|sym| *sym),
            bli_herk_determine_kc: get_symbol(&libs, b"bli_herk_determine_kc\0").map(|sym| *sym),
            bli_trmm_determine_kc: get_symbol(&libs, b"bli_trmm_determine_kc\0").map(|sym| *sym),
            bli_trsm_determine_kc: get_symbol(&libs, b"bli_trsm_determine_kc\0").map(|sym| *sym),
            bli_gemm_determine_kc_f: get_symbol(&libs, b"bli_gemm_determine_kc_f\0")
                .map(|sym| *sym),
            bli_gemm_determine_kc_b: get_symbol(&libs, b"bli_gemm_determine_kc_b\0")
                .map(|sym| *sym),
            bli_herk_determine_kc_f: get_symbol(&libs, b"bli_herk_determine_kc_f\0")
                .map(|sym| *sym),
            bli_herk_determine_kc_b: get_symbol(&libs, b"bli_herk_determine_kc_b\0")
                .map(|sym| *sym),
            bli_trmm_determine_kc_f: get_symbol(&libs, b"bli_trmm_determine_kc_f\0")
                .map(|sym| *sym),
            bli_trmm_determine_kc_b: get_symbol(&libs, b"bli_trmm_determine_kc_b\0")
                .map(|sym| *sym),
            bli_trsm_determine_kc_f: get_symbol(&libs, b"bli_trsm_determine_kc_f\0")
                .map(|sym| *sym),
            bli_trsm_determine_kc_b: get_symbol(&libs, b"bli_trsm_determine_kc_b\0")
                .map(|sym| *sym),
            bli_l3_direct: get_symbol(&libs, b"bli_l3_direct\0").map(|sym| *sym),
            bli_gemm_direct: get_symbol(&libs, b"bli_gemm_direct\0").map(|sym| *sym),
            bli_herk_direct: get_symbol(&libs, b"bli_herk_direct\0").map(|sym| *sym),
            bli_trmm_direct: get_symbol(&libs, b"bli_trmm_direct\0").map(|sym| *sym),
            bli_trsm_direct: get_symbol(&libs, b"bli_trsm_direct\0").map(|sym| *sym),
            bli_l3_prune_unref_mparts_m: get_symbol(&libs, b"bli_l3_prune_unref_mparts_m\0")
                .map(|sym| *sym),
            bli_l3_prune_unref_mparts_n: get_symbol(&libs, b"bli_l3_prune_unref_mparts_n\0")
                .map(|sym| *sym),
            bli_l3_prune_unref_mparts_k: get_symbol(&libs, b"bli_l3_prune_unref_mparts_k\0")
                .map(|sym| *sym),
            bli_gemm_prune_unref_mparts_m: get_symbol(&libs, b"bli_gemm_prune_unref_mparts_m\0")
                .map(|sym| *sym),
            bli_gemm_prune_unref_mparts_n: get_symbol(&libs, b"bli_gemm_prune_unref_mparts_n\0")
                .map(|sym| *sym),
            bli_gemm_prune_unref_mparts_k: get_symbol(&libs, b"bli_gemm_prune_unref_mparts_k\0")
                .map(|sym| *sym),
            bli_herk_prune_unref_mparts_m: get_symbol(&libs, b"bli_herk_prune_unref_mparts_m\0")
                .map(|sym| *sym),
            bli_herk_prune_unref_mparts_n: get_symbol(&libs, b"bli_herk_prune_unref_mparts_n\0")
                .map(|sym| *sym),
            bli_herk_prune_unref_mparts_k: get_symbol(&libs, b"bli_herk_prune_unref_mparts_k\0")
                .map(|sym| *sym),
            bli_trmm_prune_unref_mparts_m: get_symbol(&libs, b"bli_trmm_prune_unref_mparts_m\0")
                .map(|sym| *sym),
            bli_trmm_prune_unref_mparts_n: get_symbol(&libs, b"bli_trmm_prune_unref_mparts_n\0")
                .map(|sym| *sym),
            bli_trmm_prune_unref_mparts_k: get_symbol(&libs, b"bli_trmm_prune_unref_mparts_k\0")
                .map(|sym| *sym),
            bli_trsm_prune_unref_mparts_m: get_symbol(&libs, b"bli_trsm_prune_unref_mparts_m\0")
                .map(|sym| *sym),
            bli_trsm_prune_unref_mparts_n: get_symbol(&libs, b"bli_trsm_prune_unref_mparts_n\0")
                .map(|sym| *sym),
            bli_trsm_prune_unref_mparts_k: get_symbol(&libs, b"bli_trsm_prune_unref_mparts_k\0")
                .map(|sym| *sym),
            bli_l3_packm: get_symbol(&libs, b"bli_l3_packm\0").map(|sym| *sym),
            bli_l3_set_schemas: get_symbol(&libs, b"bli_l3_set_schemas\0").map(|sym| *sym),
            bli_gemm: get_symbol(&libs, b"bli_gemm\0").map(|sym| *sym),
            bli_gemmt: get_symbol(&libs, b"bli_gemmt\0").map(|sym| *sym),
            bli_her2k: get_symbol(&libs, b"bli_her2k\0").map(|sym| *sym),
            bli_syr2k: get_symbol(&libs, b"bli_syr2k\0").map(|sym| *sym),
            bli_hemm: get_symbol(&libs, b"bli_hemm\0").map(|sym| *sym),
            bli_symm: get_symbol(&libs, b"bli_symm\0").map(|sym| *sym),
            bli_trmm3: get_symbol(&libs, b"bli_trmm3\0").map(|sym| *sym),
            bli_herk: get_symbol(&libs, b"bli_herk\0").map(|sym| *sym),
            bli_syrk: get_symbol(&libs, b"bli_syrk\0").map(|sym| *sym),
            bli_trmm: get_symbol(&libs, b"bli_trmm\0").map(|sym| *sym),
            bli_trsm: get_symbol(&libs, b"bli_trsm\0").map(|sym| *sym),
            bli_gemm_ex: get_symbol(&libs, b"bli_gemm_ex\0").map(|sym| *sym),
            bli_gemmt_ex: get_symbol(&libs, b"bli_gemmt_ex\0").map(|sym| *sym),
            bli_her2k_ex: get_symbol(&libs, b"bli_her2k_ex\0").map(|sym| *sym),
            bli_syr2k_ex: get_symbol(&libs, b"bli_syr2k_ex\0").map(|sym| *sym),
            bli_hemm_ex: get_symbol(&libs, b"bli_hemm_ex\0").map(|sym| *sym),
            bli_symm_ex: get_symbol(&libs, b"bli_symm_ex\0").map(|sym| *sym),
            bli_trmm3_ex: get_symbol(&libs, b"bli_trmm3_ex\0").map(|sym| *sym),
            bli_herk_ex: get_symbol(&libs, b"bli_herk_ex\0").map(|sym| *sym),
            bli_syrk_ex: get_symbol(&libs, b"bli_syrk_ex\0").map(|sym| *sym),
            bli_trmm_ex: get_symbol(&libs, b"bli_trmm_ex\0").map(|sym| *sym),
            bli_trsm_ex: get_symbol(&libs, b"bli_trsm_ex\0").map(|sym| *sym),
            bli_sgemm: get_symbol(&libs, b"bli_sgemm\0").map(|sym| *sym),
            bli_dgemm: get_symbol(&libs, b"bli_dgemm\0").map(|sym| *sym),
            bli_cgemm: get_symbol(&libs, b"bli_cgemm\0").map(|sym| *sym),
            bli_zgemm: get_symbol(&libs, b"bli_zgemm\0").map(|sym| *sym),
            bli_shemm: get_symbol(&libs, b"bli_shemm\0").map(|sym| *sym),
            bli_dhemm: get_symbol(&libs, b"bli_dhemm\0").map(|sym| *sym),
            bli_chemm: get_symbol(&libs, b"bli_chemm\0").map(|sym| *sym),
            bli_zhemm: get_symbol(&libs, b"bli_zhemm\0").map(|sym| *sym),
            bli_ssymm: get_symbol(&libs, b"bli_ssymm\0").map(|sym| *sym),
            bli_dsymm: get_symbol(&libs, b"bli_dsymm\0").map(|sym| *sym),
            bli_csymm: get_symbol(&libs, b"bli_csymm\0").map(|sym| *sym),
            bli_zsymm: get_symbol(&libs, b"bli_zsymm\0").map(|sym| *sym),
            bli_sherk: get_symbol(&libs, b"bli_sherk\0").map(|sym| *sym),
            bli_dherk: get_symbol(&libs, b"bli_dherk\0").map(|sym| *sym),
            bli_cherk: get_symbol(&libs, b"bli_cherk\0").map(|sym| *sym),
            bli_zherk: get_symbol(&libs, b"bli_zherk\0").map(|sym| *sym),
            bli_sher2k: get_symbol(&libs, b"bli_sher2k\0").map(|sym| *sym),
            bli_dher2k: get_symbol(&libs, b"bli_dher2k\0").map(|sym| *sym),
            bli_cher2k: get_symbol(&libs, b"bli_cher2k\0").map(|sym| *sym),
            bli_zher2k: get_symbol(&libs, b"bli_zher2k\0").map(|sym| *sym),
            bli_ssyrk: get_symbol(&libs, b"bli_ssyrk\0").map(|sym| *sym),
            bli_dsyrk: get_symbol(&libs, b"bli_dsyrk\0").map(|sym| *sym),
            bli_csyrk: get_symbol(&libs, b"bli_csyrk\0").map(|sym| *sym),
            bli_zsyrk: get_symbol(&libs, b"bli_zsyrk\0").map(|sym| *sym),
            bli_sgemmt: get_symbol(&libs, b"bli_sgemmt\0").map(|sym| *sym),
            bli_dgemmt: get_symbol(&libs, b"bli_dgemmt\0").map(|sym| *sym),
            bli_cgemmt: get_symbol(&libs, b"bli_cgemmt\0").map(|sym| *sym),
            bli_zgemmt: get_symbol(&libs, b"bli_zgemmt\0").map(|sym| *sym),
            bli_ssyr2k: get_symbol(&libs, b"bli_ssyr2k\0").map(|sym| *sym),
            bli_dsyr2k: get_symbol(&libs, b"bli_dsyr2k\0").map(|sym| *sym),
            bli_csyr2k: get_symbol(&libs, b"bli_csyr2k\0").map(|sym| *sym),
            bli_zsyr2k: get_symbol(&libs, b"bli_zsyr2k\0").map(|sym| *sym),
            bli_strmm3: get_symbol(&libs, b"bli_strmm3\0").map(|sym| *sym),
            bli_dtrmm3: get_symbol(&libs, b"bli_dtrmm3\0").map(|sym| *sym),
            bli_ctrmm3: get_symbol(&libs, b"bli_ctrmm3\0").map(|sym| *sym),
            bli_ztrmm3: get_symbol(&libs, b"bli_ztrmm3\0").map(|sym| *sym),
            bli_strmm: get_symbol(&libs, b"bli_strmm\0").map(|sym| *sym),
            bli_dtrmm: get_symbol(&libs, b"bli_dtrmm\0").map(|sym| *sym),
            bli_ctrmm: get_symbol(&libs, b"bli_ctrmm\0").map(|sym| *sym),
            bli_ztrmm: get_symbol(&libs, b"bli_ztrmm\0").map(|sym| *sym),
            bli_strsm: get_symbol(&libs, b"bli_strsm\0").map(|sym| *sym),
            bli_dtrsm: get_symbol(&libs, b"bli_dtrsm\0").map(|sym| *sym),
            bli_ctrsm: get_symbol(&libs, b"bli_ctrsm\0").map(|sym| *sym),
            bli_ztrsm: get_symbol(&libs, b"bli_ztrsm\0").map(|sym| *sym),
            bli_sgemm_ex: get_symbol(&libs, b"bli_sgemm_ex\0").map(|sym| *sym),
            bli_dgemm_ex: get_symbol(&libs, b"bli_dgemm_ex\0").map(|sym| *sym),
            bli_cgemm_ex: get_symbol(&libs, b"bli_cgemm_ex\0").map(|sym| *sym),
            bli_zgemm_ex: get_symbol(&libs, b"bli_zgemm_ex\0").map(|sym| *sym),
            bli_shemm_ex: get_symbol(&libs, b"bli_shemm_ex\0").map(|sym| *sym),
            bli_dhemm_ex: get_symbol(&libs, b"bli_dhemm_ex\0").map(|sym| *sym),
            bli_chemm_ex: get_symbol(&libs, b"bli_chemm_ex\0").map(|sym| *sym),
            bli_zhemm_ex: get_symbol(&libs, b"bli_zhemm_ex\0").map(|sym| *sym),
            bli_ssymm_ex: get_symbol(&libs, b"bli_ssymm_ex\0").map(|sym| *sym),
            bli_dsymm_ex: get_symbol(&libs, b"bli_dsymm_ex\0").map(|sym| *sym),
            bli_csymm_ex: get_symbol(&libs, b"bli_csymm_ex\0").map(|sym| *sym),
            bli_zsymm_ex: get_symbol(&libs, b"bli_zsymm_ex\0").map(|sym| *sym),
            bli_sherk_ex: get_symbol(&libs, b"bli_sherk_ex\0").map(|sym| *sym),
            bli_dherk_ex: get_symbol(&libs, b"bli_dherk_ex\0").map(|sym| *sym),
            bli_cherk_ex: get_symbol(&libs, b"bli_cherk_ex\0").map(|sym| *sym),
            bli_zherk_ex: get_symbol(&libs, b"bli_zherk_ex\0").map(|sym| *sym),
            bli_sher2k_ex: get_symbol(&libs, b"bli_sher2k_ex\0").map(|sym| *sym),
            bli_dher2k_ex: get_symbol(&libs, b"bli_dher2k_ex\0").map(|sym| *sym),
            bli_cher2k_ex: get_symbol(&libs, b"bli_cher2k_ex\0").map(|sym| *sym),
            bli_zher2k_ex: get_symbol(&libs, b"bli_zher2k_ex\0").map(|sym| *sym),
            bli_ssyrk_ex: get_symbol(&libs, b"bli_ssyrk_ex\0").map(|sym| *sym),
            bli_dsyrk_ex: get_symbol(&libs, b"bli_dsyrk_ex\0").map(|sym| *sym),
            bli_csyrk_ex: get_symbol(&libs, b"bli_csyrk_ex\0").map(|sym| *sym),
            bli_zsyrk_ex: get_symbol(&libs, b"bli_zsyrk_ex\0").map(|sym| *sym),
            bli_sgemmt_ex: get_symbol(&libs, b"bli_sgemmt_ex\0").map(|sym| *sym),
            bli_dgemmt_ex: get_symbol(&libs, b"bli_dgemmt_ex\0").map(|sym| *sym),
            bli_cgemmt_ex: get_symbol(&libs, b"bli_cgemmt_ex\0").map(|sym| *sym),
            bli_zgemmt_ex: get_symbol(&libs, b"bli_zgemmt_ex\0").map(|sym| *sym),
            bli_ssyr2k_ex: get_symbol(&libs, b"bli_ssyr2k_ex\0").map(|sym| *sym),
            bli_dsyr2k_ex: get_symbol(&libs, b"bli_dsyr2k_ex\0").map(|sym| *sym),
            bli_csyr2k_ex: get_symbol(&libs, b"bli_csyr2k_ex\0").map(|sym| *sym),
            bli_zsyr2k_ex: get_symbol(&libs, b"bli_zsyr2k_ex\0").map(|sym| *sym),
            bli_strmm3_ex: get_symbol(&libs, b"bli_strmm3_ex\0").map(|sym| *sym),
            bli_dtrmm3_ex: get_symbol(&libs, b"bli_dtrmm3_ex\0").map(|sym| *sym),
            bli_ctrmm3_ex: get_symbol(&libs, b"bli_ctrmm3_ex\0").map(|sym| *sym),
            bli_ztrmm3_ex: get_symbol(&libs, b"bli_ztrmm3_ex\0").map(|sym| *sym),
            bli_strmm_ex: get_symbol(&libs, b"bli_strmm_ex\0").map(|sym| *sym),
            bli_dtrmm_ex: get_symbol(&libs, b"bli_dtrmm_ex\0").map(|sym| *sym),
            bli_ctrmm_ex: get_symbol(&libs, b"bli_ctrmm_ex\0").map(|sym| *sym),
            bli_ztrmm_ex: get_symbol(&libs, b"bli_ztrmm_ex\0").map(|sym| *sym),
            bli_strsm_ex: get_symbol(&libs, b"bli_strsm_ex\0").map(|sym| *sym),
            bli_dtrsm_ex: get_symbol(&libs, b"bli_dtrsm_ex\0").map(|sym| *sym),
            bli_ctrsm_ex: get_symbol(&libs, b"bli_ctrsm_ex\0").map(|sym| *sym),
            bli_ztrsm_ex: get_symbol(&libs, b"bli_ztrsm_ex\0").map(|sym| *sym),
            bli_gemmsup: get_symbol(&libs, b"bli_gemmsup\0").map(|sym| *sym),
            bli_gemmtsup: get_symbol(&libs, b"bli_gemmtsup\0").map(|sym| *sym),
            bli_syrksup: get_symbol(&libs, b"bli_syrksup\0").map(|sym| *sym),
            bli_gemmsup_ref: get_symbol(&libs, b"bli_gemmsup_ref\0").map(|sym| *sym),
            bli_gemmtsup_ref: get_symbol(&libs, b"bli_gemmtsup_ref\0").map(|sym| *sym),
            bli_gemmsup_int: get_symbol(&libs, b"bli_gemmsup_int\0").map(|sym| *sym),
            bli_gemmtsup_int: get_symbol(&libs, b"bli_gemmtsup_int\0").map(|sym| *sym),
            bli_gemmsup_ref_var1: get_symbol(&libs, b"bli_gemmsup_ref_var1\0").map(|sym| *sym),
            bli_gemmsup_ref_var2: get_symbol(&libs, b"bli_gemmsup_ref_var2\0").map(|sym| *sym),
            bli_gemmsup_ref_var1n: get_symbol(&libs, b"bli_gemmsup_ref_var1n\0").map(|sym| *sym),
            bli_gemmsup_ref_var2m: get_symbol(&libs, b"bli_gemmsup_ref_var2m\0").map(|sym| *sym),
            bli_sgemmsup_ref_var1: get_symbol(&libs, b"bli_sgemmsup_ref_var1\0").map(|sym| *sym),
            bli_dgemmsup_ref_var1: get_symbol(&libs, b"bli_dgemmsup_ref_var1\0").map(|sym| *sym),
            bli_cgemmsup_ref_var1: get_symbol(&libs, b"bli_cgemmsup_ref_var1\0").map(|sym| *sym),
            bli_zgemmsup_ref_var1: get_symbol(&libs, b"bli_zgemmsup_ref_var1\0").map(|sym| *sym),
            bli_sgemmsup_ref_var2: get_symbol(&libs, b"bli_sgemmsup_ref_var2\0").map(|sym| *sym),
            bli_dgemmsup_ref_var2: get_symbol(&libs, b"bli_dgemmsup_ref_var2\0").map(|sym| *sym),
            bli_cgemmsup_ref_var2: get_symbol(&libs, b"bli_cgemmsup_ref_var2\0").map(|sym| *sym),
            bli_zgemmsup_ref_var2: get_symbol(&libs, b"bli_zgemmsup_ref_var2\0").map(|sym| *sym),
            bli_sgemmsup_ref_var1n: get_symbol(&libs, b"bli_sgemmsup_ref_var1n\0").map(|sym| *sym),
            bli_dgemmsup_ref_var1n: get_symbol(&libs, b"bli_dgemmsup_ref_var1n\0").map(|sym| *sym),
            bli_cgemmsup_ref_var1n: get_symbol(&libs, b"bli_cgemmsup_ref_var1n\0").map(|sym| *sym),
            bli_zgemmsup_ref_var1n: get_symbol(&libs, b"bli_zgemmsup_ref_var1n\0").map(|sym| *sym),
            bli_sgemmsup_ref_var2m: get_symbol(&libs, b"bli_sgemmsup_ref_var2m\0").map(|sym| *sym),
            bli_dgemmsup_ref_var2m: get_symbol(&libs, b"bli_dgemmsup_ref_var2m\0").map(|sym| *sym),
            bli_cgemmsup_ref_var2m: get_symbol(&libs, b"bli_cgemmsup_ref_var2m\0").map(|sym| *sym),
            bli_zgemmsup_ref_var2m: get_symbol(&libs, b"bli_zgemmsup_ref_var2m\0").map(|sym| *sym),
            bli_spackm_sup_init_mem_a: get_symbol(&libs, b"bli_spackm_sup_init_mem_a\0")
                .map(|sym| *sym),
            bli_dpackm_sup_init_mem_a: get_symbol(&libs, b"bli_dpackm_sup_init_mem_a\0")
                .map(|sym| *sym),
            bli_cpackm_sup_init_mem_a: get_symbol(&libs, b"bli_cpackm_sup_init_mem_a\0")
                .map(|sym| *sym),
            bli_zpackm_sup_init_mem_a: get_symbol(&libs, b"bli_zpackm_sup_init_mem_a\0")
                .map(|sym| *sym),
            bli_spackm_sup_finalize_mem_a: get_symbol(&libs, b"bli_spackm_sup_finalize_mem_a\0")
                .map(|sym| *sym),
            bli_dpackm_sup_finalize_mem_a: get_symbol(&libs, b"bli_dpackm_sup_finalize_mem_a\0")
                .map(|sym| *sym),
            bli_cpackm_sup_finalize_mem_a: get_symbol(&libs, b"bli_cpackm_sup_finalize_mem_a\0")
                .map(|sym| *sym),
            bli_zpackm_sup_finalize_mem_a: get_symbol(&libs, b"bli_zpackm_sup_finalize_mem_a\0")
                .map(|sym| *sym),
            bli_spackm_sup_init_a: get_symbol(&libs, b"bli_spackm_sup_init_a\0").map(|sym| *sym),
            bli_dpackm_sup_init_a: get_symbol(&libs, b"bli_dpackm_sup_init_a\0").map(|sym| *sym),
            bli_cpackm_sup_init_a: get_symbol(&libs, b"bli_cpackm_sup_init_a\0").map(|sym| *sym),
            bli_zpackm_sup_init_a: get_symbol(&libs, b"bli_zpackm_sup_init_a\0").map(|sym| *sym),
            bli_spackm_sup_a: get_symbol(&libs, b"bli_spackm_sup_a\0").map(|sym| *sym),
            bli_dpackm_sup_a: get_symbol(&libs, b"bli_dpackm_sup_a\0").map(|sym| *sym),
            bli_cpackm_sup_a: get_symbol(&libs, b"bli_cpackm_sup_a\0").map(|sym| *sym),
            bli_zpackm_sup_a: get_symbol(&libs, b"bli_zpackm_sup_a\0").map(|sym| *sym),
            bli_spackm_sup_init_mem_b: get_symbol(&libs, b"bli_spackm_sup_init_mem_b\0")
                .map(|sym| *sym),
            bli_dpackm_sup_init_mem_b: get_symbol(&libs, b"bli_dpackm_sup_init_mem_b\0")
                .map(|sym| *sym),
            bli_cpackm_sup_init_mem_b: get_symbol(&libs, b"bli_cpackm_sup_init_mem_b\0")
                .map(|sym| *sym),
            bli_zpackm_sup_init_mem_b: get_symbol(&libs, b"bli_zpackm_sup_init_mem_b\0")
                .map(|sym| *sym),
            bli_spackm_sup_finalize_mem_b: get_symbol(&libs, b"bli_spackm_sup_finalize_mem_b\0")
                .map(|sym| *sym),
            bli_dpackm_sup_finalize_mem_b: get_symbol(&libs, b"bli_dpackm_sup_finalize_mem_b\0")
                .map(|sym| *sym),
            bli_cpackm_sup_finalize_mem_b: get_symbol(&libs, b"bli_cpackm_sup_finalize_mem_b\0")
                .map(|sym| *sym),
            bli_zpackm_sup_finalize_mem_b: get_symbol(&libs, b"bli_zpackm_sup_finalize_mem_b\0")
                .map(|sym| *sym),
            bli_spackm_sup_init_b: get_symbol(&libs, b"bli_spackm_sup_init_b\0").map(|sym| *sym),
            bli_dpackm_sup_init_b: get_symbol(&libs, b"bli_dpackm_sup_init_b\0").map(|sym| *sym),
            bli_cpackm_sup_init_b: get_symbol(&libs, b"bli_cpackm_sup_init_b\0").map(|sym| *sym),
            bli_zpackm_sup_init_b: get_symbol(&libs, b"bli_zpackm_sup_init_b\0").map(|sym| *sym),
            bli_spackm_sup_b: get_symbol(&libs, b"bli_spackm_sup_b\0").map(|sym| *sym),
            bli_dpackm_sup_b: get_symbol(&libs, b"bli_dpackm_sup_b\0").map(|sym| *sym),
            bli_cpackm_sup_b: get_symbol(&libs, b"bli_cpackm_sup_b\0").map(|sym| *sym),
            bli_zpackm_sup_b: get_symbol(&libs, b"bli_zpackm_sup_b\0").map(|sym| *sym),
            bli_spackm_sup_var1: get_symbol(&libs, b"bli_spackm_sup_var1\0").map(|sym| *sym),
            bli_dpackm_sup_var1: get_symbol(&libs, b"bli_dpackm_sup_var1\0").map(|sym| *sym),
            bli_cpackm_sup_var1: get_symbol(&libs, b"bli_cpackm_sup_var1\0").map(|sym| *sym),
            bli_zpackm_sup_var1: get_symbol(&libs, b"bli_zpackm_sup_var1\0").map(|sym| *sym),
            bli_spackm_sup_var2: get_symbol(&libs, b"bli_spackm_sup_var2\0").map(|sym| *sym),
            bli_dpackm_sup_var2: get_symbol(&libs, b"bli_dpackm_sup_var2\0").map(|sym| *sym),
            bli_cpackm_sup_var2: get_symbol(&libs, b"bli_cpackm_sup_var2\0").map(|sym| *sym),
            bli_zpackm_sup_var2: get_symbol(&libs, b"bli_zpackm_sup_var2\0").map(|sym| *sym),
            bli_gemm_ukernel: get_symbol(&libs, b"bli_gemm_ukernel\0").map(|sym| *sym),
            bli_gemmtrsm_ukernel: get_symbol(&libs, b"bli_gemmtrsm_ukernel\0").map(|sym| *sym),
            bli_trsm_ukernel: get_symbol(&libs, b"bli_trsm_ukernel\0").map(|sym| *sym),
            bli_sgemm_ukernel: get_symbol(&libs, b"bli_sgemm_ukernel\0").map(|sym| *sym),
            bli_dgemm_ukernel: get_symbol(&libs, b"bli_dgemm_ukernel\0").map(|sym| *sym),
            bli_cgemm_ukernel: get_symbol(&libs, b"bli_cgemm_ukernel\0").map(|sym| *sym),
            bli_zgemm_ukernel: get_symbol(&libs, b"bli_zgemm_ukernel\0").map(|sym| *sym),
            bli_sgemmtrsm_l_ukernel: get_symbol(&libs, b"bli_sgemmtrsm_l_ukernel\0")
                .map(|sym| *sym),
            bli_dgemmtrsm_l_ukernel: get_symbol(&libs, b"bli_dgemmtrsm_l_ukernel\0")
                .map(|sym| *sym),
            bli_cgemmtrsm_l_ukernel: get_symbol(&libs, b"bli_cgemmtrsm_l_ukernel\0")
                .map(|sym| *sym),
            bli_zgemmtrsm_l_ukernel: get_symbol(&libs, b"bli_zgemmtrsm_l_ukernel\0")
                .map(|sym| *sym),
            bli_sgemmtrsm_u_ukernel: get_symbol(&libs, b"bli_sgemmtrsm_u_ukernel\0")
                .map(|sym| *sym),
            bli_dgemmtrsm_u_ukernel: get_symbol(&libs, b"bli_dgemmtrsm_u_ukernel\0")
                .map(|sym| *sym),
            bli_cgemmtrsm_u_ukernel: get_symbol(&libs, b"bli_cgemmtrsm_u_ukernel\0")
                .map(|sym| *sym),
            bli_zgemmtrsm_u_ukernel: get_symbol(&libs, b"bli_zgemmtrsm_u_ukernel\0")
                .map(|sym| *sym),
            bli_strsm_l_ukernel: get_symbol(&libs, b"bli_strsm_l_ukernel\0").map(|sym| *sym),
            bli_dtrsm_l_ukernel: get_symbol(&libs, b"bli_dtrsm_l_ukernel\0").map(|sym| *sym),
            bli_ctrsm_l_ukernel: get_symbol(&libs, b"bli_ctrsm_l_ukernel\0").map(|sym| *sym),
            bli_ztrsm_l_ukernel: get_symbol(&libs, b"bli_ztrsm_l_ukernel\0").map(|sym| *sym),
            bli_strsm_u_ukernel: get_symbol(&libs, b"bli_strsm_u_ukernel\0").map(|sym| *sym),
            bli_dtrsm_u_ukernel: get_symbol(&libs, b"bli_dtrsm_u_ukernel\0").map(|sym| *sym),
            bli_ctrsm_u_ukernel: get_symbol(&libs, b"bli_ctrsm_u_ukernel\0").map(|sym| *sym),
            bli_ztrsm_u_ukernel: get_symbol(&libs, b"bli_ztrsm_u_ukernel\0").map(|sym| *sym),
            bli_gemm_ukernel_qfp: get_symbol(&libs, b"bli_gemm_ukernel_qfp\0").map(|sym| *sym),
            bli_gemmtrsm_l_ukernel_qfp: get_symbol(&libs, b"bli_gemmtrsm_l_ukernel_qfp\0")
                .map(|sym| *sym),
            bli_gemmtrsm_u_ukernel_qfp: get_symbol(&libs, b"bli_gemmtrsm_u_ukernel_qfp\0")
                .map(|sym| *sym),
            bli_trsm_l_ukernel_qfp: get_symbol(&libs, b"bli_trsm_l_ukernel_qfp\0").map(|sym| *sym),
            bli_trsm_u_ukernel_qfp: get_symbol(&libs, b"bli_trsm_u_ukernel_qfp\0").map(|sym| *sym),
            bli_gemm_cntl_create: get_symbol(&libs, b"bli_gemm_cntl_create\0").map(|sym| *sym),
            bli_gemmbp_cntl_create: get_symbol(&libs, b"bli_gemmbp_cntl_create\0").map(|sym| *sym),
            bli_gemm_cntl_free: get_symbol(&libs, b"bli_gemm_cntl_free\0").map(|sym| *sym),
            bli_gemm_cntl_create_node: get_symbol(&libs, b"bli_gemm_cntl_create_node\0")
                .map(|sym| *sym),
            bli_gemm_front: get_symbol(&libs, b"bli_gemm_front\0").map(|sym| *sym),
            bli_gemm_small: get_symbol(&libs, b"bli_gemm_small\0").map(|sym| *sym),
            bli_gemm_int: get_symbol(&libs, b"bli_gemm_int\0").map(|sym| *sym),
            bli_gemm_blk_var1: get_symbol(&libs, b"bli_gemm_blk_var1\0").map(|sym| *sym),
            bli_gemm_blk_var2: get_symbol(&libs, b"bli_gemm_blk_var2\0").map(|sym| *sym),
            bli_gemm_blk_var3: get_symbol(&libs, b"bli_gemm_blk_var3\0").map(|sym| *sym),
            bli_gemm_packa: get_symbol(&libs, b"bli_gemm_packa\0").map(|sym| *sym),
            bli_gemm_packb: get_symbol(&libs, b"bli_gemm_packb\0").map(|sym| *sym),
            bli_gemm_ker_var1: get_symbol(&libs, b"bli_gemm_ker_var1\0").map(|sym| *sym),
            bli_gemm_ker_var2: get_symbol(&libs, b"bli_gemm_ker_var2\0").map(|sym| *sym),
            bli_sgemm_ker_var2: get_symbol(&libs, b"bli_sgemm_ker_var2\0").map(|sym| *sym),
            bli_dgemm_ker_var2: get_symbol(&libs, b"bli_dgemm_ker_var2\0").map(|sym| *sym),
            bli_cgemm_ker_var2: get_symbol(&libs, b"bli_cgemm_ker_var2\0").map(|sym| *sym),
            bli_zgemm_ker_var2: get_symbol(&libs, b"bli_zgemm_ker_var2\0").map(|sym| *sym),
            bli_zgemm_tiny: get_symbol(&libs, b"bli_zgemm_tiny\0").map(|sym| *sym),
            bli_sgemm_md_c2r_ref: get_symbol(&libs, b"bli_sgemm_md_c2r_ref\0").map(|sym| *sym),
            bli_dgemm_md_c2r_ref: get_symbol(&libs, b"bli_dgemm_md_c2r_ref\0").map(|sym| *sym),
            bli_cgemm_md_c2r_ref: get_symbol(&libs, b"bli_cgemm_md_c2r_ref\0").map(|sym| *sym),
            bli_zgemm_md_c2r_ref: get_symbol(&libs, b"bli_zgemm_md_c2r_ref\0").map(|sym| *sym),
            bli_gemm_md: get_symbol(&libs, b"bli_gemm_md\0").map(|sym| *sym),
            bli_gemm_md_ccc: get_symbol(&libs, b"bli_gemm_md_ccc\0").map(|sym| *sym),
            bli_gemm_md_ccr: get_symbol(&libs, b"bli_gemm_md_ccr\0").map(|sym| *sym),
            bli_gemm_md_crc: get_symbol(&libs, b"bli_gemm_md_crc\0").map(|sym| *sym),
            bli_gemm_md_rcc: get_symbol(&libs, b"bli_gemm_md_rcc\0").map(|sym| *sym),
            bli_gemm_md_rrc: get_symbol(&libs, b"bli_gemm_md_rrc\0").map(|sym| *sym),
            bli_gemm_md_rcr: get_symbol(&libs, b"bli_gemm_md_rcr\0").map(|sym| *sym),
            bli_gemm_md_crr: get_symbol(&libs, b"bli_gemm_md_crr\0").map(|sym| *sym),
            bli_gemm_md_rrr: get_symbol(&libs, b"bli_gemm_md_rrr\0").map(|sym| *sym),
            bli_gemm_md_front: get_symbol(&libs, b"bli_gemm_md_front\0").map(|sym| *sym),
            bli_gemm_md_zgemm: get_symbol(&libs, b"bli_gemm_md_zgemm\0").map(|sym| *sym),
            bli_gemm_ker_var2_md: get_symbol(&libs, b"bli_gemm_ker_var2_md\0").map(|sym| *sym),
            bli_ssgemm_ker_var2_md: get_symbol(&libs, b"bli_ssgemm_ker_var2_md\0").map(|sym| *sym),
            bli_ddgemm_ker_var2_md: get_symbol(&libs, b"bli_ddgemm_ker_var2_md\0").map(|sym| *sym),
            bli_ccgemm_ker_var2_md: get_symbol(&libs, b"bli_ccgemm_ker_var2_md\0").map(|sym| *sym),
            bli_zzgemm_ker_var2_md: get_symbol(&libs, b"bli_zzgemm_ker_var2_md\0").map(|sym| *sym),
            bli_sdgemm_ker_var2_md: get_symbol(&libs, b"bli_sdgemm_ker_var2_md\0").map(|sym| *sym),
            bli_scgemm_ker_var2_md: get_symbol(&libs, b"bli_scgemm_ker_var2_md\0").map(|sym| *sym),
            bli_szgemm_ker_var2_md: get_symbol(&libs, b"bli_szgemm_ker_var2_md\0").map(|sym| *sym),
            bli_dsgemm_ker_var2_md: get_symbol(&libs, b"bli_dsgemm_ker_var2_md\0").map(|sym| *sym),
            bli_dcgemm_ker_var2_md: get_symbol(&libs, b"bli_dcgemm_ker_var2_md\0").map(|sym| *sym),
            bli_dzgemm_ker_var2_md: get_symbol(&libs, b"bli_dzgemm_ker_var2_md\0").map(|sym| *sym),
            bli_csgemm_ker_var2_md: get_symbol(&libs, b"bli_csgemm_ker_var2_md\0").map(|sym| *sym),
            bli_cdgemm_ker_var2_md: get_symbol(&libs, b"bli_cdgemm_ker_var2_md\0").map(|sym| *sym),
            bli_czgemm_ker_var2_md: get_symbol(&libs, b"bli_czgemm_ker_var2_md\0").map(|sym| *sym),
            bli_zsgemm_ker_var2_md: get_symbol(&libs, b"bli_zsgemm_ker_var2_md\0").map(|sym| *sym),
            bli_zdgemm_ker_var2_md: get_symbol(&libs, b"bli_zdgemm_ker_var2_md\0").map(|sym| *sym),
            bli_zcgemm_ker_var2_md: get_symbol(&libs, b"bli_zcgemm_ker_var2_md\0").map(|sym| *sym),
            bli_hemm_front: get_symbol(&libs, b"bli_hemm_front\0").map(|sym| *sym),
            bli_herk_front: get_symbol(&libs, b"bli_herk_front\0").map(|sym| *sym),
            bli_herk_x_ker_var2: get_symbol(&libs, b"bli_herk_x_ker_var2\0").map(|sym| *sym),
            bli_herk_l_ker_var2: get_symbol(&libs, b"bli_herk_l_ker_var2\0").map(|sym| *sym),
            bli_herk_u_ker_var2: get_symbol(&libs, b"bli_herk_u_ker_var2\0").map(|sym| *sym),
            bli_sherk_l_ker_var2: get_symbol(&libs, b"bli_sherk_l_ker_var2\0").map(|sym| *sym),
            bli_dherk_l_ker_var2: get_symbol(&libs, b"bli_dherk_l_ker_var2\0").map(|sym| *sym),
            bli_cherk_l_ker_var2: get_symbol(&libs, b"bli_cherk_l_ker_var2\0").map(|sym| *sym),
            bli_zherk_l_ker_var2: get_symbol(&libs, b"bli_zherk_l_ker_var2\0").map(|sym| *sym),
            bli_sherk_u_ker_var2: get_symbol(&libs, b"bli_sherk_u_ker_var2\0").map(|sym| *sym),
            bli_dherk_u_ker_var2: get_symbol(&libs, b"bli_dherk_u_ker_var2\0").map(|sym| *sym),
            bli_cherk_u_ker_var2: get_symbol(&libs, b"bli_cherk_u_ker_var2\0").map(|sym| *sym),
            bli_zherk_u_ker_var2: get_symbol(&libs, b"bli_zherk_u_ker_var2\0").map(|sym| *sym),
            bli_her2k_front: get_symbol(&libs, b"bli_her2k_front\0").map(|sym| *sym),
            bli_symm_front: get_symbol(&libs, b"bli_symm_front\0").map(|sym| *sym),
            bli_syrk_front: get_symbol(&libs, b"bli_syrk_front\0").map(|sym| *sym),
            bli_syr2k_front: get_symbol(&libs, b"bli_syr2k_front\0").map(|sym| *sym),
            bli_trmm_front: get_symbol(&libs, b"bli_trmm_front\0").map(|sym| *sym),
            bli_trmm_xx_ker_var2: get_symbol(&libs, b"bli_trmm_xx_ker_var2\0").map(|sym| *sym),
            bli_trmm_ll_ker_var2: get_symbol(&libs, b"bli_trmm_ll_ker_var2\0").map(|sym| *sym),
            bli_trmm_lu_ker_var2: get_symbol(&libs, b"bli_trmm_lu_ker_var2\0").map(|sym| *sym),
            bli_trmm_rl_ker_var2: get_symbol(&libs, b"bli_trmm_rl_ker_var2\0").map(|sym| *sym),
            bli_trmm_ru_ker_var2: get_symbol(&libs, b"bli_trmm_ru_ker_var2\0").map(|sym| *sym),
            bli_strmm_ll_ker_var2: get_symbol(&libs, b"bli_strmm_ll_ker_var2\0").map(|sym| *sym),
            bli_dtrmm_ll_ker_var2: get_symbol(&libs, b"bli_dtrmm_ll_ker_var2\0").map(|sym| *sym),
            bli_ctrmm_ll_ker_var2: get_symbol(&libs, b"bli_ctrmm_ll_ker_var2\0").map(|sym| *sym),
            bli_ztrmm_ll_ker_var2: get_symbol(&libs, b"bli_ztrmm_ll_ker_var2\0").map(|sym| *sym),
            bli_strmm_lu_ker_var2: get_symbol(&libs, b"bli_strmm_lu_ker_var2\0").map(|sym| *sym),
            bli_dtrmm_lu_ker_var2: get_symbol(&libs, b"bli_dtrmm_lu_ker_var2\0").map(|sym| *sym),
            bli_ctrmm_lu_ker_var2: get_symbol(&libs, b"bli_ctrmm_lu_ker_var2\0").map(|sym| *sym),
            bli_ztrmm_lu_ker_var2: get_symbol(&libs, b"bli_ztrmm_lu_ker_var2\0").map(|sym| *sym),
            bli_strmm_rl_ker_var2: get_symbol(&libs, b"bli_strmm_rl_ker_var2\0").map(|sym| *sym),
            bli_dtrmm_rl_ker_var2: get_symbol(&libs, b"bli_dtrmm_rl_ker_var2\0").map(|sym| *sym),
            bli_ctrmm_rl_ker_var2: get_symbol(&libs, b"bli_ctrmm_rl_ker_var2\0").map(|sym| *sym),
            bli_ztrmm_rl_ker_var2: get_symbol(&libs, b"bli_ztrmm_rl_ker_var2\0").map(|sym| *sym),
            bli_strmm_ru_ker_var2: get_symbol(&libs, b"bli_strmm_ru_ker_var2\0").map(|sym| *sym),
            bli_dtrmm_ru_ker_var2: get_symbol(&libs, b"bli_dtrmm_ru_ker_var2\0").map(|sym| *sym),
            bli_ctrmm_ru_ker_var2: get_symbol(&libs, b"bli_ctrmm_ru_ker_var2\0").map(|sym| *sym),
            bli_ztrmm_ru_ker_var2: get_symbol(&libs, b"bli_ztrmm_ru_ker_var2\0").map(|sym| *sym),
            bli_trmm3_front: get_symbol(&libs, b"bli_trmm3_front\0").map(|sym| *sym),
            bli_trsm_cntl_create: get_symbol(&libs, b"bli_trsm_cntl_create\0").map(|sym| *sym),
            bli_trsm_l_cntl_create: get_symbol(&libs, b"bli_trsm_l_cntl_create\0").map(|sym| *sym),
            bli_trsm_r_cntl_create: get_symbol(&libs, b"bli_trsm_r_cntl_create\0").map(|sym| *sym),
            bli_trsm_cntl_free: get_symbol(&libs, b"bli_trsm_cntl_free\0").map(|sym| *sym),
            bli_trsm_cntl_create_node: get_symbol(&libs, b"bli_trsm_cntl_create_node\0")
                .map(|sym| *sym),
            bli_trsm_front: get_symbol(&libs, b"bli_trsm_front\0").map(|sym| *sym),
            bli_trsm_int: get_symbol(&libs, b"bli_trsm_int\0").map(|sym| *sym),
            bli_trsm_blk_var1: get_symbol(&libs, b"bli_trsm_blk_var1\0").map(|sym| *sym),
            bli_trsm_blk_var2: get_symbol(&libs, b"bli_trsm_blk_var2\0").map(|sym| *sym),
            bli_trsm_blk_var3: get_symbol(&libs, b"bli_trsm_blk_var3\0").map(|sym| *sym),
            bli_trsm_packa: get_symbol(&libs, b"bli_trsm_packa\0").map(|sym| *sym),
            bli_trsm_packb: get_symbol(&libs, b"bli_trsm_packb\0").map(|sym| *sym),
            bli_trsm_xx_ker_var2: get_symbol(&libs, b"bli_trsm_xx_ker_var2\0").map(|sym| *sym),
            bli_trsm_ll_ker_var2: get_symbol(&libs, b"bli_trsm_ll_ker_var2\0").map(|sym| *sym),
            bli_trsm_lu_ker_var2: get_symbol(&libs, b"bli_trsm_lu_ker_var2\0").map(|sym| *sym),
            bli_trsm_rl_ker_var2: get_symbol(&libs, b"bli_trsm_rl_ker_var2\0").map(|sym| *sym),
            bli_trsm_ru_ker_var2: get_symbol(&libs, b"bli_trsm_ru_ker_var2\0").map(|sym| *sym),
            bli_strsm_ll_ker_var2: get_symbol(&libs, b"bli_strsm_ll_ker_var2\0").map(|sym| *sym),
            bli_dtrsm_ll_ker_var2: get_symbol(&libs, b"bli_dtrsm_ll_ker_var2\0").map(|sym| *sym),
            bli_ctrsm_ll_ker_var2: get_symbol(&libs, b"bli_ctrsm_ll_ker_var2\0").map(|sym| *sym),
            bli_ztrsm_ll_ker_var2: get_symbol(&libs, b"bli_ztrsm_ll_ker_var2\0").map(|sym| *sym),
            bli_strsm_lu_ker_var2: get_symbol(&libs, b"bli_strsm_lu_ker_var2\0").map(|sym| *sym),
            bli_dtrsm_lu_ker_var2: get_symbol(&libs, b"bli_dtrsm_lu_ker_var2\0").map(|sym| *sym),
            bli_ctrsm_lu_ker_var2: get_symbol(&libs, b"bli_ctrsm_lu_ker_var2\0").map(|sym| *sym),
            bli_ztrsm_lu_ker_var2: get_symbol(&libs, b"bli_ztrsm_lu_ker_var2\0").map(|sym| *sym),
            bli_strsm_rl_ker_var2: get_symbol(&libs, b"bli_strsm_rl_ker_var2\0").map(|sym| *sym),
            bli_dtrsm_rl_ker_var2: get_symbol(&libs, b"bli_dtrsm_rl_ker_var2\0").map(|sym| *sym),
            bli_ctrsm_rl_ker_var2: get_symbol(&libs, b"bli_ctrsm_rl_ker_var2\0").map(|sym| *sym),
            bli_ztrsm_rl_ker_var2: get_symbol(&libs, b"bli_ztrsm_rl_ker_var2\0").map(|sym| *sym),
            bli_strsm_ru_ker_var2: get_symbol(&libs, b"bli_strsm_ru_ker_var2\0").map(|sym| *sym),
            bli_dtrsm_ru_ker_var2: get_symbol(&libs, b"bli_dtrsm_ru_ker_var2\0").map(|sym| *sym),
            bli_ctrsm_ru_ker_var2: get_symbol(&libs, b"bli_ctrsm_ru_ker_var2\0").map(|sym| *sym),
            bli_ztrsm_ru_ker_var2: get_symbol(&libs, b"bli_ztrsm_ru_ker_var2\0").map(|sym| *sym),
            bli_gemmt_front: get_symbol(&libs, b"bli_gemmt_front\0").map(|sym| *sym),
            bli_gemmt_ker_var2: get_symbol(&libs, b"bli_gemmt_ker_var2\0").map(|sym| *sym),
            bli_sgemmt_l_ker_var2: get_symbol(&libs, b"bli_sgemmt_l_ker_var2\0").map(|sym| *sym),
            bli_dgemmt_l_ker_var2: get_symbol(&libs, b"bli_dgemmt_l_ker_var2\0").map(|sym| *sym),
            bli_sgemmt_u_ker_var2: get_symbol(&libs, b"bli_sgemmt_u_ker_var2\0").map(|sym| *sym),
            bli_dgemmt_u_ker_var2: get_symbol(&libs, b"bli_dgemmt_u_ker_var2\0").map(|sym| *sym),
            bli_cgemmt_l_ker_var2: get_symbol(&libs, b"bli_cgemmt_l_ker_var2\0").map(|sym| *sym),
            bli_zgemmt_l_ker_var2: get_symbol(&libs, b"bli_zgemmt_l_ker_var2\0").map(|sym| *sym),
            bli_cgemmt_u_ker_var2: get_symbol(&libs, b"bli_cgemmt_u_ker_var2\0").map(|sym| *sym),
            bli_zgemmt_u_ker_var2: get_symbol(&libs, b"bli_zgemmt_u_ker_var2\0").map(|sym| *sym),
            bli_gemmtsup_ref_var1n: get_symbol(&libs, b"bli_gemmtsup_ref_var1n\0").map(|sym| *sym),
            bli_gemmtsup_ref_var2m: get_symbol(&libs, b"bli_gemmtsup_ref_var2m\0").map(|sym| *sym),
            bli_sgemmtsup_l_ref_var1n: get_symbol(&libs, b"bli_sgemmtsup_l_ref_var1n\0")
                .map(|sym| *sym),
            bli_dgemmtsup_l_ref_var1n: get_symbol(&libs, b"bli_dgemmtsup_l_ref_var1n\0")
                .map(|sym| *sym),
            bli_sgemmtsup_u_ref_var1n: get_symbol(&libs, b"bli_sgemmtsup_u_ref_var1n\0")
                .map(|sym| *sym),
            bli_dgemmtsup_u_ref_var1n: get_symbol(&libs, b"bli_dgemmtsup_u_ref_var1n\0")
                .map(|sym| *sym),
            bli_cgemmtsup_l_ref_var1n: get_symbol(&libs, b"bli_cgemmtsup_l_ref_var1n\0")
                .map(|sym| *sym),
            bli_zgemmtsup_l_ref_var1n: get_symbol(&libs, b"bli_zgemmtsup_l_ref_var1n\0")
                .map(|sym| *sym),
            bli_cgemmtsup_u_ref_var1n: get_symbol(&libs, b"bli_cgemmtsup_u_ref_var1n\0")
                .map(|sym| *sym),
            bli_zgemmtsup_u_ref_var1n: get_symbol(&libs, b"bli_zgemmtsup_u_ref_var1n\0")
                .map(|sym| *sym),
            bli_sgemmtsup_l_ref_var2m: get_symbol(&libs, b"bli_sgemmtsup_l_ref_var2m\0")
                .map(|sym| *sym),
            bli_dgemmtsup_l_ref_var2m: get_symbol(&libs, b"bli_dgemmtsup_l_ref_var2m\0")
                .map(|sym| *sym),
            bli_sgemmtsup_u_ref_var2m: get_symbol(&libs, b"bli_sgemmtsup_u_ref_var2m\0")
                .map(|sym| *sym),
            bli_dgemmtsup_u_ref_var2m: get_symbol(&libs, b"bli_dgemmtsup_u_ref_var2m\0")
                .map(|sym| *sym),
            bli_cgemmtsup_l_ref_var2m: get_symbol(&libs, b"bli_cgemmtsup_l_ref_var2m\0")
                .map(|sym| *sym),
            bli_zgemmtsup_l_ref_var2m: get_symbol(&libs, b"bli_zgemmtsup_l_ref_var2m\0")
                .map(|sym| *sym),
            bli_cgemmtsup_u_ref_var2m: get_symbol(&libs, b"bli_cgemmtsup_u_ref_var2m\0")
                .map(|sym| *sym),
            bli_zgemmtsup_u_ref_var2m: get_symbol(&libs, b"bli_zgemmtsup_u_ref_var2m\0")
                .map(|sym| *sym),
            bli_gemm_smart_threading_sup: get_symbol(&libs, b"bli_gemm_smart_threading_sup\0")
                .map(|sym| *sym),
            bli_gemm_compute_init: get_symbol(&libs, b"bli_gemm_compute_init\0").map(|sym| *sym),
            bli_gemm_compute: get_symbol(&libs, b"bli_gemm_compute\0").map(|sym| *sym),
            bli_sgemm_compute: get_symbol(&libs, b"bli_sgemm_compute\0").map(|sym| *sym),
            bli_dgemm_compute: get_symbol(&libs, b"bli_dgemm_compute\0").map(|sym| *sym),
            bli_cgemm_compute: get_symbol(&libs, b"bli_cgemm_compute\0").map(|sym| *sym),
            bli_zgemm_compute: get_symbol(&libs, b"bli_zgemm_compute\0").map(|sym| *sym),
            bli_asumv_check: get_symbol(&libs, b"bli_asumv_check\0").map(|sym| *sym),
            bli_mkherm_check: get_symbol(&libs, b"bli_mkherm_check\0").map(|sym| *sym),
            bli_mksymm_check: get_symbol(&libs, b"bli_mksymm_check\0").map(|sym| *sym),
            bli_mktrim_check: get_symbol(&libs, b"bli_mktrim_check\0").map(|sym| *sym),
            bli_norm1v_check: get_symbol(&libs, b"bli_norm1v_check\0").map(|sym| *sym),
            bli_normfv_check: get_symbol(&libs, b"bli_normfv_check\0").map(|sym| *sym),
            bli_normiv_check: get_symbol(&libs, b"bli_normiv_check\0").map(|sym| *sym),
            bli_norm1m_check: get_symbol(&libs, b"bli_norm1m_check\0").map(|sym| *sym),
            bli_normfm_check: get_symbol(&libs, b"bli_normfm_check\0").map(|sym| *sym),
            bli_normim_check: get_symbol(&libs, b"bli_normim_check\0").map(|sym| *sym),
            bli_randv_check: get_symbol(&libs, b"bli_randv_check\0").map(|sym| *sym),
            bli_randnv_check: get_symbol(&libs, b"bli_randnv_check\0").map(|sym| *sym),
            bli_randm_check: get_symbol(&libs, b"bli_randm_check\0").map(|sym| *sym),
            bli_randnm_check: get_symbol(&libs, b"bli_randnm_check\0").map(|sym| *sym),
            bli_sumsqv_check: get_symbol(&libs, b"bli_sumsqv_check\0").map(|sym| *sym),
            bli_eqsc_check: get_symbol(&libs, b"bli_eqsc_check\0").map(|sym| *sym),
            bli_eqv_check: get_symbol(&libs, b"bli_eqv_check\0").map(|sym| *sym),
            bli_eqm_check: get_symbol(&libs, b"bli_eqm_check\0").map(|sym| *sym),
            bli_fprintv_check: get_symbol(&libs, b"bli_fprintv_check\0").map(|sym| *sym),
            bli_fprintm_check: get_symbol(&libs, b"bli_fprintm_check\0").map(|sym| *sym),
            bli_utilv_xi_check: get_symbol(&libs, b"bli_utilv_xi_check\0").map(|sym| *sym),
            bli_utilv_xa_check: get_symbol(&libs, b"bli_utilv_xa_check\0").map(|sym| *sym),
            bli_utilm_mkhst_check: get_symbol(&libs, b"bli_utilm_mkhst_check\0").map(|sym| *sym),
            bli_utilv_norm_check: get_symbol(&libs, b"bli_utilv_norm_check\0").map(|sym| *sym),
            bli_utilm_norm_check: get_symbol(&libs, b"bli_utilm_norm_check\0").map(|sym| *sym),
            bli_utilm_fprint_check: get_symbol(&libs, b"bli_utilm_fprint_check\0").map(|sym| *sym),
            bli_utilm_rand_check: get_symbol(&libs, b"bli_utilm_rand_check\0").map(|sym| *sym),
            bli_utilv_sumsqv_check: get_symbol(&libs, b"bli_utilv_sumsqv_check\0").map(|sym| *sym),
            bli_asumv_ex: get_symbol(&libs, b"bli_asumv_ex\0").map(|sym| *sym),
            bli_mkherm_ex: get_symbol(&libs, b"bli_mkherm_ex\0").map(|sym| *sym),
            bli_mksymm_ex: get_symbol(&libs, b"bli_mksymm_ex\0").map(|sym| *sym),
            bli_mktrim_ex: get_symbol(&libs, b"bli_mktrim_ex\0").map(|sym| *sym),
            bli_norm1v_ex: get_symbol(&libs, b"bli_norm1v_ex\0").map(|sym| *sym),
            bli_normfv_ex: get_symbol(&libs, b"bli_normfv_ex\0").map(|sym| *sym),
            bli_normiv_ex: get_symbol(&libs, b"bli_normiv_ex\0").map(|sym| *sym),
            bli_norm1m_ex: get_symbol(&libs, b"bli_norm1m_ex\0").map(|sym| *sym),
            bli_normfm_ex: get_symbol(&libs, b"bli_normfm_ex\0").map(|sym| *sym),
            bli_normim_ex: get_symbol(&libs, b"bli_normim_ex\0").map(|sym| *sym),
            bli_randv_ex: get_symbol(&libs, b"bli_randv_ex\0").map(|sym| *sym),
            bli_randnv_ex: get_symbol(&libs, b"bli_randnv_ex\0").map(|sym| *sym),
            bli_randm_ex: get_symbol(&libs, b"bli_randm_ex\0").map(|sym| *sym),
            bli_randnm_ex: get_symbol(&libs, b"bli_randnm_ex\0").map(|sym| *sym),
            bli_sumsqv_ex: get_symbol(&libs, b"bli_sumsqv_ex\0").map(|sym| *sym),
            bli_asumv: get_symbol(&libs, b"bli_asumv\0").map(|sym| *sym),
            bli_mkherm: get_symbol(&libs, b"bli_mkherm\0").map(|sym| *sym),
            bli_mksymm: get_symbol(&libs, b"bli_mksymm\0").map(|sym| *sym),
            bli_mktrim: get_symbol(&libs, b"bli_mktrim\0").map(|sym| *sym),
            bli_norm1v: get_symbol(&libs, b"bli_norm1v\0").map(|sym| *sym),
            bli_normfv: get_symbol(&libs, b"bli_normfv\0").map(|sym| *sym),
            bli_normiv: get_symbol(&libs, b"bli_normiv\0").map(|sym| *sym),
            bli_norm1m: get_symbol(&libs, b"bli_norm1m\0").map(|sym| *sym),
            bli_normfm: get_symbol(&libs, b"bli_normfm\0").map(|sym| *sym),
            bli_normim: get_symbol(&libs, b"bli_normim\0").map(|sym| *sym),
            bli_randv: get_symbol(&libs, b"bli_randv\0").map(|sym| *sym),
            bli_randnv: get_symbol(&libs, b"bli_randnv\0").map(|sym| *sym),
            bli_randm: get_symbol(&libs, b"bli_randm\0").map(|sym| *sym),
            bli_randnm: get_symbol(&libs, b"bli_randnm\0").map(|sym| *sym),
            bli_sumsqv: get_symbol(&libs, b"bli_sumsqv\0").map(|sym| *sym),
            bli_eqsc: get_symbol(&libs, b"bli_eqsc\0").map(|sym| *sym),
            bli_eqv: get_symbol(&libs, b"bli_eqv\0").map(|sym| *sym),
            bli_eqm: get_symbol(&libs, b"bli_eqm\0").map(|sym| *sym),
            bli_fprintv: get_symbol(&libs, b"bli_fprintv\0").map(|sym| *sym),
            bli_fprintm: get_symbol(&libs, b"bli_fprintm\0").map(|sym| *sym),
            bli_printv: get_symbol(&libs, b"bli_printv\0").map(|sym| *sym),
            bli_printm: get_symbol(&libs, b"bli_printm\0").map(|sym| *sym),
            bli_sasumv_ex: get_symbol(&libs, b"bli_sasumv_ex\0").map(|sym| *sym),
            bli_dasumv_ex: get_symbol(&libs, b"bli_dasumv_ex\0").map(|sym| *sym),
            bli_casumv_ex: get_symbol(&libs, b"bli_casumv_ex\0").map(|sym| *sym),
            bli_zasumv_ex: get_symbol(&libs, b"bli_zasumv_ex\0").map(|sym| *sym),
            bli_smkherm_ex: get_symbol(&libs, b"bli_smkherm_ex\0").map(|sym| *sym),
            bli_dmkherm_ex: get_symbol(&libs, b"bli_dmkherm_ex\0").map(|sym| *sym),
            bli_cmkherm_ex: get_symbol(&libs, b"bli_cmkherm_ex\0").map(|sym| *sym),
            bli_zmkherm_ex: get_symbol(&libs, b"bli_zmkherm_ex\0").map(|sym| *sym),
            bli_smksymm_ex: get_symbol(&libs, b"bli_smksymm_ex\0").map(|sym| *sym),
            bli_dmksymm_ex: get_symbol(&libs, b"bli_dmksymm_ex\0").map(|sym| *sym),
            bli_cmksymm_ex: get_symbol(&libs, b"bli_cmksymm_ex\0").map(|sym| *sym),
            bli_zmksymm_ex: get_symbol(&libs, b"bli_zmksymm_ex\0").map(|sym| *sym),
            bli_smktrim_ex: get_symbol(&libs, b"bli_smktrim_ex\0").map(|sym| *sym),
            bli_dmktrim_ex: get_symbol(&libs, b"bli_dmktrim_ex\0").map(|sym| *sym),
            bli_cmktrim_ex: get_symbol(&libs, b"bli_cmktrim_ex\0").map(|sym| *sym),
            bli_zmktrim_ex: get_symbol(&libs, b"bli_zmktrim_ex\0").map(|sym| *sym),
            bli_snorm1v_ex: get_symbol(&libs, b"bli_snorm1v_ex\0").map(|sym| *sym),
            bli_dnorm1v_ex: get_symbol(&libs, b"bli_dnorm1v_ex\0").map(|sym| *sym),
            bli_cnorm1v_ex: get_symbol(&libs, b"bli_cnorm1v_ex\0").map(|sym| *sym),
            bli_znorm1v_ex: get_symbol(&libs, b"bli_znorm1v_ex\0").map(|sym| *sym),
            bli_snormfv_ex: get_symbol(&libs, b"bli_snormfv_ex\0").map(|sym| *sym),
            bli_dnormfv_ex: get_symbol(&libs, b"bli_dnormfv_ex\0").map(|sym| *sym),
            bli_cnormfv_ex: get_symbol(&libs, b"bli_cnormfv_ex\0").map(|sym| *sym),
            bli_znormfv_ex: get_symbol(&libs, b"bli_znormfv_ex\0").map(|sym| *sym),
            bli_snormiv_ex: get_symbol(&libs, b"bli_snormiv_ex\0").map(|sym| *sym),
            bli_dnormiv_ex: get_symbol(&libs, b"bli_dnormiv_ex\0").map(|sym| *sym),
            bli_cnormiv_ex: get_symbol(&libs, b"bli_cnormiv_ex\0").map(|sym| *sym),
            bli_znormiv_ex: get_symbol(&libs, b"bli_znormiv_ex\0").map(|sym| *sym),
            bli_snorm1m_ex: get_symbol(&libs, b"bli_snorm1m_ex\0").map(|sym| *sym),
            bli_dnorm1m_ex: get_symbol(&libs, b"bli_dnorm1m_ex\0").map(|sym| *sym),
            bli_cnorm1m_ex: get_symbol(&libs, b"bli_cnorm1m_ex\0").map(|sym| *sym),
            bli_znorm1m_ex: get_symbol(&libs, b"bli_znorm1m_ex\0").map(|sym| *sym),
            bli_snormfm_ex: get_symbol(&libs, b"bli_snormfm_ex\0").map(|sym| *sym),
            bli_dnormfm_ex: get_symbol(&libs, b"bli_dnormfm_ex\0").map(|sym| *sym),
            bli_cnormfm_ex: get_symbol(&libs, b"bli_cnormfm_ex\0").map(|sym| *sym),
            bli_znormfm_ex: get_symbol(&libs, b"bli_znormfm_ex\0").map(|sym| *sym),
            bli_snormim_ex: get_symbol(&libs, b"bli_snormim_ex\0").map(|sym| *sym),
            bli_dnormim_ex: get_symbol(&libs, b"bli_dnormim_ex\0").map(|sym| *sym),
            bli_cnormim_ex: get_symbol(&libs, b"bli_cnormim_ex\0").map(|sym| *sym),
            bli_znormim_ex: get_symbol(&libs, b"bli_znormim_ex\0").map(|sym| *sym),
            bli_srandv_ex: get_symbol(&libs, b"bli_srandv_ex\0").map(|sym| *sym),
            bli_drandv_ex: get_symbol(&libs, b"bli_drandv_ex\0").map(|sym| *sym),
            bli_crandv_ex: get_symbol(&libs, b"bli_crandv_ex\0").map(|sym| *sym),
            bli_zrandv_ex: get_symbol(&libs, b"bli_zrandv_ex\0").map(|sym| *sym),
            bli_srandnv_ex: get_symbol(&libs, b"bli_srandnv_ex\0").map(|sym| *sym),
            bli_drandnv_ex: get_symbol(&libs, b"bli_drandnv_ex\0").map(|sym| *sym),
            bli_crandnv_ex: get_symbol(&libs, b"bli_crandnv_ex\0").map(|sym| *sym),
            bli_zrandnv_ex: get_symbol(&libs, b"bli_zrandnv_ex\0").map(|sym| *sym),
            bli_srandm_ex: get_symbol(&libs, b"bli_srandm_ex\0").map(|sym| *sym),
            bli_drandm_ex: get_symbol(&libs, b"bli_drandm_ex\0").map(|sym| *sym),
            bli_crandm_ex: get_symbol(&libs, b"bli_crandm_ex\0").map(|sym| *sym),
            bli_zrandm_ex: get_symbol(&libs, b"bli_zrandm_ex\0").map(|sym| *sym),
            bli_srandnm_ex: get_symbol(&libs, b"bli_srandnm_ex\0").map(|sym| *sym),
            bli_drandnm_ex: get_symbol(&libs, b"bli_drandnm_ex\0").map(|sym| *sym),
            bli_crandnm_ex: get_symbol(&libs, b"bli_crandnm_ex\0").map(|sym| *sym),
            bli_zrandnm_ex: get_symbol(&libs, b"bli_zrandnm_ex\0").map(|sym| *sym),
            bli_ssumsqv_ex: get_symbol(&libs, b"bli_ssumsqv_ex\0").map(|sym| *sym),
            bli_dsumsqv_ex: get_symbol(&libs, b"bli_dsumsqv_ex\0").map(|sym| *sym),
            bli_csumsqv_ex: get_symbol(&libs, b"bli_csumsqv_ex\0").map(|sym| *sym),
            bli_zsumsqv_ex: get_symbol(&libs, b"bli_zsumsqv_ex\0").map(|sym| *sym),
            bli_sasumv: get_symbol(&libs, b"bli_sasumv\0").map(|sym| *sym),
            bli_dasumv: get_symbol(&libs, b"bli_dasumv\0").map(|sym| *sym),
            bli_casumv: get_symbol(&libs, b"bli_casumv\0").map(|sym| *sym),
            bli_zasumv: get_symbol(&libs, b"bli_zasumv\0").map(|sym| *sym),
            bli_smkherm: get_symbol(&libs, b"bli_smkherm\0").map(|sym| *sym),
            bli_dmkherm: get_symbol(&libs, b"bli_dmkherm\0").map(|sym| *sym),
            bli_cmkherm: get_symbol(&libs, b"bli_cmkherm\0").map(|sym| *sym),
            bli_zmkherm: get_symbol(&libs, b"bli_zmkherm\0").map(|sym| *sym),
            bli_smksymm: get_symbol(&libs, b"bli_smksymm\0").map(|sym| *sym),
            bli_dmksymm: get_symbol(&libs, b"bli_dmksymm\0").map(|sym| *sym),
            bli_cmksymm: get_symbol(&libs, b"bli_cmksymm\0").map(|sym| *sym),
            bli_zmksymm: get_symbol(&libs, b"bli_zmksymm\0").map(|sym| *sym),
            bli_smktrim: get_symbol(&libs, b"bli_smktrim\0").map(|sym| *sym),
            bli_dmktrim: get_symbol(&libs, b"bli_dmktrim\0").map(|sym| *sym),
            bli_cmktrim: get_symbol(&libs, b"bli_cmktrim\0").map(|sym| *sym),
            bli_zmktrim: get_symbol(&libs, b"bli_zmktrim\0").map(|sym| *sym),
            bli_snorm1v: get_symbol(&libs, b"bli_snorm1v\0").map(|sym| *sym),
            bli_dnorm1v: get_symbol(&libs, b"bli_dnorm1v\0").map(|sym| *sym),
            bli_cnorm1v: get_symbol(&libs, b"bli_cnorm1v\0").map(|sym| *sym),
            bli_znorm1v: get_symbol(&libs, b"bli_znorm1v\0").map(|sym| *sym),
            bli_snormfv: get_symbol(&libs, b"bli_snormfv\0").map(|sym| *sym),
            bli_dnormfv: get_symbol(&libs, b"bli_dnormfv\0").map(|sym| *sym),
            bli_cnormfv: get_symbol(&libs, b"bli_cnormfv\0").map(|sym| *sym),
            bli_znormfv: get_symbol(&libs, b"bli_znormfv\0").map(|sym| *sym),
            bli_snormiv: get_symbol(&libs, b"bli_snormiv\0").map(|sym| *sym),
            bli_dnormiv: get_symbol(&libs, b"bli_dnormiv\0").map(|sym| *sym),
            bli_cnormiv: get_symbol(&libs, b"bli_cnormiv\0").map(|sym| *sym),
            bli_znormiv: get_symbol(&libs, b"bli_znormiv\0").map(|sym| *sym),
            bli_snorm1m: get_symbol(&libs, b"bli_snorm1m\0").map(|sym| *sym),
            bli_dnorm1m: get_symbol(&libs, b"bli_dnorm1m\0").map(|sym| *sym),
            bli_cnorm1m: get_symbol(&libs, b"bli_cnorm1m\0").map(|sym| *sym),
            bli_znorm1m: get_symbol(&libs, b"bli_znorm1m\0").map(|sym| *sym),
            bli_snormfm: get_symbol(&libs, b"bli_snormfm\0").map(|sym| *sym),
            bli_dnormfm: get_symbol(&libs, b"bli_dnormfm\0").map(|sym| *sym),
            bli_cnormfm: get_symbol(&libs, b"bli_cnormfm\0").map(|sym| *sym),
            bli_znormfm: get_symbol(&libs, b"bli_znormfm\0").map(|sym| *sym),
            bli_snormim: get_symbol(&libs, b"bli_snormim\0").map(|sym| *sym),
            bli_dnormim: get_symbol(&libs, b"bli_dnormim\0").map(|sym| *sym),
            bli_cnormim: get_symbol(&libs, b"bli_cnormim\0").map(|sym| *sym),
            bli_znormim: get_symbol(&libs, b"bli_znormim\0").map(|sym| *sym),
            bli_srandv: get_symbol(&libs, b"bli_srandv\0").map(|sym| *sym),
            bli_drandv: get_symbol(&libs, b"bli_drandv\0").map(|sym| *sym),
            bli_crandv: get_symbol(&libs, b"bli_crandv\0").map(|sym| *sym),
            bli_zrandv: get_symbol(&libs, b"bli_zrandv\0").map(|sym| *sym),
            bli_srandnv: get_symbol(&libs, b"bli_srandnv\0").map(|sym| *sym),
            bli_drandnv: get_symbol(&libs, b"bli_drandnv\0").map(|sym| *sym),
            bli_crandnv: get_symbol(&libs, b"bli_crandnv\0").map(|sym| *sym),
            bli_zrandnv: get_symbol(&libs, b"bli_zrandnv\0").map(|sym| *sym),
            bli_srandm: get_symbol(&libs, b"bli_srandm\0").map(|sym| *sym),
            bli_drandm: get_symbol(&libs, b"bli_drandm\0").map(|sym| *sym),
            bli_crandm: get_symbol(&libs, b"bli_crandm\0").map(|sym| *sym),
            bli_zrandm: get_symbol(&libs, b"bli_zrandm\0").map(|sym| *sym),
            bli_srandnm: get_symbol(&libs, b"bli_srandnm\0").map(|sym| *sym),
            bli_drandnm: get_symbol(&libs, b"bli_drandnm\0").map(|sym| *sym),
            bli_crandnm: get_symbol(&libs, b"bli_crandnm\0").map(|sym| *sym),
            bli_zrandnm: get_symbol(&libs, b"bli_zrandnm\0").map(|sym| *sym),
            bli_ssumsqv: get_symbol(&libs, b"bli_ssumsqv\0").map(|sym| *sym),
            bli_dsumsqv: get_symbol(&libs, b"bli_dsumsqv\0").map(|sym| *sym),
            bli_csumsqv: get_symbol(&libs, b"bli_csumsqv\0").map(|sym| *sym),
            bli_zsumsqv: get_symbol(&libs, b"bli_zsumsqv\0").map(|sym| *sym),
            bli_seqsc: get_symbol(&libs, b"bli_seqsc\0").map(|sym| *sym),
            bli_deqsc: get_symbol(&libs, b"bli_deqsc\0").map(|sym| *sym),
            bli_ceqsc: get_symbol(&libs, b"bli_ceqsc\0").map(|sym| *sym),
            bli_zeqsc: get_symbol(&libs, b"bli_zeqsc\0").map(|sym| *sym),
            bli_seqv: get_symbol(&libs, b"bli_seqv\0").map(|sym| *sym),
            bli_deqv: get_symbol(&libs, b"bli_deqv\0").map(|sym| *sym),
            bli_ceqv: get_symbol(&libs, b"bli_ceqv\0").map(|sym| *sym),
            bli_zeqv: get_symbol(&libs, b"bli_zeqv\0").map(|sym| *sym),
            bli_seqm: get_symbol(&libs, b"bli_seqm\0").map(|sym| *sym),
            bli_deqm: get_symbol(&libs, b"bli_deqm\0").map(|sym| *sym),
            bli_ceqm: get_symbol(&libs, b"bli_ceqm\0").map(|sym| *sym),
            bli_zeqm: get_symbol(&libs, b"bli_zeqm\0").map(|sym| *sym),
            bli_sprintv: get_symbol(&libs, b"bli_sprintv\0").map(|sym| *sym),
            bli_dprintv: get_symbol(&libs, b"bli_dprintv\0").map(|sym| *sym),
            bli_cprintv: get_symbol(&libs, b"bli_cprintv\0").map(|sym| *sym),
            bli_zprintv: get_symbol(&libs, b"bli_zprintv\0").map(|sym| *sym),
            bli_iprintv: get_symbol(&libs, b"bli_iprintv\0").map(|sym| *sym),
            bli_sprintm: get_symbol(&libs, b"bli_sprintm\0").map(|sym| *sym),
            bli_dprintm: get_symbol(&libs, b"bli_dprintm\0").map(|sym| *sym),
            bli_cprintm: get_symbol(&libs, b"bli_cprintm\0").map(|sym| *sym),
            bli_zprintm: get_symbol(&libs, b"bli_zprintm\0").map(|sym| *sym),
            bli_iprintm: get_symbol(&libs, b"bli_iprintm\0").map(|sym| *sym),
            bli_asumv_ex_qfp: get_symbol(&libs, b"bli_asumv_ex_qfp\0").map(|sym| *sym),
            bli_mkherm_ex_qfp: get_symbol(&libs, b"bli_mkherm_ex_qfp\0").map(|sym| *sym),
            bli_mksymm_ex_qfp: get_symbol(&libs, b"bli_mksymm_ex_qfp\0").map(|sym| *sym),
            bli_mktrim_ex_qfp: get_symbol(&libs, b"bli_mktrim_ex_qfp\0").map(|sym| *sym),
            bli_norm1v_ex_qfp: get_symbol(&libs, b"bli_norm1v_ex_qfp\0").map(|sym| *sym),
            bli_normfv_ex_qfp: get_symbol(&libs, b"bli_normfv_ex_qfp\0").map(|sym| *sym),
            bli_normiv_ex_qfp: get_symbol(&libs, b"bli_normiv_ex_qfp\0").map(|sym| *sym),
            bli_norm1m_ex_qfp: get_symbol(&libs, b"bli_norm1m_ex_qfp\0").map(|sym| *sym),
            bli_normfm_ex_qfp: get_symbol(&libs, b"bli_normfm_ex_qfp\0").map(|sym| *sym),
            bli_normim_ex_qfp: get_symbol(&libs, b"bli_normim_ex_qfp\0").map(|sym| *sym),
            bli_randv_ex_qfp: get_symbol(&libs, b"bli_randv_ex_qfp\0").map(|sym| *sym),
            bli_randnv_ex_qfp: get_symbol(&libs, b"bli_randnv_ex_qfp\0").map(|sym| *sym),
            bli_randm_ex_qfp: get_symbol(&libs, b"bli_randm_ex_qfp\0").map(|sym| *sym),
            bli_randnm_ex_qfp: get_symbol(&libs, b"bli_randnm_ex_qfp\0").map(|sym| *sym),
            bli_sumsqv_ex_qfp: get_symbol(&libs, b"bli_sumsqv_ex_qfp\0").map(|sym| *sym),
            bli_eqsc_qfp: get_symbol(&libs, b"bli_eqsc_qfp\0").map(|sym| *sym),
            bli_eqv_qfp: get_symbol(&libs, b"bli_eqv_qfp\0").map(|sym| *sym),
            bli_eqm_qfp: get_symbol(&libs, b"bli_eqm_qfp\0").map(|sym| *sym),
            bli_fprintv_qfp: get_symbol(&libs, b"bli_fprintv_qfp\0").map(|sym| *sym),
            bli_fprintm_qfp: get_symbol(&libs, b"bli_fprintm_qfp\0").map(|sym| *sym),
            bli_sasumv_unb_var1: get_symbol(&libs, b"bli_sasumv_unb_var1\0").map(|sym| *sym),
            bli_dasumv_unb_var1: get_symbol(&libs, b"bli_dasumv_unb_var1\0").map(|sym| *sym),
            bli_casumv_unb_var1: get_symbol(&libs, b"bli_casumv_unb_var1\0").map(|sym| *sym),
            bli_zasumv_unb_var1: get_symbol(&libs, b"bli_zasumv_unb_var1\0").map(|sym| *sym),
            bli_smkherm_unb_var1: get_symbol(&libs, b"bli_smkherm_unb_var1\0").map(|sym| *sym),
            bli_dmkherm_unb_var1: get_symbol(&libs, b"bli_dmkherm_unb_var1\0").map(|sym| *sym),
            bli_cmkherm_unb_var1: get_symbol(&libs, b"bli_cmkherm_unb_var1\0").map(|sym| *sym),
            bli_zmkherm_unb_var1: get_symbol(&libs, b"bli_zmkherm_unb_var1\0").map(|sym| *sym),
            bli_smksymm_unb_var1: get_symbol(&libs, b"bli_smksymm_unb_var1\0").map(|sym| *sym),
            bli_dmksymm_unb_var1: get_symbol(&libs, b"bli_dmksymm_unb_var1\0").map(|sym| *sym),
            bli_cmksymm_unb_var1: get_symbol(&libs, b"bli_cmksymm_unb_var1\0").map(|sym| *sym),
            bli_zmksymm_unb_var1: get_symbol(&libs, b"bli_zmksymm_unb_var1\0").map(|sym| *sym),
            bli_smktrim_unb_var1: get_symbol(&libs, b"bli_smktrim_unb_var1\0").map(|sym| *sym),
            bli_dmktrim_unb_var1: get_symbol(&libs, b"bli_dmktrim_unb_var1\0").map(|sym| *sym),
            bli_cmktrim_unb_var1: get_symbol(&libs, b"bli_cmktrim_unb_var1\0").map(|sym| *sym),
            bli_zmktrim_unb_var1: get_symbol(&libs, b"bli_zmktrim_unb_var1\0").map(|sym| *sym),
            bli_snorm1v_unb_var1: get_symbol(&libs, b"bli_snorm1v_unb_var1\0").map(|sym| *sym),
            bli_dnorm1v_unb_var1: get_symbol(&libs, b"bli_dnorm1v_unb_var1\0").map(|sym| *sym),
            bli_cnorm1v_unb_var1: get_symbol(&libs, b"bli_cnorm1v_unb_var1\0").map(|sym| *sym),
            bli_znorm1v_unb_var1: get_symbol(&libs, b"bli_znorm1v_unb_var1\0").map(|sym| *sym),
            bli_snormfv_unb_var1: get_symbol(&libs, b"bli_snormfv_unb_var1\0").map(|sym| *sym),
            bli_dnormfv_unb_var1: get_symbol(&libs, b"bli_dnormfv_unb_var1\0").map(|sym| *sym),
            bli_cnormfv_unb_var1: get_symbol(&libs, b"bli_cnormfv_unb_var1\0").map(|sym| *sym),
            bli_znormfv_unb_var1: get_symbol(&libs, b"bli_znormfv_unb_var1\0").map(|sym| *sym),
            bli_snormiv_unb_var1: get_symbol(&libs, b"bli_snormiv_unb_var1\0").map(|sym| *sym),
            bli_dnormiv_unb_var1: get_symbol(&libs, b"bli_dnormiv_unb_var1\0").map(|sym| *sym),
            bli_cnormiv_unb_var1: get_symbol(&libs, b"bli_cnormiv_unb_var1\0").map(|sym| *sym),
            bli_znormiv_unb_var1: get_symbol(&libs, b"bli_znormiv_unb_var1\0").map(|sym| *sym),
            bli_snorm1m_unb_var1: get_symbol(&libs, b"bli_snorm1m_unb_var1\0").map(|sym| *sym),
            bli_dnorm1m_unb_var1: get_symbol(&libs, b"bli_dnorm1m_unb_var1\0").map(|sym| *sym),
            bli_cnorm1m_unb_var1: get_symbol(&libs, b"bli_cnorm1m_unb_var1\0").map(|sym| *sym),
            bli_znorm1m_unb_var1: get_symbol(&libs, b"bli_znorm1m_unb_var1\0").map(|sym| *sym),
            bli_snormfm_unb_var1: get_symbol(&libs, b"bli_snormfm_unb_var1\0").map(|sym| *sym),
            bli_dnormfm_unb_var1: get_symbol(&libs, b"bli_dnormfm_unb_var1\0").map(|sym| *sym),
            bli_cnormfm_unb_var1: get_symbol(&libs, b"bli_cnormfm_unb_var1\0").map(|sym| *sym),
            bli_znormfm_unb_var1: get_symbol(&libs, b"bli_znormfm_unb_var1\0").map(|sym| *sym),
            bli_snormim_unb_var1: get_symbol(&libs, b"bli_snormim_unb_var1\0").map(|sym| *sym),
            bli_dnormim_unb_var1: get_symbol(&libs, b"bli_dnormim_unb_var1\0").map(|sym| *sym),
            bli_cnormim_unb_var1: get_symbol(&libs, b"bli_cnormim_unb_var1\0").map(|sym| *sym),
            bli_znormim_unb_var1: get_symbol(&libs, b"bli_znormim_unb_var1\0").map(|sym| *sym),
            bli_srandv_unb_var1: get_symbol(&libs, b"bli_srandv_unb_var1\0").map(|sym| *sym),
            bli_drandv_unb_var1: get_symbol(&libs, b"bli_drandv_unb_var1\0").map(|sym| *sym),
            bli_crandv_unb_var1: get_symbol(&libs, b"bli_crandv_unb_var1\0").map(|sym| *sym),
            bli_zrandv_unb_var1: get_symbol(&libs, b"bli_zrandv_unb_var1\0").map(|sym| *sym),
            bli_srandnv_unb_var1: get_symbol(&libs, b"bli_srandnv_unb_var1\0").map(|sym| *sym),
            bli_drandnv_unb_var1: get_symbol(&libs, b"bli_drandnv_unb_var1\0").map(|sym| *sym),
            bli_crandnv_unb_var1: get_symbol(&libs, b"bli_crandnv_unb_var1\0").map(|sym| *sym),
            bli_zrandnv_unb_var1: get_symbol(&libs, b"bli_zrandnv_unb_var1\0").map(|sym| *sym),
            bli_srandm_unb_var1: get_symbol(&libs, b"bli_srandm_unb_var1\0").map(|sym| *sym),
            bli_drandm_unb_var1: get_symbol(&libs, b"bli_drandm_unb_var1\0").map(|sym| *sym),
            bli_crandm_unb_var1: get_symbol(&libs, b"bli_crandm_unb_var1\0").map(|sym| *sym),
            bli_zrandm_unb_var1: get_symbol(&libs, b"bli_zrandm_unb_var1\0").map(|sym| *sym),
            bli_srandnm_unb_var1: get_symbol(&libs, b"bli_srandnm_unb_var1\0").map(|sym| *sym),
            bli_drandnm_unb_var1: get_symbol(&libs, b"bli_drandnm_unb_var1\0").map(|sym| *sym),
            bli_crandnm_unb_var1: get_symbol(&libs, b"bli_crandnm_unb_var1\0").map(|sym| *sym),
            bli_zrandnm_unb_var1: get_symbol(&libs, b"bli_zrandnm_unb_var1\0").map(|sym| *sym),
            bli_ssumsqv_unb_var1: get_symbol(&libs, b"bli_ssumsqv_unb_var1\0").map(|sym| *sym),
            bli_dsumsqv_unb_var1: get_symbol(&libs, b"bli_dsumsqv_unb_var1\0").map(|sym| *sym),
            bli_csumsqv_unb_var1: get_symbol(&libs, b"bli_csumsqv_unb_var1\0").map(|sym| *sym),
            bli_zsumsqv_unb_var1: get_symbol(&libs, b"bli_zsumsqv_unb_var1\0").map(|sym| *sym),
            bli_seqv_unb_var1: get_symbol(&libs, b"bli_seqv_unb_var1\0").map(|sym| *sym),
            bli_deqv_unb_var1: get_symbol(&libs, b"bli_deqv_unb_var1\0").map(|sym| *sym),
            bli_ceqv_unb_var1: get_symbol(&libs, b"bli_ceqv_unb_var1\0").map(|sym| *sym),
            bli_zeqv_unb_var1: get_symbol(&libs, b"bli_zeqv_unb_var1\0").map(|sym| *sym),
            bli_seqm_unb_var1: get_symbol(&libs, b"bli_seqm_unb_var1\0").map(|sym| *sym),
            bli_deqm_unb_var1: get_symbol(&libs, b"bli_deqm_unb_var1\0").map(|sym| *sym),
            bli_ceqm_unb_var1: get_symbol(&libs, b"bli_ceqm_unb_var1\0").map(|sym| *sym),
            bli_zeqm_unb_var1: get_symbol(&libs, b"bli_zeqm_unb_var1\0").map(|sym| *sym),
            bli_sfprintv: get_symbol(&libs, b"bli_sfprintv\0").map(|sym| *sym),
            bli_dfprintv: get_symbol(&libs, b"bli_dfprintv\0").map(|sym| *sym),
            bli_cfprintv: get_symbol(&libs, b"bli_cfprintv\0").map(|sym| *sym),
            bli_zfprintv: get_symbol(&libs, b"bli_zfprintv\0").map(|sym| *sym),
            bli_ifprintv: get_symbol(&libs, b"bli_ifprintv\0").map(|sym| *sym),
            bli_sfprintm: get_symbol(&libs, b"bli_sfprintm\0").map(|sym| *sym),
            bli_dfprintm: get_symbol(&libs, b"bli_dfprintm\0").map(|sym| *sym),
            bli_cfprintm: get_symbol(&libs, b"bli_cfprintm\0").map(|sym| *sym),
            bli_zfprintm: get_symbol(&libs, b"bli_zfprintm\0").map(|sym| *sym),
            bli_ifprintm: get_symbol(&libs, b"bli_ifprintm\0").map(|sym| *sym),
            bli_supdate_lower_triang: get_symbol(&libs, b"bli_supdate_lower_triang\0")
                .map(|sym| *sym),
            bli_dupdate_lower_triang: get_symbol(&libs, b"bli_dupdate_lower_triang\0")
                .map(|sym| *sym),
            bli_cupdate_lower_triang: get_symbol(&libs, b"bli_cupdate_lower_triang\0")
                .map(|sym| *sym),
            bli_zupdate_lower_triang: get_symbol(&libs, b"bli_zupdate_lower_triang\0")
                .map(|sym| *sym),
            bli_supdate_upper_triang: get_symbol(&libs, b"bli_supdate_upper_triang\0")
                .map(|sym| *sym),
            bli_dupdate_upper_triang: get_symbol(&libs, b"bli_dupdate_upper_triang\0")
                .map(|sym| *sym),
            bli_cupdate_upper_triang: get_symbol(&libs, b"bli_cupdate_upper_triang\0")
                .map(|sym| *sym),
            bli_zupdate_upper_triang: get_symbol(&libs, b"bli_zupdate_upper_triang\0")
                .map(|sym| *sym),
            SROTG: get_symbol(&libs, b"SROTG\0").map(|sym| *sym),
            srotg: get_symbol(&libs, b"srotg\0").map(|sym| *sym),
            SROTG_: get_symbol(&libs, b"SROTG_\0").map(|sym| *sym),
            SROTMG: get_symbol(&libs, b"SROTMG\0").map(|sym| *sym),
            srotmg: get_symbol(&libs, b"srotmg\0").map(|sym| *sym),
            SROTMG_: get_symbol(&libs, b"SROTMG_\0").map(|sym| *sym),
            SROT: get_symbol(&libs, b"SROT\0").map(|sym| *sym),
            srot: get_symbol(&libs, b"srot\0").map(|sym| *sym),
            SROT_: get_symbol(&libs, b"SROT_\0").map(|sym| *sym),
            SROTM: get_symbol(&libs, b"SROTM\0").map(|sym| *sym),
            srotm: get_symbol(&libs, b"srotm\0").map(|sym| *sym),
            SROTM_: get_symbol(&libs, b"SROTM_\0").map(|sym| *sym),
            SSWAP: get_symbol(&libs, b"SSWAP\0").map(|sym| *sym),
            sswap: get_symbol(&libs, b"sswap\0").map(|sym| *sym),
            SSWAP_: get_symbol(&libs, b"SSWAP_\0").map(|sym| *sym),
            SSCAL: get_symbol(&libs, b"SSCAL\0").map(|sym| *sym),
            sscal: get_symbol(&libs, b"sscal\0").map(|sym| *sym),
            SSCAL_: get_symbol(&libs, b"SSCAL_\0").map(|sym| *sym),
            SCOPY: get_symbol(&libs, b"SCOPY\0").map(|sym| *sym),
            scopy: get_symbol(&libs, b"scopy\0").map(|sym| *sym),
            SCOPY_: get_symbol(&libs, b"SCOPY_\0").map(|sym| *sym),
            SAXPY: get_symbol(&libs, b"SAXPY\0").map(|sym| *sym),
            saxpy: get_symbol(&libs, b"saxpy\0").map(|sym| *sym),
            SAXPY_: get_symbol(&libs, b"SAXPY_\0").map(|sym| *sym),
            SDOT: get_symbol(&libs, b"SDOT\0").map(|sym| *sym),
            sdot: get_symbol(&libs, b"sdot\0").map(|sym| *sym),
            SDOT_: get_symbol(&libs, b"SDOT_\0").map(|sym| *sym),
            SDSDOT: get_symbol(&libs, b"SDSDOT\0").map(|sym| *sym),
            sdsdot: get_symbol(&libs, b"sdsdot\0").map(|sym| *sym),
            SDSDOT_: get_symbol(&libs, b"SDSDOT_\0").map(|sym| *sym),
            SNRM2: get_symbol(&libs, b"SNRM2\0").map(|sym| *sym),
            snrm2: get_symbol(&libs, b"snrm2\0").map(|sym| *sym),
            SNRM2_: get_symbol(&libs, b"SNRM2_\0").map(|sym| *sym),
            SCNRM2: get_symbol(&libs, b"SCNRM2\0").map(|sym| *sym),
            scnrm2: get_symbol(&libs, b"scnrm2\0").map(|sym| *sym),
            SCNRM2_: get_symbol(&libs, b"SCNRM2_\0").map(|sym| *sym),
            SASUM: get_symbol(&libs, b"SASUM\0").map(|sym| *sym),
            sasum: get_symbol(&libs, b"sasum\0").map(|sym| *sym),
            SASUM_: get_symbol(&libs, b"SASUM_\0").map(|sym| *sym),
            ISAMAX: get_symbol(&libs, b"ISAMAX\0").map(|sym| *sym),
            isamax: get_symbol(&libs, b"isamax\0").map(|sym| *sym),
            ISAMAX_: get_symbol(&libs, b"ISAMAX_\0").map(|sym| *sym),
            DROTG: get_symbol(&libs, b"DROTG\0").map(|sym| *sym),
            drotg: get_symbol(&libs, b"drotg\0").map(|sym| *sym),
            DROTG_: get_symbol(&libs, b"DROTG_\0").map(|sym| *sym),
            DROTMG: get_symbol(&libs, b"DROTMG\0").map(|sym| *sym),
            drotmg: get_symbol(&libs, b"drotmg\0").map(|sym| *sym),
            DROTMG_: get_symbol(&libs, b"DROTMG_\0").map(|sym| *sym),
            DROT: get_symbol(&libs, b"DROT\0").map(|sym| *sym),
            drot: get_symbol(&libs, b"drot\0").map(|sym| *sym),
            DROT_: get_symbol(&libs, b"DROT_\0").map(|sym| *sym),
            DROTM: get_symbol(&libs, b"DROTM\0").map(|sym| *sym),
            drotm: get_symbol(&libs, b"drotm\0").map(|sym| *sym),
            DROTM_: get_symbol(&libs, b"DROTM_\0").map(|sym| *sym),
            DSWAP: get_symbol(&libs, b"DSWAP\0").map(|sym| *sym),
            dswap: get_symbol(&libs, b"dswap\0").map(|sym| *sym),
            DSWAP_: get_symbol(&libs, b"DSWAP_\0").map(|sym| *sym),
            DSCAL: get_symbol(&libs, b"DSCAL\0").map(|sym| *sym),
            dscal: get_symbol(&libs, b"dscal\0").map(|sym| *sym),
            DSCAL_: get_symbol(&libs, b"DSCAL_\0").map(|sym| *sym),
            DCOPY: get_symbol(&libs, b"DCOPY\0").map(|sym| *sym),
            dcopy: get_symbol(&libs, b"dcopy\0").map(|sym| *sym),
            DCOPY_: get_symbol(&libs, b"DCOPY_\0").map(|sym| *sym),
            DAXPY: get_symbol(&libs, b"DAXPY\0").map(|sym| *sym),
            daxpy: get_symbol(&libs, b"daxpy\0").map(|sym| *sym),
            DAXPY_: get_symbol(&libs, b"DAXPY_\0").map(|sym| *sym),
            DDOT: get_symbol(&libs, b"DDOT\0").map(|sym| *sym),
            ddot: get_symbol(&libs, b"ddot\0").map(|sym| *sym),
            DDOT_: get_symbol(&libs, b"DDOT_\0").map(|sym| *sym),
            DSDOT: get_symbol(&libs, b"DSDOT\0").map(|sym| *sym),
            dsdot: get_symbol(&libs, b"dsdot\0").map(|sym| *sym),
            DSDOT_: get_symbol(&libs, b"DSDOT_\0").map(|sym| *sym),
            DNRM2: get_symbol(&libs, b"DNRM2\0").map(|sym| *sym),
            dnrm2: get_symbol(&libs, b"dnrm2\0").map(|sym| *sym),
            DNRM2_: get_symbol(&libs, b"DNRM2_\0").map(|sym| *sym),
            DZNRM2: get_symbol(&libs, b"DZNRM2\0").map(|sym| *sym),
            dznrm2: get_symbol(&libs, b"dznrm2\0").map(|sym| *sym),
            DZNRM2_: get_symbol(&libs, b"DZNRM2_\0").map(|sym| *sym),
            DASUM: get_symbol(&libs, b"DASUM\0").map(|sym| *sym),
            dasum: get_symbol(&libs, b"dasum\0").map(|sym| *sym),
            DASUM_: get_symbol(&libs, b"DASUM_\0").map(|sym| *sym),
            IDAMAX: get_symbol(&libs, b"IDAMAX\0").map(|sym| *sym),
            idamax: get_symbol(&libs, b"idamax\0").map(|sym| *sym),
            IDAMAX_: get_symbol(&libs, b"IDAMAX_\0").map(|sym| *sym),
            CROTG: get_symbol(&libs, b"CROTG\0").map(|sym| *sym),
            crotg: get_symbol(&libs, b"crotg\0").map(|sym| *sym),
            CROTG_: get_symbol(&libs, b"CROTG_\0").map(|sym| *sym),
            CSROT: get_symbol(&libs, b"CSROT\0").map(|sym| *sym),
            csrot: get_symbol(&libs, b"csrot\0").map(|sym| *sym),
            CSROT_: get_symbol(&libs, b"CSROT_\0").map(|sym| *sym),
            CSWAP: get_symbol(&libs, b"CSWAP\0").map(|sym| *sym),
            cswap: get_symbol(&libs, b"cswap\0").map(|sym| *sym),
            CSWAP_: get_symbol(&libs, b"CSWAP_\0").map(|sym| *sym),
            CSCAL: get_symbol(&libs, b"CSCAL\0").map(|sym| *sym),
            cscal: get_symbol(&libs, b"cscal\0").map(|sym| *sym),
            CSCAL_: get_symbol(&libs, b"CSCAL_\0").map(|sym| *sym),
            CSSCAL: get_symbol(&libs, b"CSSCAL\0").map(|sym| *sym),
            csscal: get_symbol(&libs, b"csscal\0").map(|sym| *sym),
            CSSCAL_: get_symbol(&libs, b"CSSCAL_\0").map(|sym| *sym),
            CCOPY: get_symbol(&libs, b"CCOPY\0").map(|sym| *sym),
            ccopy: get_symbol(&libs, b"ccopy\0").map(|sym| *sym),
            CCOPY_: get_symbol(&libs, b"CCOPY_\0").map(|sym| *sym),
            CAXPY: get_symbol(&libs, b"CAXPY\0").map(|sym| *sym),
            caxpy: get_symbol(&libs, b"caxpy\0").map(|sym| *sym),
            CAXPY_: get_symbol(&libs, b"CAXPY_\0").map(|sym| *sym),
            CDOTC: get_symbol(&libs, b"CDOTC\0").map(|sym| *sym),
            cdotc: get_symbol(&libs, b"cdotc\0").map(|sym| *sym),
            CDOTC_: get_symbol(&libs, b"CDOTC_\0").map(|sym| *sym),
            CDOTU: get_symbol(&libs, b"CDOTU\0").map(|sym| *sym),
            cdotu: get_symbol(&libs, b"cdotu\0").map(|sym| *sym),
            CDOTU_: get_symbol(&libs, b"CDOTU_\0").map(|sym| *sym),
            ZDOTC: get_symbol(&libs, b"ZDOTC\0").map(|sym| *sym),
            zdotc: get_symbol(&libs, b"zdotc\0").map(|sym| *sym),
            ZDOTC_: get_symbol(&libs, b"ZDOTC_\0").map(|sym| *sym),
            ZDOTU: get_symbol(&libs, b"ZDOTU\0").map(|sym| *sym),
            zdotu: get_symbol(&libs, b"zdotu\0").map(|sym| *sym),
            ZDOTU_: get_symbol(&libs, b"ZDOTU_\0").map(|sym| *sym),
            SCASUM: get_symbol(&libs, b"SCASUM\0").map(|sym| *sym),
            scasum: get_symbol(&libs, b"scasum\0").map(|sym| *sym),
            SCASUM_: get_symbol(&libs, b"SCASUM_\0").map(|sym| *sym),
            ICAMAX: get_symbol(&libs, b"ICAMAX\0").map(|sym| *sym),
            icamax: get_symbol(&libs, b"icamax\0").map(|sym| *sym),
            ICAMAX_: get_symbol(&libs, b"ICAMAX_\0").map(|sym| *sym),
            ZROTG: get_symbol(&libs, b"ZROTG\0").map(|sym| *sym),
            zrotg: get_symbol(&libs, b"zrotg\0").map(|sym| *sym),
            ZROTG_: get_symbol(&libs, b"ZROTG_\0").map(|sym| *sym),
            ZDROT: get_symbol(&libs, b"ZDROT\0").map(|sym| *sym),
            zdrot: get_symbol(&libs, b"zdrot\0").map(|sym| *sym),
            ZDROT_: get_symbol(&libs, b"ZDROT_\0").map(|sym| *sym),
            ZSWAP: get_symbol(&libs, b"ZSWAP\0").map(|sym| *sym),
            zswap: get_symbol(&libs, b"zswap\0").map(|sym| *sym),
            ZSWAP_: get_symbol(&libs, b"ZSWAP_\0").map(|sym| *sym),
            ZSCAL: get_symbol(&libs, b"ZSCAL\0").map(|sym| *sym),
            zscal: get_symbol(&libs, b"zscal\0").map(|sym| *sym),
            ZSCAL_: get_symbol(&libs, b"ZSCAL_\0").map(|sym| *sym),
            ZDSCAL: get_symbol(&libs, b"ZDSCAL\0").map(|sym| *sym),
            zdscal: get_symbol(&libs, b"zdscal\0").map(|sym| *sym),
            ZDSCAL_: get_symbol(&libs, b"ZDSCAL_\0").map(|sym| *sym),
            ZCOPY: get_symbol(&libs, b"ZCOPY\0").map(|sym| *sym),
            zcopy: get_symbol(&libs, b"zcopy\0").map(|sym| *sym),
            ZCOPY_: get_symbol(&libs, b"ZCOPY_\0").map(|sym| *sym),
            ZAXPY: get_symbol(&libs, b"ZAXPY\0").map(|sym| *sym),
            zaxpy: get_symbol(&libs, b"zaxpy\0").map(|sym| *sym),
            ZAXPY_: get_symbol(&libs, b"ZAXPY_\0").map(|sym| *sym),
            DZASUM: get_symbol(&libs, b"DZASUM\0").map(|sym| *sym),
            dzasum: get_symbol(&libs, b"dzasum\0").map(|sym| *sym),
            DZASUM_: get_symbol(&libs, b"DZASUM_\0").map(|sym| *sym),
            IZAMAX: get_symbol(&libs, b"IZAMAX\0").map(|sym| *sym),
            izamax: get_symbol(&libs, b"izamax\0").map(|sym| *sym),
            IZAMAX_: get_symbol(&libs, b"IZAMAX_\0").map(|sym| *sym),
            ICAMIN: get_symbol(&libs, b"ICAMIN\0").map(|sym| *sym),
            icamin: get_symbol(&libs, b"icamin\0").map(|sym| *sym),
            ICAMIN_: get_symbol(&libs, b"ICAMIN_\0").map(|sym| *sym),
            IDAMIN: get_symbol(&libs, b"IDAMIN\0").map(|sym| *sym),
            idamin: get_symbol(&libs, b"idamin\0").map(|sym| *sym),
            IDAMIN_: get_symbol(&libs, b"IDAMIN_\0").map(|sym| *sym),
            ISAMIN: get_symbol(&libs, b"ISAMIN\0").map(|sym| *sym),
            isamin: get_symbol(&libs, b"isamin\0").map(|sym| *sym),
            ISAMIN_: get_symbol(&libs, b"ISAMIN_\0").map(|sym| *sym),
            IZAMIN: get_symbol(&libs, b"IZAMIN\0").map(|sym| *sym),
            izamin: get_symbol(&libs, b"izamin\0").map(|sym| *sym),
            IZAMIN_: get_symbol(&libs, b"IZAMIN_\0").map(|sym| *sym),
            SGEMV: get_symbol(&libs, b"SGEMV\0").map(|sym| *sym),
            sgemv: get_symbol(&libs, b"sgemv\0").map(|sym| *sym),
            SGEMV_: get_symbol(&libs, b"SGEMV_\0").map(|sym| *sym),
            SGBMV: get_symbol(&libs, b"SGBMV\0").map(|sym| *sym),
            sgbmv: get_symbol(&libs, b"sgbmv\0").map(|sym| *sym),
            SGBMV_: get_symbol(&libs, b"SGBMV_\0").map(|sym| *sym),
            SSYMV: get_symbol(&libs, b"SSYMV\0").map(|sym| *sym),
            ssymv: get_symbol(&libs, b"ssymv\0").map(|sym| *sym),
            SSYMV_: get_symbol(&libs, b"SSYMV_\0").map(|sym| *sym),
            SSBMV: get_symbol(&libs, b"SSBMV\0").map(|sym| *sym),
            ssbmv: get_symbol(&libs, b"ssbmv\0").map(|sym| *sym),
            SSBMV_: get_symbol(&libs, b"SSBMV_\0").map(|sym| *sym),
            SSPMV: get_symbol(&libs, b"SSPMV\0").map(|sym| *sym),
            sspmv: get_symbol(&libs, b"sspmv\0").map(|sym| *sym),
            SSPMV_: get_symbol(&libs, b"SSPMV_\0").map(|sym| *sym),
            STRMV: get_symbol(&libs, b"STRMV\0").map(|sym| *sym),
            strmv: get_symbol(&libs, b"strmv\0").map(|sym| *sym),
            STRMV_: get_symbol(&libs, b"STRMV_\0").map(|sym| *sym),
            STBMV: get_symbol(&libs, b"STBMV\0").map(|sym| *sym),
            stbmv: get_symbol(&libs, b"stbmv\0").map(|sym| *sym),
            STBMV_: get_symbol(&libs, b"STBMV_\0").map(|sym| *sym),
            STPMV: get_symbol(&libs, b"STPMV\0").map(|sym| *sym),
            stpmv: get_symbol(&libs, b"stpmv\0").map(|sym| *sym),
            STPMV_: get_symbol(&libs, b"STPMV_\0").map(|sym| *sym),
            STRSV: get_symbol(&libs, b"STRSV\0").map(|sym| *sym),
            strsv: get_symbol(&libs, b"strsv\0").map(|sym| *sym),
            STRSV_: get_symbol(&libs, b"STRSV_\0").map(|sym| *sym),
            STBSV: get_symbol(&libs, b"STBSV\0").map(|sym| *sym),
            stbsv: get_symbol(&libs, b"stbsv\0").map(|sym| *sym),
            STBSV_: get_symbol(&libs, b"STBSV_\0").map(|sym| *sym),
            STPSV: get_symbol(&libs, b"STPSV\0").map(|sym| *sym),
            stpsv: get_symbol(&libs, b"stpsv\0").map(|sym| *sym),
            STPSV_: get_symbol(&libs, b"STPSV_\0").map(|sym| *sym),
            SGER: get_symbol(&libs, b"SGER\0").map(|sym| *sym),
            sger: get_symbol(&libs, b"sger\0").map(|sym| *sym),
            SGER_: get_symbol(&libs, b"SGER_\0").map(|sym| *sym),
            SSYR: get_symbol(&libs, b"SSYR\0").map(|sym| *sym),
            ssyr: get_symbol(&libs, b"ssyr\0").map(|sym| *sym),
            SSYR_: get_symbol(&libs, b"SSYR_\0").map(|sym| *sym),
            SSPR: get_symbol(&libs, b"SSPR\0").map(|sym| *sym),
            sspr: get_symbol(&libs, b"sspr\0").map(|sym| *sym),
            SSPR_: get_symbol(&libs, b"SSPR_\0").map(|sym| *sym),
            SSYR2: get_symbol(&libs, b"SSYR2\0").map(|sym| *sym),
            ssyr2: get_symbol(&libs, b"ssyr2\0").map(|sym| *sym),
            SSYR2_: get_symbol(&libs, b"SSYR2_\0").map(|sym| *sym),
            SSPR2: get_symbol(&libs, b"SSPR2\0").map(|sym| *sym),
            sspr2: get_symbol(&libs, b"sspr2\0").map(|sym| *sym),
            SSPR2_: get_symbol(&libs, b"SSPR2_\0").map(|sym| *sym),
            DGEMV: get_symbol(&libs, b"DGEMV\0").map(|sym| *sym),
            dgemv: get_symbol(&libs, b"dgemv\0").map(|sym| *sym),
            DGEMV_: get_symbol(&libs, b"DGEMV_\0").map(|sym| *sym),
            DGBMV: get_symbol(&libs, b"DGBMV\0").map(|sym| *sym),
            dgbmv: get_symbol(&libs, b"dgbmv\0").map(|sym| *sym),
            DGBMV_: get_symbol(&libs, b"DGBMV_\0").map(|sym| *sym),
            DSYMV: get_symbol(&libs, b"DSYMV\0").map(|sym| *sym),
            dsymv: get_symbol(&libs, b"dsymv\0").map(|sym| *sym),
            DSYMV_: get_symbol(&libs, b"DSYMV_\0").map(|sym| *sym),
            DSBMV: get_symbol(&libs, b"DSBMV\0").map(|sym| *sym),
            dsbmv: get_symbol(&libs, b"dsbmv\0").map(|sym| *sym),
            DSBMV_: get_symbol(&libs, b"DSBMV_\0").map(|sym| *sym),
            DSPMV: get_symbol(&libs, b"DSPMV\0").map(|sym| *sym),
            dspmv: get_symbol(&libs, b"dspmv\0").map(|sym| *sym),
            DSPMV_: get_symbol(&libs, b"DSPMV_\0").map(|sym| *sym),
            DTRMV: get_symbol(&libs, b"DTRMV\0").map(|sym| *sym),
            dtrmv: get_symbol(&libs, b"dtrmv\0").map(|sym| *sym),
            DTRMV_: get_symbol(&libs, b"DTRMV_\0").map(|sym| *sym),
            DTBMV: get_symbol(&libs, b"DTBMV\0").map(|sym| *sym),
            dtbmv: get_symbol(&libs, b"dtbmv\0").map(|sym| *sym),
            DTBMV_: get_symbol(&libs, b"DTBMV_\0").map(|sym| *sym),
            DTPMV: get_symbol(&libs, b"DTPMV\0").map(|sym| *sym),
            dtpmv: get_symbol(&libs, b"dtpmv\0").map(|sym| *sym),
            DTPMV_: get_symbol(&libs, b"DTPMV_\0").map(|sym| *sym),
            DTRSV: get_symbol(&libs, b"DTRSV\0").map(|sym| *sym),
            dtrsv: get_symbol(&libs, b"dtrsv\0").map(|sym| *sym),
            DTRSV_: get_symbol(&libs, b"DTRSV_\0").map(|sym| *sym),
            DTBSV: get_symbol(&libs, b"DTBSV\0").map(|sym| *sym),
            dtbsv: get_symbol(&libs, b"dtbsv\0").map(|sym| *sym),
            DTBSV_: get_symbol(&libs, b"DTBSV_\0").map(|sym| *sym),
            DTPSV: get_symbol(&libs, b"DTPSV\0").map(|sym| *sym),
            dtpsv: get_symbol(&libs, b"dtpsv\0").map(|sym| *sym),
            DTPSV_: get_symbol(&libs, b"DTPSV_\0").map(|sym| *sym),
            DGER: get_symbol(&libs, b"DGER\0").map(|sym| *sym),
            dger: get_symbol(&libs, b"dger\0").map(|sym| *sym),
            DGER_: get_symbol(&libs, b"DGER_\0").map(|sym| *sym),
            DSYR: get_symbol(&libs, b"DSYR\0").map(|sym| *sym),
            dsyr: get_symbol(&libs, b"dsyr\0").map(|sym| *sym),
            DSYR_: get_symbol(&libs, b"DSYR_\0").map(|sym| *sym),
            DSPR: get_symbol(&libs, b"DSPR\0").map(|sym| *sym),
            dspr: get_symbol(&libs, b"dspr\0").map(|sym| *sym),
            DSPR_: get_symbol(&libs, b"DSPR_\0").map(|sym| *sym),
            DSYR2: get_symbol(&libs, b"DSYR2\0").map(|sym| *sym),
            dsyr2: get_symbol(&libs, b"dsyr2\0").map(|sym| *sym),
            DSYR2_: get_symbol(&libs, b"DSYR2_\0").map(|sym| *sym),
            DSPR2: get_symbol(&libs, b"DSPR2\0").map(|sym| *sym),
            dspr2: get_symbol(&libs, b"dspr2\0").map(|sym| *sym),
            DSPR2_: get_symbol(&libs, b"DSPR2_\0").map(|sym| *sym),
            CGEMV: get_symbol(&libs, b"CGEMV\0").map(|sym| *sym),
            cgemv: get_symbol(&libs, b"cgemv\0").map(|sym| *sym),
            CGEMV_: get_symbol(&libs, b"CGEMV_\0").map(|sym| *sym),
            CGBMV: get_symbol(&libs, b"CGBMV\0").map(|sym| *sym),
            cgbmv: get_symbol(&libs, b"cgbmv\0").map(|sym| *sym),
            CGBMV_: get_symbol(&libs, b"CGBMV_\0").map(|sym| *sym),
            CHEMV: get_symbol(&libs, b"CHEMV\0").map(|sym| *sym),
            chemv: get_symbol(&libs, b"chemv\0").map(|sym| *sym),
            CHEMV_: get_symbol(&libs, b"CHEMV_\0").map(|sym| *sym),
            CHBMV: get_symbol(&libs, b"CHBMV\0").map(|sym| *sym),
            chbmv: get_symbol(&libs, b"chbmv\0").map(|sym| *sym),
            CHBMV_: get_symbol(&libs, b"CHBMV_\0").map(|sym| *sym),
            CHPMV: get_symbol(&libs, b"CHPMV\0").map(|sym| *sym),
            chpmv: get_symbol(&libs, b"chpmv\0").map(|sym| *sym),
            CHPMV_: get_symbol(&libs, b"CHPMV_\0").map(|sym| *sym),
            CTRMV: get_symbol(&libs, b"CTRMV\0").map(|sym| *sym),
            ctrmv: get_symbol(&libs, b"ctrmv\0").map(|sym| *sym),
            CTRMV_: get_symbol(&libs, b"CTRMV_\0").map(|sym| *sym),
            CTBMV: get_symbol(&libs, b"CTBMV\0").map(|sym| *sym),
            ctbmv: get_symbol(&libs, b"ctbmv\0").map(|sym| *sym),
            CTBMV_: get_symbol(&libs, b"CTBMV_\0").map(|sym| *sym),
            CTPMV: get_symbol(&libs, b"CTPMV\0").map(|sym| *sym),
            ctpmv: get_symbol(&libs, b"ctpmv\0").map(|sym| *sym),
            CTPMV_: get_symbol(&libs, b"CTPMV_\0").map(|sym| *sym),
            CTRSV: get_symbol(&libs, b"CTRSV\0").map(|sym| *sym),
            ctrsv: get_symbol(&libs, b"ctrsv\0").map(|sym| *sym),
            CTRSV_: get_symbol(&libs, b"CTRSV_\0").map(|sym| *sym),
            CTBSV: get_symbol(&libs, b"CTBSV\0").map(|sym| *sym),
            ctbsv: get_symbol(&libs, b"ctbsv\0").map(|sym| *sym),
            CTBSV_: get_symbol(&libs, b"CTBSV_\0").map(|sym| *sym),
            CTPSV: get_symbol(&libs, b"CTPSV\0").map(|sym| *sym),
            ctpsv: get_symbol(&libs, b"ctpsv\0").map(|sym| *sym),
            CTPSV_: get_symbol(&libs, b"CTPSV_\0").map(|sym| *sym),
            CGERC: get_symbol(&libs, b"CGERC\0").map(|sym| *sym),
            cgerc: get_symbol(&libs, b"cgerc\0").map(|sym| *sym),
            CGERC_: get_symbol(&libs, b"CGERC_\0").map(|sym| *sym),
            CGERU: get_symbol(&libs, b"CGERU\0").map(|sym| *sym),
            cgeru: get_symbol(&libs, b"cgeru\0").map(|sym| *sym),
            CGERU_: get_symbol(&libs, b"CGERU_\0").map(|sym| *sym),
            CHER: get_symbol(&libs, b"CHER\0").map(|sym| *sym),
            cher: get_symbol(&libs, b"cher\0").map(|sym| *sym),
            CHER_: get_symbol(&libs, b"CHER_\0").map(|sym| *sym),
            CHPR: get_symbol(&libs, b"CHPR\0").map(|sym| *sym),
            chpr: get_symbol(&libs, b"chpr\0").map(|sym| *sym),
            CHPR_: get_symbol(&libs, b"CHPR_\0").map(|sym| *sym),
            CHER2: get_symbol(&libs, b"CHER2\0").map(|sym| *sym),
            cher2: get_symbol(&libs, b"cher2\0").map(|sym| *sym),
            CHER2_: get_symbol(&libs, b"CHER2_\0").map(|sym| *sym),
            CHPR2: get_symbol(&libs, b"CHPR2\0").map(|sym| *sym),
            chpr2: get_symbol(&libs, b"chpr2\0").map(|sym| *sym),
            CHPR2_: get_symbol(&libs, b"CHPR2_\0").map(|sym| *sym),
            ZGEMV: get_symbol(&libs, b"ZGEMV\0").map(|sym| *sym),
            zgemv: get_symbol(&libs, b"zgemv\0").map(|sym| *sym),
            ZGEMV_: get_symbol(&libs, b"ZGEMV_\0").map(|sym| *sym),
            ZGBMV: get_symbol(&libs, b"ZGBMV\0").map(|sym| *sym),
            zgbmv: get_symbol(&libs, b"zgbmv\0").map(|sym| *sym),
            ZGBMV_: get_symbol(&libs, b"ZGBMV_\0").map(|sym| *sym),
            ZHEMV: get_symbol(&libs, b"ZHEMV\0").map(|sym| *sym),
            zhemv: get_symbol(&libs, b"zhemv\0").map(|sym| *sym),
            ZHEMV_: get_symbol(&libs, b"ZHEMV_\0").map(|sym| *sym),
            ZHBMV: get_symbol(&libs, b"ZHBMV\0").map(|sym| *sym),
            zhbmv: get_symbol(&libs, b"zhbmv\0").map(|sym| *sym),
            ZHBMV_: get_symbol(&libs, b"ZHBMV_\0").map(|sym| *sym),
            ZHPMV: get_symbol(&libs, b"ZHPMV\0").map(|sym| *sym),
            zhpmv: get_symbol(&libs, b"zhpmv\0").map(|sym| *sym),
            ZHPMV_: get_symbol(&libs, b"ZHPMV_\0").map(|sym| *sym),
            ZTRMV: get_symbol(&libs, b"ZTRMV\0").map(|sym| *sym),
            ztrmv: get_symbol(&libs, b"ztrmv\0").map(|sym| *sym),
            ZTRMV_: get_symbol(&libs, b"ZTRMV_\0").map(|sym| *sym),
            ZTBMV: get_symbol(&libs, b"ZTBMV\0").map(|sym| *sym),
            ztbmv: get_symbol(&libs, b"ztbmv\0").map(|sym| *sym),
            ZTBMV_: get_symbol(&libs, b"ZTBMV_\0").map(|sym| *sym),
            ZTPMV: get_symbol(&libs, b"ZTPMV\0").map(|sym| *sym),
            ztpmv: get_symbol(&libs, b"ztpmv\0").map(|sym| *sym),
            ZTPMV_: get_symbol(&libs, b"ZTPMV_\0").map(|sym| *sym),
            ZTRSV: get_symbol(&libs, b"ZTRSV\0").map(|sym| *sym),
            ztrsv: get_symbol(&libs, b"ztrsv\0").map(|sym| *sym),
            ZTRSV_: get_symbol(&libs, b"ZTRSV_\0").map(|sym| *sym),
            ZTBSV: get_symbol(&libs, b"ZTBSV\0").map(|sym| *sym),
            ztbsv: get_symbol(&libs, b"ztbsv\0").map(|sym| *sym),
            ZTBSV_: get_symbol(&libs, b"ZTBSV_\0").map(|sym| *sym),
            ZTPSV: get_symbol(&libs, b"ZTPSV\0").map(|sym| *sym),
            ztpsv: get_symbol(&libs, b"ztpsv\0").map(|sym| *sym),
            ZTPSV_: get_symbol(&libs, b"ZTPSV_\0").map(|sym| *sym),
            ZGERU: get_symbol(&libs, b"ZGERU\0").map(|sym| *sym),
            zgeru: get_symbol(&libs, b"zgeru\0").map(|sym| *sym),
            ZGERU_: get_symbol(&libs, b"ZGERU_\0").map(|sym| *sym),
            ZGERC: get_symbol(&libs, b"ZGERC\0").map(|sym| *sym),
            zgerc: get_symbol(&libs, b"zgerc\0").map(|sym| *sym),
            ZGERC_: get_symbol(&libs, b"ZGERC_\0").map(|sym| *sym),
            ZHER: get_symbol(&libs, b"ZHER\0").map(|sym| *sym),
            zher: get_symbol(&libs, b"zher\0").map(|sym| *sym),
            ZHER_: get_symbol(&libs, b"ZHER_\0").map(|sym| *sym),
            ZHPR: get_symbol(&libs, b"ZHPR\0").map(|sym| *sym),
            zhpr: get_symbol(&libs, b"zhpr\0").map(|sym| *sym),
            ZHPR_: get_symbol(&libs, b"ZHPR_\0").map(|sym| *sym),
            ZHER2: get_symbol(&libs, b"ZHER2\0").map(|sym| *sym),
            zher2: get_symbol(&libs, b"zher2\0").map(|sym| *sym),
            ZHER2_: get_symbol(&libs, b"ZHER2_\0").map(|sym| *sym),
            ZHPR2: get_symbol(&libs, b"ZHPR2\0").map(|sym| *sym),
            zhpr2: get_symbol(&libs, b"zhpr2\0").map(|sym| *sym),
            ZHPR2_: get_symbol(&libs, b"ZHPR2_\0").map(|sym| *sym),
            SGEMM: get_symbol(&libs, b"SGEMM\0").map(|sym| *sym),
            sgemm: get_symbol(&libs, b"sgemm\0").map(|sym| *sym),
            SGEMM_: get_symbol(&libs, b"SGEMM_\0").map(|sym| *sym),
            SSYMM: get_symbol(&libs, b"SSYMM\0").map(|sym| *sym),
            ssymm: get_symbol(&libs, b"ssymm\0").map(|sym| *sym),
            SSYMM_: get_symbol(&libs, b"SSYMM_\0").map(|sym| *sym),
            SSYRK: get_symbol(&libs, b"SSYRK\0").map(|sym| *sym),
            ssyrk: get_symbol(&libs, b"ssyrk\0").map(|sym| *sym),
            SSYRK_: get_symbol(&libs, b"SSYRK_\0").map(|sym| *sym),
            SSYR2K: get_symbol(&libs, b"SSYR2K\0").map(|sym| *sym),
            ssyr2k: get_symbol(&libs, b"ssyr2k\0").map(|sym| *sym),
            SSYR2K_: get_symbol(&libs, b"SSYR2K_\0").map(|sym| *sym),
            STRMM: get_symbol(&libs, b"STRMM\0").map(|sym| *sym),
            strmm: get_symbol(&libs, b"strmm\0").map(|sym| *sym),
            STRMM_: get_symbol(&libs, b"STRMM_\0").map(|sym| *sym),
            STRSM: get_symbol(&libs, b"STRSM\0").map(|sym| *sym),
            strsm: get_symbol(&libs, b"strsm\0").map(|sym| *sym),
            STRSM_: get_symbol(&libs, b"STRSM_\0").map(|sym| *sym),
            DGEMM: get_symbol(&libs, b"DGEMM\0").map(|sym| *sym),
            dgemm: get_symbol(&libs, b"dgemm\0").map(|sym| *sym),
            DGEMM_: get_symbol(&libs, b"DGEMM_\0").map(|sym| *sym),
            DZGEMM: get_symbol(&libs, b"DZGEMM\0").map(|sym| *sym),
            dzgemm: get_symbol(&libs, b"dzgemm\0").map(|sym| *sym),
            DZGEMM_: get_symbol(&libs, b"DZGEMM_\0").map(|sym| *sym),
            DSYMM: get_symbol(&libs, b"DSYMM\0").map(|sym| *sym),
            dsymm: get_symbol(&libs, b"dsymm\0").map(|sym| *sym),
            DSYMM_: get_symbol(&libs, b"DSYMM_\0").map(|sym| *sym),
            DSYRK: get_symbol(&libs, b"DSYRK\0").map(|sym| *sym),
            dsyrk: get_symbol(&libs, b"dsyrk\0").map(|sym| *sym),
            DSYRK_: get_symbol(&libs, b"DSYRK_\0").map(|sym| *sym),
            DSYR2K: get_symbol(&libs, b"DSYR2K\0").map(|sym| *sym),
            dsyr2k: get_symbol(&libs, b"dsyr2k\0").map(|sym| *sym),
            DSYR2K_: get_symbol(&libs, b"DSYR2K_\0").map(|sym| *sym),
            DTRMM: get_symbol(&libs, b"DTRMM\0").map(|sym| *sym),
            dtrmm: get_symbol(&libs, b"dtrmm\0").map(|sym| *sym),
            DTRMM_: get_symbol(&libs, b"DTRMM_\0").map(|sym| *sym),
            DTRSM: get_symbol(&libs, b"DTRSM\0").map(|sym| *sym),
            dtrsm: get_symbol(&libs, b"dtrsm\0").map(|sym| *sym),
            DTRSM_: get_symbol(&libs, b"DTRSM_\0").map(|sym| *sym),
            CGEMM: get_symbol(&libs, b"CGEMM\0").map(|sym| *sym),
            cgemm: get_symbol(&libs, b"cgemm\0").map(|sym| *sym),
            CGEMM_: get_symbol(&libs, b"CGEMM_\0").map(|sym| *sym),
            CSYMM: get_symbol(&libs, b"CSYMM\0").map(|sym| *sym),
            csymm: get_symbol(&libs, b"csymm\0").map(|sym| *sym),
            CSYMM_: get_symbol(&libs, b"CSYMM_\0").map(|sym| *sym),
            CHEMM: get_symbol(&libs, b"CHEMM\0").map(|sym| *sym),
            chemm: get_symbol(&libs, b"chemm\0").map(|sym| *sym),
            CHEMM_: get_symbol(&libs, b"CHEMM_\0").map(|sym| *sym),
            CSYRK: get_symbol(&libs, b"CSYRK\0").map(|sym| *sym),
            csyrk: get_symbol(&libs, b"csyrk\0").map(|sym| *sym),
            CSYRK_: get_symbol(&libs, b"CSYRK_\0").map(|sym| *sym),
            CHERK: get_symbol(&libs, b"CHERK\0").map(|sym| *sym),
            cherk: get_symbol(&libs, b"cherk\0").map(|sym| *sym),
            CHERK_: get_symbol(&libs, b"CHERK_\0").map(|sym| *sym),
            CSYR2K: get_symbol(&libs, b"CSYR2K\0").map(|sym| *sym),
            csyr2k: get_symbol(&libs, b"csyr2k\0").map(|sym| *sym),
            CSYR2K_: get_symbol(&libs, b"CSYR2K_\0").map(|sym| *sym),
            CHER2K: get_symbol(&libs, b"CHER2K\0").map(|sym| *sym),
            cher2k: get_symbol(&libs, b"cher2k\0").map(|sym| *sym),
            CHER2K_: get_symbol(&libs, b"CHER2K_\0").map(|sym| *sym),
            CTRMM: get_symbol(&libs, b"CTRMM\0").map(|sym| *sym),
            ctrmm: get_symbol(&libs, b"ctrmm\0").map(|sym| *sym),
            CTRMM_: get_symbol(&libs, b"CTRMM_\0").map(|sym| *sym),
            CTRSM: get_symbol(&libs, b"CTRSM\0").map(|sym| *sym),
            ctrsm: get_symbol(&libs, b"ctrsm\0").map(|sym| *sym),
            CTRSM_: get_symbol(&libs, b"CTRSM_\0").map(|sym| *sym),
            ZGEMM: get_symbol(&libs, b"ZGEMM\0").map(|sym| *sym),
            zgemm: get_symbol(&libs, b"zgemm\0").map(|sym| *sym),
            ZGEMM_: get_symbol(&libs, b"ZGEMM_\0").map(|sym| *sym),
            ZSYMM: get_symbol(&libs, b"ZSYMM\0").map(|sym| *sym),
            zsymm: get_symbol(&libs, b"zsymm\0").map(|sym| *sym),
            ZSYMM_: get_symbol(&libs, b"ZSYMM_\0").map(|sym| *sym),
            ZHEMM: get_symbol(&libs, b"ZHEMM\0").map(|sym| *sym),
            zhemm: get_symbol(&libs, b"zhemm\0").map(|sym| *sym),
            ZHEMM_: get_symbol(&libs, b"ZHEMM_\0").map(|sym| *sym),
            ZSYRK: get_symbol(&libs, b"ZSYRK\0").map(|sym| *sym),
            zsyrk: get_symbol(&libs, b"zsyrk\0").map(|sym| *sym),
            ZSYRK_: get_symbol(&libs, b"ZSYRK_\0").map(|sym| *sym),
            ZHERK: get_symbol(&libs, b"ZHERK\0").map(|sym| *sym),
            zherk: get_symbol(&libs, b"zherk\0").map(|sym| *sym),
            ZHERK_: get_symbol(&libs, b"ZHERK_\0").map(|sym| *sym),
            ZSYR2K: get_symbol(&libs, b"ZSYR2K\0").map(|sym| *sym),
            zsyr2k: get_symbol(&libs, b"zsyr2k\0").map(|sym| *sym),
            ZSYR2K_: get_symbol(&libs, b"ZSYR2K_\0").map(|sym| *sym),
            ZHER2K: get_symbol(&libs, b"ZHER2K\0").map(|sym| *sym),
            zher2k: get_symbol(&libs, b"zher2k\0").map(|sym| *sym),
            ZHER2K_: get_symbol(&libs, b"ZHER2K_\0").map(|sym| *sym),
            ZTRMM: get_symbol(&libs, b"ZTRMM\0").map(|sym| *sym),
            ztrmm: get_symbol(&libs, b"ztrmm\0").map(|sym| *sym),
            ZTRMM_: get_symbol(&libs, b"ZTRMM_\0").map(|sym| *sym),
            ZTRSM: get_symbol(&libs, b"ZTRSM\0").map(|sym| *sym),
            ztrsm: get_symbol(&libs, b"ztrsm\0").map(|sym| *sym),
            ZTRSM_: get_symbol(&libs, b"ZTRSM_\0").map(|sym| *sym),
            CDOTCSUB: get_symbol(&libs, b"CDOTCSUB\0").map(|sym| *sym),
            cdotcsub: get_symbol(&libs, b"cdotcsub\0").map(|sym| *sym),
            CDOTCSUB_: get_symbol(&libs, b"CDOTCSUB_\0").map(|sym| *sym),
            CDOTUSUB: get_symbol(&libs, b"CDOTUSUB\0").map(|sym| *sym),
            cdotusub: get_symbol(&libs, b"cdotusub\0").map(|sym| *sym),
            CDOTUSUB_: get_symbol(&libs, b"CDOTUSUB_\0").map(|sym| *sym),
            DASUMSUB: get_symbol(&libs, b"DASUMSUB\0").map(|sym| *sym),
            dasumsub: get_symbol(&libs, b"dasumsub\0").map(|sym| *sym),
            DASUMSUB_: get_symbol(&libs, b"DASUMSUB_\0").map(|sym| *sym),
            DDOTSUB: get_symbol(&libs, b"DDOTSUB\0").map(|sym| *sym),
            ddotsub: get_symbol(&libs, b"ddotsub\0").map(|sym| *sym),
            DDOTSUB_: get_symbol(&libs, b"DDOTSUB_\0").map(|sym| *sym),
            DNRM2SUB: get_symbol(&libs, b"DNRM2SUB\0").map(|sym| *sym),
            dnrm2sub: get_symbol(&libs, b"dnrm2sub\0").map(|sym| *sym),
            DNRM2SUB_: get_symbol(&libs, b"DNRM2SUB_\0").map(|sym| *sym),
            DZASUMSUB: get_symbol(&libs, b"DZASUMSUB\0").map(|sym| *sym),
            dzasumsub: get_symbol(&libs, b"dzasumsub\0").map(|sym| *sym),
            DZASUMSUB_: get_symbol(&libs, b"DZASUMSUB_\0").map(|sym| *sym),
            DZNRM2SUB: get_symbol(&libs, b"DZNRM2SUB\0").map(|sym| *sym),
            dznrm2sub: get_symbol(&libs, b"dznrm2sub\0").map(|sym| *sym),
            DZNRM2SUB_: get_symbol(&libs, b"DZNRM2SUB_\0").map(|sym| *sym),
            ICAMAXSUB: get_symbol(&libs, b"ICAMAXSUB\0").map(|sym| *sym),
            icamaxsub: get_symbol(&libs, b"icamaxsub\0").map(|sym| *sym),
            ICAMAXSUB_: get_symbol(&libs, b"ICAMAXSUB_\0").map(|sym| *sym),
            ICAMINSUB: get_symbol(&libs, b"ICAMINSUB\0").map(|sym| *sym),
            icaminsub: get_symbol(&libs, b"icaminsub\0").map(|sym| *sym),
            ICAMINSUB_: get_symbol(&libs, b"ICAMINSUB_\0").map(|sym| *sym),
            IDAMAXSUB: get_symbol(&libs, b"IDAMAXSUB\0").map(|sym| *sym),
            idamaxsub: get_symbol(&libs, b"idamaxsub\0").map(|sym| *sym),
            IDAMAXSUB_: get_symbol(&libs, b"IDAMAXSUB_\0").map(|sym| *sym),
            IDAMINSUB: get_symbol(&libs, b"IDAMINSUB\0").map(|sym| *sym),
            idaminsub: get_symbol(&libs, b"idaminsub\0").map(|sym| *sym),
            IDAMINSUB_: get_symbol(&libs, b"IDAMINSUB_\0").map(|sym| *sym),
            ISAMAXSUB: get_symbol(&libs, b"ISAMAXSUB\0").map(|sym| *sym),
            isamaxsub: get_symbol(&libs, b"isamaxsub\0").map(|sym| *sym),
            ISAMAXSUB_: get_symbol(&libs, b"ISAMAXSUB_\0").map(|sym| *sym),
            ISAMINSUB: get_symbol(&libs, b"ISAMINSUB\0").map(|sym| *sym),
            isaminsub: get_symbol(&libs, b"isaminsub\0").map(|sym| *sym),
            ISAMINSUB_: get_symbol(&libs, b"ISAMINSUB_\0").map(|sym| *sym),
            IZAMINSUB: get_symbol(&libs, b"IZAMINSUB\0").map(|sym| *sym),
            izaminsub: get_symbol(&libs, b"izaminsub\0").map(|sym| *sym),
            IZAMINSUB_: get_symbol(&libs, b"IZAMINSUB_\0").map(|sym| *sym),
            IZAMAXSUB: get_symbol(&libs, b"IZAMAXSUB\0").map(|sym| *sym),
            izamaxsub: get_symbol(&libs, b"izamaxsub\0").map(|sym| *sym),
            IZAMAXSUB_: get_symbol(&libs, b"IZAMAXSUB_\0").map(|sym| *sym),
            SASUMSUB: get_symbol(&libs, b"SASUMSUB\0").map(|sym| *sym),
            sasumsub: get_symbol(&libs, b"sasumsub\0").map(|sym| *sym),
            SASUMSUB_: get_symbol(&libs, b"SASUMSUB_\0").map(|sym| *sym),
            SCASUMSUB: get_symbol(&libs, b"SCASUMSUB\0").map(|sym| *sym),
            scasumsub: get_symbol(&libs, b"scasumsub\0").map(|sym| *sym),
            SCASUMSUB_: get_symbol(&libs, b"SCASUMSUB_\0").map(|sym| *sym),
            SCNRM2SUB: get_symbol(&libs, b"SCNRM2SUB\0").map(|sym| *sym),
            scnrm2sub: get_symbol(&libs, b"scnrm2sub\0").map(|sym| *sym),
            SCNRM2SUB_: get_symbol(&libs, b"SCNRM2SUB_\0").map(|sym| *sym),
            SDOTSUB: get_symbol(&libs, b"SDOTSUB\0").map(|sym| *sym),
            sdotsub: get_symbol(&libs, b"sdotsub\0").map(|sym| *sym),
            SDOTSUB_: get_symbol(&libs, b"SDOTSUB_\0").map(|sym| *sym),
            SNRM2SUB: get_symbol(&libs, b"SNRM2SUB\0").map(|sym| *sym),
            snrm2sub: get_symbol(&libs, b"snrm2sub\0").map(|sym| *sym),
            SNRM2SUB_: get_symbol(&libs, b"SNRM2SUB_\0").map(|sym| *sym),
            ZDOTCSUB: get_symbol(&libs, b"ZDOTCSUB\0").map(|sym| *sym),
            zdotcsub: get_symbol(&libs, b"zdotcsub\0").map(|sym| *sym),
            ZDOTCSUB_: get_symbol(&libs, b"ZDOTCSUB_\0").map(|sym| *sym),
            ZDOTUSUB: get_symbol(&libs, b"ZDOTUSUB\0").map(|sym| *sym),
            zdotusub: get_symbol(&libs, b"zdotusub\0").map(|sym| *sym),
            ZDOTUSUB_: get_symbol(&libs, b"ZDOTUSUB_\0").map(|sym| *sym),
            SDSDOTSUB: get_symbol(&libs, b"SDSDOTSUB\0").map(|sym| *sym),
            sdsdotsub: get_symbol(&libs, b"sdsdotsub\0").map(|sym| *sym),
            SDSDOTSUB_: get_symbol(&libs, b"SDSDOTSUB_\0").map(|sym| *sym),
            DSDOTSUB: get_symbol(&libs, b"DSDOTSUB\0").map(|sym| *sym),
            dsdotsub: get_symbol(&libs, b"dsdotsub\0").map(|sym| *sym),
            DSDOTSUB_: get_symbol(&libs, b"DSDOTSUB_\0").map(|sym| *sym),
            LSAME: get_symbol(&libs, b"LSAME\0").map(|sym| *sym),
            lsame: get_symbol(&libs, b"lsame\0").map(|sym| *sym),
            LSAME_: get_symbol(&libs, b"LSAME_\0").map(|sym| *sym),
            XERBLA: get_symbol(&libs, b"XERBLA\0").map(|sym| *sym),
            xerbla: get_symbol(&libs, b"xerbla\0").map(|sym| *sym),
            XERBLA_: get_symbol(&libs, b"XERBLA_\0").map(|sym| *sym),
            DCABS1: get_symbol(&libs, b"DCABS1\0").map(|sym| *sym),
            dcabs1: get_symbol(&libs, b"dcabs1\0").map(|sym| *sym),
            DCABS1_: get_symbol(&libs, b"DCABS1_\0").map(|sym| *sym),
            SCABS1: get_symbol(&libs, b"SCABS1\0").map(|sym| *sym),
            scabs1: get_symbol(&libs, b"scabs1\0").map(|sym| *sym),
            SCABS1_: get_symbol(&libs, b"SCABS1_\0").map(|sym| *sym),
            CAXPBY: get_symbol(&libs, b"CAXPBY\0").map(|sym| *sym),
            caxpby: get_symbol(&libs, b"caxpby\0").map(|sym| *sym),
            CAXPBY_: get_symbol(&libs, b"CAXPBY_\0").map(|sym| *sym),
            CGEMM3M: get_symbol(&libs, b"CGEMM3M\0").map(|sym| *sym),
            cgemm3m: get_symbol(&libs, b"cgemm3m\0").map(|sym| *sym),
            CGEMM3M_: get_symbol(&libs, b"CGEMM3M_\0").map(|sym| *sym),
            CGEMM_BATCH: get_symbol(&libs, b"CGEMM_BATCH\0").map(|sym| *sym),
            cgemm_batch: get_symbol(&libs, b"cgemm_batch\0").map(|sym| *sym),
            CGEMM_BATCH_: get_symbol(&libs, b"CGEMM_BATCH_\0").map(|sym| *sym),
            CGEMMT: get_symbol(&libs, b"CGEMMT\0").map(|sym| *sym),
            cgemmt: get_symbol(&libs, b"cgemmt\0").map(|sym| *sym),
            CGEMMT_: get_symbol(&libs, b"CGEMMT_\0").map(|sym| *sym),
            DAXPBY: get_symbol(&libs, b"DAXPBY\0").map(|sym| *sym),
            daxpby: get_symbol(&libs, b"daxpby\0").map(|sym| *sym),
            DAXPBY_: get_symbol(&libs, b"DAXPBY_\0").map(|sym| *sym),
            DGEMM_BATCH: get_symbol(&libs, b"DGEMM_BATCH\0").map(|sym| *sym),
            dgemm_batch: get_symbol(&libs, b"dgemm_batch\0").map(|sym| *sym),
            DGEMM_BATCH_: get_symbol(&libs, b"DGEMM_BATCH_\0").map(|sym| *sym),
            DGEMM_PACK_GET_SIZE: get_symbol(&libs, b"DGEMM_PACK_GET_SIZE\0").map(|sym| *sym),
            dgemm_pack_get_size: get_symbol(&libs, b"dgemm_pack_get_size\0").map(|sym| *sym),
            DGEMM_PACK_GET_SIZE_: get_symbol(&libs, b"DGEMM_PACK_GET_SIZE_\0").map(|sym| *sym),
            DGEMM_PACK: get_symbol(&libs, b"DGEMM_PACK\0").map(|sym| *sym),
            dgemm_pack: get_symbol(&libs, b"dgemm_pack\0").map(|sym| *sym),
            DGEMM_PACK_: get_symbol(&libs, b"DGEMM_PACK_\0").map(|sym| *sym),
            DGEMM_COMPUTE: get_symbol(&libs, b"DGEMM_COMPUTE\0").map(|sym| *sym),
            dgemm_compute: get_symbol(&libs, b"dgemm_compute\0").map(|sym| *sym),
            DGEMM_COMPUTE_: get_symbol(&libs, b"DGEMM_COMPUTE_\0").map(|sym| *sym),
            DGEMMT: get_symbol(&libs, b"DGEMMT\0").map(|sym| *sym),
            dgemmt: get_symbol(&libs, b"dgemmt\0").map(|sym| *sym),
            DGEMMT_: get_symbol(&libs, b"DGEMMT_\0").map(|sym| *sym),
            SAXPBY: get_symbol(&libs, b"SAXPBY\0").map(|sym| *sym),
            saxpby: get_symbol(&libs, b"saxpby\0").map(|sym| *sym),
            SAXPBY_: get_symbol(&libs, b"SAXPBY_\0").map(|sym| *sym),
            SGEMM_BATCH: get_symbol(&libs, b"SGEMM_BATCH\0").map(|sym| *sym),
            sgemm_batch: get_symbol(&libs, b"sgemm_batch\0").map(|sym| *sym),
            SGEMM_BATCH_: get_symbol(&libs, b"SGEMM_BATCH_\0").map(|sym| *sym),
            SGEMM_PACK_GET_SIZE: get_symbol(&libs, b"SGEMM_PACK_GET_SIZE\0").map(|sym| *sym),
            sgemm_pack_get_size: get_symbol(&libs, b"sgemm_pack_get_size\0").map(|sym| *sym),
            SGEMM_PACK_GET_SIZE_: get_symbol(&libs, b"SGEMM_PACK_GET_SIZE_\0").map(|sym| *sym),
            SGEMM_PACK: get_symbol(&libs, b"SGEMM_PACK\0").map(|sym| *sym),
            sgemm_pack: get_symbol(&libs, b"sgemm_pack\0").map(|sym| *sym),
            SGEMM_PACK_: get_symbol(&libs, b"SGEMM_PACK_\0").map(|sym| *sym),
            SGEMM_COMPUTE: get_symbol(&libs, b"SGEMM_COMPUTE\0").map(|sym| *sym),
            sgemm_compute: get_symbol(&libs, b"sgemm_compute\0").map(|sym| *sym),
            SGEMM_COMPUTE_: get_symbol(&libs, b"SGEMM_COMPUTE_\0").map(|sym| *sym),
            SGEMMT: get_symbol(&libs, b"SGEMMT\0").map(|sym| *sym),
            sgemmt: get_symbol(&libs, b"sgemmt\0").map(|sym| *sym),
            SGEMMT_: get_symbol(&libs, b"SGEMMT_\0").map(|sym| *sym),
            ZAXPBY: get_symbol(&libs, b"ZAXPBY\0").map(|sym| *sym),
            zaxpby: get_symbol(&libs, b"zaxpby\0").map(|sym| *sym),
            ZAXPBY_: get_symbol(&libs, b"ZAXPBY_\0").map(|sym| *sym),
            ZGEMM3M: get_symbol(&libs, b"ZGEMM3M\0").map(|sym| *sym),
            zgemm3m: get_symbol(&libs, b"zgemm3m\0").map(|sym| *sym),
            ZGEMM3M_: get_symbol(&libs, b"ZGEMM3M_\0").map(|sym| *sym),
            ZGEMM_BATCH: get_symbol(&libs, b"ZGEMM_BATCH\0").map(|sym| *sym),
            zgemm_batch: get_symbol(&libs, b"zgemm_batch\0").map(|sym| *sym),
            ZGEMM_BATCH_: get_symbol(&libs, b"ZGEMM_BATCH_\0").map(|sym| *sym),
            ZGEMMT: get_symbol(&libs, b"ZGEMMT\0").map(|sym| *sym),
            zgemmt: get_symbol(&libs, b"zgemmt\0").map(|sym| *sym),
            ZGEMMT_: get_symbol(&libs, b"ZGEMMT_\0").map(|sym| *sym),
            CIMATCOPY: get_symbol(&libs, b"CIMATCOPY\0").map(|sym| *sym),
            cimatcopy: get_symbol(&libs, b"cimatcopy\0").map(|sym| *sym),
            CIMATCOPY_: get_symbol(&libs, b"CIMATCOPY_\0").map(|sym| *sym),
            COMATADD: get_symbol(&libs, b"COMATADD\0").map(|sym| *sym),
            comatadd: get_symbol(&libs, b"comatadd\0").map(|sym| *sym),
            COMATADD_: get_symbol(&libs, b"COMATADD_\0").map(|sym| *sym),
            COMATCOPY2: get_symbol(&libs, b"COMATCOPY2\0").map(|sym| *sym),
            comatcopy2: get_symbol(&libs, b"comatcopy2\0").map(|sym| *sym),
            COMATCOPY2_: get_symbol(&libs, b"COMATCOPY2_\0").map(|sym| *sym),
            COMATCOPY: get_symbol(&libs, b"COMATCOPY\0").map(|sym| *sym),
            comatcopy: get_symbol(&libs, b"comatcopy\0").map(|sym| *sym),
            COMATCOPY_: get_symbol(&libs, b"COMATCOPY_\0").map(|sym| *sym),
            DOMATADD: get_symbol(&libs, b"DOMATADD\0").map(|sym| *sym),
            domatadd: get_symbol(&libs, b"domatadd\0").map(|sym| *sym),
            DOMATADD_: get_symbol(&libs, b"DOMATADD_\0").map(|sym| *sym),
            DOMATCOPY2: get_symbol(&libs, b"DOMATCOPY2\0").map(|sym| *sym),
            domatcopy2: get_symbol(&libs, b"domatcopy2\0").map(|sym| *sym),
            DOMATCOPY2_: get_symbol(&libs, b"DOMATCOPY2_\0").map(|sym| *sym),
            DOMATCOPY: get_symbol(&libs, b"DOMATCOPY\0").map(|sym| *sym),
            domatcopy: get_symbol(&libs, b"domatcopy\0").map(|sym| *sym),
            DOMATCOPY_: get_symbol(&libs, b"DOMATCOPY_\0").map(|sym| *sym),
            SIMATCOPY: get_symbol(&libs, b"SIMATCOPY\0").map(|sym| *sym),
            simatcopy: get_symbol(&libs, b"simatcopy\0").map(|sym| *sym),
            SIMATCOPY_: get_symbol(&libs, b"SIMATCOPY_\0").map(|sym| *sym),
            SOMATADD: get_symbol(&libs, b"SOMATADD\0").map(|sym| *sym),
            somatadd: get_symbol(&libs, b"somatadd\0").map(|sym| *sym),
            SOMATADD_: get_symbol(&libs, b"SOMATADD_\0").map(|sym| *sym),
            SOMATCOPY2: get_symbol(&libs, b"SOMATCOPY2\0").map(|sym| *sym),
            somatcopy2: get_symbol(&libs, b"somatcopy2\0").map(|sym| *sym),
            SOMATCOPY2_: get_symbol(&libs, b"SOMATCOPY2_\0").map(|sym| *sym),
            SOMATCOPY: get_symbol(&libs, b"SOMATCOPY\0").map(|sym| *sym),
            somatcopy: get_symbol(&libs, b"somatcopy\0").map(|sym| *sym),
            SOMATCOPY_: get_symbol(&libs, b"SOMATCOPY_\0").map(|sym| *sym),
            ZIMATCOPY: get_symbol(&libs, b"ZIMATCOPY\0").map(|sym| *sym),
            zimatcopy: get_symbol(&libs, b"zimatcopy\0").map(|sym| *sym),
            ZIMATCOPY_: get_symbol(&libs, b"ZIMATCOPY_\0").map(|sym| *sym),
            ZOMATADD: get_symbol(&libs, b"ZOMATADD\0").map(|sym| *sym),
            zomatadd: get_symbol(&libs, b"zomatadd\0").map(|sym| *sym),
            ZOMATADD_: get_symbol(&libs, b"ZOMATADD_\0").map(|sym| *sym),
            ZOMATCOPY2: get_symbol(&libs, b"ZOMATCOPY2\0").map(|sym| *sym),
            zomatcopy2: get_symbol(&libs, b"zomatcopy2\0").map(|sym| *sym),
            ZOMATCOPY2_: get_symbol(&libs, b"ZOMATCOPY2_\0").map(|sym| *sym),
            ZOMATCOPY: get_symbol(&libs, b"ZOMATCOPY\0").map(|sym| *sym),
            zomatcopy: get_symbol(&libs, b"zomatcopy\0").map(|sym| *sym),
            ZOMATCOPY_: get_symbol(&libs, b"ZOMATCOPY_\0").map(|sym| *sym),
            SROTG_BLIS_IMPL: get_symbol(&libs, b"SROTG_BLIS_IMPL\0").map(|sym| *sym),
            srotg_blis_impl_: get_symbol(&libs, b"srotg_blis_impl_\0").map(|sym| *sym),
            SROTG_BLIS_IMPL_: get_symbol(&libs, b"SROTG_BLIS_IMPL_\0").map(|sym| *sym),
            SROTMG_BLIS_IMPL: get_symbol(&libs, b"SROTMG_BLIS_IMPL\0").map(|sym| *sym),
            srotmg_blis_impl_: get_symbol(&libs, b"srotmg_blis_impl_\0").map(|sym| *sym),
            SROTMG_BLIS_IMPL_: get_symbol(&libs, b"SROTMG_BLIS_IMPL_\0").map(|sym| *sym),
            SROT_BLIS_IMPL: get_symbol(&libs, b"SROT_BLIS_IMPL\0").map(|sym| *sym),
            srot_blis_impl_: get_symbol(&libs, b"srot_blis_impl_\0").map(|sym| *sym),
            SROT_BLIS_IMPL_: get_symbol(&libs, b"SROT_BLIS_IMPL_\0").map(|sym| *sym),
            SROTM_BLIS_IMPL: get_symbol(&libs, b"SROTM_BLIS_IMPL\0").map(|sym| *sym),
            srotm_blis_impl_: get_symbol(&libs, b"srotm_blis_impl_\0").map(|sym| *sym),
            SROTM_BLIS_IMPL_: get_symbol(&libs, b"SROTM_BLIS_IMPL_\0").map(|sym| *sym),
            SSWAP_BLIS_IMPL: get_symbol(&libs, b"SSWAP_BLIS_IMPL\0").map(|sym| *sym),
            sswap_blis_impl_: get_symbol(&libs, b"sswap_blis_impl_\0").map(|sym| *sym),
            SSWAP_BLIS_IMPL_: get_symbol(&libs, b"SSWAP_BLIS_IMPL_\0").map(|sym| *sym),
            SSCAL_BLIS_IMPL: get_symbol(&libs, b"SSCAL_BLIS_IMPL\0").map(|sym| *sym),
            sscal_blis_impl_: get_symbol(&libs, b"sscal_blis_impl_\0").map(|sym| *sym),
            SSCAL_BLIS_IMPL_: get_symbol(&libs, b"SSCAL_BLIS_IMPL_\0").map(|sym| *sym),
            SCOPY_BLIS_IMPL: get_symbol(&libs, b"SCOPY_BLIS_IMPL\0").map(|sym| *sym),
            scopy_blis_impl_: get_symbol(&libs, b"scopy_blis_impl_\0").map(|sym| *sym),
            SCOPY_BLIS_IMPL_: get_symbol(&libs, b"SCOPY_BLIS_IMPL_\0").map(|sym| *sym),
            SAXPY_BLIS_IMPL: get_symbol(&libs, b"SAXPY_BLIS_IMPL\0").map(|sym| *sym),
            saxpy_blis_impl_: get_symbol(&libs, b"saxpy_blis_impl_\0").map(|sym| *sym),
            SAXPY_BLIS_IMPL_: get_symbol(&libs, b"SAXPY_BLIS_IMPL_\0").map(|sym| *sym),
            SDOT_BLIS_IMPL: get_symbol(&libs, b"SDOT_BLIS_IMPL\0").map(|sym| *sym),
            sdot_blis_impl_: get_symbol(&libs, b"sdot_blis_impl_\0").map(|sym| *sym),
            SDOT_BLIS_IMPL_: get_symbol(&libs, b"SDOT_BLIS_IMPL_\0").map(|sym| *sym),
            SDSDOT_BLIS_IMPL: get_symbol(&libs, b"SDSDOT_BLIS_IMPL\0").map(|sym| *sym),
            sdsdot_blis_impl_: get_symbol(&libs, b"sdsdot_blis_impl_\0").map(|sym| *sym),
            SDSDOT_BLIS_IMPL_: get_symbol(&libs, b"SDSDOT_BLIS_IMPL_\0").map(|sym| *sym),
            SNRM2_BLIS_IMPL: get_symbol(&libs, b"SNRM2_BLIS_IMPL\0").map(|sym| *sym),
            snrm2_blis_impl_: get_symbol(&libs, b"snrm2_blis_impl_\0").map(|sym| *sym),
            SNRM2_BLIS_IMPL_: get_symbol(&libs, b"SNRM2_BLIS_IMPL_\0").map(|sym| *sym),
            SCNRM2_BLIS_IMPL: get_symbol(&libs, b"SCNRM2_BLIS_IMPL\0").map(|sym| *sym),
            scnrm2_blis_impl_: get_symbol(&libs, b"scnrm2_blis_impl_\0").map(|sym| *sym),
            SCNRM2_BLIS_IMPL_: get_symbol(&libs, b"SCNRM2_BLIS_IMPL_\0").map(|sym| *sym),
            SASUM_BLIS_IMPL: get_symbol(&libs, b"SASUM_BLIS_IMPL\0").map(|sym| *sym),
            sasum_blis_impl_: get_symbol(&libs, b"sasum_blis_impl_\0").map(|sym| *sym),
            SASUM_BLIS_IMPL_: get_symbol(&libs, b"SASUM_BLIS_IMPL_\0").map(|sym| *sym),
            ISAMAX_BLIS_IMPL: get_symbol(&libs, b"ISAMAX_BLIS_IMPL\0").map(|sym| *sym),
            isamax_blis_impl_: get_symbol(&libs, b"isamax_blis_impl_\0").map(|sym| *sym),
            ISAMAX_BLIS_IMPL_: get_symbol(&libs, b"ISAMAX_BLIS_IMPL_\0").map(|sym| *sym),
            DROTG_BLIS_IMPL: get_symbol(&libs, b"DROTG_BLIS_IMPL\0").map(|sym| *sym),
            drotg_blis_impl_: get_symbol(&libs, b"drotg_blis_impl_\0").map(|sym| *sym),
            DROTG_BLIS_IMPL_: get_symbol(&libs, b"DROTG_BLIS_IMPL_\0").map(|sym| *sym),
            DROTMG_BLIS_IMPL: get_symbol(&libs, b"DROTMG_BLIS_IMPL\0").map(|sym| *sym),
            drotmg_blis_impl_: get_symbol(&libs, b"drotmg_blis_impl_\0").map(|sym| *sym),
            DROTMG_BLIS_IMPL_: get_symbol(&libs, b"DROTMG_BLIS_IMPL_\0").map(|sym| *sym),
            DROT_BLIS_IMPL: get_symbol(&libs, b"DROT_BLIS_IMPL\0").map(|sym| *sym),
            drot_blis_impl_: get_symbol(&libs, b"drot_blis_impl_\0").map(|sym| *sym),
            DROT_BLIS_IMPL_: get_symbol(&libs, b"DROT_BLIS_IMPL_\0").map(|sym| *sym),
            DROTM_BLIS_IMPL: get_symbol(&libs, b"DROTM_BLIS_IMPL\0").map(|sym| *sym),
            drotm_blis_impl_: get_symbol(&libs, b"drotm_blis_impl_\0").map(|sym| *sym),
            DROTM_BLIS_IMPL_: get_symbol(&libs, b"DROTM_BLIS_IMPL_\0").map(|sym| *sym),
            DSWAP_BLIS_IMPL: get_symbol(&libs, b"DSWAP_BLIS_IMPL\0").map(|sym| *sym),
            dswap_blis_impl_: get_symbol(&libs, b"dswap_blis_impl_\0").map(|sym| *sym),
            DSWAP_BLIS_IMPL_: get_symbol(&libs, b"DSWAP_BLIS_IMPL_\0").map(|sym| *sym),
            DSCAL_BLIS_IMPL: get_symbol(&libs, b"DSCAL_BLIS_IMPL\0").map(|sym| *sym),
            dscal_blis_impl_: get_symbol(&libs, b"dscal_blis_impl_\0").map(|sym| *sym),
            DSCAL_BLIS_IMPL_: get_symbol(&libs, b"DSCAL_BLIS_IMPL_\0").map(|sym| *sym),
            DCOPY_BLIS_IMPL: get_symbol(&libs, b"DCOPY_BLIS_IMPL\0").map(|sym| *sym),
            dcopy_blis_impl_: get_symbol(&libs, b"dcopy_blis_impl_\0").map(|sym| *sym),
            DCOPY_BLIS_IMPL_: get_symbol(&libs, b"DCOPY_BLIS_IMPL_\0").map(|sym| *sym),
            DAXPY_BLIS_IMPL: get_symbol(&libs, b"DAXPY_BLIS_IMPL\0").map(|sym| *sym),
            daxpy_blis_impl_: get_symbol(&libs, b"daxpy_blis_impl_\0").map(|sym| *sym),
            DAXPY_BLIS_IMPL_: get_symbol(&libs, b"DAXPY_BLIS_IMPL_\0").map(|sym| *sym),
            DDOT_BLIS_IMPL: get_symbol(&libs, b"DDOT_BLIS_IMPL\0").map(|sym| *sym),
            ddot_blis_impl_: get_symbol(&libs, b"ddot_blis_impl_\0").map(|sym| *sym),
            DDOT_BLIS_IMPL_: get_symbol(&libs, b"DDOT_BLIS_IMPL_\0").map(|sym| *sym),
            DSDOT_BLIS_IMPL: get_symbol(&libs, b"DSDOT_BLIS_IMPL\0").map(|sym| *sym),
            dsdot_blis_impl_: get_symbol(&libs, b"dsdot_blis_impl_\0").map(|sym| *sym),
            DSDOT_BLIS_IMPL_: get_symbol(&libs, b"DSDOT_BLIS_IMPL_\0").map(|sym| *sym),
            DNRM2_BLIS_IMPL: get_symbol(&libs, b"DNRM2_BLIS_IMPL\0").map(|sym| *sym),
            dnrm2_blis_impl_: get_symbol(&libs, b"dnrm2_blis_impl_\0").map(|sym| *sym),
            DNRM2_BLIS_IMPL_: get_symbol(&libs, b"DNRM2_BLIS_IMPL_\0").map(|sym| *sym),
            DZNRM2_BLIS_IMPL: get_symbol(&libs, b"DZNRM2_BLIS_IMPL\0").map(|sym| *sym),
            dznrm2_blis_impl_: get_symbol(&libs, b"dznrm2_blis_impl_\0").map(|sym| *sym),
            DZNRM2_BLIS_IMPL_: get_symbol(&libs, b"DZNRM2_BLIS_IMPL_\0").map(|sym| *sym),
            DASUM_BLIS_IMPL: get_symbol(&libs, b"DASUM_BLIS_IMPL\0").map(|sym| *sym),
            dasum_blis_impl_: get_symbol(&libs, b"dasum_blis_impl_\0").map(|sym| *sym),
            DASUM_BLIS_IMPL_: get_symbol(&libs, b"DASUM_BLIS_IMPL_\0").map(|sym| *sym),
            IDAMAX_BLIS_IMPL: get_symbol(&libs, b"IDAMAX_BLIS_IMPL\0").map(|sym| *sym),
            idamax_blis_impl_: get_symbol(&libs, b"idamax_blis_impl_\0").map(|sym| *sym),
            IDAMAX_BLIS_IMPL_: get_symbol(&libs, b"IDAMAX_BLIS_IMPL_\0").map(|sym| *sym),
            CROTG_BLIS_IMPL: get_symbol(&libs, b"CROTG_BLIS_IMPL\0").map(|sym| *sym),
            crotg_blis_impl_: get_symbol(&libs, b"crotg_blis_impl_\0").map(|sym| *sym),
            CROTG_BLIS_IMPL_: get_symbol(&libs, b"CROTG_BLIS_IMPL_\0").map(|sym| *sym),
            CSROT_BLIS_IMPL: get_symbol(&libs, b"CSROT_BLIS_IMPL\0").map(|sym| *sym),
            csrot_blis_impl_: get_symbol(&libs, b"csrot_blis_impl_\0").map(|sym| *sym),
            CSROT_BLIS_IMPL_: get_symbol(&libs, b"CSROT_BLIS_IMPL_\0").map(|sym| *sym),
            CSWAP_BLIS_IMPL: get_symbol(&libs, b"CSWAP_BLIS_IMPL\0").map(|sym| *sym),
            cswap_blis_impl_: get_symbol(&libs, b"cswap_blis_impl_\0").map(|sym| *sym),
            CSWAP_BLIS_IMPL_: get_symbol(&libs, b"CSWAP_BLIS_IMPL_\0").map(|sym| *sym),
            CSCAL_BLIS_IMPL: get_symbol(&libs, b"CSCAL_BLIS_IMPL\0").map(|sym| *sym),
            cscal_blis_impl_: get_symbol(&libs, b"cscal_blis_impl_\0").map(|sym| *sym),
            CSCAL_BLIS_IMPL_: get_symbol(&libs, b"CSCAL_BLIS_IMPL_\0").map(|sym| *sym),
            CSSCAL_BLIS_IMPL: get_symbol(&libs, b"CSSCAL_BLIS_IMPL\0").map(|sym| *sym),
            csscal_blis_impl_: get_symbol(&libs, b"csscal_blis_impl_\0").map(|sym| *sym),
            CSSCAL_BLIS_IMPL_: get_symbol(&libs, b"CSSCAL_BLIS_IMPL_\0").map(|sym| *sym),
            CCOPY_BLIS_IMPL: get_symbol(&libs, b"CCOPY_BLIS_IMPL\0").map(|sym| *sym),
            ccopy_blis_impl_: get_symbol(&libs, b"ccopy_blis_impl_\0").map(|sym| *sym),
            CCOPY_BLIS_IMPL_: get_symbol(&libs, b"CCOPY_BLIS_IMPL_\0").map(|sym| *sym),
            CAXPY_BLIS_IMPL: get_symbol(&libs, b"CAXPY_BLIS_IMPL\0").map(|sym| *sym),
            caxpy_blis_impl_: get_symbol(&libs, b"caxpy_blis_impl_\0").map(|sym| *sym),
            CAXPY_BLIS_IMPL_: get_symbol(&libs, b"CAXPY_BLIS_IMPL_\0").map(|sym| *sym),
            CDOTC_BLIS_IMPL: get_symbol(&libs, b"CDOTC_BLIS_IMPL\0").map(|sym| *sym),
            cdotc_blis_impl_: get_symbol(&libs, b"cdotc_blis_impl_\0").map(|sym| *sym),
            CDOTC_BLIS_IMPL_: get_symbol(&libs, b"CDOTC_BLIS_IMPL_\0").map(|sym| *sym),
            CDOTU_BLIS_IMPL: get_symbol(&libs, b"CDOTU_BLIS_IMPL\0").map(|sym| *sym),
            cdotu_blis_impl_: get_symbol(&libs, b"cdotu_blis_impl_\0").map(|sym| *sym),
            CDOTU_BLIS_IMPL_: get_symbol(&libs, b"CDOTU_BLIS_IMPL_\0").map(|sym| *sym),
            ZDOTC_BLIS_IMPL: get_symbol(&libs, b"ZDOTC_BLIS_IMPL\0").map(|sym| *sym),
            zdotc_blis_impl_: get_symbol(&libs, b"zdotc_blis_impl_\0").map(|sym| *sym),
            ZDOTC_BLIS_IMPL_: get_symbol(&libs, b"ZDOTC_BLIS_IMPL_\0").map(|sym| *sym),
            ZDOTU_BLIS_IMPL: get_symbol(&libs, b"ZDOTU_BLIS_IMPL\0").map(|sym| *sym),
            zdotu_blis_impl_: get_symbol(&libs, b"zdotu_blis_impl_\0").map(|sym| *sym),
            ZDOTU_BLIS_IMPL_: get_symbol(&libs, b"ZDOTU_BLIS_IMPL_\0").map(|sym| *sym),
            SCASUM_BLIS_IMPL: get_symbol(&libs, b"SCASUM_BLIS_IMPL\0").map(|sym| *sym),
            scasum_blis_impl_: get_symbol(&libs, b"scasum_blis_impl_\0").map(|sym| *sym),
            SCASUM_BLIS_IMPL_: get_symbol(&libs, b"SCASUM_BLIS_IMPL_\0").map(|sym| *sym),
            ICAMAX_BLIS_IMPL: get_symbol(&libs, b"ICAMAX_BLIS_IMPL\0").map(|sym| *sym),
            icamax_blis_impl_: get_symbol(&libs, b"icamax_blis_impl_\0").map(|sym| *sym),
            ICAMAX_BLIS_IMPL_: get_symbol(&libs, b"ICAMAX_BLIS_IMPL_\0").map(|sym| *sym),
            ZROTG_BLIS_IMPL: get_symbol(&libs, b"ZROTG_BLIS_IMPL\0").map(|sym| *sym),
            zrotg_blis_impl_: get_symbol(&libs, b"zrotg_blis_impl_\0").map(|sym| *sym),
            ZROTG_BLIS_IMPL_: get_symbol(&libs, b"ZROTG_BLIS_IMPL_\0").map(|sym| *sym),
            ZDROT_BLIS_IMPL: get_symbol(&libs, b"ZDROT_BLIS_IMPL\0").map(|sym| *sym),
            zdrot_blis_impl_: get_symbol(&libs, b"zdrot_blis_impl_\0").map(|sym| *sym),
            ZDROT_BLIS_IMPL_: get_symbol(&libs, b"ZDROT_BLIS_IMPL_\0").map(|sym| *sym),
            ZSWAP_BLIS_IMPL: get_symbol(&libs, b"ZSWAP_BLIS_IMPL\0").map(|sym| *sym),
            zswap_blis_impl_: get_symbol(&libs, b"zswap_blis_impl_\0").map(|sym| *sym),
            ZSWAP_BLIS_IMPL_: get_symbol(&libs, b"ZSWAP_BLIS_IMPL_\0").map(|sym| *sym),
            ZSCAL_BLIS_IMPL: get_symbol(&libs, b"ZSCAL_BLIS_IMPL\0").map(|sym| *sym),
            zscal_blis_impl_: get_symbol(&libs, b"zscal_blis_impl_\0").map(|sym| *sym),
            ZSCAL_BLIS_IMPL_: get_symbol(&libs, b"ZSCAL_BLIS_IMPL_\0").map(|sym| *sym),
            ZDSCAL_BLIS_IMPL: get_symbol(&libs, b"ZDSCAL_BLIS_IMPL\0").map(|sym| *sym),
            zdscal_blis_impl_: get_symbol(&libs, b"zdscal_blis_impl_\0").map(|sym| *sym),
            ZDSCAL_BLIS_IMPL_: get_symbol(&libs, b"ZDSCAL_BLIS_IMPL_\0").map(|sym| *sym),
            ZCOPY_BLIS_IMPL: get_symbol(&libs, b"ZCOPY_BLIS_IMPL\0").map(|sym| *sym),
            zcopy_blis_impl_: get_symbol(&libs, b"zcopy_blis_impl_\0").map(|sym| *sym),
            ZCOPY_BLIS_IMPL_: get_symbol(&libs, b"ZCOPY_BLIS_IMPL_\0").map(|sym| *sym),
            ZAXPY_BLIS_IMPL: get_symbol(&libs, b"ZAXPY_BLIS_IMPL\0").map(|sym| *sym),
            zaxpy_blis_impl_: get_symbol(&libs, b"zaxpy_blis_impl_\0").map(|sym| *sym),
            ZAXPY_BLIS_IMPL_: get_symbol(&libs, b"ZAXPY_BLIS_IMPL_\0").map(|sym| *sym),
            DZASUM_BLIS_IMPL: get_symbol(&libs, b"DZASUM_BLIS_IMPL\0").map(|sym| *sym),
            dzasum_blis_impl_: get_symbol(&libs, b"dzasum_blis_impl_\0").map(|sym| *sym),
            DZASUM_BLIS_IMPL_: get_symbol(&libs, b"DZASUM_BLIS_IMPL_\0").map(|sym| *sym),
            IZAMAX_BLIS_IMPL: get_symbol(&libs, b"IZAMAX_BLIS_IMPL\0").map(|sym| *sym),
            izamax_blis_impl_: get_symbol(&libs, b"izamax_blis_impl_\0").map(|sym| *sym),
            IZAMAX_BLIS_IMPL_: get_symbol(&libs, b"IZAMAX_BLIS_IMPL_\0").map(|sym| *sym),
            ICAMIN_BLIS_IMPL: get_symbol(&libs, b"ICAMIN_BLIS_IMPL\0").map(|sym| *sym),
            icamin_blis_impl_: get_symbol(&libs, b"icamin_blis_impl_\0").map(|sym| *sym),
            ICAMIN_BLIS_IMPL_: get_symbol(&libs, b"ICAMIN_BLIS_IMPL_\0").map(|sym| *sym),
            IDAMIN_BLIS_IMPL: get_symbol(&libs, b"IDAMIN_BLIS_IMPL\0").map(|sym| *sym),
            idamin_blis_impl_: get_symbol(&libs, b"idamin_blis_impl_\0").map(|sym| *sym),
            IDAMIN_BLIS_IMPL_: get_symbol(&libs, b"IDAMIN_BLIS_IMPL_\0").map(|sym| *sym),
            ISAMIN_BLIS_IMPL: get_symbol(&libs, b"ISAMIN_BLIS_IMPL\0").map(|sym| *sym),
            isamin_blis_impl_: get_symbol(&libs, b"isamin_blis_impl_\0").map(|sym| *sym),
            ISAMIN_BLIS_IMPL_: get_symbol(&libs, b"ISAMIN_BLIS_IMPL_\0").map(|sym| *sym),
            IZAMIN_BLIS_IMPL: get_symbol(&libs, b"IZAMIN_BLIS_IMPL\0").map(|sym| *sym),
            izamin_blis_impl_: get_symbol(&libs, b"izamin_blis_impl_\0").map(|sym| *sym),
            IZAMIN_BLIS_IMPL_: get_symbol(&libs, b"IZAMIN_BLIS_IMPL_\0").map(|sym| *sym),
            SGEMV_BLIS_IMPL: get_symbol(&libs, b"SGEMV_BLIS_IMPL\0").map(|sym| *sym),
            sgemv_blis_impl_: get_symbol(&libs, b"sgemv_blis_impl_\0").map(|sym| *sym),
            SGEMV_BLIS_IMPL_: get_symbol(&libs, b"SGEMV_BLIS_IMPL_\0").map(|sym| *sym),
            SGBMV_BLIS_IMPL: get_symbol(&libs, b"SGBMV_BLIS_IMPL\0").map(|sym| *sym),
            sgbmv_blis_impl_: get_symbol(&libs, b"sgbmv_blis_impl_\0").map(|sym| *sym),
            SGBMV_BLIS_IMPL_: get_symbol(&libs, b"SGBMV_BLIS_IMPL_\0").map(|sym| *sym),
            SSYMV_BLIS_IMPL: get_symbol(&libs, b"SSYMV_BLIS_IMPL\0").map(|sym| *sym),
            ssymv_blis_impl_: get_symbol(&libs, b"ssymv_blis_impl_\0").map(|sym| *sym),
            SSYMV_BLIS_IMPL_: get_symbol(&libs, b"SSYMV_BLIS_IMPL_\0").map(|sym| *sym),
            SSBMV_BLIS_IMPL: get_symbol(&libs, b"SSBMV_BLIS_IMPL\0").map(|sym| *sym),
            ssbmv_blis_impl_: get_symbol(&libs, b"ssbmv_blis_impl_\0").map(|sym| *sym),
            SSBMV_BLIS_IMPL_: get_symbol(&libs, b"SSBMV_BLIS_IMPL_\0").map(|sym| *sym),
            SSPMV_BLIS_IMPL: get_symbol(&libs, b"SSPMV_BLIS_IMPL\0").map(|sym| *sym),
            sspmv_blis_impl_: get_symbol(&libs, b"sspmv_blis_impl_\0").map(|sym| *sym),
            SSPMV_BLIS_IMPL_: get_symbol(&libs, b"SSPMV_BLIS_IMPL_\0").map(|sym| *sym),
            STRMV_BLIS_IMPL: get_symbol(&libs, b"STRMV_BLIS_IMPL\0").map(|sym| *sym),
            strmv_blis_impl_: get_symbol(&libs, b"strmv_blis_impl_\0").map(|sym| *sym),
            STRMV_BLIS_IMPL_: get_symbol(&libs, b"STRMV_BLIS_IMPL_\0").map(|sym| *sym),
            STBMV_BLIS_IMPL: get_symbol(&libs, b"STBMV_BLIS_IMPL\0").map(|sym| *sym),
            stbmv_blis_impl_: get_symbol(&libs, b"stbmv_blis_impl_\0").map(|sym| *sym),
            STBMV_BLIS_IMPL_: get_symbol(&libs, b"STBMV_BLIS_IMPL_\0").map(|sym| *sym),
            STPMV_BLIS_IMPL: get_symbol(&libs, b"STPMV_BLIS_IMPL\0").map(|sym| *sym),
            stpmv_blis_impl_: get_symbol(&libs, b"stpmv_blis_impl_\0").map(|sym| *sym),
            STPMV_BLIS_IMPL_: get_symbol(&libs, b"STPMV_BLIS_IMPL_\0").map(|sym| *sym),
            STRSV_BLIS_IMPL: get_symbol(&libs, b"STRSV_BLIS_IMPL\0").map(|sym| *sym),
            strsv_blis_impl_: get_symbol(&libs, b"strsv_blis_impl_\0").map(|sym| *sym),
            STRSV_BLIS_IMPL_: get_symbol(&libs, b"STRSV_BLIS_IMPL_\0").map(|sym| *sym),
            STBSV_BLIS_IMPL: get_symbol(&libs, b"STBSV_BLIS_IMPL\0").map(|sym| *sym),
            stbsv_blis_impl_: get_symbol(&libs, b"stbsv_blis_impl_\0").map(|sym| *sym),
            STBSV_BLIS_IMPL_: get_symbol(&libs, b"STBSV_BLIS_IMPL_\0").map(|sym| *sym),
            STPSV_BLIS_IMPL: get_symbol(&libs, b"STPSV_BLIS_IMPL\0").map(|sym| *sym),
            stpsv_blis_impl_: get_symbol(&libs, b"stpsv_blis_impl_\0").map(|sym| *sym),
            STPSV_BLIS_IMPL_: get_symbol(&libs, b"STPSV_BLIS_IMPL_\0").map(|sym| *sym),
            SGER_BLIS_IMPL: get_symbol(&libs, b"SGER_BLIS_IMPL\0").map(|sym| *sym),
            sger_blis_impl_: get_symbol(&libs, b"sger_blis_impl_\0").map(|sym| *sym),
            SGER_BLIS_IMPL_: get_symbol(&libs, b"SGER_BLIS_IMPL_\0").map(|sym| *sym),
            SSYR_BLIS_IMPL: get_symbol(&libs, b"SSYR_BLIS_IMPL\0").map(|sym| *sym),
            ssyr_blis_impl_: get_symbol(&libs, b"ssyr_blis_impl_\0").map(|sym| *sym),
            SSYR_BLIS_IMPL_: get_symbol(&libs, b"SSYR_BLIS_IMPL_\0").map(|sym| *sym),
            SSPR_BLIS_IMPL: get_symbol(&libs, b"SSPR_BLIS_IMPL\0").map(|sym| *sym),
            sspr_blis_impl_: get_symbol(&libs, b"sspr_blis_impl_\0").map(|sym| *sym),
            SSPR_BLIS_IMPL_: get_symbol(&libs, b"SSPR_BLIS_IMPL_\0").map(|sym| *sym),
            SSYR2_BLIS_IMPL: get_symbol(&libs, b"SSYR2_BLIS_IMPL\0").map(|sym| *sym),
            ssyr2_blis_impl_: get_symbol(&libs, b"ssyr2_blis_impl_\0").map(|sym| *sym),
            SSYR2_BLIS_IMPL_: get_symbol(&libs, b"SSYR2_BLIS_IMPL_\0").map(|sym| *sym),
            SSPR2_BLIS_IMPL: get_symbol(&libs, b"SSPR2_BLIS_IMPL\0").map(|sym| *sym),
            sspr2_blis_impl_: get_symbol(&libs, b"sspr2_blis_impl_\0").map(|sym| *sym),
            SSPR2_BLIS_IMPL_: get_symbol(&libs, b"SSPR2_BLIS_IMPL_\0").map(|sym| *sym),
            DGEMV_BLIS_IMPL: get_symbol(&libs, b"DGEMV_BLIS_IMPL\0").map(|sym| *sym),
            dgemv_blis_impl_: get_symbol(&libs, b"dgemv_blis_impl_\0").map(|sym| *sym),
            DGEMV_BLIS_IMPL_: get_symbol(&libs, b"DGEMV_BLIS_IMPL_\0").map(|sym| *sym),
            DGBMV_BLIS_IMPL: get_symbol(&libs, b"DGBMV_BLIS_IMPL\0").map(|sym| *sym),
            dgbmv_blis_impl_: get_symbol(&libs, b"dgbmv_blis_impl_\0").map(|sym| *sym),
            DGBMV_BLIS_IMPL_: get_symbol(&libs, b"DGBMV_BLIS_IMPL_\0").map(|sym| *sym),
            DSYMV_BLIS_IMPL: get_symbol(&libs, b"DSYMV_BLIS_IMPL\0").map(|sym| *sym),
            dsymv_blis_impl_: get_symbol(&libs, b"dsymv_blis_impl_\0").map(|sym| *sym),
            DSYMV_BLIS_IMPL_: get_symbol(&libs, b"DSYMV_BLIS_IMPL_\0").map(|sym| *sym),
            DSBMV_BLIS_IMPL: get_symbol(&libs, b"DSBMV_BLIS_IMPL\0").map(|sym| *sym),
            dsbmv_blis_impl_: get_symbol(&libs, b"dsbmv_blis_impl_\0").map(|sym| *sym),
            DSBMV_BLIS_IMPL_: get_symbol(&libs, b"DSBMV_BLIS_IMPL_\0").map(|sym| *sym),
            DSPMV_BLIS_IMPL: get_symbol(&libs, b"DSPMV_BLIS_IMPL\0").map(|sym| *sym),
            dspmv_blis_impl_: get_symbol(&libs, b"dspmv_blis_impl_\0").map(|sym| *sym),
            DSPMV_BLIS_IMPL_: get_symbol(&libs, b"DSPMV_BLIS_IMPL_\0").map(|sym| *sym),
            DTRMV_BLIS_IMPL: get_symbol(&libs, b"DTRMV_BLIS_IMPL\0").map(|sym| *sym),
            dtrmv_blis_impl_: get_symbol(&libs, b"dtrmv_blis_impl_\0").map(|sym| *sym),
            DTRMV_BLIS_IMPL_: get_symbol(&libs, b"DTRMV_BLIS_IMPL_\0").map(|sym| *sym),
            DTBMV_BLIS_IMPL: get_symbol(&libs, b"DTBMV_BLIS_IMPL\0").map(|sym| *sym),
            dtbmv_blis_impl_: get_symbol(&libs, b"dtbmv_blis_impl_\0").map(|sym| *sym),
            DTBMV_BLIS_IMPL_: get_symbol(&libs, b"DTBMV_BLIS_IMPL_\0").map(|sym| *sym),
            DTPMV_BLIS_IMPL: get_symbol(&libs, b"DTPMV_BLIS_IMPL\0").map(|sym| *sym),
            dtpmv_blis_impl_: get_symbol(&libs, b"dtpmv_blis_impl_\0").map(|sym| *sym),
            DTPMV_BLIS_IMPL_: get_symbol(&libs, b"DTPMV_BLIS_IMPL_\0").map(|sym| *sym),
            DTRSV_BLIS_IMPL: get_symbol(&libs, b"DTRSV_BLIS_IMPL\0").map(|sym| *sym),
            dtrsv_blis_impl_: get_symbol(&libs, b"dtrsv_blis_impl_\0").map(|sym| *sym),
            DTRSV_BLIS_IMPL_: get_symbol(&libs, b"DTRSV_BLIS_IMPL_\0").map(|sym| *sym),
            DTBSV_BLIS_IMPL: get_symbol(&libs, b"DTBSV_BLIS_IMPL\0").map(|sym| *sym),
            dtbsv_blis_impl_: get_symbol(&libs, b"dtbsv_blis_impl_\0").map(|sym| *sym),
            DTBSV_BLIS_IMPL_: get_symbol(&libs, b"DTBSV_BLIS_IMPL_\0").map(|sym| *sym),
            DTPSV_BLIS_IMPL: get_symbol(&libs, b"DTPSV_BLIS_IMPL\0").map(|sym| *sym),
            dtpsv_blis_impl_: get_symbol(&libs, b"dtpsv_blis_impl_\0").map(|sym| *sym),
            DTPSV_BLIS_IMPL_: get_symbol(&libs, b"DTPSV_BLIS_IMPL_\0").map(|sym| *sym),
            DGER_BLIS_IMPL: get_symbol(&libs, b"DGER_BLIS_IMPL\0").map(|sym| *sym),
            dger_blis_impl_: get_symbol(&libs, b"dger_blis_impl_\0").map(|sym| *sym),
            DGER_BLIS_IMPL_: get_symbol(&libs, b"DGER_BLIS_IMPL_\0").map(|sym| *sym),
            DSYR_BLIS_IMPL: get_symbol(&libs, b"DSYR_BLIS_IMPL\0").map(|sym| *sym),
            dsyr_blis_impl_: get_symbol(&libs, b"dsyr_blis_impl_\0").map(|sym| *sym),
            DSYR_BLIS_IMPL_: get_symbol(&libs, b"DSYR_BLIS_IMPL_\0").map(|sym| *sym),
            DSPR_BLIS_IMPL: get_symbol(&libs, b"DSPR_BLIS_IMPL\0").map(|sym| *sym),
            dspr_blis_impl_: get_symbol(&libs, b"dspr_blis_impl_\0").map(|sym| *sym),
            DSPR_BLIS_IMPL_: get_symbol(&libs, b"DSPR_BLIS_IMPL_\0").map(|sym| *sym),
            DSYR2_BLIS_IMPL: get_symbol(&libs, b"DSYR2_BLIS_IMPL\0").map(|sym| *sym),
            dsyr2_blis_impl_: get_symbol(&libs, b"dsyr2_blis_impl_\0").map(|sym| *sym),
            DSYR2_BLIS_IMPL_: get_symbol(&libs, b"DSYR2_BLIS_IMPL_\0").map(|sym| *sym),
            DSPR2_BLIS_IMPL: get_symbol(&libs, b"DSPR2_BLIS_IMPL\0").map(|sym| *sym),
            dspr2_blis_impl_: get_symbol(&libs, b"dspr2_blis_impl_\0").map(|sym| *sym),
            DSPR2_BLIS_IMPL_: get_symbol(&libs, b"DSPR2_BLIS_IMPL_\0").map(|sym| *sym),
            CGEMV_BLIS_IMPL: get_symbol(&libs, b"CGEMV_BLIS_IMPL\0").map(|sym| *sym),
            cgemv_blis_impl_: get_symbol(&libs, b"cgemv_blis_impl_\0").map(|sym| *sym),
            CGEMV_BLIS_IMPL_: get_symbol(&libs, b"CGEMV_BLIS_IMPL_\0").map(|sym| *sym),
            CGBMV_BLIS_IMPL: get_symbol(&libs, b"CGBMV_BLIS_IMPL\0").map(|sym| *sym),
            cgbmv_blis_impl_: get_symbol(&libs, b"cgbmv_blis_impl_\0").map(|sym| *sym),
            CGBMV_BLIS_IMPL_: get_symbol(&libs, b"CGBMV_BLIS_IMPL_\0").map(|sym| *sym),
            CHEMV_BLIS_IMPL: get_symbol(&libs, b"CHEMV_BLIS_IMPL\0").map(|sym| *sym),
            chemv_blis_impl_: get_symbol(&libs, b"chemv_blis_impl_\0").map(|sym| *sym),
            CHEMV_BLIS_IMPL_: get_symbol(&libs, b"CHEMV_BLIS_IMPL_\0").map(|sym| *sym),
            CHBMV_BLIS_IMPL: get_symbol(&libs, b"CHBMV_BLIS_IMPL\0").map(|sym| *sym),
            chbmv_blis_impl_: get_symbol(&libs, b"chbmv_blis_impl_\0").map(|sym| *sym),
            CHBMV_BLIS_IMPL_: get_symbol(&libs, b"CHBMV_BLIS_IMPL_\0").map(|sym| *sym),
            CHPMV_BLIS_IMPL: get_symbol(&libs, b"CHPMV_BLIS_IMPL\0").map(|sym| *sym),
            chpmv_blis_impl_: get_symbol(&libs, b"chpmv_blis_impl_\0").map(|sym| *sym),
            CHPMV_BLIS_IMPL_: get_symbol(&libs, b"CHPMV_BLIS_IMPL_\0").map(|sym| *sym),
            CTRMV_BLIS_IMPL: get_symbol(&libs, b"CTRMV_BLIS_IMPL\0").map(|sym| *sym),
            ctrmv_blis_impl_: get_symbol(&libs, b"ctrmv_blis_impl_\0").map(|sym| *sym),
            CTRMV_BLIS_IMPL_: get_symbol(&libs, b"CTRMV_BLIS_IMPL_\0").map(|sym| *sym),
            CTBMV_BLIS_IMPL: get_symbol(&libs, b"CTBMV_BLIS_IMPL\0").map(|sym| *sym),
            ctbmv_blis_impl_: get_symbol(&libs, b"ctbmv_blis_impl_\0").map(|sym| *sym),
            CTBMV_BLIS_IMPL_: get_symbol(&libs, b"CTBMV_BLIS_IMPL_\0").map(|sym| *sym),
            CTPMV_BLIS_IMPL: get_symbol(&libs, b"CTPMV_BLIS_IMPL\0").map(|sym| *sym),
            ctpmv_blis_impl_: get_symbol(&libs, b"ctpmv_blis_impl_\0").map(|sym| *sym),
            CTPMV_BLIS_IMPL_: get_symbol(&libs, b"CTPMV_BLIS_IMPL_\0").map(|sym| *sym),
            CTRSV_BLIS_IMPL: get_symbol(&libs, b"CTRSV_BLIS_IMPL\0").map(|sym| *sym),
            ctrsv_blis_impl_: get_symbol(&libs, b"ctrsv_blis_impl_\0").map(|sym| *sym),
            CTRSV_BLIS_IMPL_: get_symbol(&libs, b"CTRSV_BLIS_IMPL_\0").map(|sym| *sym),
            CTBSV_BLIS_IMPL: get_symbol(&libs, b"CTBSV_BLIS_IMPL\0").map(|sym| *sym),
            ctbsv_blis_impl_: get_symbol(&libs, b"ctbsv_blis_impl_\0").map(|sym| *sym),
            CTBSV_BLIS_IMPL_: get_symbol(&libs, b"CTBSV_BLIS_IMPL_\0").map(|sym| *sym),
            CTPSV_BLIS_IMPL: get_symbol(&libs, b"CTPSV_BLIS_IMPL\0").map(|sym| *sym),
            ctpsv_blis_impl_: get_symbol(&libs, b"ctpsv_blis_impl_\0").map(|sym| *sym),
            CTPSV_BLIS_IMPL_: get_symbol(&libs, b"CTPSV_BLIS_IMPL_\0").map(|sym| *sym),
            CGERC_BLIS_IMPL: get_symbol(&libs, b"CGERC_BLIS_IMPL\0").map(|sym| *sym),
            cgerc_blis_impl_: get_symbol(&libs, b"cgerc_blis_impl_\0").map(|sym| *sym),
            CGERC_BLIS_IMPL_: get_symbol(&libs, b"CGERC_BLIS_IMPL_\0").map(|sym| *sym),
            CGERU_BLIS_IMPL: get_symbol(&libs, b"CGERU_BLIS_IMPL\0").map(|sym| *sym),
            cgeru_blis_impl_: get_symbol(&libs, b"cgeru_blis_impl_\0").map(|sym| *sym),
            CGERU_BLIS_IMPL_: get_symbol(&libs, b"CGERU_BLIS_IMPL_\0").map(|sym| *sym),
            CHER_BLIS_IMPL: get_symbol(&libs, b"CHER_BLIS_IMPL\0").map(|sym| *sym),
            cher_blis_impl_: get_symbol(&libs, b"cher_blis_impl_\0").map(|sym| *sym),
            CHER_BLIS_IMPL_: get_symbol(&libs, b"CHER_BLIS_IMPL_\0").map(|sym| *sym),
            CHPR_BLIS_IMPL: get_symbol(&libs, b"CHPR_BLIS_IMPL\0").map(|sym| *sym),
            chpr_blis_impl_: get_symbol(&libs, b"chpr_blis_impl_\0").map(|sym| *sym),
            CHPR_BLIS_IMPL_: get_symbol(&libs, b"CHPR_BLIS_IMPL_\0").map(|sym| *sym),
            CHER2_BLIS_IMPL: get_symbol(&libs, b"CHER2_BLIS_IMPL\0").map(|sym| *sym),
            cher2_blis_impl_: get_symbol(&libs, b"cher2_blis_impl_\0").map(|sym| *sym),
            CHER2_BLIS_IMPL_: get_symbol(&libs, b"CHER2_BLIS_IMPL_\0").map(|sym| *sym),
            CHPR2_BLIS_IMPL: get_symbol(&libs, b"CHPR2_BLIS_IMPL\0").map(|sym| *sym),
            chpr2_blis_impl_: get_symbol(&libs, b"chpr2_blis_impl_\0").map(|sym| *sym),
            CHPR2_BLIS_IMPL_: get_symbol(&libs, b"CHPR2_BLIS_IMPL_\0").map(|sym| *sym),
            ZGEMV_BLIS_IMPL: get_symbol(&libs, b"ZGEMV_BLIS_IMPL\0").map(|sym| *sym),
            zgemv_blis_impl_: get_symbol(&libs, b"zgemv_blis_impl_\0").map(|sym| *sym),
            ZGEMV_BLIS_IMPL_: get_symbol(&libs, b"ZGEMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZGBMV_BLIS_IMPL: get_symbol(&libs, b"ZGBMV_BLIS_IMPL\0").map(|sym| *sym),
            zgbmv_blis_impl_: get_symbol(&libs, b"zgbmv_blis_impl_\0").map(|sym| *sym),
            ZGBMV_BLIS_IMPL_: get_symbol(&libs, b"ZGBMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZHEMV_BLIS_IMPL: get_symbol(&libs, b"ZHEMV_BLIS_IMPL\0").map(|sym| *sym),
            zhemv_blis_impl_: get_symbol(&libs, b"zhemv_blis_impl_\0").map(|sym| *sym),
            ZHEMV_BLIS_IMPL_: get_symbol(&libs, b"ZHEMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZHBMV_BLIS_IMPL: get_symbol(&libs, b"ZHBMV_BLIS_IMPL\0").map(|sym| *sym),
            zhbmv_blis_impl_: get_symbol(&libs, b"zhbmv_blis_impl_\0").map(|sym| *sym),
            ZHBMV_BLIS_IMPL_: get_symbol(&libs, b"ZHBMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZHPMV_BLIS_IMPL: get_symbol(&libs, b"ZHPMV_BLIS_IMPL\0").map(|sym| *sym),
            zhpmv_blis_impl_: get_symbol(&libs, b"zhpmv_blis_impl_\0").map(|sym| *sym),
            ZHPMV_BLIS_IMPL_: get_symbol(&libs, b"ZHPMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZTRMV_BLIS_IMPL: get_symbol(&libs, b"ZTRMV_BLIS_IMPL\0").map(|sym| *sym),
            ztrmv_blis_impl_: get_symbol(&libs, b"ztrmv_blis_impl_\0").map(|sym| *sym),
            ZTRMV_BLIS_IMPL_: get_symbol(&libs, b"ZTRMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZTBMV_BLIS_IMPL: get_symbol(&libs, b"ZTBMV_BLIS_IMPL\0").map(|sym| *sym),
            ztbmv_blis_impl_: get_symbol(&libs, b"ztbmv_blis_impl_\0").map(|sym| *sym),
            ZTBMV_BLIS_IMPL_: get_symbol(&libs, b"ZTBMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZTPMV_BLIS_IMPL: get_symbol(&libs, b"ZTPMV_BLIS_IMPL\0").map(|sym| *sym),
            ztpmv_blis_impl_: get_symbol(&libs, b"ztpmv_blis_impl_\0").map(|sym| *sym),
            ZTPMV_BLIS_IMPL_: get_symbol(&libs, b"ZTPMV_BLIS_IMPL_\0").map(|sym| *sym),
            ZTRSV_BLIS_IMPL: get_symbol(&libs, b"ZTRSV_BLIS_IMPL\0").map(|sym| *sym),
            ztrsv_blis_impl_: get_symbol(&libs, b"ztrsv_blis_impl_\0").map(|sym| *sym),
            ZTRSV_BLIS_IMPL_: get_symbol(&libs, b"ZTRSV_BLIS_IMPL_\0").map(|sym| *sym),
            ZTBSV_BLIS_IMPL: get_symbol(&libs, b"ZTBSV_BLIS_IMPL\0").map(|sym| *sym),
            ztbsv_blis_impl_: get_symbol(&libs, b"ztbsv_blis_impl_\0").map(|sym| *sym),
            ZTBSV_BLIS_IMPL_: get_symbol(&libs, b"ZTBSV_BLIS_IMPL_\0").map(|sym| *sym),
            ZTPSV_BLIS_IMPL: get_symbol(&libs, b"ZTPSV_BLIS_IMPL\0").map(|sym| *sym),
            ztpsv_blis_impl_: get_symbol(&libs, b"ztpsv_blis_impl_\0").map(|sym| *sym),
            ZTPSV_BLIS_IMPL_: get_symbol(&libs, b"ZTPSV_BLIS_IMPL_\0").map(|sym| *sym),
            ZGERU_BLIS_IMPL: get_symbol(&libs, b"ZGERU_BLIS_IMPL\0").map(|sym| *sym),
            zgeru_blis_impl_: get_symbol(&libs, b"zgeru_blis_impl_\0").map(|sym| *sym),
            ZGERU_BLIS_IMPL_: get_symbol(&libs, b"ZGERU_BLIS_IMPL_\0").map(|sym| *sym),
            ZGERC_BLIS_IMPL: get_symbol(&libs, b"ZGERC_BLIS_IMPL\0").map(|sym| *sym),
            zgerc_blis_impl_: get_symbol(&libs, b"zgerc_blis_impl_\0").map(|sym| *sym),
            ZGERC_BLIS_IMPL_: get_symbol(&libs, b"ZGERC_BLIS_IMPL_\0").map(|sym| *sym),
            ZHER_BLIS_IMPL: get_symbol(&libs, b"ZHER_BLIS_IMPL\0").map(|sym| *sym),
            zher_blis_impl_: get_symbol(&libs, b"zher_blis_impl_\0").map(|sym| *sym),
            ZHER_BLIS_IMPL_: get_symbol(&libs, b"ZHER_BLIS_IMPL_\0").map(|sym| *sym),
            ZHPR_BLIS_IMPL: get_symbol(&libs, b"ZHPR_BLIS_IMPL\0").map(|sym| *sym),
            zhpr_blis_impl_: get_symbol(&libs, b"zhpr_blis_impl_\0").map(|sym| *sym),
            ZHPR_BLIS_IMPL_: get_symbol(&libs, b"ZHPR_BLIS_IMPL_\0").map(|sym| *sym),
            ZHER2_BLIS_IMPL: get_symbol(&libs, b"ZHER2_BLIS_IMPL\0").map(|sym| *sym),
            zher2_blis_impl_: get_symbol(&libs, b"zher2_blis_impl_\0").map(|sym| *sym),
            ZHER2_BLIS_IMPL_: get_symbol(&libs, b"ZHER2_BLIS_IMPL_\0").map(|sym| *sym),
            ZHPR2_BLIS_IMPL: get_symbol(&libs, b"ZHPR2_BLIS_IMPL\0").map(|sym| *sym),
            zhpr2_blis_impl_: get_symbol(&libs, b"zhpr2_blis_impl_\0").map(|sym| *sym),
            ZHPR2_BLIS_IMPL_: get_symbol(&libs, b"ZHPR2_BLIS_IMPL_\0").map(|sym| *sym),
            SGEMM_BLIS_IMPL: get_symbol(&libs, b"SGEMM_BLIS_IMPL\0").map(|sym| *sym),
            sgemm_blis_impl_: get_symbol(&libs, b"sgemm_blis_impl_\0").map(|sym| *sym),
            SGEMM_BLIS_IMPL_: get_symbol(&libs, b"SGEMM_BLIS_IMPL_\0").map(|sym| *sym),
            SSYMM_BLIS_IMPL: get_symbol(&libs, b"SSYMM_BLIS_IMPL\0").map(|sym| *sym),
            ssymm_blis_impl_: get_symbol(&libs, b"ssymm_blis_impl_\0").map(|sym| *sym),
            SSYMM_BLIS_IMPL_: get_symbol(&libs, b"SSYMM_BLIS_IMPL_\0").map(|sym| *sym),
            SSYRK_BLIS_IMPL: get_symbol(&libs, b"SSYRK_BLIS_IMPL\0").map(|sym| *sym),
            ssyrk_blis_impl_: get_symbol(&libs, b"ssyrk_blis_impl_\0").map(|sym| *sym),
            SSYRK_BLIS_IMPL_: get_symbol(&libs, b"SSYRK_BLIS_IMPL_\0").map(|sym| *sym),
            SSYR2K_BLIS_IMPL: get_symbol(&libs, b"SSYR2K_BLIS_IMPL\0").map(|sym| *sym),
            ssyr2k_blis_impl_: get_symbol(&libs, b"ssyr2k_blis_impl_\0").map(|sym| *sym),
            SSYR2K_BLIS_IMPL_: get_symbol(&libs, b"SSYR2K_BLIS_IMPL_\0").map(|sym| *sym),
            STRMM_BLIS_IMPL: get_symbol(&libs, b"STRMM_BLIS_IMPL\0").map(|sym| *sym),
            strmm_blis_impl_: get_symbol(&libs, b"strmm_blis_impl_\0").map(|sym| *sym),
            STRMM_BLIS_IMPL_: get_symbol(&libs, b"STRMM_BLIS_IMPL_\0").map(|sym| *sym),
            STRSM_BLIS_IMPL: get_symbol(&libs, b"STRSM_BLIS_IMPL\0").map(|sym| *sym),
            strsm_blis_impl_: get_symbol(&libs, b"strsm_blis_impl_\0").map(|sym| *sym),
            STRSM_BLIS_IMPL_: get_symbol(&libs, b"STRSM_BLIS_IMPL_\0").map(|sym| *sym),
            DGEMM_BLIS_IMPL: get_symbol(&libs, b"DGEMM_BLIS_IMPL\0").map(|sym| *sym),
            dgemm_blis_impl_: get_symbol(&libs, b"dgemm_blis_impl_\0").map(|sym| *sym),
            DGEMM_BLIS_IMPL_: get_symbol(&libs, b"DGEMM_BLIS_IMPL_\0").map(|sym| *sym),
            DZGEMM_BLIS_IMPL: get_symbol(&libs, b"DZGEMM_BLIS_IMPL\0").map(|sym| *sym),
            dzgemm_blis_impl_: get_symbol(&libs, b"dzgemm_blis_impl_\0").map(|sym| *sym),
            DZGEMM_BLIS_IMPL_: get_symbol(&libs, b"DZGEMM_BLIS_IMPL_\0").map(|sym| *sym),
            DSYMM_BLIS_IMPL: get_symbol(&libs, b"DSYMM_BLIS_IMPL\0").map(|sym| *sym),
            dsymm_blis_impl_: get_symbol(&libs, b"dsymm_blis_impl_\0").map(|sym| *sym),
            DSYMM_BLIS_IMPL_: get_symbol(&libs, b"DSYMM_BLIS_IMPL_\0").map(|sym| *sym),
            DSYRK_BLIS_IMPL: get_symbol(&libs, b"DSYRK_BLIS_IMPL\0").map(|sym| *sym),
            dsyrk_blis_impl_: get_symbol(&libs, b"dsyrk_blis_impl_\0").map(|sym| *sym),
            DSYRK_BLIS_IMPL_: get_symbol(&libs, b"DSYRK_BLIS_IMPL_\0").map(|sym| *sym),
            DSYR2K_BLIS_IMPL: get_symbol(&libs, b"DSYR2K_BLIS_IMPL\0").map(|sym| *sym),
            dsyr2k_blis_impl_: get_symbol(&libs, b"dsyr2k_blis_impl_\0").map(|sym| *sym),
            DSYR2K_BLIS_IMPL_: get_symbol(&libs, b"DSYR2K_BLIS_IMPL_\0").map(|sym| *sym),
            DTRMM_BLIS_IMPL: get_symbol(&libs, b"DTRMM_BLIS_IMPL\0").map(|sym| *sym),
            dtrmm_blis_impl_: get_symbol(&libs, b"dtrmm_blis_impl_\0").map(|sym| *sym),
            DTRMM_BLIS_IMPL_: get_symbol(&libs, b"DTRMM_BLIS_IMPL_\0").map(|sym| *sym),
            DTRSM_BLIS_IMPL: get_symbol(&libs, b"DTRSM_BLIS_IMPL\0").map(|sym| *sym),
            dtrsm_blis_impl_: get_symbol(&libs, b"dtrsm_blis_impl_\0").map(|sym| *sym),
            DTRSM_BLIS_IMPL_: get_symbol(&libs, b"DTRSM_BLIS_IMPL_\0").map(|sym| *sym),
            CGEMM_BLIS_IMPL: get_symbol(&libs, b"CGEMM_BLIS_IMPL\0").map(|sym| *sym),
            cgemm_blis_impl_: get_symbol(&libs, b"cgemm_blis_impl_\0").map(|sym| *sym),
            CGEMM_BLIS_IMPL_: get_symbol(&libs, b"CGEMM_BLIS_IMPL_\0").map(|sym| *sym),
            CSYMM_BLIS_IMPL: get_symbol(&libs, b"CSYMM_BLIS_IMPL\0").map(|sym| *sym),
            csymm_blis_impl_: get_symbol(&libs, b"csymm_blis_impl_\0").map(|sym| *sym),
            CSYMM_BLIS_IMPL_: get_symbol(&libs, b"CSYMM_BLIS_IMPL_\0").map(|sym| *sym),
            CHEMM_BLIS_IMPL: get_symbol(&libs, b"CHEMM_BLIS_IMPL\0").map(|sym| *sym),
            chemm_blis_impl_: get_symbol(&libs, b"chemm_blis_impl_\0").map(|sym| *sym),
            CHEMM_BLIS_IMPL_: get_symbol(&libs, b"CHEMM_BLIS_IMPL_\0").map(|sym| *sym),
            CSYRK_BLIS_IMPL: get_symbol(&libs, b"CSYRK_BLIS_IMPL\0").map(|sym| *sym),
            csyrk_blis_impl_: get_symbol(&libs, b"csyrk_blis_impl_\0").map(|sym| *sym),
            CSYRK_BLIS_IMPL_: get_symbol(&libs, b"CSYRK_BLIS_IMPL_\0").map(|sym| *sym),
            CHERK_BLIS_IMPL: get_symbol(&libs, b"CHERK_BLIS_IMPL\0").map(|sym| *sym),
            cherk_blis_impl_: get_symbol(&libs, b"cherk_blis_impl_\0").map(|sym| *sym),
            CHERK_BLIS_IMPL_: get_symbol(&libs, b"CHERK_BLIS_IMPL_\0").map(|sym| *sym),
            CSYR2K_BLIS_IMPL: get_symbol(&libs, b"CSYR2K_BLIS_IMPL\0").map(|sym| *sym),
            csyr2k_blis_impl_: get_symbol(&libs, b"csyr2k_blis_impl_\0").map(|sym| *sym),
            CSYR2K_BLIS_IMPL_: get_symbol(&libs, b"CSYR2K_BLIS_IMPL_\0").map(|sym| *sym),
            CHER2K_BLIS_IMPL: get_symbol(&libs, b"CHER2K_BLIS_IMPL\0").map(|sym| *sym),
            cher2k_blis_impl_: get_symbol(&libs, b"cher2k_blis_impl_\0").map(|sym| *sym),
            CHER2K_BLIS_IMPL_: get_symbol(&libs, b"CHER2K_BLIS_IMPL_\0").map(|sym| *sym),
            CTRMM_BLIS_IMPL: get_symbol(&libs, b"CTRMM_BLIS_IMPL\0").map(|sym| *sym),
            ctrmm_blis_impl_: get_symbol(&libs, b"ctrmm_blis_impl_\0").map(|sym| *sym),
            CTRMM_BLIS_IMPL_: get_symbol(&libs, b"CTRMM_BLIS_IMPL_\0").map(|sym| *sym),
            CTRSM_BLIS_IMPL: get_symbol(&libs, b"CTRSM_BLIS_IMPL\0").map(|sym| *sym),
            ctrsm_blis_impl_: get_symbol(&libs, b"ctrsm_blis_impl_\0").map(|sym| *sym),
            CTRSM_BLIS_IMPL_: get_symbol(&libs, b"CTRSM_BLIS_IMPL_\0").map(|sym| *sym),
            ZGEMM_BLIS_IMPL: get_symbol(&libs, b"ZGEMM_BLIS_IMPL\0").map(|sym| *sym),
            zgemm_blis_impl_: get_symbol(&libs, b"zgemm_blis_impl_\0").map(|sym| *sym),
            ZGEMM_BLIS_IMPL_: get_symbol(&libs, b"ZGEMM_BLIS_IMPL_\0").map(|sym| *sym),
            ZSYMM_BLIS_IMPL: get_symbol(&libs, b"ZSYMM_BLIS_IMPL\0").map(|sym| *sym),
            zsymm_blis_impl_: get_symbol(&libs, b"zsymm_blis_impl_\0").map(|sym| *sym),
            ZSYMM_BLIS_IMPL_: get_symbol(&libs, b"ZSYMM_BLIS_IMPL_\0").map(|sym| *sym),
            ZHEMM_BLIS_IMPL: get_symbol(&libs, b"ZHEMM_BLIS_IMPL\0").map(|sym| *sym),
            zhemm_blis_impl_: get_symbol(&libs, b"zhemm_blis_impl_\0").map(|sym| *sym),
            ZHEMM_BLIS_IMPL_: get_symbol(&libs, b"ZHEMM_BLIS_IMPL_\0").map(|sym| *sym),
            ZSYRK_BLIS_IMPL: get_symbol(&libs, b"ZSYRK_BLIS_IMPL\0").map(|sym| *sym),
            zsyrk_blis_impl_: get_symbol(&libs, b"zsyrk_blis_impl_\0").map(|sym| *sym),
            ZSYRK_BLIS_IMPL_: get_symbol(&libs, b"ZSYRK_BLIS_IMPL_\0").map(|sym| *sym),
            ZHERK_BLIS_IMPL: get_symbol(&libs, b"ZHERK_BLIS_IMPL\0").map(|sym| *sym),
            zherk_blis_impl_: get_symbol(&libs, b"zherk_blis_impl_\0").map(|sym| *sym),
            ZHERK_BLIS_IMPL_: get_symbol(&libs, b"ZHERK_BLIS_IMPL_\0").map(|sym| *sym),
            ZSYR2K_BLIS_IMPL: get_symbol(&libs, b"ZSYR2K_BLIS_IMPL\0").map(|sym| *sym),
            zsyr2k_blis_impl_: get_symbol(&libs, b"zsyr2k_blis_impl_\0").map(|sym| *sym),
            ZSYR2K_BLIS_IMPL_: get_symbol(&libs, b"ZSYR2K_BLIS_IMPL_\0").map(|sym| *sym),
            ZHER2K_BLIS_IMPL: get_symbol(&libs, b"ZHER2K_BLIS_IMPL\0").map(|sym| *sym),
            zher2k_blis_impl_: get_symbol(&libs, b"zher2k_blis_impl_\0").map(|sym| *sym),
            ZHER2K_BLIS_IMPL_: get_symbol(&libs, b"ZHER2K_BLIS_IMPL_\0").map(|sym| *sym),
            ZTRMM_BLIS_IMPL: get_symbol(&libs, b"ZTRMM_BLIS_IMPL\0").map(|sym| *sym),
            ztrmm_blis_impl_: get_symbol(&libs, b"ztrmm_blis_impl_\0").map(|sym| *sym),
            ZTRMM_BLIS_IMPL_: get_symbol(&libs, b"ZTRMM_BLIS_IMPL_\0").map(|sym| *sym),
            ZTRSM_BLIS_IMPL: get_symbol(&libs, b"ZTRSM_BLIS_IMPL\0").map(|sym| *sym),
            ztrsm_blis_impl_: get_symbol(&libs, b"ztrsm_blis_impl_\0").map(|sym| *sym),
            ZTRSM_BLIS_IMPL_: get_symbol(&libs, b"ZTRSM_BLIS_IMPL_\0").map(|sym| *sym),
            CDOTCSUB_BLIS_IMPL: get_symbol(&libs, b"CDOTCSUB_BLIS_IMPL\0").map(|sym| *sym),
            cdotcsub_blis_impl_: get_symbol(&libs, b"cdotcsub_blis_impl_\0").map(|sym| *sym),
            CDOTCSUB_BLIS_IMPL_: get_symbol(&libs, b"CDOTCSUB_BLIS_IMPL_\0").map(|sym| *sym),
            CDOTUSUB_BLIS_IMPL: get_symbol(&libs, b"CDOTUSUB_BLIS_IMPL\0").map(|sym| *sym),
            cdotusub_blis_impl_: get_symbol(&libs, b"cdotusub_blis_impl_\0").map(|sym| *sym),
            CDOTUSUB_BLIS_IMPL_: get_symbol(&libs, b"CDOTUSUB_BLIS_IMPL_\0").map(|sym| *sym),
            DASUMSUB_BLIS_IMPL: get_symbol(&libs, b"DASUMSUB_BLIS_IMPL\0").map(|sym| *sym),
            dasumsub_blis_impl_: get_symbol(&libs, b"dasumsub_blis_impl_\0").map(|sym| *sym),
            DASUMSUB_BLIS_IMPL_: get_symbol(&libs, b"DASUMSUB_BLIS_IMPL_\0").map(|sym| *sym),
            DDOTSUB_BLIS_IMPL: get_symbol(&libs, b"DDOTSUB_BLIS_IMPL\0").map(|sym| *sym),
            ddotsub_blis_impl_: get_symbol(&libs, b"ddotsub_blis_impl_\0").map(|sym| *sym),
            DDOTSUB_BLIS_IMPL_: get_symbol(&libs, b"DDOTSUB_BLIS_IMPL_\0").map(|sym| *sym),
            DNRM2SUB_BLIS_IMPL: get_symbol(&libs, b"DNRM2SUB_BLIS_IMPL\0").map(|sym| *sym),
            dnrm2sub_blis_impl_: get_symbol(&libs, b"dnrm2sub_blis_impl_\0").map(|sym| *sym),
            DNRM2SUB_BLIS_IMPL_: get_symbol(&libs, b"DNRM2SUB_BLIS_IMPL_\0").map(|sym| *sym),
            DZASUMSUB_BLIS_IMPL: get_symbol(&libs, b"DZASUMSUB_BLIS_IMPL\0").map(|sym| *sym),
            dzasumsub_blis_impl_: get_symbol(&libs, b"dzasumsub_blis_impl_\0").map(|sym| *sym),
            DZASUMSUB_BLIS_IMPL_: get_symbol(&libs, b"DZASUMSUB_BLIS_IMPL_\0").map(|sym| *sym),
            DZNRM2SUB_BLIS_IMPL: get_symbol(&libs, b"DZNRM2SUB_BLIS_IMPL\0").map(|sym| *sym),
            dznrm2sub_blis_impl_: get_symbol(&libs, b"dznrm2sub_blis_impl_\0").map(|sym| *sym),
            DZNRM2SUB_BLIS_IMPL_: get_symbol(&libs, b"DZNRM2SUB_BLIS_IMPL_\0").map(|sym| *sym),
            ICAMAXSUB_BLIS_IMPL: get_symbol(&libs, b"ICAMAXSUB_BLIS_IMPL\0").map(|sym| *sym),
            icamaxsub_blis_impl_: get_symbol(&libs, b"icamaxsub_blis_impl_\0").map(|sym| *sym),
            ICAMAXSUB_BLIS_IMPL_: get_symbol(&libs, b"ICAMAXSUB_BLIS_IMPL_\0").map(|sym| *sym),
            ICAMINSUB_BLIS_IMPL: get_symbol(&libs, b"ICAMINSUB_BLIS_IMPL\0").map(|sym| *sym),
            icaminsub_blis_impl_: get_symbol(&libs, b"icaminsub_blis_impl_\0").map(|sym| *sym),
            ICAMINSUB_BLIS_IMPL_: get_symbol(&libs, b"ICAMINSUB_BLIS_IMPL_\0").map(|sym| *sym),
            IDAMAXSUB_BLIS_IMPL: get_symbol(&libs, b"IDAMAXSUB_BLIS_IMPL\0").map(|sym| *sym),
            idamaxsub_blis_impl_: get_symbol(&libs, b"idamaxsub_blis_impl_\0").map(|sym| *sym),
            IDAMAXSUB_BLIS_IMPL_: get_symbol(&libs, b"IDAMAXSUB_BLIS_IMPL_\0").map(|sym| *sym),
            IDAMINSUB_BLIS_IMPL: get_symbol(&libs, b"IDAMINSUB_BLIS_IMPL\0").map(|sym| *sym),
            idaminsub_blis_impl_: get_symbol(&libs, b"idaminsub_blis_impl_\0").map(|sym| *sym),
            IDAMINSUB_BLIS_IMPL_: get_symbol(&libs, b"IDAMINSUB_BLIS_IMPL_\0").map(|sym| *sym),
            ISAMAXSUB_BLIS_IMPL: get_symbol(&libs, b"ISAMAXSUB_BLIS_IMPL\0").map(|sym| *sym),
            isamaxsub_blis_impl_: get_symbol(&libs, b"isamaxsub_blis_impl_\0").map(|sym| *sym),
            ISAMAXSUB_BLIS_IMPL_: get_symbol(&libs, b"ISAMAXSUB_BLIS_IMPL_\0").map(|sym| *sym),
            ISAMINSUB_BLIS_IMPL: get_symbol(&libs, b"ISAMINSUB_BLIS_IMPL\0").map(|sym| *sym),
            isaminsub_blis_impl_: get_symbol(&libs, b"isaminsub_blis_impl_\0").map(|sym| *sym),
            ISAMINSUB_BLIS_IMPL_: get_symbol(&libs, b"ISAMINSUB_BLIS_IMPL_\0").map(|sym| *sym),
            IZAMINSUB_BLIS_IMPL: get_symbol(&libs, b"IZAMINSUB_BLIS_IMPL\0").map(|sym| *sym),
            izaminsub_blis_impl_: get_symbol(&libs, b"izaminsub_blis_impl_\0").map(|sym| *sym),
            IZAMINSUB_BLIS_IMPL_: get_symbol(&libs, b"IZAMINSUB_BLIS_IMPL_\0").map(|sym| *sym),
            IZAMAXSUB_BLIS_IMPL: get_symbol(&libs, b"IZAMAXSUB_BLIS_IMPL\0").map(|sym| *sym),
            izamaxsub_blis_impl_: get_symbol(&libs, b"izamaxsub_blis_impl_\0").map(|sym| *sym),
            IZAMAXSUB_BLIS_IMPL_: get_symbol(&libs, b"IZAMAXSUB_BLIS_IMPL_\0").map(|sym| *sym),
            SASUMSUB_BLIS_IMPL: get_symbol(&libs, b"SASUMSUB_BLIS_IMPL\0").map(|sym| *sym),
            sasumsub_blis_impl_: get_symbol(&libs, b"sasumsub_blis_impl_\0").map(|sym| *sym),
            SASUMSUB_BLIS_IMPL_: get_symbol(&libs, b"SASUMSUB_BLIS_IMPL_\0").map(|sym| *sym),
            SCASUMSUB_BLIS_IMPL: get_symbol(&libs, b"SCASUMSUB_BLIS_IMPL\0").map(|sym| *sym),
            scasumsub_blis_impl_: get_symbol(&libs, b"scasumsub_blis_impl_\0").map(|sym| *sym),
            SCASUMSUB_BLIS_IMPL_: get_symbol(&libs, b"SCASUMSUB_BLIS_IMPL_\0").map(|sym| *sym),
            SCNRM2SUB_BLIS_IMPL: get_symbol(&libs, b"SCNRM2SUB_BLIS_IMPL\0").map(|sym| *sym),
            scnrm2sub_blis_impl_: get_symbol(&libs, b"scnrm2sub_blis_impl_\0").map(|sym| *sym),
            SCNRM2SUB_BLIS_IMPL_: get_symbol(&libs, b"SCNRM2SUB_BLIS_IMPL_\0").map(|sym| *sym),
            SDOTSUB_BLIS_IMPL: get_symbol(&libs, b"SDOTSUB_BLIS_IMPL\0").map(|sym| *sym),
            sdotsub_blis_impl_: get_symbol(&libs, b"sdotsub_blis_impl_\0").map(|sym| *sym),
            SDOTSUB_BLIS_IMPL_: get_symbol(&libs, b"SDOTSUB_BLIS_IMPL_\0").map(|sym| *sym),
            SNRM2SUB_BLIS_IMPL: get_symbol(&libs, b"SNRM2SUB_BLIS_IMPL\0").map(|sym| *sym),
            snrm2sub_blis_impl_: get_symbol(&libs, b"snrm2sub_blis_impl_\0").map(|sym| *sym),
            SNRM2SUB_BLIS_IMPL_: get_symbol(&libs, b"SNRM2SUB_BLIS_IMPL_\0").map(|sym| *sym),
            ZDOTCSUB_BLIS_IMPL: get_symbol(&libs, b"ZDOTCSUB_BLIS_IMPL\0").map(|sym| *sym),
            zdotcsub_blis_impl_: get_symbol(&libs, b"zdotcsub_blis_impl_\0").map(|sym| *sym),
            ZDOTCSUB_BLIS_IMPL_: get_symbol(&libs, b"ZDOTCSUB_BLIS_IMPL_\0").map(|sym| *sym),
            ZDOTUSUB_BLIS_IMPL: get_symbol(&libs, b"ZDOTUSUB_BLIS_IMPL\0").map(|sym| *sym),
            zdotusub_blis_impl_: get_symbol(&libs, b"zdotusub_blis_impl_\0").map(|sym| *sym),
            ZDOTUSUB_BLIS_IMPL_: get_symbol(&libs, b"ZDOTUSUB_BLIS_IMPL_\0").map(|sym| *sym),
            SDSDOTSUB_BLIS_IMPL: get_symbol(&libs, b"SDSDOTSUB_BLIS_IMPL\0").map(|sym| *sym),
            sdsdotsub_blis_impl_: get_symbol(&libs, b"sdsdotsub_blis_impl_\0").map(|sym| *sym),
            SDSDOTSUB_BLIS_IMPL_: get_symbol(&libs, b"SDSDOTSUB_BLIS_IMPL_\0").map(|sym| *sym),
            DSDOTSUB_BLIS_IMPL: get_symbol(&libs, b"DSDOTSUB_BLIS_IMPL\0").map(|sym| *sym),
            dsdotsub_blis_impl_: get_symbol(&libs, b"dsdotsub_blis_impl_\0").map(|sym| *sym),
            DSDOTSUB_BLIS_IMPL_: get_symbol(&libs, b"DSDOTSUB_BLIS_IMPL_\0").map(|sym| *sym),
            LSAME_BLIS_IMPL: get_symbol(&libs, b"LSAME_BLIS_IMPL\0").map(|sym| *sym),
            lsame_blis_impl_: get_symbol(&libs, b"lsame_blis_impl_\0").map(|sym| *sym),
            LSAME_BLIS_IMPL_: get_symbol(&libs, b"LSAME_BLIS_IMPL_\0").map(|sym| *sym),
            XERBLA_BLIS_IMPL: get_symbol(&libs, b"XERBLA_BLIS_IMPL\0").map(|sym| *sym),
            xerbla_blis_impl_: get_symbol(&libs, b"xerbla_blis_impl_\0").map(|sym| *sym),
            XERBLA_BLIS_IMPL_: get_symbol(&libs, b"XERBLA_BLIS_IMPL_\0").map(|sym| *sym),
            DCABS1_BLIS_IMPL: get_symbol(&libs, b"DCABS1_BLIS_IMPL\0").map(|sym| *sym),
            dcabs1_blis_impl_: get_symbol(&libs, b"dcabs1_blis_impl_\0").map(|sym| *sym),
            DCABS1_BLIS_IMPL_: get_symbol(&libs, b"DCABS1_BLIS_IMPL_\0").map(|sym| *sym),
            SCABS1_BLIS_IMPL: get_symbol(&libs, b"SCABS1_BLIS_IMPL\0").map(|sym| *sym),
            scabs1_blis_impl_: get_symbol(&libs, b"scabs1_blis_impl_\0").map(|sym| *sym),
            SCABS1_BLIS_IMPL_: get_symbol(&libs, b"SCABS1_BLIS_IMPL_\0").map(|sym| *sym),
            CAXPBY_BLIS_IMPL: get_symbol(&libs, b"CAXPBY_BLIS_IMPL\0").map(|sym| *sym),
            caxpby_blis_impl_: get_symbol(&libs, b"caxpby_blis_impl_\0").map(|sym| *sym),
            CAXPBY_BLIS_IMPL_: get_symbol(&libs, b"CAXPBY_BLIS_IMPL_\0").map(|sym| *sym),
            CGEMM3M_BLIS_IMPL: get_symbol(&libs, b"CGEMM3M_BLIS_IMPL\0").map(|sym| *sym),
            cgemm3m_blis_impl_: get_symbol(&libs, b"cgemm3m_blis_impl_\0").map(|sym| *sym),
            CGEMM3M_BLIS_IMPL_: get_symbol(&libs, b"CGEMM3M_BLIS_IMPL_\0").map(|sym| *sym),
            CGEMM_BATCH_BLIS_IMPL: get_symbol(&libs, b"CGEMM_BATCH_BLIS_IMPL\0").map(|sym| *sym),
            cgemm_batch_blis_impl_: get_symbol(&libs, b"cgemm_batch_blis_impl_\0").map(|sym| *sym),
            CGEMM_BATCH_BLIS_IMPL_: get_symbol(&libs, b"CGEMM_BATCH_BLIS_IMPL_\0").map(|sym| *sym),
            CGEMMT_BLIS_IMPL: get_symbol(&libs, b"CGEMMT_BLIS_IMPL\0").map(|sym| *sym),
            cgemmt_blis_impl_: get_symbol(&libs, b"cgemmt_blis_impl_\0").map(|sym| *sym),
            CGEMMT_BLIS_IMPL_: get_symbol(&libs, b"CGEMMT_BLIS_IMPL_\0").map(|sym| *sym),
            DAXPBY_BLIS_IMPL: get_symbol(&libs, b"DAXPBY_BLIS_IMPL\0").map(|sym| *sym),
            daxpby_blis_impl_: get_symbol(&libs, b"daxpby_blis_impl_\0").map(|sym| *sym),
            DAXPBY_BLIS_IMPL_: get_symbol(&libs, b"DAXPBY_BLIS_IMPL_\0").map(|sym| *sym),
            DGEMM_BATCH_BLIS_IMPL: get_symbol(&libs, b"DGEMM_BATCH_BLIS_IMPL\0").map(|sym| *sym),
            dgemm_batch_blis_impl_: get_symbol(&libs, b"dgemm_batch_blis_impl_\0").map(|sym| *sym),
            DGEMM_BATCH_BLIS_IMPL_: get_symbol(&libs, b"DGEMM_BATCH_BLIS_IMPL_\0").map(|sym| *sym),
            DGEMM_PACK_GET_SIZE_BLIS_IMPL: get_symbol(&libs, b"DGEMM_PACK_GET_SIZE_BLIS_IMPL\0")
                .map(|sym| *sym),
            dgemm_pack_get_size_blis_impl_: get_symbol(&libs, b"dgemm_pack_get_size_blis_impl_\0")
                .map(|sym| *sym),
            DGEMM_PACK_GET_SIZE_BLIS_IMPL_: get_symbol(&libs, b"DGEMM_PACK_GET_SIZE_BLIS_IMPL_\0")
                .map(|sym| *sym),
            DGEMM_PACK_BLIS_IMPL: get_symbol(&libs, b"DGEMM_PACK_BLIS_IMPL\0").map(|sym| *sym),
            dgemm_pack_blis_impl_: get_symbol(&libs, b"dgemm_pack_blis_impl_\0").map(|sym| *sym),
            DGEMM_PACK_BLIS_IMPL_: get_symbol(&libs, b"DGEMM_PACK_BLIS_IMPL_\0").map(|sym| *sym),
            DGEMM_COMPUTE_BLIS_IMPL: get_symbol(&libs, b"DGEMM_COMPUTE_BLIS_IMPL\0")
                .map(|sym| *sym),
            dgemm_compute_blis_impl_: get_symbol(&libs, b"dgemm_compute_blis_impl_\0")
                .map(|sym| *sym),
            DGEMM_COMPUTE_BLIS_IMPL_: get_symbol(&libs, b"DGEMM_COMPUTE_BLIS_IMPL_\0")
                .map(|sym| *sym),
            DGEMMT_BLIS_IMPL: get_symbol(&libs, b"DGEMMT_BLIS_IMPL\0").map(|sym| *sym),
            dgemmt_blis_impl_: get_symbol(&libs, b"dgemmt_blis_impl_\0").map(|sym| *sym),
            DGEMMT_BLIS_IMPL_: get_symbol(&libs, b"DGEMMT_BLIS_IMPL_\0").map(|sym| *sym),
            SAXPBY_BLIS_IMPL: get_symbol(&libs, b"SAXPBY_BLIS_IMPL\0").map(|sym| *sym),
            saxpby_blis_impl_: get_symbol(&libs, b"saxpby_blis_impl_\0").map(|sym| *sym),
            SAXPBY_BLIS_IMPL_: get_symbol(&libs, b"SAXPBY_BLIS_IMPL_\0").map(|sym| *sym),
            SGEMM_BATCH_BLIS_IMPL: get_symbol(&libs, b"SGEMM_BATCH_BLIS_IMPL\0").map(|sym| *sym),
            sgemm_batch_blis_impl_: get_symbol(&libs, b"sgemm_batch_blis_impl_\0").map(|sym| *sym),
            SGEMM_BATCH_BLIS_IMPL_: get_symbol(&libs, b"SGEMM_BATCH_BLIS_IMPL_\0").map(|sym| *sym),
            SGEMM_PACK_GET_SIZE_BLIS_IMPL: get_symbol(&libs, b"SGEMM_PACK_GET_SIZE_BLIS_IMPL\0")
                .map(|sym| *sym),
            sgemm_pack_get_size_blis_impl_: get_symbol(&libs, b"sgemm_pack_get_size_blis_impl_\0")
                .map(|sym| *sym),
            SGEMM_PACK_GET_SIZE_BLIS_IMPL_: get_symbol(&libs, b"SGEMM_PACK_GET_SIZE_BLIS_IMPL_\0")
                .map(|sym| *sym),
            SGEMM_PACK_BLIS_IMPL: get_symbol(&libs, b"SGEMM_PACK_BLIS_IMPL\0").map(|sym| *sym),
            sgemm_pack_blis_impl_: get_symbol(&libs, b"sgemm_pack_blis_impl_\0").map(|sym| *sym),
            SGEMM_PACK_BLIS_IMPL_: get_symbol(&libs, b"SGEMM_PACK_BLIS_IMPL_\0").map(|sym| *sym),
            SGEMM_COMPUTE_BLIS_IMPL: get_symbol(&libs, b"SGEMM_COMPUTE_BLIS_IMPL\0")
                .map(|sym| *sym),
            sgemm_compute_blis_impl_: get_symbol(&libs, b"sgemm_compute_blis_impl_\0")
                .map(|sym| *sym),
            SGEMM_COMPUTE_BLIS_IMPL_: get_symbol(&libs, b"SGEMM_COMPUTE_BLIS_IMPL_\0")
                .map(|sym| *sym),
            SGEMMT_BLIS_IMPL: get_symbol(&libs, b"SGEMMT_BLIS_IMPL\0").map(|sym| *sym),
            sgemmt_blis_impl_: get_symbol(&libs, b"sgemmt_blis_impl_\0").map(|sym| *sym),
            SGEMMT_BLIS_IMPL_: get_symbol(&libs, b"SGEMMT_BLIS_IMPL_\0").map(|sym| *sym),
            ZAXPBY_BLIS_IMPL: get_symbol(&libs, b"ZAXPBY_BLIS_IMPL\0").map(|sym| *sym),
            zaxpby_blis_impl_: get_symbol(&libs, b"zaxpby_blis_impl_\0").map(|sym| *sym),
            ZAXPBY_BLIS_IMPL_: get_symbol(&libs, b"ZAXPBY_BLIS_IMPL_\0").map(|sym| *sym),
            ZGEMM3M_BLIS_IMPL: get_symbol(&libs, b"ZGEMM3M_BLIS_IMPL\0").map(|sym| *sym),
            zgemm3m_blis_impl_: get_symbol(&libs, b"zgemm3m_blis_impl_\0").map(|sym| *sym),
            ZGEMM3M_BLIS_IMPL_: get_symbol(&libs, b"ZGEMM3M_BLIS_IMPL_\0").map(|sym| *sym),
            ZGEMM_BATCH_BLIS_IMPL: get_symbol(&libs, b"ZGEMM_BATCH_BLIS_IMPL\0").map(|sym| *sym),
            zgemm_batch_blis_impl_: get_symbol(&libs, b"zgemm_batch_blis_impl_\0").map(|sym| *sym),
            ZGEMM_BATCH_BLIS_IMPL_: get_symbol(&libs, b"ZGEMM_BATCH_BLIS_IMPL_\0").map(|sym| *sym),
            ZGEMMT_BLIS_IMPL: get_symbol(&libs, b"ZGEMMT_BLIS_IMPL\0").map(|sym| *sym),
            zgemmt_blis_impl_: get_symbol(&libs, b"zgemmt_blis_impl_\0").map(|sym| *sym),
            ZGEMMT_BLIS_IMPL_: get_symbol(&libs, b"ZGEMMT_BLIS_IMPL_\0").map(|sym| *sym),
            AOCL_BLIS_set_progress: get_symbol(&libs, b"AOCL_BLIS_set_progress\0").map(|sym| *sym),
            aocl_get_reorder_buf_size_f32f32f32of32: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_f32f32f32of32\0",
            )
            .map(|sym| *sym),
            aocl_get_reorder_buf_size_u8s8s32os32: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_u8s8s32os32\0",
            )
            .map(|sym| *sym),
            aocl_get_reorder_buf_size_bf16bf16f32of32: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_bf16bf16f32of32\0",
            )
            .map(|sym| *sym),
            aocl_get_reorder_buf_size_s8s8s32os32: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_s8s8s32os32\0",
            )
            .map(|sym| *sym),
            aocl_get_reorder_buf_size_u8s4s32os32: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_u8s4s32os32\0",
            )
            .map(|sym| *sym),
            aocl_get_reorder_buf_size_bf16s4f32of32: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_bf16s4f32of32\0",
            )
            .map(|sym| *sym),
            aocl_get_reorder_buf_size_s8s8s32os32_sym_quant: get_symbol(
                &libs,
                b"aocl_get_reorder_buf_size_s8s8s32os32_sym_quant\0",
            )
            .map(|sym| *sym),
            aocl_reorder_f32f32f32of32: get_symbol(&libs, b"aocl_reorder_f32f32f32of32\0")
                .map(|sym| *sym),
            aocl_reorder_u8s8s32os32: get_symbol(&libs, b"aocl_reorder_u8s8s32os32\0")
                .map(|sym| *sym),
            aocl_reorder_bf16bf16f32of32: get_symbol(&libs, b"aocl_reorder_bf16bf16f32of32\0")
                .map(|sym| *sym),
            aocl_reorder_bf16bf16f32of32_reference: get_symbol(
                &libs,
                b"aocl_reorder_bf16bf16f32of32_reference\0",
            )
            .map(|sym| *sym),
            aocl_reorder_s8s8s32os32: get_symbol(&libs, b"aocl_reorder_s8s8s32os32\0")
                .map(|sym| *sym),
            aocl_reorder_u8s4s32os32: get_symbol(&libs, b"aocl_reorder_u8s4s32os32\0")
                .map(|sym| *sym),
            aocl_reorder_bf16s4f32of32: get_symbol(&libs, b"aocl_reorder_bf16s4f32of32\0")
                .map(|sym| *sym),
            aocl_reorder_s8s8s32os32_sym_quant: get_symbol(
                &libs,
                b"aocl_reorder_s8s8s32os32_sym_quant\0",
            )
            .map(|sym| *sym),
            aocl_reorder_f32obf16: get_symbol(&libs, b"aocl_reorder_f32obf16\0").map(|sym| *sym),
            aocl_unreorder_bf16bf16f32of32: get_symbol(&libs, b"aocl_unreorder_bf16bf16f32of32\0")
                .map(|sym| *sym),
            aocl_unreorder_bf16bf16f32of32_reference: get_symbol(
                &libs,
                b"aocl_unreorder_bf16bf16f32of32_reference\0",
            )
            .map(|sym| *sym),
            aocl_unreorder_s8s8s32os32_reference: get_symbol(
                &libs,
                b"aocl_unreorder_s8s8s32os32_reference\0",
            )
            .map(|sym| *sym),
            aocl_gemm_u8s8s32os32: get_symbol(&libs, b"aocl_gemm_u8s8s32os32\0").map(|sym| *sym),
            aocl_gemm_u8s8s32os8: get_symbol(&libs, b"aocl_gemm_u8s8s32os8\0").map(|sym| *sym),
            aocl_gemm_u8s8s32obf16: get_symbol(&libs, b"aocl_gemm_u8s8s32obf16\0").map(|sym| *sym),
            aocl_gemm_u8s8s32of32: get_symbol(&libs, b"aocl_gemm_u8s8s32of32\0").map(|sym| *sym),
            aocl_gemm_u8s8s32ou8: get_symbol(&libs, b"aocl_gemm_u8s8s32ou8\0").map(|sym| *sym),
            aocl_gemm_s8s8s32os32: get_symbol(&libs, b"aocl_gemm_s8s8s32os32\0").map(|sym| *sym),
            aocl_gemm_s8s8s32os8: get_symbol(&libs, b"aocl_gemm_s8s8s32os8\0").map(|sym| *sym),
            aocl_gemm_s8s8s32obf16: get_symbol(&libs, b"aocl_gemm_s8s8s32obf16\0").map(|sym| *sym),
            aocl_gemm_s8s8s32of32: get_symbol(&libs, b"aocl_gemm_s8s8s32of32\0").map(|sym| *sym),
            aocl_gemm_s8s8s32ou8: get_symbol(&libs, b"aocl_gemm_s8s8s32ou8\0").map(|sym| *sym),
            aocl_gemm_s8s8s32of32_sym_quant: get_symbol(
                &libs,
                b"aocl_gemm_s8s8s32of32_sym_quant\0",
            )
            .map(|sym| *sym),
            aocl_gemm_s8s8s32obf16_sym_quant: get_symbol(
                &libs,
                b"aocl_gemm_s8s8s32obf16_sym_quant\0",
            )
            .map(|sym| *sym),
            aocl_gemm_bf16bf16f32obf16: get_symbol(&libs, b"aocl_gemm_bf16bf16f32obf16\0")
                .map(|sym| *sym),
            aocl_gemm_bf16bf16f32of32: get_symbol(&libs, b"aocl_gemm_bf16bf16f32of32\0")
                .map(|sym| *sym),
            aocl_gemm_bf16s4f32of32: get_symbol(&libs, b"aocl_gemm_bf16s4f32of32\0")
                .map(|sym| *sym),
            aocl_gemm_bf16s4f32obf16: get_symbol(&libs, b"aocl_gemm_bf16s4f32obf16\0")
                .map(|sym| *sym),
            aocl_gemm_f32f32f32of32: get_symbol(&libs, b"aocl_gemm_f32f32f32of32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_bf16bf16f32of32: get_symbol(
                &libs,
                b"aocl_batch_gemm_bf16bf16f32of32\0",
            )
            .map(|sym| *sym),
            aocl_batch_gemm_bf16bf16f32obf16: get_symbol(
                &libs,
                b"aocl_batch_gemm_bf16bf16f32obf16\0",
            )
            .map(|sym| *sym),
            aocl_batch_gemm_bf16s4f32of32: get_symbol(&libs, b"aocl_batch_gemm_bf16s4f32of32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_bf16s4f32obf16: get_symbol(&libs, b"aocl_batch_gemm_bf16s4f32obf16\0")
                .map(|sym| *sym),
            aocl_batch_gemm_f32f32f32of32: get_symbol(&libs, b"aocl_batch_gemm_f32f32f32of32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_u8s8s32os32: get_symbol(&libs, b"aocl_batch_gemm_u8s8s32os32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_u8s8s32os8: get_symbol(&libs, b"aocl_batch_gemm_u8s8s32os8\0")
                .map(|sym| *sym),
            aocl_batch_gemm_u8s8s32of32: get_symbol(&libs, b"aocl_batch_gemm_u8s8s32of32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_u8s8s32obf16: get_symbol(&libs, b"aocl_batch_gemm_u8s8s32obf16\0")
                .map(|sym| *sym),
            aocl_batch_gemm_u8s8s32ou8: get_symbol(&libs, b"aocl_batch_gemm_u8s8s32ou8\0")
                .map(|sym| *sym),
            aocl_batch_gemm_s8s8s32os32: get_symbol(&libs, b"aocl_batch_gemm_s8s8s32os32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_s8s8s32os8: get_symbol(&libs, b"aocl_batch_gemm_s8s8s32os8\0")
                .map(|sym| *sym),
            aocl_batch_gemm_s8s8s32of32: get_symbol(&libs, b"aocl_batch_gemm_s8s8s32of32\0")
                .map(|sym| *sym),
            aocl_batch_gemm_s8s8s32obf16: get_symbol(&libs, b"aocl_batch_gemm_s8s8s32obf16\0")
                .map(|sym| *sym),
            aocl_batch_gemm_s8s8s32ou8: get_symbol(&libs, b"aocl_batch_gemm_s8s8s32ou8\0")
                .map(|sym| *sym),
            aocl_gemm_gelu_tanh_f32: get_symbol(&libs, b"aocl_gemm_gelu_tanh_f32\0")
                .map(|sym| *sym),
            aocl_gemm_gelu_erf_f32: get_symbol(&libs, b"aocl_gemm_gelu_erf_f32\0").map(|sym| *sym),
            aocl_gemm_softmax_f32: get_symbol(&libs, b"aocl_gemm_softmax_f32\0").map(|sym| *sym),
            aocl_gemm_eltwise_ops_bf16of32: get_symbol(&libs, b"aocl_gemm_eltwise_ops_bf16of32\0")
                .map(|sym| *sym),
            aocl_gemm_eltwise_ops_bf16obf16: get_symbol(
                &libs,
                b"aocl_gemm_eltwise_ops_bf16obf16\0",
            )
            .map(|sym| *sym),
            aocl_gemm_eltwise_ops_f32of32: get_symbol(&libs, b"aocl_gemm_eltwise_ops_f32of32\0")
                .map(|sym| *sym),
            aocl_gemm_eltwise_ops_f32obf16: get_symbol(&libs, b"aocl_gemm_eltwise_ops_f32obf16\0")
                .map(|sym| *sym),
            aocl_gemm_eltwise_ops_f32os32: get_symbol(&libs, b"aocl_gemm_eltwise_ops_f32os32\0")
                .map(|sym| *sym),
            aocl_gemm_eltwise_ops_f32os8: get_symbol(&libs, b"aocl_gemm_eltwise_ops_f32os8\0")
                .map(|sym| *sym),
            aocl_gemm_eltwise_ops_f32ou8: get_symbol(&libs, b"aocl_gemm_eltwise_ops_f32ou8\0")
                .map(|sym| *sym),
            aocl_lpgemm_init_global_cntx: get_symbol(&libs, b"aocl_lpgemm_init_global_cntx\0")
                .map(|sym| *sym),
            lpgemm_get_global_cntx_obj: get_symbol(&libs, b"lpgemm_get_global_cntx_obj\0")
                .map(|sym| *sym),
            lpgemm_util_get_global_cntx_obj: get_symbol(
                &libs,
                b"lpgemm_util_get_global_cntx_obj\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_get_global_cntx_obj: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_get_global_cntx_obj\0",
            )
            .map(|sym| *sym),
            lpgemm_get_block_size_MC_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_block_size_MC_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_block_size_NC_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_block_size_NC_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_block_size_KC_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_block_size_KC_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_block_size_NR_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_block_size_NR_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_block_size_MR_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_block_size_MR_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_sup_thres_MT_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_sup_thres_MT_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_sup_thres_NT_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_sup_thres_NT_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_sup_thres_KT_global_cntx: get_symbol(
                &libs,
                b"lpgemm_get_sup_thres_KT_global_cntx\0",
            )
            .map(|sym| *sym),
            lpgemm_get_enabled_arch: get_symbol(&libs, b"lpgemm_get_enabled_arch\0")
                .map(|sym| *sym),
            lpgemm_get_packa_strides: get_symbol(&libs, b"lpgemm_get_packa_strides\0")
                .map(|sym| *sym),
            lpgemm_get_packb_strides: get_symbol(&libs, b"lpgemm_get_packb_strides\0")
                .map(|sym| *sym),
            lpgemm_set_jit_kernel: get_symbol(&libs, b"lpgemm_set_jit_kernel\0").map(|sym| *sym),
            lpgemm_get_jit_kernel: get_symbol(&libs, b"lpgemm_get_jit_kernel\0").map(|sym| *sym),
            get_jit_kernels_generated: get_symbol(&libs, b"get_jit_kernels_generated\0")
                .map(|sym| *sym),
            lpgemm_mod_block_size_s16: get_symbol(&libs, b"lpgemm_mod_block_size_s16\0")
                .map(|sym| *sym),
            lpgemm_translate_to_post_ops_list: get_symbol(
                &libs,
                b"lpgemm_translate_to_post_ops_list\0",
            )
            .map(|sym| *sym),
            lpgemm_translate_to_pre_ops_list: get_symbol(
                &libs,
                b"lpgemm_translate_to_pre_ops_list\0",
            )
            .map(|sym| *sym),
            lpgemm_translate_to_group_postops_list: get_symbol(
                &libs,
                b"lpgemm_translate_to_group_postops_list\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_6x64: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_6x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_6x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_6x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_6x16m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_6x16m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_6x64m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_6x64m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x64: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_6x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_6x64m: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_6x64m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x64m_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_6x64m_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_5x64: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_5x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_4x64: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_4x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_3x64: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_3x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_2x64: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_2x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_1x64: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_1x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_5x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_5x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_4x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_4x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_3x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_3x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_2x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_2x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_1x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_1x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_5x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_5x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_4x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_4x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_3x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_3x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_2x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_2x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_1x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_1x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_5x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_5x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_4x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_4x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_3x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_3x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_2x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_2x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_1x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_1x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_5x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_5x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_4x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_4x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_3x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_3x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_2x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_2x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_1x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_1x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_5x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_5x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_4x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_4x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_3x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_3x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_2x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_2x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_1x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_1x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_5x8: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_5x8\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_4x8: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_4x8\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_3x8: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_3x8\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_2x8: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_2x8\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_1x8: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_1x8\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_5x4: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_5x4\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_4x4: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_4x4\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_3x4: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_3x4\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_2x4: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_2x4\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_1x4: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_1x4\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_5x2: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_5x2\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_4x2: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_4x2\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_3x2: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_3x2\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_2x2: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_2x2\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_1x2: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_1x2\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_5x1: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_5x1\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_4x1: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_4x1\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_3x1: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_3x1\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_2x1: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_2x1\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_1x1: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_1x1\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x64: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_5x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x64: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_4x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x64: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_3x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x64: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_2x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x64: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_1x64\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_5x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_5x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_4x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_4x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_3x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_3x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_2x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_2x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_1x64: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_1x64\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x64_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_5x64_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x64_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_4x64_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x64_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_3x64_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x64_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_2x64_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x64_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_1x64_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_6x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_6x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_12x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_12x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_6x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_6x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_9x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_9x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_6x48: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_6x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_6x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_6x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_6x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_6x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_6x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_6x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_6x48m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_6x48m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_avx512_6x32m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_avx512_6x32m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_6x8m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_6x8m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_6x4m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_6x4m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_6x2m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_6x2m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_f32f32f32of32_6x1m: get_symbol(
                &libs,
                b"lpgemm_rowvar_f32f32f32of32_6x1m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x16: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_6x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x32: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_6x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x48: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_6x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_6x16m: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_6x16m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_6x32m: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_6x32m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_6x48m: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_6x48m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x48_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_6x48_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x32_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_6x32_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6x16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_6x16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6xlt16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_6xlt16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_6xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_6xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_12xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_12xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_6xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_6xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_6xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_6xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_6xlt16m: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_6xlt16m\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_5x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_5x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_4x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_4x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_3x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_3x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_2x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_2x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_1x16: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_1x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_5x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_5x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_4x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_4x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_3x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_3x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_2x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_2x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_1x32: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_1x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_5x48: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_5x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_4x48: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_4x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_3x48: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_3x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_2x48: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_2x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_1x48: get_symbol(&libs, b"lpgemm_rowvar_u8s8s32o32_1x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_5x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_5x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_4x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_4x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_3x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_3x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_2x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_2x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_1x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_1x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_5x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_5x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_4x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_4x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_3x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_3x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_2x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_2x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_1x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_1x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_5x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_5x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_4x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_4x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_3x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_3x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_2x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_2x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_1x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_1x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x16: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_5x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x16: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_4x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x16: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_3x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x16: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_2x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x16: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_1x16\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x32: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_5x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x32: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_4x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x32: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_3x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x32: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_2x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x32: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_1x32\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x48: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_5x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x48: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_4x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x48: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_3x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x48: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_2x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x48: get_symbol(&libs, b"lpgemm_rowvar_s8s8s32os32_1x48\0")
                .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_5x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_5x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_4x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_4x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_3x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_3x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_2x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_2x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_1x16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_1x16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_5x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_5x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_4x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_4x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_3x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_3x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_2x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_2x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_1x32: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_1x32\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_5x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_5x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_4x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_4x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_3x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_3x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_2x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_2x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_1x48: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_1x48\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x48_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_5x48_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x48_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_4x48_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x48_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_3x48_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x48_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_2x48_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x48_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_1x48_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x32_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_5x32_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x32_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_4x32_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x32_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_3x32_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x32_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_2x32_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x32_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_1x32_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5x16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_5x16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4x16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_4x16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3x16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_3x16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2x16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_2x16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1x16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_1x16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_5xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_5xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_4xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_4xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_3xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_3xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_2xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_2xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_u8s8s32o32_1xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_u8s8s32o32_1xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_5xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_5xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_4xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_4xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_3xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_3xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_2xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_2xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16bf16f32of32_1xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16bf16f32of32_1xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_5xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_4xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_3xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_2xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_1xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_5xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_5xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_4xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_4xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_3xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_3xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_2xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_2xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_bf16s4f32of32_1xlt16: get_symbol(
                &libs,
                b"lpgemm_rowvar_bf16s4f32of32_1xlt16\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_5xlt16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_5xlt16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_4xlt16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_4xlt16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_3xlt16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_3xlt16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_2xlt16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_2xlt16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemm_rowvar_s8s8s32os32_1xlt16_sym_quant: get_symbol(
                &libs,
                b"lpgemm_rowvar_s8s8s32os32_1xlt16_sym_quant\0",
            )
            .map(|sym| *sym),
            lpgemv_m_one_f32f32f32of32: get_symbol(&libs, b"lpgemv_m_one_f32f32f32of32\0")
                .map(|sym| *sym),
            lpgemv_m_one_bf16bf16f32of32: get_symbol(&libs, b"lpgemv_m_one_bf16bf16f32of32\0")
                .map(|sym| *sym),
            lpgemv_m_one_u8s8s32os32: get_symbol(&libs, b"lpgemv_m_one_u8s8s32os32\0")
                .map(|sym| *sym),
            lpgemv_m_one_s8s8s32os32: get_symbol(&libs, b"lpgemv_m_one_s8s8s32os32\0")
                .map(|sym| *sym),
            lpgemv_n_one_f32f32f32of32: get_symbol(&libs, b"lpgemv_n_one_f32f32f32of32\0")
                .map(|sym| *sym),
            lpgemv_n_one_bf16bf16f32of32: get_symbol(&libs, b"lpgemv_n_one_bf16bf16f32of32\0")
                .map(|sym| *sym),
            lpgemv_n_one_u8s8s32os32: get_symbol(&libs, b"lpgemv_n_one_u8s8s32os32\0")
                .map(|sym| *sym),
            lpgemv_n_one_s8s8s32os32: get_symbol(&libs, b"lpgemv_n_one_s8s8s32os32\0")
                .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_bf16of32_6x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_bf16of32_6x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_f32of32_6x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_f32of32_6x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_bf16of32_5x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_bf16of32_5x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_bf16of32_4x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_bf16of32_4x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_bf16of32_3x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_bf16of32_3x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_bf16of32_2x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_bf16of32_2x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_bf16of32_1x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_bf16of32_1x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_f32of32_5x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_f32of32_5x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_f32of32_4x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_f32of32_4x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_f32of32_3x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_f32of32_3x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_f32of32_2x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_f32of32_2x64\0",
            )
            .map(|sym| *sym),
            lpgemm_eltwise_ops_kernel_f32of32_1x64: get_symbol(
                &libs,
                b"lpgemm_eltwise_ops_kernel_f32of32_1x64\0",
            )
            .map(|sym| *sym),
            lpgemm_util_f32_gelu_tanh_avx512_kernel: get_symbol(
                &libs,
                b"lpgemm_util_f32_gelu_tanh_avx512_kernel\0",
            )
            .map(|sym| *sym),
            lpgemm_util_f32_gelu_erf_avx512_kernel: get_symbol(
                &libs,
                b"lpgemm_util_f32_gelu_erf_avx512_kernel\0",
            )
            .map(|sym| *sym),
            lpgemm_util_f32_softmax_avx512_kernel: get_symbol(
                &libs,
                b"lpgemm_util_f32_softmax_avx512_kernel\0",
            )
            .map(|sym| *sym),
            lpgemm_util_f32_gelu_tanh_avx2_kernel: get_symbol(
                &libs,
                b"lpgemm_util_f32_gelu_tanh_avx2_kernel\0",
            )
            .map(|sym| *sym),
            lpgemm_util_f32_gelu_erf_avx2_kernel: get_symbol(
                &libs,
                b"lpgemm_util_f32_gelu_erf_avx2_kernel\0",
            )
            .map(|sym| *sym),
            lpgemm_util_f32_softmax_avx2_kernel: get_symbol(
                &libs,
                b"lpgemm_util_f32_softmax_avx2_kernel\0",
            )
            .map(|sym| *sym),
            packb_mxp_nr64_f32obf16: get_symbol(&libs, b"packb_mxp_nr64_f32obf16\0")
                .map(|sym| *sym),
            packb_nr64_bf16bf16f32of32: get_symbol(&libs, b"packb_nr64_bf16bf16f32of32\0")
                .map(|sym| *sym),
            packb_nr64_bf16s4f32of32: get_symbol(&libs, b"packb_nr64_bf16s4f32of32\0")
                .map(|sym| *sym),
            packsclb_nr64_bf16s4f32of32: get_symbol(&libs, b"packsclb_nr64_bf16s4f32of32\0")
                .map(|sym| *sym),
            packa_mr16_bf16bf16f32of32: get_symbol(&libs, b"packa_mr16_bf16bf16f32of32\0")
                .map(|sym| *sym),
            unpackb_nr64_bf16bf16f32of32: get_symbol(&libs, b"unpackb_nr64_bf16bf16f32of32\0")
                .map(|sym| *sym),
            unpackb_nr64_bf16_f32: get_symbol(&libs, b"unpackb_nr64_bf16_f32\0").map(|sym| *sym),
            cvt_bf16_f32: get_symbol(&libs, b"cvt_bf16_f32\0").map(|sym| *sym),
            packa_u8s8s32os32: get_symbol(&libs, b"packa_u8s8s32os32\0").map(|sym| *sym),
            packb_nr64_u8s8s32o32: get_symbol(&libs, b"packb_nr64_u8s8s32o32\0").map(|sym| *sym),
            packb_nr64_u8s4s32o32: get_symbol(&libs, b"packb_nr64_u8s4s32o32\0").map(|sym| *sym),
            packa_k64_s8s8s32os32: get_symbol(&libs, b"packa_k64_s8s8s32os32\0").map(|sym| *sym),
            packb_nr64_s8s8s32os32: get_symbol(&libs, b"packb_nr64_s8s8s32os32\0").map(|sym| *sym),
            packa_mr16_f32f32f32of32_col_major: get_symbol(
                &libs,
                b"packa_mr16_f32f32f32of32_col_major\0",
            )
            .map(|sym| *sym),
            packa_mr6_f32f32f32of32_avx512: get_symbol(&libs, b"packa_mr6_f32f32f32of32_avx512\0")
                .map(|sym| *sym),
            packa_mr6_f32f32f32of32_avx2: get_symbol(&libs, b"packa_mr6_f32f32f32of32_avx2\0")
                .map(|sym| *sym),
            packb_nr64_f32f32f32of32: get_symbol(&libs, b"packb_nr64_f32f32f32of32\0")
                .map(|sym| *sym),
            packb_nr16_f32f32f32of32: get_symbol(&libs, b"packb_nr16_f32f32f32of32\0")
                .map(|sym| *sym),
            bla_r_sign: get_symbol(&libs, b"bla_r_sign\0").map(|sym| *sym),
            bla_d_sign: get_symbol(&libs, b"bla_d_sign\0").map(|sym| *sym),
            bla_r_cnjg: get_symbol(&libs, b"bla_r_cnjg\0").map(|sym| *sym),
            bla_d_cnjg: get_symbol(&libs, b"bla_d_cnjg\0").map(|sym| *sym),
            bla_r_imag: get_symbol(&libs, b"bla_r_imag\0").map(|sym| *sym),
            bla_d_imag: get_symbol(&libs, b"bla_d_imag\0").map(|sym| *sym),
            bla_c_div: get_symbol(&libs, b"bla_c_div\0").map(|sym| *sym),
            bla_z_div: get_symbol(&libs, b"bla_z_div\0").map(|sym| *sym),
            bla_f__cabs: get_symbol(&libs, b"bla_f__cabs\0").map(|sym| *sym),
            bla_r_abs: get_symbol(&libs, b"bla_r_abs\0").map(|sym| *sym),
            bla_d_abs: get_symbol(&libs, b"bla_d_abs\0").map(|sym| *sym),
            bla_c_abs: get_symbol(&libs, b"bla_c_abs\0").map(|sym| *sym),
            bla_z_abs: get_symbol(&libs, b"bla_z_abs\0").map(|sym| *sym),
            lsame_blis_impl: get_symbol(&libs, b"lsame_blis_impl\0").map(|sym| *sym),
            lsame_: get_symbol(&libs, b"lsame_\0").map(|sym| *sym),
            xerbla_blis_impl: get_symbol(&libs, b"xerbla_blis_impl\0").map(|sym| *sym),
            xerbla_: get_symbol(&libs, b"xerbla_\0").map(|sym| *sym),
            xerbla_array_blis_impl: get_symbol(&libs, b"xerbla_array_blis_impl\0").map(|sym| *sym),
            xerbla_array_: get_symbol(&libs, b"xerbla_array_\0").map(|sym| *sym),
            isamax_: get_symbol(&libs, b"isamax_\0").map(|sym| *sym),
            isamax_blis_impl: get_symbol(&libs, b"isamax_blis_impl\0").map(|sym| *sym),
            idamax_: get_symbol(&libs, b"idamax_\0").map(|sym| *sym),
            idamax_blis_impl: get_symbol(&libs, b"idamax_blis_impl\0").map(|sym| *sym),
            icamax_: get_symbol(&libs, b"icamax_\0").map(|sym| *sym),
            icamax_blis_impl: get_symbol(&libs, b"icamax_blis_impl\0").map(|sym| *sym),
            izamax_: get_symbol(&libs, b"izamax_\0").map(|sym| *sym),
            izamax_blis_impl: get_symbol(&libs, b"izamax_blis_impl\0").map(|sym| *sym),
            sasum_: get_symbol(&libs, b"sasum_\0").map(|sym| *sym),
            sasum_blis_impl: get_symbol(&libs, b"sasum_blis_impl\0").map(|sym| *sym),
            dasum_: get_symbol(&libs, b"dasum_\0").map(|sym| *sym),
            dasum_blis_impl: get_symbol(&libs, b"dasum_blis_impl\0").map(|sym| *sym),
            scasum_: get_symbol(&libs, b"scasum_\0").map(|sym| *sym),
            scasum_blis_impl: get_symbol(&libs, b"scasum_blis_impl\0").map(|sym| *sym),
            dzasum_: get_symbol(&libs, b"dzasum_\0").map(|sym| *sym),
            dzasum_blis_impl: get_symbol(&libs, b"dzasum_blis_impl\0").map(|sym| *sym),
            saxpy_: get_symbol(&libs, b"saxpy_\0").map(|sym| *sym),
            saxpy_blis_impl: get_symbol(&libs, b"saxpy_blis_impl\0").map(|sym| *sym),
            daxpy_: get_symbol(&libs, b"daxpy_\0").map(|sym| *sym),
            daxpy_blis_impl: get_symbol(&libs, b"daxpy_blis_impl\0").map(|sym| *sym),
            caxpy_: get_symbol(&libs, b"caxpy_\0").map(|sym| *sym),
            caxpy_blis_impl: get_symbol(&libs, b"caxpy_blis_impl\0").map(|sym| *sym),
            zaxpy_: get_symbol(&libs, b"zaxpy_\0").map(|sym| *sym),
            zaxpy_blis_impl: get_symbol(&libs, b"zaxpy_blis_impl\0").map(|sym| *sym),
            saxpby_: get_symbol(&libs, b"saxpby_\0").map(|sym| *sym),
            saxpby_blis_impl: get_symbol(&libs, b"saxpby_blis_impl\0").map(|sym| *sym),
            daxpby_: get_symbol(&libs, b"daxpby_\0").map(|sym| *sym),
            daxpby_blis_impl: get_symbol(&libs, b"daxpby_blis_impl\0").map(|sym| *sym),
            caxpby_: get_symbol(&libs, b"caxpby_\0").map(|sym| *sym),
            caxpby_blis_impl: get_symbol(&libs, b"caxpby_blis_impl\0").map(|sym| *sym),
            zaxpby_: get_symbol(&libs, b"zaxpby_\0").map(|sym| *sym),
            zaxpby_blis_impl: get_symbol(&libs, b"zaxpby_blis_impl\0").map(|sym| *sym),
            scopy_: get_symbol(&libs, b"scopy_\0").map(|sym| *sym),
            scopy_blis_impl: get_symbol(&libs, b"scopy_blis_impl\0").map(|sym| *sym),
            dcopy_: get_symbol(&libs, b"dcopy_\0").map(|sym| *sym),
            dcopy_blis_impl: get_symbol(&libs, b"dcopy_blis_impl\0").map(|sym| *sym),
            ccopy_: get_symbol(&libs, b"ccopy_\0").map(|sym| *sym),
            ccopy_blis_impl: get_symbol(&libs, b"ccopy_blis_impl\0").map(|sym| *sym),
            zcopy_: get_symbol(&libs, b"zcopy_\0").map(|sym| *sym),
            zcopy_blis_impl: get_symbol(&libs, b"zcopy_blis_impl\0").map(|sym| *sym),
            sdot_: get_symbol(&libs, b"sdot_\0").map(|sym| *sym),
            sdot_blis_impl: get_symbol(&libs, b"sdot_blis_impl\0").map(|sym| *sym),
            ddot_: get_symbol(&libs, b"ddot_\0").map(|sym| *sym),
            ddot_blis_impl: get_symbol(&libs, b"ddot_blis_impl\0").map(|sym| *sym),
            cdotc_: get_symbol(&libs, b"cdotc_\0").map(|sym| *sym),
            cdotc_blis_impl: get_symbol(&libs, b"cdotc_blis_impl\0").map(|sym| *sym),
            cdotu_: get_symbol(&libs, b"cdotu_\0").map(|sym| *sym),
            cdotu_blis_impl: get_symbol(&libs, b"cdotu_blis_impl\0").map(|sym| *sym),
            zdotc_: get_symbol(&libs, b"zdotc_\0").map(|sym| *sym),
            zdotc_blis_impl: get_symbol(&libs, b"zdotc_blis_impl\0").map(|sym| *sym),
            zdotu_: get_symbol(&libs, b"zdotu_\0").map(|sym| *sym),
            zdotu_blis_impl: get_symbol(&libs, b"zdotu_blis_impl\0").map(|sym| *sym),
            sdsdot_: get_symbol(&libs, b"sdsdot_\0").map(|sym| *sym),
            sdsdot_blis_impl: get_symbol(&libs, b"sdsdot_blis_impl\0").map(|sym| *sym),
            dsdot_: get_symbol(&libs, b"dsdot_\0").map(|sym| *sym),
            dsdot_blis_impl: get_symbol(&libs, b"dsdot_blis_impl\0").map(|sym| *sym),
            snrm2_: get_symbol(&libs, b"snrm2_\0").map(|sym| *sym),
            snrm2_blis_impl: get_symbol(&libs, b"snrm2_blis_impl\0").map(|sym| *sym),
            dnrm2_: get_symbol(&libs, b"dnrm2_\0").map(|sym| *sym),
            dnrm2_blis_impl: get_symbol(&libs, b"dnrm2_blis_impl\0").map(|sym| *sym),
            scnrm2_: get_symbol(&libs, b"scnrm2_\0").map(|sym| *sym),
            scnrm2_blis_impl: get_symbol(&libs, b"scnrm2_blis_impl\0").map(|sym| *sym),
            dznrm2_: get_symbol(&libs, b"dznrm2_\0").map(|sym| *sym),
            dznrm2_blis_impl: get_symbol(&libs, b"dznrm2_blis_impl\0").map(|sym| *sym),
            srot_: get_symbol(&libs, b"srot_\0").map(|sym| *sym),
            drot_: get_symbol(&libs, b"drot_\0").map(|sym| *sym),
            csrot_: get_symbol(&libs, b"csrot_\0").map(|sym| *sym),
            zdrot_: get_symbol(&libs, b"zdrot_\0").map(|sym| *sym),
            srot_blis_impl: get_symbol(&libs, b"srot_blis_impl\0").map(|sym| *sym),
            drot_blis_impl: get_symbol(&libs, b"drot_blis_impl\0").map(|sym| *sym),
            csrot_blis_impl: get_symbol(&libs, b"csrot_blis_impl\0").map(|sym| *sym),
            zdrot_blis_impl: get_symbol(&libs, b"zdrot_blis_impl\0").map(|sym| *sym),
            srotg_: get_symbol(&libs, b"srotg_\0").map(|sym| *sym),
            drotg_: get_symbol(&libs, b"drotg_\0").map(|sym| *sym),
            crotg_: get_symbol(&libs, b"crotg_\0").map(|sym| *sym),
            zrotg_: get_symbol(&libs, b"zrotg_\0").map(|sym| *sym),
            srotg_blis_impl: get_symbol(&libs, b"srotg_blis_impl\0").map(|sym| *sym),
            drotg_blis_impl: get_symbol(&libs, b"drotg_blis_impl\0").map(|sym| *sym),
            crotg_blis_impl: get_symbol(&libs, b"crotg_blis_impl\0").map(|sym| *sym),
            zrotg_blis_impl: get_symbol(&libs, b"zrotg_blis_impl\0").map(|sym| *sym),
            srotm_: get_symbol(&libs, b"srotm_\0").map(|sym| *sym),
            drotm_: get_symbol(&libs, b"drotm_\0").map(|sym| *sym),
            srotm_blis_impl: get_symbol(&libs, b"srotm_blis_impl\0").map(|sym| *sym),
            drotm_blis_impl: get_symbol(&libs, b"drotm_blis_impl\0").map(|sym| *sym),
            srotmg_: get_symbol(&libs, b"srotmg_\0").map(|sym| *sym),
            drotmg_: get_symbol(&libs, b"drotmg_\0").map(|sym| *sym),
            srotmg_blis_impl: get_symbol(&libs, b"srotmg_blis_impl\0").map(|sym| *sym),
            drotmg_blis_impl: get_symbol(&libs, b"drotmg_blis_impl\0").map(|sym| *sym),
            sscal_: get_symbol(&libs, b"sscal_\0").map(|sym| *sym),
            sscal_blis_impl: get_symbol(&libs, b"sscal_blis_impl\0").map(|sym| *sym),
            dscal_: get_symbol(&libs, b"dscal_\0").map(|sym| *sym),
            dscal_blis_impl: get_symbol(&libs, b"dscal_blis_impl\0").map(|sym| *sym),
            cscal_: get_symbol(&libs, b"cscal_\0").map(|sym| *sym),
            cscal_blis_impl: get_symbol(&libs, b"cscal_blis_impl\0").map(|sym| *sym),
            zscal_: get_symbol(&libs, b"zscal_\0").map(|sym| *sym),
            zscal_blis_impl: get_symbol(&libs, b"zscal_blis_impl\0").map(|sym| *sym),
            csscal_: get_symbol(&libs, b"csscal_\0").map(|sym| *sym),
            csscal_blis_impl: get_symbol(&libs, b"csscal_blis_impl\0").map(|sym| *sym),
            zdscal_: get_symbol(&libs, b"zdscal_\0").map(|sym| *sym),
            zdscal_blis_impl: get_symbol(&libs, b"zdscal_blis_impl\0").map(|sym| *sym),
            sswap_: get_symbol(&libs, b"sswap_\0").map(|sym| *sym),
            sswap_blis_impl: get_symbol(&libs, b"sswap_blis_impl\0").map(|sym| *sym),
            dswap_: get_symbol(&libs, b"dswap_\0").map(|sym| *sym),
            dswap_blis_impl: get_symbol(&libs, b"dswap_blis_impl\0").map(|sym| *sym),
            cswap_: get_symbol(&libs, b"cswap_\0").map(|sym| *sym),
            cswap_blis_impl: get_symbol(&libs, b"cswap_blis_impl\0").map(|sym| *sym),
            zswap_: get_symbol(&libs, b"zswap_\0").map(|sym| *sym),
            zswap_blis_impl: get_symbol(&libs, b"zswap_blis_impl\0").map(|sym| *sym),
            isamaxsub_: get_symbol(&libs, b"isamaxsub_\0").map(|sym| *sym),
            isamaxsub_blis_impl: get_symbol(&libs, b"isamaxsub_blis_impl\0").map(|sym| *sym),
            idamaxsub_: get_symbol(&libs, b"idamaxsub_\0").map(|sym| *sym),
            idamaxsub_blis_impl: get_symbol(&libs, b"idamaxsub_blis_impl\0").map(|sym| *sym),
            icamaxsub_: get_symbol(&libs, b"icamaxsub_\0").map(|sym| *sym),
            icamaxsub_blis_impl: get_symbol(&libs, b"icamaxsub_blis_impl\0").map(|sym| *sym),
            izamaxsub_: get_symbol(&libs, b"izamaxsub_\0").map(|sym| *sym),
            izamaxsub_blis_impl: get_symbol(&libs, b"izamaxsub_blis_impl\0").map(|sym| *sym),
            sasumsub_: get_symbol(&libs, b"sasumsub_\0").map(|sym| *sym),
            sasumsub_blis_impl: get_symbol(&libs, b"sasumsub_blis_impl\0").map(|sym| *sym),
            dasumsub_: get_symbol(&libs, b"dasumsub_\0").map(|sym| *sym),
            dasumsub_blis_impl: get_symbol(&libs, b"dasumsub_blis_impl\0").map(|sym| *sym),
            scasumsub_: get_symbol(&libs, b"scasumsub_\0").map(|sym| *sym),
            scasumsub_blis_impl: get_symbol(&libs, b"scasumsub_blis_impl\0").map(|sym| *sym),
            dzasumsub_: get_symbol(&libs, b"dzasumsub_\0").map(|sym| *sym),
            dzasumsub_blis_impl: get_symbol(&libs, b"dzasumsub_blis_impl\0").map(|sym| *sym),
            sdotsub_: get_symbol(&libs, b"sdotsub_\0").map(|sym| *sym),
            sdotsub_blis_impl: get_symbol(&libs, b"sdotsub_blis_impl\0").map(|sym| *sym),
            ddotsub_: get_symbol(&libs, b"ddotsub_\0").map(|sym| *sym),
            ddotsub_blis_impl: get_symbol(&libs, b"ddotsub_blis_impl\0").map(|sym| *sym),
            cdotcsub_: get_symbol(&libs, b"cdotcsub_\0").map(|sym| *sym),
            cdotcsub_blis_impl: get_symbol(&libs, b"cdotcsub_blis_impl\0").map(|sym| *sym),
            cdotusub_: get_symbol(&libs, b"cdotusub_\0").map(|sym| *sym),
            cdotusub_blis_impl: get_symbol(&libs, b"cdotusub_blis_impl\0").map(|sym| *sym),
            zdotcsub_: get_symbol(&libs, b"zdotcsub_\0").map(|sym| *sym),
            zdotcsub_blis_impl: get_symbol(&libs, b"zdotcsub_blis_impl\0").map(|sym| *sym),
            zdotusub_: get_symbol(&libs, b"zdotusub_\0").map(|sym| *sym),
            zdotusub_blis_impl: get_symbol(&libs, b"zdotusub_blis_impl\0").map(|sym| *sym),
            sdsdotsub_: get_symbol(&libs, b"sdsdotsub_\0").map(|sym| *sym),
            sdsdotsub_blis_impl: get_symbol(&libs, b"sdsdotsub_blis_impl\0").map(|sym| *sym),
            dsdotsub_: get_symbol(&libs, b"dsdotsub_\0").map(|sym| *sym),
            dsdotsub_blis_impl: get_symbol(&libs, b"dsdotsub_blis_impl\0").map(|sym| *sym),
            snrm2sub_: get_symbol(&libs, b"snrm2sub_\0").map(|sym| *sym),
            snrm2sub_blis_impl: get_symbol(&libs, b"snrm2sub_blis_impl\0").map(|sym| *sym),
            dnrm2sub_: get_symbol(&libs, b"dnrm2sub_\0").map(|sym| *sym),
            dnrm2sub_blis_impl: get_symbol(&libs, b"dnrm2sub_blis_impl\0").map(|sym| *sym),
            scnrm2sub_: get_symbol(&libs, b"scnrm2sub_\0").map(|sym| *sym),
            scnrm2sub_blis_impl: get_symbol(&libs, b"scnrm2sub_blis_impl\0").map(|sym| *sym),
            dznrm2sub_: get_symbol(&libs, b"dznrm2sub_\0").map(|sym| *sym),
            dznrm2sub_blis_impl: get_symbol(&libs, b"dznrm2sub_blis_impl\0").map(|sym| *sym),
            sgemv_: get_symbol(&libs, b"sgemv_\0").map(|sym| *sym),
            sgemv_blis_impl: get_symbol(&libs, b"sgemv_blis_impl\0").map(|sym| *sym),
            dgemv_: get_symbol(&libs, b"dgemv_\0").map(|sym| *sym),
            dgemv_blis_impl: get_symbol(&libs, b"dgemv_blis_impl\0").map(|sym| *sym),
            cgemv_: get_symbol(&libs, b"cgemv_\0").map(|sym| *sym),
            cgemv_blis_impl: get_symbol(&libs, b"cgemv_blis_impl\0").map(|sym| *sym),
            zgemv_: get_symbol(&libs, b"zgemv_\0").map(|sym| *sym),
            zgemv_blis_impl: get_symbol(&libs, b"zgemv_blis_impl\0").map(|sym| *sym),
            sger_: get_symbol(&libs, b"sger_\0").map(|sym| *sym),
            sger_blis_impl: get_symbol(&libs, b"sger_blis_impl\0").map(|sym| *sym),
            dger_: get_symbol(&libs, b"dger_\0").map(|sym| *sym),
            dger_blis_impl: get_symbol(&libs, b"dger_blis_impl\0").map(|sym| *sym),
            cgerc_: get_symbol(&libs, b"cgerc_\0").map(|sym| *sym),
            cgerc_blis_impl: get_symbol(&libs, b"cgerc_blis_impl\0").map(|sym| *sym),
            cgeru_: get_symbol(&libs, b"cgeru_\0").map(|sym| *sym),
            cgeru_blis_impl: get_symbol(&libs, b"cgeru_blis_impl\0").map(|sym| *sym),
            zgerc_: get_symbol(&libs, b"zgerc_\0").map(|sym| *sym),
            zgerc_blis_impl: get_symbol(&libs, b"zgerc_blis_impl\0").map(|sym| *sym),
            zgeru_: get_symbol(&libs, b"zgeru_\0").map(|sym| *sym),
            zgeru_blis_impl: get_symbol(&libs, b"zgeru_blis_impl\0").map(|sym| *sym),
            chemv_: get_symbol(&libs, b"chemv_\0").map(|sym| *sym),
            chemv_blis_impl: get_symbol(&libs, b"chemv_blis_impl\0").map(|sym| *sym),
            zhemv_: get_symbol(&libs, b"zhemv_\0").map(|sym| *sym),
            zhemv_blis_impl: get_symbol(&libs, b"zhemv_blis_impl\0").map(|sym| *sym),
            cher_: get_symbol(&libs, b"cher_\0").map(|sym| *sym),
            cher_blis_impl: get_symbol(&libs, b"cher_blis_impl\0").map(|sym| *sym),
            zher_: get_symbol(&libs, b"zher_\0").map(|sym| *sym),
            zher_blis_impl: get_symbol(&libs, b"zher_blis_impl\0").map(|sym| *sym),
            cher2_: get_symbol(&libs, b"cher2_\0").map(|sym| *sym),
            cher2_blis_impl: get_symbol(&libs, b"cher2_blis_impl\0").map(|sym| *sym),
            zher2_: get_symbol(&libs, b"zher2_\0").map(|sym| *sym),
            zher2_blis_impl: get_symbol(&libs, b"zher2_blis_impl\0").map(|sym| *sym),
            ssymv_: get_symbol(&libs, b"ssymv_\0").map(|sym| *sym),
            ssymv_blis_impl: get_symbol(&libs, b"ssymv_blis_impl\0").map(|sym| *sym),
            dsymv_: get_symbol(&libs, b"dsymv_\0").map(|sym| *sym),
            dsymv_blis_impl: get_symbol(&libs, b"dsymv_blis_impl\0").map(|sym| *sym),
            ssyr_: get_symbol(&libs, b"ssyr_\0").map(|sym| *sym),
            ssyr_blis_impl: get_symbol(&libs, b"ssyr_blis_impl\0").map(|sym| *sym),
            dsyr_: get_symbol(&libs, b"dsyr_\0").map(|sym| *sym),
            dsyr_blis_impl: get_symbol(&libs, b"dsyr_blis_impl\0").map(|sym| *sym),
            ssyr2_: get_symbol(&libs, b"ssyr2_\0").map(|sym| *sym),
            ssyr2_blis_impl: get_symbol(&libs, b"ssyr2_blis_impl\0").map(|sym| *sym),
            dsyr2_: get_symbol(&libs, b"dsyr2_\0").map(|sym| *sym),
            dsyr2_blis_impl: get_symbol(&libs, b"dsyr2_blis_impl\0").map(|sym| *sym),
            strmv_: get_symbol(&libs, b"strmv_\0").map(|sym| *sym),
            strmv_blis_impl: get_symbol(&libs, b"strmv_blis_impl\0").map(|sym| *sym),
            dtrmv_: get_symbol(&libs, b"dtrmv_\0").map(|sym| *sym),
            dtrmv_blis_impl: get_symbol(&libs, b"dtrmv_blis_impl\0").map(|sym| *sym),
            ctrmv_: get_symbol(&libs, b"ctrmv_\0").map(|sym| *sym),
            ctrmv_blis_impl: get_symbol(&libs, b"ctrmv_blis_impl\0").map(|sym| *sym),
            ztrmv_: get_symbol(&libs, b"ztrmv_\0").map(|sym| *sym),
            ztrmv_blis_impl: get_symbol(&libs, b"ztrmv_blis_impl\0").map(|sym| *sym),
            strsv_: get_symbol(&libs, b"strsv_\0").map(|sym| *sym),
            strsv_blis_impl: get_symbol(&libs, b"strsv_blis_impl\0").map(|sym| *sym),
            dtrsv_: get_symbol(&libs, b"dtrsv_\0").map(|sym| *sym),
            dtrsv_blis_impl: get_symbol(&libs, b"dtrsv_blis_impl\0").map(|sym| *sym),
            ctrsv_: get_symbol(&libs, b"ctrsv_\0").map(|sym| *sym),
            ctrsv_blis_impl: get_symbol(&libs, b"ctrsv_blis_impl\0").map(|sym| *sym),
            ztrsv_: get_symbol(&libs, b"ztrsv_\0").map(|sym| *sym),
            ztrsv_blis_impl: get_symbol(&libs, b"ztrsv_blis_impl\0").map(|sym| *sym),
            chpmv_: get_symbol(&libs, b"chpmv_\0").map(|sym| *sym),
            zhpmv_: get_symbol(&libs, b"zhpmv_\0").map(|sym| *sym),
            chpmv_blis_impl: get_symbol(&libs, b"chpmv_blis_impl\0").map(|sym| *sym),
            zhpmv_blis_impl: get_symbol(&libs, b"zhpmv_blis_impl\0").map(|sym| *sym),
            chpr_: get_symbol(&libs, b"chpr_\0").map(|sym| *sym),
            zhpr_: get_symbol(&libs, b"zhpr_\0").map(|sym| *sym),
            chpr_blis_impl: get_symbol(&libs, b"chpr_blis_impl\0").map(|sym| *sym),
            zhpr_blis_impl: get_symbol(&libs, b"zhpr_blis_impl\0").map(|sym| *sym),
            chpr2_: get_symbol(&libs, b"chpr2_\0").map(|sym| *sym),
            zhpr2_: get_symbol(&libs, b"zhpr2_\0").map(|sym| *sym),
            chpr2_blis_impl: get_symbol(&libs, b"chpr2_blis_impl\0").map(|sym| *sym),
            zhpr2_blis_impl: get_symbol(&libs, b"zhpr2_blis_impl\0").map(|sym| *sym),
            dspmv_: get_symbol(&libs, b"dspmv_\0").map(|sym| *sym),
            sspmv_: get_symbol(&libs, b"sspmv_\0").map(|sym| *sym),
            dspmv_blis_impl: get_symbol(&libs, b"dspmv_blis_impl\0").map(|sym| *sym),
            sspmv_blis_impl: get_symbol(&libs, b"sspmv_blis_impl\0").map(|sym| *sym),
            dspr_: get_symbol(&libs, b"dspr_\0").map(|sym| *sym),
            sspr_: get_symbol(&libs, b"sspr_\0").map(|sym| *sym),
            dspr_blis_impl: get_symbol(&libs, b"dspr_blis_impl\0").map(|sym| *sym),
            sspr_blis_impl: get_symbol(&libs, b"sspr_blis_impl\0").map(|sym| *sym),
            dspr2_: get_symbol(&libs, b"dspr2_\0").map(|sym| *sym),
            sspr2_: get_symbol(&libs, b"sspr2_\0").map(|sym| *sym),
            dspr2_blis_impl: get_symbol(&libs, b"dspr2_blis_impl\0").map(|sym| *sym),
            sspr2_blis_impl: get_symbol(&libs, b"sspr2_blis_impl\0").map(|sym| *sym),
            ctpmv_: get_symbol(&libs, b"ctpmv_\0").map(|sym| *sym),
            dtpmv_: get_symbol(&libs, b"dtpmv_\0").map(|sym| *sym),
            stpmv_: get_symbol(&libs, b"stpmv_\0").map(|sym| *sym),
            ztpmv_: get_symbol(&libs, b"ztpmv_\0").map(|sym| *sym),
            ctpmv_blis_impl: get_symbol(&libs, b"ctpmv_blis_impl\0").map(|sym| *sym),
            dtpmv_blis_impl: get_symbol(&libs, b"dtpmv_blis_impl\0").map(|sym| *sym),
            stpmv_blis_impl: get_symbol(&libs, b"stpmv_blis_impl\0").map(|sym| *sym),
            ztpmv_blis_impl: get_symbol(&libs, b"ztpmv_blis_impl\0").map(|sym| *sym),
            ctpsv_: get_symbol(&libs, b"ctpsv_\0").map(|sym| *sym),
            dtpsv_: get_symbol(&libs, b"dtpsv_\0").map(|sym| *sym),
            stpsv_: get_symbol(&libs, b"stpsv_\0").map(|sym| *sym),
            ztpsv_: get_symbol(&libs, b"ztpsv_\0").map(|sym| *sym),
            ctpsv_blis_impl: get_symbol(&libs, b"ctpsv_blis_impl\0").map(|sym| *sym),
            dtpsv_blis_impl: get_symbol(&libs, b"dtpsv_blis_impl\0").map(|sym| *sym),
            stpsv_blis_impl: get_symbol(&libs, b"stpsv_blis_impl\0").map(|sym| *sym),
            ztpsv_blis_impl: get_symbol(&libs, b"ztpsv_blis_impl\0").map(|sym| *sym),
            cgbmv_: get_symbol(&libs, b"cgbmv_\0").map(|sym| *sym),
            dgbmv_: get_symbol(&libs, b"dgbmv_\0").map(|sym| *sym),
            sgbmv_: get_symbol(&libs, b"sgbmv_\0").map(|sym| *sym),
            zgbmv_: get_symbol(&libs, b"zgbmv_\0").map(|sym| *sym),
            cgbmv_blis_impl: get_symbol(&libs, b"cgbmv_blis_impl\0").map(|sym| *sym),
            dgbmv_blis_impl: get_symbol(&libs, b"dgbmv_blis_impl\0").map(|sym| *sym),
            sgbmv_blis_impl: get_symbol(&libs, b"sgbmv_blis_impl\0").map(|sym| *sym),
            zgbmv_blis_impl: get_symbol(&libs, b"zgbmv_blis_impl\0").map(|sym| *sym),
            chbmv_: get_symbol(&libs, b"chbmv_\0").map(|sym| *sym),
            zhbmv_: get_symbol(&libs, b"zhbmv_\0").map(|sym| *sym),
            chbmv_blis_impl: get_symbol(&libs, b"chbmv_blis_impl\0").map(|sym| *sym),
            zhbmv_blis_impl: get_symbol(&libs, b"zhbmv_blis_impl\0").map(|sym| *sym),
            dsbmv_: get_symbol(&libs, b"dsbmv_\0").map(|sym| *sym),
            ssbmv_: get_symbol(&libs, b"ssbmv_\0").map(|sym| *sym),
            dsbmv_blis_impl: get_symbol(&libs, b"dsbmv_blis_impl\0").map(|sym| *sym),
            ssbmv_blis_impl: get_symbol(&libs, b"ssbmv_blis_impl\0").map(|sym| *sym),
            ctbmv_: get_symbol(&libs, b"ctbmv_\0").map(|sym| *sym),
            dtbmv_: get_symbol(&libs, b"dtbmv_\0").map(|sym| *sym),
            stbmv_: get_symbol(&libs, b"stbmv_\0").map(|sym| *sym),
            ztbmv_: get_symbol(&libs, b"ztbmv_\0").map(|sym| *sym),
            ctbmv_blis_impl: get_symbol(&libs, b"ctbmv_blis_impl\0").map(|sym| *sym),
            dtbmv_blis_impl: get_symbol(&libs, b"dtbmv_blis_impl\0").map(|sym| *sym),
            stbmv_blis_impl: get_symbol(&libs, b"stbmv_blis_impl\0").map(|sym| *sym),
            ztbmv_blis_impl: get_symbol(&libs, b"ztbmv_blis_impl\0").map(|sym| *sym),
            ctbsv_: get_symbol(&libs, b"ctbsv_\0").map(|sym| *sym),
            dtbsv_: get_symbol(&libs, b"dtbsv_\0").map(|sym| *sym),
            stbsv_: get_symbol(&libs, b"stbsv_\0").map(|sym| *sym),
            ztbsv_: get_symbol(&libs, b"ztbsv_\0").map(|sym| *sym),
            ctbsv_blis_impl: get_symbol(&libs, b"ctbsv_blis_impl\0").map(|sym| *sym),
            dtbsv_blis_impl: get_symbol(&libs, b"dtbsv_blis_impl\0").map(|sym| *sym),
            stbsv_blis_impl: get_symbol(&libs, b"stbsv_blis_impl\0").map(|sym| *sym),
            ztbsv_blis_impl: get_symbol(&libs, b"ztbsv_blis_impl\0").map(|sym| *sym),
            sgemm_: get_symbol(&libs, b"sgemm_\0").map(|sym| *sym),
            sgemm_blis_impl: get_symbol(&libs, b"sgemm_blis_impl\0").map(|sym| *sym),
            dgemm_: get_symbol(&libs, b"dgemm_\0").map(|sym| *sym),
            dgemm_blis_impl: get_symbol(&libs, b"dgemm_blis_impl\0").map(|sym| *sym),
            cgemm_: get_symbol(&libs, b"cgemm_\0").map(|sym| *sym),
            cgemm_blis_impl: get_symbol(&libs, b"cgemm_blis_impl\0").map(|sym| *sym),
            zgemm_: get_symbol(&libs, b"zgemm_\0").map(|sym| *sym),
            zgemm_blis_impl: get_symbol(&libs, b"zgemm_blis_impl\0").map(|sym| *sym),
            dzgemm_: get_symbol(&libs, b"dzgemm_\0").map(|sym| *sym),
            dzgemm_blis_impl: get_symbol(&libs, b"dzgemm_blis_impl\0").map(|sym| *sym),
            chemm_: get_symbol(&libs, b"chemm_\0").map(|sym| *sym),
            chemm_blis_impl: get_symbol(&libs, b"chemm_blis_impl\0").map(|sym| *sym),
            zhemm_: get_symbol(&libs, b"zhemm_\0").map(|sym| *sym),
            zhemm_blis_impl: get_symbol(&libs, b"zhemm_blis_impl\0").map(|sym| *sym),
            cherk_: get_symbol(&libs, b"cherk_\0").map(|sym| *sym),
            cherk_blis_impl: get_symbol(&libs, b"cherk_blis_impl\0").map(|sym| *sym),
            zherk_: get_symbol(&libs, b"zherk_\0").map(|sym| *sym),
            zherk_blis_impl: get_symbol(&libs, b"zherk_blis_impl\0").map(|sym| *sym),
            cher2k_: get_symbol(&libs, b"cher2k_\0").map(|sym| *sym),
            cher2k_blis_impl: get_symbol(&libs, b"cher2k_blis_impl\0").map(|sym| *sym),
            zher2k_: get_symbol(&libs, b"zher2k_\0").map(|sym| *sym),
            zher2k_blis_impl: get_symbol(&libs, b"zher2k_blis_impl\0").map(|sym| *sym),
            ssymm_: get_symbol(&libs, b"ssymm_\0").map(|sym| *sym),
            ssymm_blis_impl: get_symbol(&libs, b"ssymm_blis_impl\0").map(|sym| *sym),
            dsymm_: get_symbol(&libs, b"dsymm_\0").map(|sym| *sym),
            dsymm_blis_impl: get_symbol(&libs, b"dsymm_blis_impl\0").map(|sym| *sym),
            csymm_: get_symbol(&libs, b"csymm_\0").map(|sym| *sym),
            csymm_blis_impl: get_symbol(&libs, b"csymm_blis_impl\0").map(|sym| *sym),
            zsymm_: get_symbol(&libs, b"zsymm_\0").map(|sym| *sym),
            zsymm_blis_impl: get_symbol(&libs, b"zsymm_blis_impl\0").map(|sym| *sym),
            ssyrk_: get_symbol(&libs, b"ssyrk_\0").map(|sym| *sym),
            ssyrk_blis_impl: get_symbol(&libs, b"ssyrk_blis_impl\0").map(|sym| *sym),
            dsyrk_: get_symbol(&libs, b"dsyrk_\0").map(|sym| *sym),
            dsyrk_blis_impl: get_symbol(&libs, b"dsyrk_blis_impl\0").map(|sym| *sym),
            csyrk_: get_symbol(&libs, b"csyrk_\0").map(|sym| *sym),
            csyrk_blis_impl: get_symbol(&libs, b"csyrk_blis_impl\0").map(|sym| *sym),
            zsyrk_: get_symbol(&libs, b"zsyrk_\0").map(|sym| *sym),
            zsyrk_blis_impl: get_symbol(&libs, b"zsyrk_blis_impl\0").map(|sym| *sym),
            ssyr2k_: get_symbol(&libs, b"ssyr2k_\0").map(|sym| *sym),
            ssyr2k_blis_impl: get_symbol(&libs, b"ssyr2k_blis_impl\0").map(|sym| *sym),
            dsyr2k_: get_symbol(&libs, b"dsyr2k_\0").map(|sym| *sym),
            dsyr2k_blis_impl: get_symbol(&libs, b"dsyr2k_blis_impl\0").map(|sym| *sym),
            csyr2k_: get_symbol(&libs, b"csyr2k_\0").map(|sym| *sym),
            csyr2k_blis_impl: get_symbol(&libs, b"csyr2k_blis_impl\0").map(|sym| *sym),
            zsyr2k_: get_symbol(&libs, b"zsyr2k_\0").map(|sym| *sym),
            zsyr2k_blis_impl: get_symbol(&libs, b"zsyr2k_blis_impl\0").map(|sym| *sym),
            strmm_: get_symbol(&libs, b"strmm_\0").map(|sym| *sym),
            strmm_blis_impl: get_symbol(&libs, b"strmm_blis_impl\0").map(|sym| *sym),
            dtrmm_: get_symbol(&libs, b"dtrmm_\0").map(|sym| *sym),
            dtrmm_blis_impl: get_symbol(&libs, b"dtrmm_blis_impl\0").map(|sym| *sym),
            ctrmm_: get_symbol(&libs, b"ctrmm_\0").map(|sym| *sym),
            ctrmm_blis_impl: get_symbol(&libs, b"ctrmm_blis_impl\0").map(|sym| *sym),
            ztrmm_: get_symbol(&libs, b"ztrmm_\0").map(|sym| *sym),
            ztrmm_blis_impl: get_symbol(&libs, b"ztrmm_blis_impl\0").map(|sym| *sym),
            strsm_: get_symbol(&libs, b"strsm_\0").map(|sym| *sym),
            strsm_blis_impl: get_symbol(&libs, b"strsm_blis_impl\0").map(|sym| *sym),
            dtrsm_: get_symbol(&libs, b"dtrsm_\0").map(|sym| *sym),
            dtrsm_blis_impl: get_symbol(&libs, b"dtrsm_blis_impl\0").map(|sym| *sym),
            ctrsm_: get_symbol(&libs, b"ctrsm_\0").map(|sym| *sym),
            ctrsm_blis_impl: get_symbol(&libs, b"ctrsm_blis_impl\0").map(|sym| *sym),
            ztrsm_: get_symbol(&libs, b"ztrsm_\0").map(|sym| *sym),
            ztrsm_blis_impl: get_symbol(&libs, b"ztrsm_blis_impl\0").map(|sym| *sym),
            sgemmt_: get_symbol(&libs, b"sgemmt_\0").map(|sym| *sym),
            sgemmt_blis_impl: get_symbol(&libs, b"sgemmt_blis_impl\0").map(|sym| *sym),
            dgemmt_: get_symbol(&libs, b"dgemmt_\0").map(|sym| *sym),
            dgemmt_blis_impl: get_symbol(&libs, b"dgemmt_blis_impl\0").map(|sym| *sym),
            cgemmt_: get_symbol(&libs, b"cgemmt_\0").map(|sym| *sym),
            cgemmt_blis_impl: get_symbol(&libs, b"cgemmt_blis_impl\0").map(|sym| *sym),
            zgemmt_: get_symbol(&libs, b"zgemmt_\0").map(|sym| *sym),
            zgemmt_blis_impl: get_symbol(&libs, b"zgemmt_blis_impl\0").map(|sym| *sym),
            sgemm_compute_: get_symbol(&libs, b"sgemm_compute_\0").map(|sym| *sym),
            sgemm_compute_blis_impl: get_symbol(&libs, b"sgemm_compute_blis_impl\0")
                .map(|sym| *sym),
            dgemm_compute_: get_symbol(&libs, b"dgemm_compute_\0").map(|sym| *sym),
            dgemm_compute_blis_impl: get_symbol(&libs, b"dgemm_compute_blis_impl\0")
                .map(|sym| *sym),
            sgemm_batch_: get_symbol(&libs, b"sgemm_batch_\0").map(|sym| *sym),
            sgemm_batch_blis_impl: get_symbol(&libs, b"sgemm_batch_blis_impl\0").map(|sym| *sym),
            dgemm_batch_: get_symbol(&libs, b"dgemm_batch_\0").map(|sym| *sym),
            dgemm_batch_blis_impl: get_symbol(&libs, b"dgemm_batch_blis_impl\0").map(|sym| *sym),
            cgemm_batch_: get_symbol(&libs, b"cgemm_batch_\0").map(|sym| *sym),
            cgemm_batch_blis_impl: get_symbol(&libs, b"cgemm_batch_blis_impl\0").map(|sym| *sym),
            zgemm_batch_: get_symbol(&libs, b"zgemm_batch_\0").map(|sym| *sym),
            zgemm_batch_blis_impl: get_symbol(&libs, b"zgemm_batch_blis_impl\0").map(|sym| *sym),
            cgemm3m_: get_symbol(&libs, b"cgemm3m_\0").map(|sym| *sym),
            cgemm3m_blis_impl: get_symbol(&libs, b"cgemm3m_blis_impl\0").map(|sym| *sym),
            zgemm3m_: get_symbol(&libs, b"zgemm3m_\0").map(|sym| *sym),
            zgemm3m_blis_impl: get_symbol(&libs, b"zgemm3m_blis_impl\0").map(|sym| *sym),
            sgemm_pack_get_size_: get_symbol(&libs, b"sgemm_pack_get_size_\0").map(|sym| *sym),
            sgemm_pack_get_size_blis_impl: get_symbol(&libs, b"sgemm_pack_get_size_blis_impl\0")
                .map(|sym| *sym),
            dgemm_pack_get_size_: get_symbol(&libs, b"dgemm_pack_get_size_\0").map(|sym| *sym),
            dgemm_pack_get_size_blis_impl: get_symbol(&libs, b"dgemm_pack_get_size_blis_impl\0")
                .map(|sym| *sym),
            sgemm_pack_: get_symbol(&libs, b"sgemm_pack_\0").map(|sym| *sym),
            sgemm_pack_blis_impl: get_symbol(&libs, b"sgemm_pack_blis_impl\0").map(|sym| *sym),
            dgemm_pack_: get_symbol(&libs, b"dgemm_pack_\0").map(|sym| *sym),
            dgemm_pack_blis_impl: get_symbol(&libs, b"dgemm_pack_blis_impl\0").map(|sym| *sym),
            somatadd_: get_symbol(&libs, b"somatadd_\0").map(|sym| *sym),
            domatadd_: get_symbol(&libs, b"domatadd_\0").map(|sym| *sym),
            comatadd_: get_symbol(&libs, b"comatadd_\0").map(|sym| *sym),
            zomatadd_: get_symbol(&libs, b"zomatadd_\0").map(|sym| *sym),
            somatcopy_: get_symbol(&libs, b"somatcopy_\0").map(|sym| *sym),
            domatcopy_: get_symbol(&libs, b"domatcopy_\0").map(|sym| *sym),
            comatcopy_: get_symbol(&libs, b"comatcopy_\0").map(|sym| *sym),
            zomatcopy_: get_symbol(&libs, b"zomatcopy_\0").map(|sym| *sym),
            somatcopy2_: get_symbol(&libs, b"somatcopy2_\0").map(|sym| *sym),
            domatcopy2_: get_symbol(&libs, b"domatcopy2_\0").map(|sym| *sym),
            comatcopy2_: get_symbol(&libs, b"comatcopy2_\0").map(|sym| *sym),
            zomatcopy2_: get_symbol(&libs, b"zomatcopy2_\0").map(|sym| *sym),
            simatcopy_: get_symbol(&libs, b"simatcopy_\0").map(|sym| *sym),
            dimatcopy_: get_symbol(&libs, b"dimatcopy_\0").map(|sym| *sym),
            cimatcopy_: get_symbol(&libs, b"cimatcopy_\0").map(|sym| *sym),
            zimatcopy_: get_symbol(&libs, b"zimatcopy_\0").map(|sym| *sym),
            bli_thread_set_ways_: get_symbol(&libs, b"bli_thread_set_ways_\0").map(|sym| *sym),
            bli_thread_set_num_threads_: get_symbol(&libs, b"bli_thread_set_num_threads_\0")
                .map(|sym| *sym),
            bli_thread_get_jc_nt_: get_symbol(&libs, b"bli_thread_get_jc_nt_\0").map(|sym| *sym),
            bli_thread_get_pc_nt_: get_symbol(&libs, b"bli_thread_get_pc_nt_\0").map(|sym| *sym),
            bli_thread_get_ic_nt_: get_symbol(&libs, b"bli_thread_get_ic_nt_\0").map(|sym| *sym),
            bli_thread_get_jr_nt_: get_symbol(&libs, b"bli_thread_get_jr_nt_\0").map(|sym| *sym),
            bli_thread_get_ir_nt_: get_symbol(&libs, b"bli_thread_get_ir_nt_\0").map(|sym| *sym),
            bli_thread_get_num_threads_: get_symbol(&libs, b"bli_thread_get_num_threads_\0")
                .map(|sym| *sym),
            bli_info_get_info_value_: get_symbol(&libs, b"bli_info_get_info_value_\0")
                .map(|sym| *sym),
            scabs1_: get_symbol(&libs, b"scabs1_\0").map(|sym| *sym),
            dcabs1_: get_symbol(&libs, b"dcabs1_\0").map(|sym| *sym),
            scabs1_blis_impl: get_symbol(&libs, b"scabs1_blis_impl\0").map(|sym| *sym),
            dcabs1_blis_impl: get_symbol(&libs, b"dcabs1_blis_impl\0").map(|sym| *sym),
            isamin_: get_symbol(&libs, b"isamin_\0").map(|sym| *sym),
            isamin_blis_impl: get_symbol(&libs, b"isamin_blis_impl\0").map(|sym| *sym),
            idamin_: get_symbol(&libs, b"idamin_\0").map(|sym| *sym),
            idamin_blis_impl: get_symbol(&libs, b"idamin_blis_impl\0").map(|sym| *sym),
            icamin_: get_symbol(&libs, b"icamin_\0").map(|sym| *sym),
            icamin_blis_impl: get_symbol(&libs, b"icamin_blis_impl\0").map(|sym| *sym),
            izamin_: get_symbol(&libs, b"izamin_\0").map(|sym| *sym),
            izamin_blis_impl: get_symbol(&libs, b"izamin_blis_impl\0").map(|sym| *sym),
            isaminsub_: get_symbol(&libs, b"isaminsub_\0").map(|sym| *sym),
            isaminsub_blis_impl: get_symbol(&libs, b"isaminsub_blis_impl\0").map(|sym| *sym),
            idaminsub_: get_symbol(&libs, b"idaminsub_\0").map(|sym| *sym),
            idaminsub_blis_impl: get_symbol(&libs, b"idaminsub_blis_impl\0").map(|sym| *sym),
            icaminsub_: get_symbol(&libs, b"icaminsub_\0").map(|sym| *sym),
            icaminsub_blis_impl: get_symbol(&libs, b"icaminsub_blis_impl\0").map(|sym| *sym),
            izaminsub_: get_symbol(&libs, b"izaminsub_\0").map(|sym| *sym),
            izaminsub_blis_impl: get_symbol(&libs, b"izaminsub_blis_impl\0").map(|sym| *sym),
            cblas_sdsdot: get_symbol(&libs, b"cblas_sdsdot\0").map(|sym| *sym),
            cblas_dsdot: get_symbol(&libs, b"cblas_dsdot\0").map(|sym| *sym),
            cblas_sdot: get_symbol(&libs, b"cblas_sdot\0").map(|sym| *sym),
            cblas_ddot: get_symbol(&libs, b"cblas_ddot\0").map(|sym| *sym),
            cblas_cdotu_sub: get_symbol(&libs, b"cblas_cdotu_sub\0").map(|sym| *sym),
            cblas_cdotc_sub: get_symbol(&libs, b"cblas_cdotc_sub\0").map(|sym| *sym),
            cblas_zdotu_sub: get_symbol(&libs, b"cblas_zdotu_sub\0").map(|sym| *sym),
            cblas_zdotc_sub: get_symbol(&libs, b"cblas_zdotc_sub\0").map(|sym| *sym),
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
            cblas_sswap: get_symbol(&libs, b"cblas_sswap\0").map(|sym| *sym),
            cblas_scopy: get_symbol(&libs, b"cblas_scopy\0").map(|sym| *sym),
            cblas_saxpy: get_symbol(&libs, b"cblas_saxpy\0").map(|sym| *sym),
            cblas_saxpby: get_symbol(&libs, b"cblas_saxpby\0").map(|sym| *sym),
            cblas_dswap: get_symbol(&libs, b"cblas_dswap\0").map(|sym| *sym),
            cblas_dcopy: get_symbol(&libs, b"cblas_dcopy\0").map(|sym| *sym),
            cblas_daxpy: get_symbol(&libs, b"cblas_daxpy\0").map(|sym| *sym),
            cblas_daxpby: get_symbol(&libs, b"cblas_daxpby\0").map(|sym| *sym),
            cblas_cswap: get_symbol(&libs, b"cblas_cswap\0").map(|sym| *sym),
            cblas_ccopy: get_symbol(&libs, b"cblas_ccopy\0").map(|sym| *sym),
            cblas_caxpy: get_symbol(&libs, b"cblas_caxpy\0").map(|sym| *sym),
            cblas_caxpby: get_symbol(&libs, b"cblas_caxpby\0").map(|sym| *sym),
            cblas_zswap: get_symbol(&libs, b"cblas_zswap\0").map(|sym| *sym),
            cblas_zcopy: get_symbol(&libs, b"cblas_zcopy\0").map(|sym| *sym),
            cblas_zaxpy: get_symbol(&libs, b"cblas_zaxpy\0").map(|sym| *sym),
            cblas_zaxpby: get_symbol(&libs, b"cblas_zaxpby\0").map(|sym| *sym),
            cblas_srotg: get_symbol(&libs, b"cblas_srotg\0").map(|sym| *sym),
            cblas_srotmg: get_symbol(&libs, b"cblas_srotmg\0").map(|sym| *sym),
            cblas_srot: get_symbol(&libs, b"cblas_srot\0").map(|sym| *sym),
            cblas_srotm: get_symbol(&libs, b"cblas_srotm\0").map(|sym| *sym),
            cblas_drotg: get_symbol(&libs, b"cblas_drotg\0").map(|sym| *sym),
            cblas_drotmg: get_symbol(&libs, b"cblas_drotmg\0").map(|sym| *sym),
            cblas_drot: get_symbol(&libs, b"cblas_drot\0").map(|sym| *sym),
            cblas_drotm: get_symbol(&libs, b"cblas_drotm\0").map(|sym| *sym),
            cblas_crotg: get_symbol(&libs, b"cblas_crotg\0").map(|sym| *sym),
            cblas_csrot: get_symbol(&libs, b"cblas_csrot\0").map(|sym| *sym),
            cblas_zrotg: get_symbol(&libs, b"cblas_zrotg\0").map(|sym| *sym),
            cblas_zdrot: get_symbol(&libs, b"cblas_zdrot\0").map(|sym| *sym),
            cblas_sscal: get_symbol(&libs, b"cblas_sscal\0").map(|sym| *sym),
            cblas_dscal: get_symbol(&libs, b"cblas_dscal\0").map(|sym| *sym),
            cblas_cscal: get_symbol(&libs, b"cblas_cscal\0").map(|sym| *sym),
            cblas_zscal: get_symbol(&libs, b"cblas_zscal\0").map(|sym| *sym),
            cblas_csscal: get_symbol(&libs, b"cblas_csscal\0").map(|sym| *sym),
            cblas_zdscal: get_symbol(&libs, b"cblas_zdscal\0").map(|sym| *sym),
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
            cblas_ssymm: get_symbol(&libs, b"cblas_ssymm\0").map(|sym| *sym),
            cblas_ssyrk: get_symbol(&libs, b"cblas_ssyrk\0").map(|sym| *sym),
            cblas_ssyr2k: get_symbol(&libs, b"cblas_ssyr2k\0").map(|sym| *sym),
            cblas_strmm: get_symbol(&libs, b"cblas_strmm\0").map(|sym| *sym),
            cblas_strsm: get_symbol(&libs, b"cblas_strsm\0").map(|sym| *sym),
            cblas_sgemmt: get_symbol(&libs, b"cblas_sgemmt\0").map(|sym| *sym),
            cblas_dgemm: get_symbol(&libs, b"cblas_dgemm\0").map(|sym| *sym),
            cblas_dsymm: get_symbol(&libs, b"cblas_dsymm\0").map(|sym| *sym),
            cblas_dsyrk: get_symbol(&libs, b"cblas_dsyrk\0").map(|sym| *sym),
            cblas_dsyr2k: get_symbol(&libs, b"cblas_dsyr2k\0").map(|sym| *sym),
            cblas_dtrmm: get_symbol(&libs, b"cblas_dtrmm\0").map(|sym| *sym),
            cblas_dtrsm: get_symbol(&libs, b"cblas_dtrsm\0").map(|sym| *sym),
            cblas_dgemmt: get_symbol(&libs, b"cblas_dgemmt\0").map(|sym| *sym),
            cblas_cgemm: get_symbol(&libs, b"cblas_cgemm\0").map(|sym| *sym),
            cblas_csymm: get_symbol(&libs, b"cblas_csymm\0").map(|sym| *sym),
            cblas_csyrk: get_symbol(&libs, b"cblas_csyrk\0").map(|sym| *sym),
            cblas_csyr2k: get_symbol(&libs, b"cblas_csyr2k\0").map(|sym| *sym),
            cblas_ctrmm: get_symbol(&libs, b"cblas_ctrmm\0").map(|sym| *sym),
            cblas_ctrsm: get_symbol(&libs, b"cblas_ctrsm\0").map(|sym| *sym),
            cblas_cgemmt: get_symbol(&libs, b"cblas_cgemmt\0").map(|sym| *sym),
            cblas_zgemm: get_symbol(&libs, b"cblas_zgemm\0").map(|sym| *sym),
            cblas_zsymm: get_symbol(&libs, b"cblas_zsymm\0").map(|sym| *sym),
            cblas_zsyrk: get_symbol(&libs, b"cblas_zsyrk\0").map(|sym| *sym),
            cblas_zsyr2k: get_symbol(&libs, b"cblas_zsyr2k\0").map(|sym| *sym),
            cblas_ztrmm: get_symbol(&libs, b"cblas_ztrmm\0").map(|sym| *sym),
            cblas_ztrsm: get_symbol(&libs, b"cblas_ztrsm\0").map(|sym| *sym),
            cblas_zgemmt: get_symbol(&libs, b"cblas_zgemmt\0").map(|sym| *sym),
            cblas_chemm: get_symbol(&libs, b"cblas_chemm\0").map(|sym| *sym),
            cblas_cherk: get_symbol(&libs, b"cblas_cherk\0").map(|sym| *sym),
            cblas_cher2k: get_symbol(&libs, b"cblas_cher2k\0").map(|sym| *sym),
            cblas_zhemm: get_symbol(&libs, b"cblas_zhemm\0").map(|sym| *sym),
            cblas_zherk: get_symbol(&libs, b"cblas_zherk\0").map(|sym| *sym),
            cblas_zher2k: get_symbol(&libs, b"cblas_zher2k\0").map(|sym| *sym),
            cblas_xerbla: get_symbol(&libs, b"cblas_xerbla\0").map(|sym| *sym),
            cblas_scabs1: get_symbol(&libs, b"cblas_scabs1\0").map(|sym| *sym),
            cblas_dcabs1: get_symbol(&libs, b"cblas_dcabs1\0").map(|sym| *sym),
            cblas_sgemm_batch: get_symbol(&libs, b"cblas_sgemm_batch\0").map(|sym| *sym),
            cblas_dgemm_batch: get_symbol(&libs, b"cblas_dgemm_batch\0").map(|sym| *sym),
            cblas_cgemm_batch: get_symbol(&libs, b"cblas_cgemm_batch\0").map(|sym| *sym),
            cblas_zgemm_batch: get_symbol(&libs, b"cblas_zgemm_batch\0").map(|sym| *sym),
            cblas_cgemm3m: get_symbol(&libs, b"cblas_cgemm3m\0").map(|sym| *sym),
            cblas_zgemm3m: get_symbol(&libs, b"cblas_zgemm3m\0").map(|sym| *sym),
            cblas_isamin: get_symbol(&libs, b"cblas_isamin\0").map(|sym| *sym),
            cblas_idamin: get_symbol(&libs, b"cblas_idamin\0").map(|sym| *sym),
            cblas_icamin: get_symbol(&libs, b"cblas_icamin\0").map(|sym| *sym),
            cblas_izamin: get_symbol(&libs, b"cblas_izamin\0").map(|sym| *sym),
            cblas_sgemm_pack_get_size: get_symbol(&libs, b"cblas_sgemm_pack_get_size\0")
                .map(|sym| *sym),
            cblas_dgemm_pack_get_size: get_symbol(&libs, b"cblas_dgemm_pack_get_size\0")
                .map(|sym| *sym),
            cblas_sgemm_pack: get_symbol(&libs, b"cblas_sgemm_pack\0").map(|sym| *sym),
            cblas_dgemm_pack: get_symbol(&libs, b"cblas_dgemm_pack\0").map(|sym| *sym),
            cblas_sgemm_compute: get_symbol(&libs, b"cblas_sgemm_compute\0").map(|sym| *sym),
            cblas_dgemm_compute: get_symbol(&libs, b"cblas_dgemm_compute\0").map(|sym| *sym),
            bli_sleep: get_symbol(&libs, b"bli_sleep\0").map(|sym| *sym),
            AOCL_FAL_Close: get_symbol(&libs, b"AOCL_FAL_Close\0").map(|sym| *sym),
            AOCL_FAL_Error: get_symbol(&libs, b"AOCL_FAL_Error\0").map(|sym| *sym),
            AOCL_FAL_Open: get_symbol(&libs, b"AOCL_FAL_Open\0").map(|sym| *sym),
            AOCL_FAL_Read: get_symbol(&libs, b"AOCL_FAL_Read\0").map(|sym| *sym),
            AOCL_FAL_Write: get_symbol(&libs, b"AOCL_FAL_Write\0").map(|sym| *sym),
            AOCL_FLIST_IsEmpty: get_symbol(&libs, b"AOCL_FLIST_IsEmpty\0").map(|sym| *sym),
            AOCL_FLIST_GetNode: get_symbol(&libs, b"AOCL_FLIST_GetNode\0").map(|sym| *sym),
            AOCL_FLIST_GetFile: get_symbol(&libs, b"AOCL_FLIST_GetFile\0").map(|sym| *sym),
            AOCL_FLIST_AddFile: get_symbol(&libs, b"AOCL_FLIST_AddFile\0").map(|sym| *sym),
            AOCL_FLIST_CloseFile: get_symbol(&libs, b"AOCL_FLIST_CloseFile\0").map(|sym| *sym),
            AOCL_FLIST_CloseAll: get_symbol(&libs, b"AOCL_FLIST_CloseAll\0").map(|sym| *sym),
            AOCL_gettid: get_symbol(&libs, b"AOCL_gettid\0").map(|sym| *sym),
            AOCL_getpid: get_symbol(&libs, b"AOCL_getpid\0").map(|sym| *sym),
            AOCL_getTimestamp: get_symbol(&libs, b"AOCL_getTimestamp\0").map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
