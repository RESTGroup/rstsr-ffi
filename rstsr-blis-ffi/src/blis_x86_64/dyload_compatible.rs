
//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn bli_pthread_create(thread: *mut bli_pthread_t, attr: *const bli_pthread_attr_t, start_routine: Option<
            extern "C" fn(arg1: *mut c_void) -> *mut c_void,
        >, arg: *mut c_void) -> c_int {
                dyload_lib().bli_pthread_create.unwrap()(thread, attr, start_routine, arg)
            }

pub unsafe fn bli_pthread_join(thread: bli_pthread_t, retval: *mut *mut c_void) -> c_int {
                dyload_lib().bli_pthread_join.unwrap()(thread, retval)
            }

pub unsafe fn bli_pthread_mutex_init(mutex: *mut bli_pthread_mutex_t, attr: *const bli_pthread_mutexattr_t) -> c_int {
                dyload_lib().bli_pthread_mutex_init.unwrap()(mutex, attr)
            }

pub unsafe fn bli_pthread_mutex_destroy(mutex: *mut bli_pthread_mutex_t) -> c_int {
                dyload_lib().bli_pthread_mutex_destroy.unwrap()(mutex)
            }

pub unsafe fn bli_pthread_mutex_lock(mutex: *mut bli_pthread_mutex_t) -> c_int {
                dyload_lib().bli_pthread_mutex_lock.unwrap()(mutex)
            }

pub unsafe fn bli_pthread_mutex_trylock(mutex: *mut bli_pthread_mutex_t) -> c_int {
                dyload_lib().bli_pthread_mutex_trylock.unwrap()(mutex)
            }

pub unsafe fn bli_pthread_mutex_unlock(mutex: *mut bli_pthread_mutex_t) -> c_int {
                dyload_lib().bli_pthread_mutex_unlock.unwrap()(mutex)
            }

pub unsafe fn bli_pthread_cond_init(cond: *mut bli_pthread_cond_t, attr: *const bli_pthread_condattr_t) -> c_int {
                dyload_lib().bli_pthread_cond_init.unwrap()(cond, attr)
            }

pub unsafe fn bli_pthread_cond_destroy(cond: *mut bli_pthread_cond_t) -> c_int {
                dyload_lib().bli_pthread_cond_destroy.unwrap()(cond)
            }

pub unsafe fn bli_pthread_cond_wait(cond: *mut bli_pthread_cond_t, mutex: *mut bli_pthread_mutex_t) -> c_int {
                dyload_lib().bli_pthread_cond_wait.unwrap()(cond, mutex)
            }

pub unsafe fn bli_pthread_cond_broadcast(cond: *mut bli_pthread_cond_t) -> c_int {
                dyload_lib().bli_pthread_cond_broadcast.unwrap()(cond)
            }

pub unsafe fn bli_pthread_once(once: *mut bli_pthread_once_t, init: Option<extern "C" fn()>) {
                dyload_lib().bli_pthread_once.unwrap()(once, init)
            }

pub unsafe fn bli_pthread_barrier_init(barrier: *mut bli_pthread_barrier_t, attr: *const bli_pthread_barrierattr_t, count: c_uint) -> c_int {
                dyload_lib().bli_pthread_barrier_init.unwrap()(barrier, attr, count)
            }

pub unsafe fn bli_pthread_barrier_destroy(barrier: *mut bli_pthread_barrier_t) -> c_int {
                dyload_lib().bli_pthread_barrier_destroy.unwrap()(barrier)
            }

pub unsafe fn bli_pthread_barrier_wait(barrier: *mut bli_pthread_barrier_t) -> c_int {
                dyload_lib().bli_pthread_barrier_wait.unwrap()(barrier)
            }

pub unsafe fn bli_pthread_switch_on(sw: *mut bli_pthread_switch_t, init: Option<extern "C" fn() -> c_int>) -> c_int {
                dyload_lib().bli_pthread_switch_on.unwrap()(sw, init)
            }

pub unsafe fn bli_pthread_switch_off(sw: *mut bli_pthread_switch_t, deinit: Option<extern "C" fn() -> c_int>) -> c_int {
                dyload_lib().bli_pthread_switch_off.unwrap()(sw, deinit)
            }

pub unsafe fn bli_cntx_init_skx(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_skx.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_skx_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_skx_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_skx_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_skx_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_knl(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_knl.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_knl_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_knl_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_knl_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_knl_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_haswell(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_haswell.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_haswell_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_haswell_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_haswell_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_haswell_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_sandybridge(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_sandybridge.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_sandybridge_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_sandybridge_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_sandybridge_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_sandybridge_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_penryn(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_penryn.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_penryn_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_penryn_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_penryn_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_penryn_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_zen3(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen3.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_zen3_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen3_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_zen3_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen3_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_zen2(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen2.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_zen2_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen2_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_zen2_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen2_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_zen(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_zen_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_zen_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_zen_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_excavator(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_excavator.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_excavator_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_excavator_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_excavator_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_excavator_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_steamroller(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_steamroller.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_steamroller_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_steamroller_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_steamroller_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_steamroller_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_piledriver(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_piledriver.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_piledriver_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_piledriver_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_piledriver_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_piledriver_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_bulldozer(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_bulldozer.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_bulldozer_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_bulldozer_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_bulldozer_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_bulldozer_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_cntx_init_generic(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_generic.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_generic_ref(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_generic_ref.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_init_generic_ind(method: ind_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_init_generic_ind.unwrap()(method, cntx)
            }

pub unsafe fn bli_sgemm_skx_asm_32x12_l2(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_skx_asm_32x12_l2.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_skx_asm_12x32_l2(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_skx_asm_12x32_l2.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_skx_asm_16x12_l2(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_skx_asm_16x12_l2.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_skx_asm_16x14(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_skx_asm_16x14.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_knl_asm_24x16(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_knl_asm_24x16.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_knl_asm_24x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_knl_asm_24x8.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_spackm_knl_asm_24x16(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_spackm_knl_asm_24x16.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_dpackm_knl_asm_24x8(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_knl_asm_24x8.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_dgemm_knl_asm_12x16(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_knl_asm_12x16.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_knl_asm_30x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_knl_asm_30x8.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_knl_asm_8x24(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_knl_asm_8x24.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dpackm_knl_asm_30x8(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_knl_asm_30x8.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_spackm_haswell_asm_6x16(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_spackm_haswell_asm_6x16.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_dpackm_haswell_asm_6x8(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_haswell_asm_6x8.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_cpackm_haswell_asm_3x8(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_cpackm_haswell_asm_3x8.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_zpackm_haswell_asm_3x4(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_zpackm_haswell_asm_3x4.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_sgemm_haswell_asm_6x16(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_haswell_asm_6x16.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_haswell_asm_6x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_haswell_asm_6x8.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_haswell_asm_3x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_haswell_asm_3x8.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_haswell_asm_3x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_haswell_asm_3x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_haswell_asm_16x6(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_haswell_asm_16x6.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_haswell_asm_8x6(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_haswell_asm_8x6.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_haswell_asm_8x3(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_haswell_asm_8x3.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_haswell_asm_4x3(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_haswell_asm_4x3.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmtrsm_l_haswell_asm_6x16(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a1x: *const c_void, a11: *const c_void, bx1: *const c_void, b11: *mut c_void, c11: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmtrsm_l_haswell_asm_6x16.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmtrsm_l_haswell_asm_6x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a1x: *const c_void, a11: *const c_void, bx1: *const c_void, b11: *mut c_void, c11: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmtrsm_l_haswell_asm_6x8.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmtrsm_u_haswell_asm_6x16(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a1x: *const c_void, a11: *const c_void, bx1: *const c_void, b11: *mut c_void, c11: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmtrsm_u_haswell_asm_6x16.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmtrsm_u_haswell_asm_6x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a1x: *const c_void, a11: *const c_void, bx1: *const c_void, b11: *mut c_void, c11: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmtrsm_u_haswell_asm_6x8.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_haswell_ref_6x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_haswell_ref_6x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_haswell_ref_5x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_haswell_ref_5x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_haswell_ref_4x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_haswell_ref_4x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_haswell_ref_3x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_haswell_ref_3x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_haswell_ref_2x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_haswell_ref_2x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_haswell_ref_1x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_haswell_ref_1x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x16m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x16m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x12m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x12m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x6m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x6m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_6x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_6x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_5x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_5x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_4x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_4x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_3x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_3x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_2x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_2x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_haswell_asm_1x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_haswell_asm_1x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x12(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x12.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_3x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_3x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x16m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x16m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x12m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x12m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_6x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_6x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_3x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_3x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_2x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_2x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_haswell_asm_1x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_haswell_asm_1x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_r_haswell_ref_6x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_r_haswell_ref_6x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_r_haswell_ref_5x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_r_haswell_ref_5x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_r_haswell_ref_4x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_r_haswell_ref_4x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_r_haswell_ref_3x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_r_haswell_ref_3x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_r_haswell_ref_2x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_r_haswell_ref_2x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_r_haswell_ref_1x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_r_haswell_ref_1x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_5x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_5x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_4x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_4x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_3x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_3x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_5x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_5x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_4x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_4x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_3x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_3x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_2x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_2x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_1x6(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_1x6.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_5x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_5x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_4x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_4x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_3x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_3x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_5x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_5x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_4x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_4x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x6m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x6m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_6x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_6x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_5x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_5x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_4x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_4x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_3x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_3x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_2x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_2x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rv_haswell_asm_1x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rv_haswell_asm_1x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_3x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_3x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_2x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_2x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_1x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_1x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_6x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_6x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_3x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_3x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_2x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_2x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmsup_rd_haswell_asm_1x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmsup_rd_haswell_asm_1x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_sandybridge_asm_8x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_sandybridge_asm_8x8.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_sandybridge_asm_8x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_sandybridge_asm_8x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_sandybridge_asm_8x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_sandybridge_asm_8x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_sandybridge_asm_4x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_sandybridge_asm_4x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_sandybridge_int_8x8(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_sandybridge_int_8x8.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_sandybridge_int_8x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_sandybridge_int_8x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_sandybridge_int_8x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_sandybridge_int_8x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_sandybridge_int_4x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_sandybridge_int_4x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_penryn_asm_8x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_penryn_asm_8x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_penryn_asm_4x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_penryn_asm_4x4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmtrsm_l_penryn_asm_4x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a1x: *const c_void, a11: *const c_void, bx1: *const c_void, b11: *mut c_void, c11: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmtrsm_l_penryn_asm_4x4.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmtrsm_u_penryn_asm_4x4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a1x: *const c_void, a11: *const c_void, bx1: *const c_void, b11: *mut c_void, c11: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmtrsm_u_penryn_asm_4x4.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dtrsm_l_penryn_asm_4x4(a: *const c_void, b: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dtrsm_l_penryn_asm_4x4.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dtrsm_u_penryn_asm_4x4(a: *const c_void, b: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dtrsm_u_penryn_asm_4x4.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_saxpyf_zen_int_5(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_saxpyf_zen_int_5.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_daxpyf_zen_int_5(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_daxpyf_zen_int_5.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_dpackm_8xk_gen_zen(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_8xk_gen_zen.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_dpackm_6xk_gen_zen(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_6xk_gen_zen.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_dpackm_8xk_nn_zen(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_8xk_nn_zen.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_dpackm_6xk_nn_zen(conja: conj_t, schema: pack_t, cdim: dim_t, cdim_max: dim_t, cdim_bcast: dim_t, n: dim_t, n_max: dim_t, kappa: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dpackm_6xk_nn_zen.unwrap()(conja, schema, cdim, cdim_max, cdim_bcast, n, n_max, kappa, a, inca, lda, p, ldp, params, cntx)
            }

pub unsafe fn bli_samaxv_zen_int(n: dim_t, x: *const c_void, incx: inc_t, index: *mut dim_t, cntx: *const cntx_t) {
                dyload_lib().bli_samaxv_zen_int.unwrap()(n, x, incx, index, cntx)
            }

pub unsafe fn bli_damaxv_zen_int(n: dim_t, x: *const c_void, incx: inc_t, index: *mut dim_t, cntx: *const cntx_t) {
                dyload_lib().bli_damaxv_zen_int.unwrap()(n, x, incx, index, cntx)
            }

pub unsafe fn bli_saxpyv_zen_int(conjx: conj_t, n: dim_t, alpha: *const c_void, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_saxpyv_zen_int.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_daxpyv_zen_int(conjx: conj_t, n: dim_t, alpha: *const c_void, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_daxpyv_zen_int.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_saxpyv_zen_int10(conjx: conj_t, n: dim_t, alpha: *const c_void, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_saxpyv_zen_int10.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_daxpyv_zen_int10(conjx: conj_t, n: dim_t, alpha: *const c_void, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_daxpyv_zen_int10.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_sdotv_zen_int(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *const c_void, incy: inc_t, rho: *mut c_void, cntx: *const cntx_t) {
                dyload_lib().bli_sdotv_zen_int.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx)
            }

pub unsafe fn bli_ddotv_zen_int(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *const c_void, incy: inc_t, rho: *mut c_void, cntx: *const cntx_t) {
                dyload_lib().bli_ddotv_zen_int.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx)
            }

pub unsafe fn bli_sdotv_zen_int10(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *const c_void, incy: inc_t, rho: *mut c_void, cntx: *const cntx_t) {
                dyload_lib().bli_sdotv_zen_int10.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx)
            }

pub unsafe fn bli_ddotv_zen_int10(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *const c_void, incy: inc_t, rho: *mut c_void, cntx: *const cntx_t) {
                dyload_lib().bli_ddotv_zen_int10.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx)
            }

pub unsafe fn bli_sdotxv_zen_int(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const c_void, x: *const c_void, incx: inc_t, y: *const c_void, incy: inc_t, beta: *const c_void, rho: *mut c_void, cntx: *const cntx_t) {
                dyload_lib().bli_sdotxv_zen_int.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho, cntx)
            }

pub unsafe fn bli_ddotxv_zen_int(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const c_void, x: *const c_void, incx: inc_t, y: *const c_void, incy: inc_t, beta: *const c_void, rho: *mut c_void, cntx: *const cntx_t) {
                dyload_lib().bli_ddotxv_zen_int.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho, cntx)
            }

pub unsafe fn bli_sscalv_zen_int(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_sscalv_zen_int.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_dscalv_zen_int(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_dscalv_zen_int.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_sscalv_zen_int10(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_sscalv_zen_int10.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_dscalv_zen_int10(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_dscalv_zen_int10.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_cscalv_zen_int10(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_cscalv_zen_int10.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_sswapv_zen_int8(n: dim_t, x: *mut c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_sswapv_zen_int8.unwrap()(n, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_dswapv_zen_int8(n: dim_t, x: *mut c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_dswapv_zen_int8.unwrap()(n, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_scopyv_zen_int(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_scopyv_zen_int.unwrap()(conjx, n, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_dcopyv_zen_int(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_dcopyv_zen_int.unwrap()(conjx, n, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_ssetv_zen_int(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_ssetv_zen_int.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_dsetv_zen_int(conjalpha: conj_t, n: dim_t, alpha: *const c_void, x: *mut c_void, incx: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_dsetv_zen_int.unwrap()(conjalpha, n, alpha, x, incx, cntx)
            }

pub unsafe fn bli_saxpyf_zen_int_8(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_saxpyf_zen_int_8.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_daxpyf_zen_int_8(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_daxpyf_zen_int_8.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_daxpyf_zen_int_16x4(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_daxpyf_zen_int_16x4.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_caxpyf_zen_int_4(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_caxpyf_zen_int_4.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx)
            }

pub unsafe fn bli_sdotxf_zen_int_8(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, beta: *const c_void, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_sdotxf_zen_int_8.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_ddotxf_zen_int_8(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const c_void, a: *const c_void, inca: inc_t, lda: inc_t, x: *const c_void, incx: inc_t, beta: *const c_void, y: *mut c_void, incy: inc_t, cntx: *const cntx_t) {
                dyload_lib().bli_ddotxf_zen_int_8.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_5x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_5x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_4x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_4x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_3x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_3x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_2x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_2x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_1x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_1x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_5x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_5x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_4x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_4x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_3x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_3x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_5x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_5x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_4x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_4x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_3x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_3x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_5x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_5x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_4x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_4x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_zen_ref_6x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_zen_ref_6x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_zen_ref_5x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_zen_ref_5x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_zen_ref_4x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_zen_ref_4x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_zen_ref_3x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_zen_ref_3x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_zen_ref_2x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_zen_ref_2x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_r_zen_ref_1x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_r_zen_ref_1x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x16m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x16m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_6x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_6x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_5x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_5x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_4x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_4x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_3x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_3x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_2x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_2x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rv_zen_asm_1x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rv_zen_asm_1x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_2x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_2x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_1x16(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_1x16.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x16m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x16m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_6x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_6x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_3x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_3x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_2x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_2x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmsup_rd_zen_asm_1x16n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmsup_rd_zen_asm_1x16n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_3x8m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_3x8m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_3x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_3x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_3x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_3x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_2x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_2x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_1x8(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_1x8.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_3x4m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_3x4m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_3x2m(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_3x2m.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_2x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_2x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_1x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_1x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_2x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_2x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_1x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_1x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_3x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_3x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_2x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_2x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_1x8n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_1x8n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_3x4(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_3x4.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmsup_rv_zen_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmsup_rv_zen_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_3x4n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_3x4n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_2x4n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_2x4n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_1x4n(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_1x4n.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_3x2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_3x2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmsup_rv_zen_asm_3x1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *const c_void, rs_b: inc_t, cs_b: inc_t, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmsup_rv_zen_asm_3x1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_piledriver_asm_16x3(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_piledriver_asm_16x3.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_piledriver_asm_8x3(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_piledriver_asm_8x3.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_piledriver_asm_4x2(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_piledriver_asm_4x2.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_piledriver_asm_2x2(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_piledriver_asm_2x2.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemm_bulldozer_asm_8x8_fma4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_bulldozer_asm_8x8_fma4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_bulldozer_asm_4x6_fma4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_bulldozer_asm_4x6_fma4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_bulldozer_asm_8x4_fma4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_bulldozer_asm_8x4_fma4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_bulldozer_asm_4x4_fma4(m: dim_t, n: dim_t, k: dim_t, alpha: *const c_void, a: *const c_void, b: *const c_void, beta: *const c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_bulldozer_asm_4x4_fma4.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_thrcomm_init_single(nt: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_init_single.unwrap()(nt, comm)
            }

pub unsafe fn bli_thrcomm_cleanup_single(comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_cleanup_single.unwrap()(comm)
            }

pub unsafe fn bli_thrcomm_barrier_single(tid: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_barrier_single.unwrap()(tid, comm)
            }

pub unsafe fn bli_thrcomm_init_openmp(nt: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_init_openmp.unwrap()(nt, comm)
            }

pub unsafe fn bli_thrcomm_cleanup_openmp(comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_cleanup_openmp.unwrap()(comm)
            }

pub unsafe fn bli_thrcomm_barrier_openmp(tid: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_barrier_openmp.unwrap()(tid, comm)
            }

pub unsafe fn bli_thrcomm_init_pthreads(nt: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_init_pthreads.unwrap()(nt, comm)
            }

pub unsafe fn bli_thrcomm_cleanup_pthreads(comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_cleanup_pthreads.unwrap()(comm)
            }

pub unsafe fn bli_thrcomm_barrier_pthreads(tid: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_barrier_pthreads.unwrap()(tid, comm)
            }

pub unsafe fn bli_thrcomm_create(ti: timpl_t, sba_pool: *mut pool_t, n_threads: dim_t) -> *mut thrcomm_t {
                dyload_lib().bli_thrcomm_create.unwrap()(ti, sba_pool, n_threads)
            }

pub unsafe fn bli_thrcomm_free(sba_pool: *mut pool_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_free.unwrap()(sba_pool, comm)
            }

pub unsafe fn bli_thrcomm_init(ti: timpl_t, n_threads: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_init.unwrap()(ti, n_threads, comm)
            }

pub unsafe fn bli_thrcomm_cleanup(comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_cleanup.unwrap()(comm)
            }

pub unsafe fn bli_thrcomm_barrier(thread_id: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_barrier.unwrap()(thread_id, comm)
            }

pub unsafe fn bli_thrcomm_bcast(inside_id: dim_t, to_send: *mut c_void, comm: *mut thrcomm_t) -> *mut c_void {
                dyload_lib().bli_thrcomm_bcast.unwrap()(inside_id, to_send, comm)
            }

pub unsafe fn bli_thrcomm_barrier_atomic(thread_id: dim_t, comm: *mut thrcomm_t) {
                dyload_lib().bli_thrcomm_barrier_atomic.unwrap()(thread_id, comm)
            }

pub unsafe fn bli_thrinfo_attach_sub_node(sub_node: *mut thrinfo_t, t: *mut thrinfo_t) {
                dyload_lib().bli_thrinfo_attach_sub_node.unwrap()(sub_node, t)
            }

pub unsafe fn bli_thrinfo_create_root(comm: *mut thrcomm_t, thread_id: dim_t, sba_pool: *mut pool_t, pba: *mut pba_t) -> *mut thrinfo_t {
                dyload_lib().bli_thrinfo_create_root.unwrap()(comm, thread_id, sba_pool, pba)
            }

pub unsafe fn bli_thrinfo_create(comm: *mut thrcomm_t, thread_id: dim_t, n_way: dim_t, work_id: dim_t, free_comm: bool, sba_pool: *mut pool_t, pba: *mut pba_t) -> *mut thrinfo_t {
                dyload_lib().bli_thrinfo_create.unwrap()(comm, thread_id, n_way, work_id, free_comm, sba_pool, pba)
            }

pub unsafe fn bli_thrinfo_free(thread: *mut thrinfo_t) {
                dyload_lib().bli_thrinfo_free.unwrap()(thread)
            }

pub unsafe fn bli_thrinfo_split(n_way: dim_t, thread_par: *mut thrinfo_t) -> *mut thrinfo_t {
                dyload_lib().bli_thrinfo_split.unwrap()(n_way, thread_par)
            }

pub unsafe fn bli_thrinfo_print(thread: *mut thrinfo_t) {
                dyload_lib().bli_thrinfo_print.unwrap()(thread)
            }

pub unsafe fn bli_thrinfo_print_sub(thread: *mut thrinfo_t, level: gint_t) {
                dyload_lib().bli_thrinfo_print_sub.unwrap()(thread, level)
            }

pub unsafe fn bli_thread_launch_openmp(nt: dim_t, func: thread_func_t, params: *const c_void) {
                dyload_lib().bli_thread_launch_openmp.unwrap()(nt, func, params)
            }

pub unsafe fn bli_thread_launch_pthreads(nt: dim_t, func: thread_func_t, params: *const c_void) {
                dyload_lib().bli_thread_launch_pthreads.unwrap()(nt, func, params)
            }

pub unsafe fn bli_thread_launch_single(nt: dim_t, func: thread_func_t, params: *const c_void) {
                dyload_lib().bli_thread_launch_single.unwrap()(nt, func, params)
            }

pub unsafe fn bli_thread_init() -> c_int {
                dyload_lib().bli_thread_init.unwrap()()
            }

pub unsafe fn bli_thread_finalize() -> c_int {
                dyload_lib().bli_thread_finalize.unwrap()()
            }

pub unsafe fn bli_thread_launch(ti: timpl_t, nt: dim_t, func: thread_func_t, params: *const c_void) {
                dyload_lib().bli_thread_launch.unwrap()(ti, nt, func, params)
            }

pub unsafe fn bli_prime_factorization(n: dim_t, factors: *mut bli_prime_factors_t) {
                dyload_lib().bli_prime_factorization.unwrap()(n, factors)
            }

pub unsafe fn bli_next_prime_factor(factors: *mut bli_prime_factors_t) -> dim_t {
                dyload_lib().bli_next_prime_factor.unwrap()(factors)
            }

pub unsafe fn bli_is_prime(n: dim_t) -> bool {
                dyload_lib().bli_is_prime.unwrap()(n)
            }

pub unsafe fn bli_thread_partition_2x2(n_thread: dim_t, work1: dim_t, work2: dim_t, nt1: *mut dim_t, nt2: *mut dim_t) {
                dyload_lib().bli_thread_partition_2x2.unwrap()(n_thread, work1, work2, nt1, nt2)
            }

pub unsafe fn bli_thread_partition_2x2_slow(n_thread: dim_t, work1: dim_t, work2: dim_t, nt1: *mut dim_t, nt2: *mut dim_t) {
                dyload_lib().bli_thread_partition_2x2_slow.unwrap()(n_thread, work1, work2, nt1, nt2)
            }

pub unsafe fn bli_thread_partition_2x2_fast(n_thread: dim_t, work1: dim_t, work2: dim_t, nt1: *mut dim_t, nt2: *mut dim_t) {
                dyload_lib().bli_thread_partition_2x2_fast.unwrap()(n_thread, work1, work2, nt1, nt2)
            }

pub unsafe fn bli_gcd(x: dim_t, y: dim_t) -> dim_t {
                dyload_lib().bli_gcd.unwrap()(x, y)
            }

pub unsafe fn bli_lcm(x: dim_t, y: dim_t) -> dim_t {
                dyload_lib().bli_lcm.unwrap()(x, y)
            }

pub unsafe fn bli_ipow(base: dim_t, power: dim_t) -> dim_t {
                dyload_lib().bli_ipow.unwrap()(base, power)
            }

pub unsafe fn bli_thread_get_jc_nt() -> dim_t {
                dyload_lib().bli_thread_get_jc_nt.unwrap()()
            }

pub unsafe fn bli_thread_get_pc_nt() -> dim_t {
                dyload_lib().bli_thread_get_pc_nt.unwrap()()
            }

pub unsafe fn bli_thread_get_ic_nt() -> dim_t {
                dyload_lib().bli_thread_get_ic_nt.unwrap()()
            }

pub unsafe fn bli_thread_get_jr_nt() -> dim_t {
                dyload_lib().bli_thread_get_jr_nt.unwrap()()
            }

pub unsafe fn bli_thread_get_ir_nt() -> dim_t {
                dyload_lib().bli_thread_get_ir_nt.unwrap()()
            }

pub unsafe fn bli_thread_get_num_threads() -> dim_t {
                dyload_lib().bli_thread_get_num_threads.unwrap()()
            }

pub unsafe fn bli_thread_get_thread_impl() -> timpl_t {
                dyload_lib().bli_thread_get_thread_impl.unwrap()()
            }

pub unsafe fn bli_thread_get_thread_impl_str(ti: timpl_t) -> *const c_char {
                dyload_lib().bli_thread_get_thread_impl_str.unwrap()(ti)
            }

pub unsafe fn bli_thread_set_ways(jc: dim_t, pc: dim_t, ic: dim_t, jr: dim_t, ir: dim_t) {
                dyload_lib().bli_thread_set_ways.unwrap()(jc, pc, ic, jr, ir)
            }

pub unsafe fn bli_thread_set_num_threads(value: dim_t) {
                dyload_lib().bli_thread_set_num_threads.unwrap()(value)
            }

pub unsafe fn bli_thread_set_thread_impl(ti: timpl_t) {
                dyload_lib().bli_thread_set_thread_impl.unwrap()(ti)
            }

pub unsafe fn bli_thread_reset() {
                dyload_lib().bli_thread_reset.unwrap()()
            }

pub unsafe fn bli_thread_range_sub(work_id: dim_t, n_way: dim_t, n: dim_t, bf: dim_t, handle_edge_low: bool, start: *mut dim_t, end: *mut dim_t) {
                dyload_lib().bli_thread_range_sub.unwrap()(work_id, n_way, n, bf, handle_edge_low, start, end)
            }

pub unsafe fn bli_thread_range_mdim(direct: dir_t, bmult: dim_t, use_weighted: bool, thr: *const thrinfo_t, a: *const obj_t, b: *const obj_t, c: *const obj_t, start: *mut dim_t, end: *mut dim_t) -> siz_t {
                dyload_lib().bli_thread_range_mdim.unwrap()(direct, bmult, use_weighted, thr, a, b, c, start, end)
            }

pub unsafe fn bli_thread_range_ndim(direct: dir_t, bmult: dim_t, use_weighted: bool, thr: *const thrinfo_t, a: *const obj_t, b: *const obj_t, c: *const obj_t, start: *mut dim_t, end: *mut dim_t) -> siz_t {
                dyload_lib().bli_thread_range_ndim.unwrap()(direct, bmult, use_weighted, thr, a, b, c, start, end)
            }

pub unsafe fn bli_thread_range(thr: *const thrinfo_t, a: *const obj_t, bf: dim_t, direct: dir_t, dim: mdim_t, use_weighted: bool, start: *mut dim_t, end: *mut dim_t) -> siz_t {
                dyload_lib().bli_thread_range.unwrap()(thr, a, bf, direct, dim, use_weighted, start, end)
            }

pub unsafe fn bli_thread_range_width_l(diagoff_j: doff_t, m: dim_t, n_j: dim_t, j: dim_t, n_way: dim_t, bf: dim_t, bf_left: dim_t, area_per_thr: f64, handle_edge_low: bool) -> dim_t {
                dyload_lib().bli_thread_range_width_l.unwrap()(diagoff_j, m, n_j, j, n_way, bf, bf_left, area_per_thr, handle_edge_low)
            }

pub unsafe fn bli_find_area_trap_l(diagoff: doff_t, m: dim_t, n: dim_t, bf: dim_t) -> siz_t {
                dyload_lib().bli_find_area_trap_l.unwrap()(diagoff, m, n, bf)
            }

pub unsafe fn bli_thread_range_weighted_sub(thread: *const thrinfo_t, diagoff: doff_t, uplo: uplo_t, uplo_orig: uplo_t, m: dim_t, n: dim_t, bf: dim_t, handle_edge_low: bool, j_start_thr: *mut dim_t, j_end_thr: *mut dim_t) -> siz_t {
                dyload_lib().bli_thread_range_weighted_sub.unwrap()(thread, diagoff, uplo, uplo_orig, m, n, bf, handle_edge_low, j_start_thr, j_end_thr)
            }

pub unsafe fn bli_thread_range_quad(thread: *const thrinfo_t, diagoff: doff_t, uplo: uplo_t, m: dim_t, n: dim_t, bf: dim_t, handle_edge_low: bool, start: *mut dim_t, end: *mut dim_t, inc: *mut dim_t) {
                dyload_lib().bli_thread_range_quad.unwrap()(thread, diagoff, uplo, m, n, bf, handle_edge_low, start, end, inc)
            }

pub unsafe fn bli_thread_range_tlb_l(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_l.unwrap()(nt, tid, diagoff, m_iter, n_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_u(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_u.unwrap()(nt, tid, diagoff, m_iter, n_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_d(nt: dim_t, tid: dim_t, m_iter: dim_t, n_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_d.unwrap()(nt, tid, m_iter, n_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_trmm_ll(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, k_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_trmm_ll.unwrap()(nt, tid, diagoff, m_iter, n_iter, k_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_trmm_lu(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, k_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_trmm_lu.unwrap()(nt, tid, diagoff, m_iter, n_iter, k_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_trmm_lx_impl(nt: dim_t, tid: dim_t, diagoff: doff_t, uplo: uplo_t, m_iter: dim_t, n_iter: dim_t, k_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_trmm_lx_impl.unwrap()(nt, tid, diagoff, uplo, m_iter, n_iter, k_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_trmm_rl(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, k_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_trmm_rl.unwrap()(nt, tid, diagoff, m_iter, n_iter, k_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_trmm_ru(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, k_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_trmm_ru.unwrap()(nt, tid, diagoff, m_iter, n_iter, k_iter, mr, nr, j_st_p, i_st_p)
            }

pub unsafe fn bli_thread_range_tlb_trmm_rl_impl(nt: dim_t, tid: dim_t, diagoff: doff_t, m_iter: dim_t, n_iter: dim_t, k_iter: dim_t, mr: dim_t, nr: dim_t, j_st_p: *mut inc_t, i_st_p: *mut inc_t, j_en_p: *mut inc_t, i_en_p: *mut inc_t) -> dim_t {
                dyload_lib().bli_thread_range_tlb_trmm_rl_impl.unwrap()(nt, tid, diagoff, m_iter, n_iter, k_iter, mr, nr, j_st_p, i_st_p, j_en_p, i_en_p)
            }

pub unsafe fn bli_init() {
                dyload_lib().bli_init.unwrap()()
            }

pub unsafe fn bli_finalize() {
                dyload_lib().bli_finalize.unwrap()()
            }

pub unsafe fn bli_init_auto() {
                dyload_lib().bli_init_auto.unwrap()()
            }

pub unsafe fn bli_finalize_auto() {
                dyload_lib().bli_finalize_auto.unwrap()()
            }

pub unsafe fn bli_init_once() {
                dyload_lib().bli_init_once.unwrap()()
            }

pub unsafe fn bli_finalize_once() {
                dyload_lib().bli_finalize_once.unwrap()()
            }

pub unsafe fn bli_init_apis() -> c_int {
                dyload_lib().bli_init_apis.unwrap()()
            }

pub unsafe fn bli_finalize_apis() -> c_int {
                dyload_lib().bli_finalize_apis.unwrap()()
            }

pub unsafe fn bli_malloc_intl(size: usize, r_val: *mut err_t) -> *mut c_void {
                dyload_lib().bli_malloc_intl.unwrap()(size, r_val)
            }

pub unsafe fn bli_calloc_intl(size: usize, r_val: *mut err_t) -> *mut c_void {
                dyload_lib().bli_calloc_intl.unwrap()(size, r_val)
            }

pub unsafe fn bli_free_intl(p: *mut c_void) {
                dyload_lib().bli_free_intl.unwrap()(p)
            }

pub unsafe fn bli_malloc_user(size: usize, r_val: *mut err_t) -> *mut c_void {
                dyload_lib().bli_malloc_user.unwrap()(size, r_val)
            }

pub unsafe fn bli_free_user(p: *mut c_void) {
                dyload_lib().bli_free_user.unwrap()(p)
            }

pub unsafe fn bli_fmalloc_align(f: malloc_ft, size: usize, align_size: usize, r_val: *mut err_t) -> *mut c_void {
                dyload_lib().bli_fmalloc_align.unwrap()(f, size, align_size, r_val)
            }

pub unsafe fn bli_ffree_align(f: free_ft, p: *mut c_void) {
                dyload_lib().bli_ffree_align.unwrap()(f, p)
            }

pub unsafe fn bli_fmalloc_noalign(f: malloc_ft, size: usize, r_val: *mut err_t) -> *mut c_void {
                dyload_lib().bli_fmalloc_noalign.unwrap()(f, size, r_val)
            }

pub unsafe fn bli_ffree_noalign(f: free_ft, p: *mut c_void) {
                dyload_lib().bli_ffree_noalign.unwrap()(f, p)
            }

pub unsafe fn bli_fmalloc_align_check(f: malloc_ft, size: usize, align_size: usize) {
                dyload_lib().bli_fmalloc_align_check.unwrap()(f, size, align_size)
            }

pub unsafe fn bli_fmalloc_post_check(p: *mut c_void) {
                dyload_lib().bli_fmalloc_post_check.unwrap()(p)
            }

pub unsafe fn bli_const_init() {
                dyload_lib().bli_const_init.unwrap()()
            }

pub unsafe fn bli_const_finalize() {
                dyload_lib().bli_const_finalize.unwrap()()
            }

pub unsafe fn bli_obj_create_check(dt: num_t, m: dim_t, n: dim_t, rs: inc_t, cs: inc_t, obj: *const obj_t) {
                dyload_lib().bli_obj_create_check.unwrap()(dt, m, n, rs, cs, obj)
            }

pub unsafe fn bli_obj_create_without_buffer_check(dt: num_t, m: dim_t, n: dim_t, obj: *const obj_t) {
                dyload_lib().bli_obj_create_without_buffer_check.unwrap()(dt, m, n, obj)
            }

pub unsafe fn bli_obj_alloc_buffer_check(rs: inc_t, cs: inc_t, is: inc_t, obj: *const obj_t) {
                dyload_lib().bli_obj_alloc_buffer_check.unwrap()(rs, cs, is, obj)
            }

pub unsafe fn bli_obj_attach_buffer_check(p: *const c_void, rs: inc_t, cs: inc_t, is: inc_t, obj: *const obj_t) {
                dyload_lib().bli_obj_attach_buffer_check.unwrap()(p, rs, cs, is, obj)
            }

pub unsafe fn bli_obj_create_scalar_check(dt: num_t, obj: *const obj_t) {
                dyload_lib().bli_obj_create_scalar_check.unwrap()(dt, obj)
            }

pub unsafe fn bli_obj_free_check(obj: *const obj_t) {
                dyload_lib().bli_obj_free_check.unwrap()(obj)
            }

pub unsafe fn bli_obj_create_const_check(value: f64, obj: *const obj_t) {
                dyload_lib().bli_obj_create_const_check.unwrap()(value, obj)
            }

pub unsafe fn bli_obj_create_const_copy_of_check(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_obj_create_const_copy_of_check.unwrap()(a, b)
            }

pub unsafe fn bli_dt_size_check(dt: num_t) {
                dyload_lib().bli_dt_size_check.unwrap()(dt)
            }

pub unsafe fn bli_dt_string_check(dt: num_t) {
                dyload_lib().bli_dt_string_check.unwrap()(dt)
            }

pub unsafe fn bli_dt_union_check(dt1: num_t, dt2: num_t) {
                dyload_lib().bli_dt_union_check.unwrap()(dt1, dt2)
            }

pub unsafe fn bli_obj_print_check(label: *const c_char, obj: *const obj_t) {
                dyload_lib().bli_obj_print_check.unwrap()(label, obj)
            }

pub unsafe fn bli_obj_create(dt: num_t, m: dim_t, n: dim_t, rs: inc_t, cs: inc_t, obj: *mut obj_t) {
                dyload_lib().bli_obj_create.unwrap()(dt, m, n, rs, cs, obj)
            }

pub unsafe fn bli_obj_create_with_attached_buffer(dt: num_t, m: dim_t, n: dim_t, p: *mut c_void, rs: inc_t, cs: inc_t, obj: *mut obj_t) {
                dyload_lib().bli_obj_create_with_attached_buffer.unwrap()(dt, m, n, p, rs, cs, obj)
            }

pub unsafe fn bli_obj_create_without_buffer(dt: num_t, m: dim_t, n: dim_t, obj: *mut obj_t) {
                dyload_lib().bli_obj_create_without_buffer.unwrap()(dt, m, n, obj)
            }

pub unsafe fn bli_obj_alloc_buffer(rs: inc_t, cs: inc_t, is: inc_t, obj: *mut obj_t) {
                dyload_lib().bli_obj_alloc_buffer.unwrap()(rs, cs, is, obj)
            }

pub unsafe fn bli_obj_attach_buffer(p: *mut c_void, rs: inc_t, cs: inc_t, is: inc_t, obj: *mut obj_t) {
                dyload_lib().bli_obj_attach_buffer.unwrap()(p, rs, cs, is, obj)
            }

pub unsafe fn bli_obj_create_1x1(dt: num_t, obj: *mut obj_t) {
                dyload_lib().bli_obj_create_1x1.unwrap()(dt, obj)
            }

pub unsafe fn bli_obj_create_1x1_with_attached_buffer(dt: num_t, p: *mut c_void, obj: *mut obj_t) {
                dyload_lib().bli_obj_create_1x1_with_attached_buffer.unwrap()(dt, p, obj)
            }

pub unsafe fn bli_obj_create_conf_to(s: *const obj_t, d: *mut obj_t) {
                dyload_lib().bli_obj_create_conf_to.unwrap()(s, d)
            }

pub unsafe fn bli_obj_free(obj: *mut obj_t) {
                dyload_lib().bli_obj_free.unwrap()(obj)
            }

pub unsafe fn bli_adjust_strides(m: dim_t, n: dim_t, elem_size: siz_t, rs: *mut inc_t, cs: *mut inc_t, is: *mut inc_t) {
                dyload_lib().bli_adjust_strides.unwrap()(m, n, elem_size, rs, cs, is)
            }

pub unsafe fn bli_dt_size(dt: num_t) -> siz_t {
                dyload_lib().bli_dt_size.unwrap()(dt)
            }

pub unsafe fn bli_dt_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_dt_string.unwrap()(dt)
            }

pub unsafe fn bli_align_dim_to_mult(dim: dim_t, dim_mult: dim_t, round_up: bool) -> dim_t {
                dyload_lib().bli_align_dim_to_mult.unwrap()(dim, dim_mult, round_up)
            }

pub unsafe fn bli_align_dim_to_size(dim: dim_t, elem_size: siz_t, align_size: siz_t) -> dim_t {
                dyload_lib().bli_align_dim_to_size.unwrap()(dim, elem_size, align_size)
            }

pub unsafe fn bli_align_ptr_to_size(p: *const c_void, align_size: usize) -> dim_t {
                dyload_lib().bli_align_ptr_to_size.unwrap()(p, align_size)
            }

pub unsafe fn bli_obj_print(label: *const c_char, obj: *const obj_t) {
                dyload_lib().bli_obj_print.unwrap()(label, obj)
            }

pub unsafe fn bli_obj_scalar_init_detached(dt: num_t, beta: *mut obj_t) {
                dyload_lib().bli_obj_scalar_init_detached.unwrap()(dt, beta)
            }

pub unsafe fn bli_obj_scalar_init_detached_copy_of(dt: num_t, conj: conj_t, alpha: *const obj_t, beta: *mut obj_t) {
                dyload_lib().bli_obj_scalar_init_detached_copy_of.unwrap()(dt, conj, alpha, beta)
            }

pub unsafe fn bli_obj_scalar_detach(a: *const obj_t, alpha: *mut obj_t) {
                dyload_lib().bli_obj_scalar_detach.unwrap()(a, alpha)
            }

pub unsafe fn bli_obj_scalar_attach(conj: conj_t, alpha: *const obj_t, a: *mut obj_t) {
                dyload_lib().bli_obj_scalar_attach.unwrap()(conj, alpha, a)
            }

pub unsafe fn bli_obj_scalar_cast_to(dt: num_t, a: *mut obj_t) {
                dyload_lib().bli_obj_scalar_cast_to.unwrap()(dt, a)
            }

pub unsafe fn bli_obj_scalar_apply_scalar(alpha: *const obj_t, a: *mut obj_t) {
                dyload_lib().bli_obj_scalar_apply_scalar.unwrap()(alpha, a)
            }

pub unsafe fn bli_obj_scalar_reset(a: *mut obj_t) {
                dyload_lib().bli_obj_scalar_reset.unwrap()(a)
            }

pub unsafe fn bli_obj_scalar_has_nonzero_imag(a: *mut obj_t) -> bool {
                dyload_lib().bli_obj_scalar_has_nonzero_imag.unwrap()(a)
            }

pub unsafe fn bli_obj_scalar_equals(a: *const obj_t, beta: *const obj_t) -> bool {
                dyload_lib().bli_obj_scalar_equals.unwrap()(a, beta)
            }

pub unsafe fn bli_blksz_create_ed(b_s: dim_t, be_s: dim_t, b_d: dim_t, be_d: dim_t, b_c: dim_t, be_c: dim_t, b_z: dim_t, be_z: dim_t) -> *mut blksz_t {
                dyload_lib().bli_blksz_create_ed.unwrap()(b_s, be_s, b_d, be_d, b_c, be_c, b_z, be_z)
            }

pub unsafe fn bli_blksz_create(b_s: dim_t, b_d: dim_t, b_c: dim_t, b_z: dim_t, be_s: dim_t, be_d: dim_t, be_c: dim_t, be_z: dim_t) -> *mut blksz_t {
                dyload_lib().bli_blksz_create.unwrap()(b_s, b_d, b_c, b_z, be_s, be_d, be_c, be_z)
            }

pub unsafe fn bli_blksz_init_ed(b: *mut blksz_t, b_s: dim_t, be_s: dim_t, b_d: dim_t, be_d: dim_t, b_c: dim_t, be_c: dim_t, b_z: dim_t, be_z: dim_t) {
                dyload_lib().bli_blksz_init_ed.unwrap()(b, b_s, be_s, b_d, be_d, b_c, be_c, b_z, be_z)
            }

pub unsafe fn bli_blksz_init(b: *mut blksz_t, b_s: dim_t, b_d: dim_t, b_c: dim_t, b_z: dim_t, be_s: dim_t, be_d: dim_t, be_c: dim_t, be_z: dim_t) {
                dyload_lib().bli_blksz_init.unwrap()(b, b_s, b_d, b_c, b_z, be_s, be_d, be_c, be_z)
            }

pub unsafe fn bli_blksz_init_easy(b: *mut blksz_t, b_s: dim_t, b_d: dim_t, b_c: dim_t, b_z: dim_t) {
                dyload_lib().bli_blksz_init_easy.unwrap()(b, b_s, b_d, b_c, b_z)
            }

pub unsafe fn bli_blksz_free(b: *mut blksz_t) {
                dyload_lib().bli_blksz_free.unwrap()(b)
            }

pub unsafe fn bli_blksz_reduce_def_to(dt_bm: num_t, bmult: *mut blksz_t, dt_bs: num_t, blksz: *mut blksz_t) {
                dyload_lib().bli_blksz_reduce_def_to.unwrap()(dt_bm, bmult, dt_bs, blksz)
            }

pub unsafe fn bli_blksz_reduce_max_to(dt_bm: num_t, bmult: *mut blksz_t, dt_bs: num_t, blksz: *mut blksz_t) {
                dyload_lib().bli_blksz_reduce_max_to.unwrap()(dt_bm, bmult, dt_bs, blksz)
            }

pub unsafe fn bli_determine_blocksize(direct: dir_t, i: dim_t, dim: dim_t, b_alg: dim_t, b_max: dim_t) -> dim_t {
                dyload_lib().bli_determine_blocksize.unwrap()(direct, i, dim, b_alg, b_max)
            }

pub unsafe fn bli_func_create(ptr_s: void_fp, ptr_d: void_fp, ptr_c: void_fp, ptr_z: void_fp) -> *mut func_t {
                dyload_lib().bli_func_create.unwrap()(ptr_s, ptr_d, ptr_c, ptr_z)
            }

pub unsafe fn bli_func_init(f: *mut func_t, ptr_s: void_fp, ptr_d: void_fp, ptr_c: void_fp, ptr_z: void_fp) {
                dyload_lib().bli_func_init.unwrap()(f, ptr_s, ptr_d, ptr_c, ptr_z)
            }

pub unsafe fn bli_func_init_null(f: *mut func_t) {
                dyload_lib().bli_func_init_null.unwrap()(f)
            }

pub unsafe fn bli_func_free(f: *mut func_t) {
                dyload_lib().bli_func_free.unwrap()(f)
            }

pub unsafe fn bli_func2_create(ptr_ss: void_fp, ptr_sd: void_fp, ptr_sc: void_fp, ptr_sz: void_fp, ptr_ds: void_fp, ptr_dd: void_fp, ptr_dc: void_fp, ptr_dz: void_fp, ptr_cs: void_fp, ptr_cd: void_fp, ptr_cc: void_fp, ptr_cz: void_fp, ptr_zs: void_fp, ptr_zd: void_fp, ptr_zc: void_fp, ptr_zz: void_fp) -> *mut func2_t {
                dyload_lib().bli_func2_create.unwrap()(ptr_ss, ptr_sd, ptr_sc, ptr_sz, ptr_ds, ptr_dd, ptr_dc, ptr_dz, ptr_cs, ptr_cd, ptr_cc, ptr_cz, ptr_zs, ptr_zd, ptr_zc, ptr_zz)
            }

pub unsafe fn bli_func2_init(f: *mut func2_t, ptr_ss: void_fp, ptr_sd: void_fp, ptr_sc: void_fp, ptr_sz: void_fp, ptr_ds: void_fp, ptr_dd: void_fp, ptr_dc: void_fp, ptr_dz: void_fp, ptr_cs: void_fp, ptr_cd: void_fp, ptr_cc: void_fp, ptr_cz: void_fp, ptr_zs: void_fp, ptr_zd: void_fp, ptr_zc: void_fp, ptr_zz: void_fp) {
                dyload_lib().bli_func2_init.unwrap()(f, ptr_ss, ptr_sd, ptr_sc, ptr_sz, ptr_ds, ptr_dd, ptr_dc, ptr_dz, ptr_cs, ptr_cd, ptr_cc, ptr_cz, ptr_zs, ptr_zd, ptr_zc, ptr_zz)
            }

pub unsafe fn bli_func2_init_null(f: *mut func2_t) {
                dyload_lib().bli_func2_init_null.unwrap()(f)
            }

pub unsafe fn bli_func2_free(f: *mut func2_t) {
                dyload_lib().bli_func2_free.unwrap()(f)
            }

pub unsafe fn bli_func_is_null_dt(dt: num_t, f: *const func_t) -> bool {
                dyload_lib().bli_func_is_null_dt.unwrap()(dt, f)
            }

pub unsafe fn bli_func_is_null(f: *const func_t) -> bool {
                dyload_lib().bli_func_is_null.unwrap()(f)
            }

pub unsafe fn bli_func2_is_null_dt(dt1: num_t, dt2: num_t, f: *const func2_t) -> bool {
                dyload_lib().bli_func2_is_null_dt.unwrap()(dt1, dt2, f)
            }

pub unsafe fn bli_func2_is_null(f: *const func2_t) -> bool {
                dyload_lib().bli_func2_is_null.unwrap()(f)
            }

pub unsafe fn bli_mbool_create(b_s: bool, b_d: bool, b_c: bool, b_z: bool) -> *mut mbool_t {
                dyload_lib().bli_mbool_create.unwrap()(b_s, b_d, b_c, b_z)
            }

pub unsafe fn bli_mbool_init(b: *mut mbool_t, b_s: bool, b_d: bool, b_c: bool, b_z: bool) {
                dyload_lib().bli_mbool_init.unwrap()(b, b_s, b_d, b_c, b_z)
            }

pub unsafe fn bli_mbool_free(b: *mut mbool_t) {
                dyload_lib().bli_mbool_free.unwrap()(b)
            }

pub unsafe fn bli_stack_init(elem_size: siz_t, block_len: siz_t, max_blocks: siz_t, initial_size: siz_t, stack: *mut stck_t) -> err_t {
                dyload_lib().bli_stack_init.unwrap()(elem_size, block_len, max_blocks, initial_size, stack)
            }

pub unsafe fn bli_stack_finalize(stack: *mut stck_t) -> err_t {
                dyload_lib().bli_stack_finalize.unwrap()(stack)
            }

pub unsafe fn bli_stack_get(i: siz_t, elem: *mut *mut c_void, stack: *const stck_t) -> err_t {
                dyload_lib().bli_stack_get.unwrap()(i, elem, stack)
            }

pub unsafe fn bli_stack_push(i: *mut siz_t, stack: *mut stck_t) -> err_t {
                dyload_lib().bli_stack_push.unwrap()(i, stack)
            }

pub unsafe fn bli_check_error_code_helper(code: gint_t, file: *const c_char, line: guint_t) -> err_t {
                dyload_lib().bli_check_error_code_helper.unwrap()(code, file, line)
            }

pub unsafe fn bli_check_valid_error_level(level: errlev_t) -> err_t {
                dyload_lib().bli_check_valid_error_level.unwrap()(level)
            }

pub unsafe fn bli_check_null_pointer(ptr: *const c_void) -> err_t {
                dyload_lib().bli_check_null_pointer.unwrap()(ptr)
            }

pub unsafe fn bli_check_valid_side(side: side_t) -> err_t {
                dyload_lib().bli_check_valid_side.unwrap()(side)
            }

pub unsafe fn bli_check_valid_uplo(uplo: uplo_t) -> err_t {
                dyload_lib().bli_check_valid_uplo.unwrap()(uplo)
            }

pub unsafe fn bli_check_valid_trans(trans: trans_t) -> err_t {
                dyload_lib().bli_check_valid_trans.unwrap()(trans)
            }

pub unsafe fn bli_check_valid_diag(diag: diag_t) -> err_t {
                dyload_lib().bli_check_valid_diag.unwrap()(diag)
            }

pub unsafe fn bli_check_nonunit_diag(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_nonunit_diag.unwrap()(a)
            }

pub unsafe fn bli_check_valid_datatype(dt: num_t) -> err_t {
                dyload_lib().bli_check_valid_datatype.unwrap()(dt)
            }

pub unsafe fn bli_check_object_valid_datatype(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_object_valid_datatype.unwrap()(a)
            }

pub unsafe fn bli_check_noninteger_datatype(dt: num_t) -> err_t {
                dyload_lib().bli_check_noninteger_datatype.unwrap()(dt)
            }

pub unsafe fn bli_check_noninteger_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_noninteger_object.unwrap()(a)
            }

pub unsafe fn bli_check_nonconstant_datatype(dt: num_t) -> err_t {
                dyload_lib().bli_check_nonconstant_datatype.unwrap()(dt)
            }

pub unsafe fn bli_check_nonconstant_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_nonconstant_object.unwrap()(a)
            }

pub unsafe fn bli_check_floating_datatype(dt: num_t) -> err_t {
                dyload_lib().bli_check_floating_datatype.unwrap()(dt)
            }

pub unsafe fn bli_check_floating_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_floating_object.unwrap()(a)
            }

pub unsafe fn bli_check_real_datatype(dt: num_t) -> err_t {
                dyload_lib().bli_check_real_datatype.unwrap()(dt)
            }

pub unsafe fn bli_check_real_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_real_object.unwrap()(a)
            }

pub unsafe fn bli_check_integer_datatype(dt: num_t) -> err_t {
                dyload_lib().bli_check_integer_datatype.unwrap()(dt)
            }

pub unsafe fn bli_check_integer_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_integer_object.unwrap()(a)
            }

pub unsafe fn bli_check_consistent_datatypes(dt_a: num_t, dt_b: num_t) -> err_t {
                dyload_lib().bli_check_consistent_datatypes.unwrap()(dt_a, dt_b)
            }

pub unsafe fn bli_check_consistent_object_datatypes(a: *const obj_t, b: *const obj_t) -> err_t {
                dyload_lib().bli_check_consistent_object_datatypes.unwrap()(a, b)
            }

pub unsafe fn bli_check_datatype_real_proj_of(dt_c: num_t, dt_r: num_t) -> err_t {
                dyload_lib().bli_check_datatype_real_proj_of.unwrap()(dt_c, dt_r)
            }

pub unsafe fn bli_check_object_real_proj_of(c: *const obj_t, r: *const obj_t) -> err_t {
                dyload_lib().bli_check_object_real_proj_of.unwrap()(c, r)
            }

pub unsafe fn bli_check_real_valued_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_real_valued_object.unwrap()(a)
            }

pub unsafe fn bli_check_consistent_precisions(dt_a: num_t, dt_b: num_t) -> err_t {
                dyload_lib().bli_check_consistent_precisions.unwrap()(dt_a, dt_b)
            }

pub unsafe fn bli_check_consistent_object_precisions(a: *const obj_t, b: *const obj_t) -> err_t {
                dyload_lib().bli_check_consistent_object_precisions.unwrap()(a, b)
            }

pub unsafe fn bli_check_conformal_dims(a: *const obj_t, b: *const obj_t) -> err_t {
                dyload_lib().bli_check_conformal_dims.unwrap()(a, b)
            }

pub unsafe fn bli_check_level3_dims(a: *const obj_t, b: *const obj_t, c: *const obj_t) -> err_t {
                dyload_lib().bli_check_level3_dims.unwrap()(a, b, c)
            }

pub unsafe fn bli_check_scalar_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_scalar_object.unwrap()(a)
            }

pub unsafe fn bli_check_vector_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_vector_object.unwrap()(a)
            }

pub unsafe fn bli_check_matrix_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_matrix_object.unwrap()(a)
            }

pub unsafe fn bli_check_equal_vector_lengths(x: *const obj_t, y: *const obj_t) -> err_t {
                dyload_lib().bli_check_equal_vector_lengths.unwrap()(x, y)
            }

pub unsafe fn bli_check_square_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_square_object.unwrap()(a)
            }

pub unsafe fn bli_check_object_length_equals(a: *const obj_t, m: dim_t) -> err_t {
                dyload_lib().bli_check_object_length_equals.unwrap()(a, m)
            }

pub unsafe fn bli_check_object_width_equals(a: *const obj_t, n: dim_t) -> err_t {
                dyload_lib().bli_check_object_width_equals.unwrap()(a, n)
            }

pub unsafe fn bli_check_vector_dim_equals(a: *const obj_t, n: dim_t) -> err_t {
                dyload_lib().bli_check_vector_dim_equals.unwrap()(a, n)
            }

pub unsafe fn bli_check_object_diag_offset_equals(a: *const obj_t, offset: doff_t) -> err_t {
                dyload_lib().bli_check_object_diag_offset_equals.unwrap()(a, offset)
            }

pub unsafe fn bli_check_matrix_strides(m: dim_t, n: dim_t, rs: inc_t, cs: inc_t, is: inc_t) -> err_t {
                dyload_lib().bli_check_matrix_strides.unwrap()(m, n, rs, cs, is)
            }

pub unsafe fn bli_check_general_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_general_object.unwrap()(a)
            }

pub unsafe fn bli_check_hermitian_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_hermitian_object.unwrap()(a)
            }

pub unsafe fn bli_check_symmetric_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_symmetric_object.unwrap()(a)
            }

pub unsafe fn bli_check_triangular_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_triangular_object.unwrap()(a)
            }

pub unsafe fn bli_check_object_struc(a: *const obj_t, struc: struc_t) -> err_t {
                dyload_lib().bli_check_object_struc.unwrap()(a, struc)
            }

pub unsafe fn bli_check_upper_or_lower_object(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_upper_or_lower_object.unwrap()(a)
            }

pub unsafe fn bli_check_valid_3x1_subpart(part: subpart_t) -> err_t {
                dyload_lib().bli_check_valid_3x1_subpart.unwrap()(part)
            }

pub unsafe fn bli_check_valid_1x3_subpart(part: subpart_t) -> err_t {
                dyload_lib().bli_check_valid_1x3_subpart.unwrap()(part)
            }

pub unsafe fn bli_check_valid_3x3_subpart(part: subpart_t) -> err_t {
                dyload_lib().bli_check_valid_3x3_subpart.unwrap()(part)
            }

pub unsafe fn bli_check_valid_cntl(cntl: *const c_void) -> err_t {
                dyload_lib().bli_check_valid_cntl.unwrap()(cntl)
            }

pub unsafe fn bli_check_packm_schema_on_unpack(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_packm_schema_on_unpack.unwrap()(a)
            }

pub unsafe fn bli_check_packv_schema_on_unpack(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_packv_schema_on_unpack.unwrap()(a)
            }

pub unsafe fn bli_check_object_buffer(a: *const obj_t) -> err_t {
                dyload_lib().bli_check_object_buffer.unwrap()(a)
            }

pub unsafe fn bli_check_valid_malloc_buf(ptr: *const c_void) -> err_t {
                dyload_lib().bli_check_valid_malloc_buf.unwrap()(ptr)
            }

pub unsafe fn bli_check_valid_packbuf(buf_type: packbuf_t) -> err_t {
                dyload_lib().bli_check_valid_packbuf.unwrap()(buf_type)
            }

pub unsafe fn bli_check_if_exhausted_pool(pool: *const pool_t) -> err_t {
                dyload_lib().bli_check_if_exhausted_pool.unwrap()(pool)
            }

pub unsafe fn bli_check_sufficient_stack_buf_size(cntx: *const cntx_t) -> err_t {
                dyload_lib().bli_check_sufficient_stack_buf_size.unwrap()(cntx)
            }

pub unsafe fn bli_check_alignment_is_power_of_two(align_size: usize) -> err_t {
                dyload_lib().bli_check_alignment_is_power_of_two.unwrap()(align_size)
            }

pub unsafe fn bli_check_alignment_is_mult_of_ptr_size(align_size: usize) -> err_t {
                dyload_lib().bli_check_alignment_is_mult_of_ptr_size.unwrap()(align_size)
            }

pub unsafe fn bli_check_object_alias_of(a: *const obj_t, b: *const obj_t) -> err_t {
                dyload_lib().bli_check_object_alias_of.unwrap()(a, b)
            }

pub unsafe fn bli_check_valid_arch_id(id: arch_t) -> err_t {
                dyload_lib().bli_check_valid_arch_id.unwrap()(id)
            }

pub unsafe fn bli_check_initialized_gks_cntx(cntx: *const cntx_t) -> err_t {
                dyload_lib().bli_check_initialized_gks_cntx.unwrap()(cntx)
            }

pub unsafe fn bli_check_valid_mc_mod_mult(mc: *const blksz_t, mr: *const blksz_t) -> err_t {
                dyload_lib().bli_check_valid_mc_mod_mult.unwrap()(mc, mr)
            }

pub unsafe fn bli_check_valid_nc_mod_mult(nc: *const blksz_t, nr: *const blksz_t) -> err_t {
                dyload_lib().bli_check_valid_nc_mod_mult.unwrap()(nc, nr)
            }

pub unsafe fn bli_check_valid_kc_mod_mult(kc: *const blksz_t, kr: *const blksz_t) -> err_t {
                dyload_lib().bli_check_valid_kc_mod_mult.unwrap()(kc, kr)
            }

pub unsafe fn bli_check_valid_mr_even(mr: *const blksz_t, row_pref: *const mbool_t) -> err_t {
                dyload_lib().bli_check_valid_mr_even.unwrap()(mr, row_pref)
            }

pub unsafe fn bli_check_valid_nr_even(mr: *const blksz_t, row_pref: *const mbool_t) -> err_t {
                dyload_lib().bli_check_valid_nr_even.unwrap()(mr, row_pref)
            }

pub unsafe fn bli_cntx_init(cntx: *mut cntx_t) -> err_t {
                dyload_lib().bli_cntx_init.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_free(cntx: *mut cntx_t) -> err_t {
                dyload_lib().bli_cntx_free.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_set_blkszs(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_set_blkszs.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_set_ukrs(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_set_ukrs.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_set_ukr2s(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_set_ukr2s.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_set_ukr_prefs(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_set_ukr_prefs.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_print(cntx: *const cntx_t) {
                dyload_lib().bli_cntx_print.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_set_l3_sup_handlers(cntx: *mut cntx_t) {
                dyload_lib().bli_cntx_set_l3_sup_handlers.unwrap()(cntx)
            }

pub unsafe fn bli_cntx_register_blksz(bs_id: *mut kerid_t, blksz: *const blksz_t, bmult_id: kerid_t, cntx: *mut cntx_t) -> err_t {
                dyload_lib().bli_cntx_register_blksz.unwrap()(bs_id, blksz, bmult_id, cntx)
            }

pub unsafe fn bli_cntx_register_ukr(ukr_id: *mut kerid_t, ukr: *const func_t, cntx: *mut cntx_t) -> err_t {
                dyload_lib().bli_cntx_register_ukr.unwrap()(ukr_id, ukr, cntx)
            }

pub unsafe fn bli_cntx_register_ukr2(ukr_id: *mut kerid_t, ukr: *const func2_t, cntx: *mut cntx_t) -> err_t {
                dyload_lib().bli_cntx_register_ukr2.unwrap()(ukr_id, ukr, cntx)
            }

pub unsafe fn bli_cntx_register_ukr_pref(ukr_pref_id: *mut kerid_t, ukr_pref: *const mbool_t, cntx: *mut cntx_t) -> err_t {
                dyload_lib().bli_cntx_register_ukr_pref.unwrap()(ukr_pref_id, ukr_pref, cntx)
            }

pub unsafe fn bli_global_rntm() -> *mut rntm_t {
                dyload_lib().bli_global_rntm.unwrap()()
            }

pub unsafe fn bli_global_rntm_at_init() -> *mut rntm_t {
                dyload_lib().bli_global_rntm_at_init.unwrap()()
            }

pub unsafe fn bli_global_rntm_mutex() -> *mut bli_pthread_mutex_t {
                dyload_lib().bli_global_rntm_mutex.unwrap()()
            }

pub unsafe fn bli_rntm_init() -> c_int {
                dyload_lib().bli_rntm_init.unwrap()()
            }

pub unsafe fn bli_rntm_finalize() -> c_int {
                dyload_lib().bli_rntm_finalize.unwrap()()
            }

pub unsafe fn bli_rntm_init_from_env(rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_init_from_env.unwrap()(rntm)
            }

pub unsafe fn bli_rntm_init_from_global(rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_init_from_global.unwrap()(rntm)
            }

pub unsafe fn bli_rntm_set_num_threads(nt: dim_t, rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_set_num_threads.unwrap()(nt, rntm)
            }

pub unsafe fn bli_rntm_set_ways(jc: dim_t, pc: dim_t, ic: dim_t, jr: dim_t, ir: dim_t, rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_set_ways.unwrap()(jc, pc, ic, jr, ir, rntm)
            }

pub unsafe fn bli_rntm_sanitize(rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_sanitize.unwrap()(rntm)
            }

pub unsafe fn bli_rntm_factorize(m: dim_t, n: dim_t, k: dim_t, rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_factorize.unwrap()(m, n, k, rntm)
            }

pub unsafe fn bli_rntm_factorize_sup(m: dim_t, n: dim_t, k: dim_t, rntm: *mut rntm_t) {
                dyload_lib().bli_rntm_factorize_sup.unwrap()(m, n, k, rntm)
            }

pub unsafe fn bli_rntm_print(rntm: *const rntm_t) {
                dyload_lib().bli_rntm_print.unwrap()(rntm)
            }

pub unsafe fn bli_rntm_calc_num_threads_in(bszid_cur: *const bszid_t, rntm: *const rntm_t) -> dim_t {
                dyload_lib().bli_rntm_calc_num_threads_in.unwrap()(bszid_cur, rntm)
            }

pub unsafe fn bli_gks_init() -> c_int {
                dyload_lib().bli_gks_init.unwrap()()
            }

pub unsafe fn bli_gks_finalize() -> c_int {
                dyload_lib().bli_gks_finalize.unwrap()()
            }

pub unsafe fn bli_gks_init_index() {
                dyload_lib().bli_gks_init_index.unwrap()()
            }

pub unsafe fn bli_gks_lookup_id(id: arch_t) -> *const cntx_t {
                dyload_lib().bli_gks_lookup_id.unwrap()(id)
            }

pub unsafe fn bli_gks_register_cntx(id: arch_t, nat_fp: void_fp, ref_fp: void_fp) {
                dyload_lib().bli_gks_register_cntx.unwrap()(id, nat_fp, ref_fp)
            }

pub unsafe fn bli_gks_query_cntx() -> *const cntx_t {
                dyload_lib().bli_gks_query_cntx.unwrap()()
            }

pub unsafe fn bli_gks_query_cntx_noinit() -> *const cntx_t {
                dyload_lib().bli_gks_query_cntx_noinit.unwrap()()
            }

pub unsafe fn bli_gks_query_cntx_impl() -> *const cntx_t {
                dyload_lib().bli_gks_query_cntx_impl.unwrap()()
            }

pub unsafe fn bli_gks_init_ref_cntx(cntx: *mut cntx_t) {
                dyload_lib().bli_gks_init_ref_cntx.unwrap()(cntx)
            }

pub unsafe fn bli_gks_cntx_ukr_is_ref(dt: num_t, ukr_id: ukr_t, cntx: *const cntx_t) -> bool {
                dyload_lib().bli_gks_cntx_ukr_is_ref.unwrap()(dt, ukr_id, cntx)
            }

pub unsafe fn bli_gks_cntx_ukr2_is_ref(dt1: num_t, dt2: num_t, ukr_id: ukr_t, cntx: *const cntx_t) -> bool {
                dyload_lib().bli_gks_cntx_ukr2_is_ref.unwrap()(dt1, dt2, ukr_id, cntx)
            }

pub unsafe fn bli_gks_l3_ukr_impl_string(ukr: ukr_t, method: ind_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_gks_l3_ukr_impl_string.unwrap()(ukr, method, dt)
            }

pub unsafe fn bli_gks_l3_ukr_impl_type(ukr: ukr_t, method: ind_t, dt: num_t) -> kimpl_t {
                dyload_lib().bli_gks_l3_ukr_impl_type.unwrap()(ukr, method, dt)
            }

pub unsafe fn bli_gks_register_blksz(bs_id: *mut kerid_t) -> err_t {
                dyload_lib().bli_gks_register_blksz.unwrap()(bs_id)
            }

pub unsafe fn bli_gks_register_ukr(ukr_id: *mut kerid_t) -> err_t {
                dyload_lib().bli_gks_register_ukr.unwrap()(ukr_id)
            }

pub unsafe fn bli_gks_register_ukr2(ukr_id: *mut kerid_t) -> err_t {
                dyload_lib().bli_gks_register_ukr2.unwrap()(ukr_id)
            }

pub unsafe fn bli_gks_register_ukr_pref(ukr_pref_id: *mut kerid_t) -> err_t {
                dyload_lib().bli_gks_register_ukr_pref.unwrap()(ukr_pref_id)
            }

pub unsafe fn bli_gemmind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_gemmind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_gemmtind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_gemmtind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_hemmind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_hemmind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_symmind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_symmind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_trmm3ind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_trmm3ind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_trmmind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_trmmind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_trsmind_find_avail(dt: num_t) -> ind_t {
                dyload_lib().bli_trsmind_find_avail.unwrap()(dt)
            }

pub unsafe fn bli_l3_ind_oper_find_avail(oper: opid_t, dt: num_t) -> ind_t {
                dyload_lib().bli_l3_ind_oper_find_avail.unwrap()(oper, dt)
            }

pub unsafe fn bli_l3_ind_set_enable_dt(method: ind_t, dt: num_t, status: bool) {
                dyload_lib().bli_l3_ind_set_enable_dt.unwrap()(method, dt, status)
            }

pub unsafe fn bli_l3_ind_oper_enable_only(oper: opid_t, method: ind_t, dt: num_t) {
                dyload_lib().bli_l3_ind_oper_enable_only.unwrap()(oper, method, dt)
            }

pub unsafe fn bli_l3_ind_oper_set_enable_all(oper: opid_t, dt: num_t, status: bool) {
                dyload_lib().bli_l3_ind_oper_set_enable_all.unwrap()(oper, dt, status)
            }

pub unsafe fn bli_l3_ind_oper_set_enable(oper: opid_t, method: ind_t, dt: num_t, status: bool) {
                dyload_lib().bli_l3_ind_oper_set_enable.unwrap()(oper, method, dt, status)
            }

pub unsafe fn bli_l3_ind_oper_get_enable(oper: opid_t, method: ind_t, dt: num_t) -> bool {
                dyload_lib().bli_l3_ind_oper_get_enable.unwrap()(oper, method, dt)
            }

pub unsafe fn bli_l3_ind_oper_is_impl(oper: opid_t, method: ind_t) -> bool {
                dyload_lib().bli_l3_ind_oper_is_impl.unwrap()(oper, method)
            }

pub unsafe fn bli_ind_init() -> c_int {
                dyload_lib().bli_ind_init.unwrap()()
            }

pub unsafe fn bli_ind_finalize() -> c_int {
                dyload_lib().bli_ind_finalize.unwrap()()
            }

pub unsafe fn bli_ind_enable(method: ind_t) {
                dyload_lib().bli_ind_enable.unwrap()(method)
            }

pub unsafe fn bli_ind_disable(method: ind_t) {
                dyload_lib().bli_ind_disable.unwrap()(method)
            }

pub unsafe fn bli_ind_disable_all() {
                dyload_lib().bli_ind_disable_all.unwrap()()
            }

pub unsafe fn bli_ind_enable_dt(method: ind_t, dt: num_t) {
                dyload_lib().bli_ind_enable_dt.unwrap()(method, dt)
            }

pub unsafe fn bli_ind_disable_dt(method: ind_t, dt: num_t) {
                dyload_lib().bli_ind_disable_dt.unwrap()(method, dt)
            }

pub unsafe fn bli_ind_disable_all_dt(dt: num_t) {
                dyload_lib().bli_ind_disable_all_dt.unwrap()(dt)
            }

pub unsafe fn bli_ind_oper_enable_only(oper: opid_t, method: ind_t, dt: num_t) {
                dyload_lib().bli_ind_oper_enable_only.unwrap()(oper, method, dt)
            }

pub unsafe fn bli_ind_oper_is_impl(oper: opid_t, method: ind_t) -> bool {
                dyload_lib().bli_ind_oper_is_impl.unwrap()(oper, method)
            }

pub unsafe fn bli_ind_oper_find_avail(oper: opid_t, dt: num_t) -> ind_t {
                dyload_lib().bli_ind_oper_find_avail.unwrap()(oper, dt)
            }

pub unsafe fn bli_ind_oper_get_avail_impl_string(oper: opid_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_ind_oper_get_avail_impl_string.unwrap()(oper, dt)
            }

pub unsafe fn bli_ind_get_impl_string(method: ind_t) -> *const c_char {
                dyload_lib().bli_ind_get_impl_string.unwrap()(method)
            }

pub unsafe fn bli_ind_map_cdt_to_index(dt: num_t) -> num_t {
                dyload_lib().bli_ind_map_cdt_to_index.unwrap()(dt)
            }

pub unsafe fn bli_pba_query() -> *mut pba_t {
                dyload_lib().bli_pba_query.unwrap()()
            }

pub unsafe fn bli_pba_init(cntx: *const cntx_t) {
                dyload_lib().bli_pba_init.unwrap()(cntx)
            }

pub unsafe fn bli_pba_finalize() {
                dyload_lib().bli_pba_finalize.unwrap()()
            }

pub unsafe fn bli_pba_acquire_m(pba: *mut pba_t, req_size: siz_t, buf_type: packbuf_t, mem: *mut mem_t) {
                dyload_lib().bli_pba_acquire_m.unwrap()(pba, req_size, buf_type, mem)
            }

pub unsafe fn bli_pba_release(pba: *mut pba_t, mem: *mut mem_t) {
                dyload_lib().bli_pba_release.unwrap()(pba, mem)
            }

pub unsafe fn bli_pba_pool_size(pba: *const pba_t, buf_type: packbuf_t) -> siz_t {
                dyload_lib().bli_pba_pool_size.unwrap()(pba, buf_type)
            }

pub unsafe fn bli_pba_init_pools(cntx: *const cntx_t, pba: *mut pba_t) {
                dyload_lib().bli_pba_init_pools.unwrap()(cntx, pba)
            }

pub unsafe fn bli_pba_finalize_pools(pba: *mut pba_t) {
                dyload_lib().bli_pba_finalize_pools.unwrap()(pba)
            }

pub unsafe fn bli_pba_compute_pool_block_sizes(bs_a: *mut siz_t, bs_b: *mut siz_t, bs_c: *mut siz_t, cntx: *const cntx_t) {
                dyload_lib().bli_pba_compute_pool_block_sizes.unwrap()(bs_a, bs_b, bs_c, cntx)
            }

pub unsafe fn bli_pba_compute_pool_block_sizes_dt(dt: num_t, bs_a: *mut siz_t, bs_b: *mut siz_t, bs_c: *mut siz_t, cntx: *const cntx_t) {
                dyload_lib().bli_pba_compute_pool_block_sizes_dt.unwrap()(dt, bs_a, bs_b, bs_c, cntx)
            }

pub unsafe fn bli_pool_init(num_blocks: siz_t, block_ptrs_len: siz_t, block_size: siz_t, align_size: siz_t, offset_size: siz_t, malloc_fp: malloc_ft, free_fp: free_ft, pool: *mut pool_t) {
                dyload_lib().bli_pool_init.unwrap()(num_blocks, block_ptrs_len, block_size, align_size, offset_size, malloc_fp, free_fp, pool)
            }

pub unsafe fn bli_pool_finalize(pool: *mut pool_t, reinit: bool) {
                dyload_lib().bli_pool_finalize.unwrap()(pool, reinit)
            }

pub unsafe fn bli_pool_reinit(num_blocks_new: siz_t, block_ptrs_len_new: siz_t, block_size_new: siz_t, align_size_new: siz_t, offset_size_new: siz_t, pool: *mut pool_t) {
                dyload_lib().bli_pool_reinit.unwrap()(num_blocks_new, block_ptrs_len_new, block_size_new, align_size_new, offset_size_new, pool)
            }

pub unsafe fn bli_pool_checkout_block(req_size: siz_t, block: *mut pblk_t, pool: *mut pool_t) {
                dyload_lib().bli_pool_checkout_block.unwrap()(req_size, block, pool)
            }

pub unsafe fn bli_pool_checkin_block(block: *mut pblk_t, pool: *mut pool_t) {
                dyload_lib().bli_pool_checkin_block.unwrap()(block, pool)
            }

pub unsafe fn bli_pool_grow(num_blocks_add: siz_t, pool: *mut pool_t) {
                dyload_lib().bli_pool_grow.unwrap()(num_blocks_add, pool)
            }

pub unsafe fn bli_pool_shrink(num_blocks_sub: siz_t, pool: *mut pool_t) {
                dyload_lib().bli_pool_shrink.unwrap()(num_blocks_sub, pool)
            }

pub unsafe fn bli_pool_alloc_block(block_size: siz_t, align_size: siz_t, offset_size: siz_t, malloc_fp: malloc_ft, block: *mut pblk_t) {
                dyload_lib().bli_pool_alloc_block.unwrap()(block_size, align_size, offset_size, malloc_fp, block)
            }

pub unsafe fn bli_pool_free_block(offset_size: siz_t, free_fp: free_ft, block: *mut pblk_t) {
                dyload_lib().bli_pool_free_block.unwrap()(offset_size, free_fp, block)
            }

pub unsafe fn bli_pool_print(pool: *const pool_t) {
                dyload_lib().bli_pool_print.unwrap()(pool)
            }

pub unsafe fn bli_pblk_print(pblk: *const pblk_t) {
                dyload_lib().bli_pblk_print.unwrap()(pblk)
            }

pub unsafe fn bli_array_init(num_elem: siz_t, elem_size: siz_t, array: *mut array_t) {
                dyload_lib().bli_array_init.unwrap()(num_elem, elem_size, array)
            }

pub unsafe fn bli_array_resize(num_elem_new: siz_t, array: *mut array_t) {
                dyload_lib().bli_array_resize.unwrap()(num_elem_new, array)
            }

pub unsafe fn bli_array_finalize(array: *mut array_t) {
                dyload_lib().bli_array_finalize.unwrap()(array)
            }

pub unsafe fn bli_array_elem(index: siz_t, array: *const array_t) -> *mut c_void {
                dyload_lib().bli_array_elem.unwrap()(index, array)
            }

pub unsafe fn bli_array_set_elem(elem: *mut c_void, index: siz_t, array: *mut array_t) {
                dyload_lib().bli_array_set_elem.unwrap()(elem, index, array)
            }

pub unsafe fn bli_apool_init(apool: *mut apool_t) {
                dyload_lib().bli_apool_init.unwrap()(apool)
            }

pub unsafe fn bli_apool_finalize(apool: *mut apool_t) {
                dyload_lib().bli_apool_finalize.unwrap()(apool)
            }

pub unsafe fn bli_apool_checkout_array(n_threads: siz_t, apool: *mut apool_t) -> *mut array_t {
                dyload_lib().bli_apool_checkout_array.unwrap()(n_threads, apool)
            }

pub unsafe fn bli_apool_checkin_array(array: *mut array_t, apool: *mut apool_t) {
                dyload_lib().bli_apool_checkin_array.unwrap()(array, apool)
            }

pub unsafe fn bli_apool_array_elem(index: siz_t, array: *mut array_t) -> *mut pool_t {
                dyload_lib().bli_apool_array_elem.unwrap()(index, array)
            }

pub unsafe fn bli_apool_grow(num_blocks_add: siz_t, apool: *mut apool_t) {
                dyload_lib().bli_apool_grow.unwrap()(num_blocks_add, apool)
            }

pub unsafe fn bli_apool_alloc_block(num_elem: siz_t, array_p: *mut *mut array_t) {
                dyload_lib().bli_apool_alloc_block.unwrap()(num_elem, array_p)
            }

pub unsafe fn bli_apool_free_block(array: *mut array_t) {
                dyload_lib().bli_apool_free_block.unwrap()(array)
            }

pub unsafe fn bli_sba_query() -> *mut apool_t {
                dyload_lib().bli_sba_query.unwrap()()
            }

pub unsafe fn bli_sba_init() {
                dyload_lib().bli_sba_init.unwrap()()
            }

pub unsafe fn bli_sba_finalize() {
                dyload_lib().bli_sba_finalize.unwrap()()
            }

pub unsafe fn bli_sba_acquire(sba_pool: *mut pool_t, req_size: siz_t) -> *mut c_void {
                dyload_lib().bli_sba_acquire.unwrap()(sba_pool, req_size)
            }

pub unsafe fn bli_sba_release(sba_pool: *mut pool_t, block: *mut c_void) {
                dyload_lib().bli_sba_release.unwrap()(sba_pool, block)
            }

pub unsafe fn bli_sba_checkout_array(n_threads: siz_t) -> *mut array_t {
                dyload_lib().bli_sba_checkout_array.unwrap()(n_threads)
            }

pub unsafe fn bli_sba_checkin_array(array: *mut array_t) {
                dyload_lib().bli_sba_checkin_array.unwrap()(array)
            }

pub unsafe fn bli_sba_array_elem(index: siz_t, array: *mut array_t) -> *mut pool_t {
                dyload_lib().bli_sba_array_elem.unwrap()(index, array)
            }

pub unsafe fn bli_memsys_init() -> c_int {
                dyload_lib().bli_memsys_init.unwrap()()
            }

pub unsafe fn bli_memsys_finalize() -> c_int {
                dyload_lib().bli_memsys_finalize.unwrap()()
            }

pub unsafe fn bli_acquire_mpart_t2b_check(requested_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *const obj_t) {
                dyload_lib().bli_acquire_mpart_t2b_check.unwrap()(requested_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_l2r_check(requested_part: subpart_t, j: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *const obj_t) {
                dyload_lib().bli_acquire_mpart_l2r_check.unwrap()(requested_part, j, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_tl2br_check(requested_part: subpart_t, ij: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *const obj_t) {
                dyload_lib().bli_acquire_mpart_tl2br_check.unwrap()(requested_part, ij, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart(i: dim_t, j: dim_t, m: dim_t, n: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart.unwrap()(i, j, m, n, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_t2b(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_t2b.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_b2t(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_b2t.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_l2r(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_l2r.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_r2l(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_r2l.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_tl2br(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_tl2br.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_br2tl(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_br2tl.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_mdim(direct: dir_t, req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_mdim.unwrap()(direct, req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_ndim(direct: dir_t, req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_ndim.unwrap()(direct, req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mpart_mndim(direct: dir_t, req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mpart_mndim.unwrap()(direct, req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_vpart_f2b(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_vpart_f2b.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_vpart_b2f(req_part: subpart_t, i: dim_t, b: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_vpart_b2f.unwrap()(req_part, i, b, obj, sub_obj)
            }

pub unsafe fn bli_acquire_mij(i: dim_t, j: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_mij.unwrap()(i, j, obj, sub_obj)
            }

pub unsafe fn bli_acquire_vi(i: dim_t, obj: *const obj_t, sub_obj: *mut obj_t) {
                dyload_lib().bli_acquire_vi.unwrap()(i, obj, sub_obj)
            }

pub unsafe fn bli_part_cntl_init_node(var_func: void_fp, b_dt: num_t, b_alg: dim_t, b_max: dim_t, b_scale: dim_t, b_mult: dim_t, b_mult_scale: dim_t, direct: dir_t, use_weighted: bool, cntl: *mut part_cntl_t) {
                dyload_lib().bli_part_cntl_init_node.unwrap()(var_func, b_dt, b_alg, b_max, b_scale, b_mult, b_mult_scale, direct, use_weighted, cntl)
            }

pub unsafe fn bli_prune_unref_mparts(p: *mut obj_t, mdim_p: mdim_t, s: *mut obj_t, mdim_s: mdim_t) {
                dyload_lib().bli_prune_unref_mparts.unwrap()(p, mdim_p, s, mdim_s)
            }

pub unsafe fn bli_obj_equals(a: *const obj_t, b: *const obj_t) -> bool {
                dyload_lib().bli_obj_equals.unwrap()(a, b)
            }

pub unsafe fn bli_obj_imag_equals(a: *const obj_t, b: *const obj_t) -> bool {
                dyload_lib().bli_obj_imag_equals.unwrap()(a, b)
            }

pub unsafe fn bli_obj_imag_is_zero(a: *const obj_t) -> bool {
                dyload_lib().bli_obj_imag_is_zero.unwrap()(a)
            }

pub unsafe fn bli_param_map_blis_to_netlib_side(side: side_t, blas_side: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_netlib_side.unwrap()(side, blas_side)
            }

pub unsafe fn bli_param_map_blis_to_netlib_uplo(uplo: uplo_t, blas_uplo: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_netlib_uplo.unwrap()(uplo, blas_uplo)
            }

pub unsafe fn bli_param_map_blis_to_netlib_trans(trans: trans_t, blas_trans: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_netlib_trans.unwrap()(trans, blas_trans)
            }

pub unsafe fn bli_param_map_blis_to_netlib_diag(diag: diag_t, blas_diag: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_netlib_diag.unwrap()(diag, blas_diag)
            }

pub unsafe fn bli_param_map_blis_to_netlib_machval(machval: machval_t, blas_machval: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_netlib_machval.unwrap()(machval, blas_machval)
            }

pub unsafe fn bli_param_map_char_to_blis_side(side: c_char, blis_side: *mut side_t) {
                dyload_lib().bli_param_map_char_to_blis_side.unwrap()(side, blis_side)
            }

pub unsafe fn bli_param_map_char_to_blis_uplo(uplo: c_char, blis_uplo: *mut uplo_t) {
                dyload_lib().bli_param_map_char_to_blis_uplo.unwrap()(uplo, blis_uplo)
            }

pub unsafe fn bli_param_map_char_to_blis_trans(trans: c_char, blis_trans: *mut trans_t) {
                dyload_lib().bli_param_map_char_to_blis_trans.unwrap()(trans, blis_trans)
            }

pub unsafe fn bli_param_map_char_to_blis_conj(conj: c_char, blis_conj: *mut conj_t) {
                dyload_lib().bli_param_map_char_to_blis_conj.unwrap()(conj, blis_conj)
            }

pub unsafe fn bli_param_map_char_to_blis_diag(diag: c_char, blis_diag: *mut diag_t) {
                dyload_lib().bli_param_map_char_to_blis_diag.unwrap()(diag, blis_diag)
            }

pub unsafe fn bli_param_map_char_to_blis_dt(dt: c_char, blis_dt: *mut num_t) {
                dyload_lib().bli_param_map_char_to_blis_dt.unwrap()(dt, blis_dt)
            }

pub unsafe fn bli_param_map_blis_to_char_side(blis_side: side_t, side: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_char_side.unwrap()(blis_side, side)
            }

pub unsafe fn bli_param_map_blis_to_char_uplo(blis_uplo: uplo_t, uplo: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_char_uplo.unwrap()(blis_uplo, uplo)
            }

pub unsafe fn bli_param_map_blis_to_char_trans(blis_trans: trans_t, trans: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_char_trans.unwrap()(blis_trans, trans)
            }

pub unsafe fn bli_param_map_blis_to_char_conj(blis_conj: conj_t, conj: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_char_conj.unwrap()(blis_conj, conj)
            }

pub unsafe fn bli_param_map_blis_to_char_diag(blis_diag: diag_t, diag: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_char_diag.unwrap()(blis_diag, diag)
            }

pub unsafe fn bli_param_map_blis_to_char_dt(blis_dt: num_t, dt: *mut c_char) {
                dyload_lib().bli_param_map_blis_to_char_dt.unwrap()(blis_dt, dt)
            }

pub unsafe fn bli_clock() -> f64 {
                dyload_lib().bli_clock.unwrap()()
            }

pub unsafe fn bli_clock_min_diff(time_min: f64, time_start: f64) -> f64 {
                dyload_lib().bli_clock_min_diff.unwrap()(time_min, time_start)
            }

pub unsafe fn bli_clock_helper() -> f64 {
                dyload_lib().bli_clock_helper.unwrap()()
            }

pub unsafe fn bli_error_checking_level() -> errlev_t {
                dyload_lib().bli_error_checking_level.unwrap()()
            }

pub unsafe fn bli_error_checking_level_set(new_level: errlev_t) {
                dyload_lib().bli_error_checking_level_set.unwrap()(new_level)
            }

pub unsafe fn bli_error_checking_is_enabled() -> bool {
                dyload_lib().bli_error_checking_is_enabled.unwrap()()
            }

pub unsafe fn bli_print_msg(str_: *const c_char, file: *const c_char, line: guint_t) {
                dyload_lib().bli_print_msg.unwrap()(str_, file, line)
            }

pub unsafe fn bli_abort() {
                dyload_lib().bli_abort.unwrap()()
            }

pub unsafe fn bli_error_string_for_code(code: gint_t) -> *const c_char {
                dyload_lib().bli_error_string_for_code.unwrap()(code)
            }

pub unsafe fn bli_lsame(ca: *mut bla_character, cb: *mut bla_character, ca_len: ftnlen, cb_len: ftnlen) -> bla_logical {
                dyload_lib().bli_lsame.unwrap()(ca, cb, ca_len, cb_len)
            }

pub unsafe fn bli_slamch(cmach: *mut bla_character, cmach_len: ftnlen) -> bla_real {
                dyload_lib().bli_slamch.unwrap()(cmach, cmach_len)
            }

pub unsafe fn bli_dlamch(cmach: *mut bla_character, cmach_len: ftnlen) -> bla_double {
                dyload_lib().bli_dlamch.unwrap()(cmach, cmach_len)
            }

pub unsafe fn bli_machval(mval: machval_t, v: *mut obj_t) {
                dyload_lib().bli_machval.unwrap()(mval, v)
            }

pub unsafe fn bli_smachval(mval: machval_t, v: *mut c_void) {
                dyload_lib().bli_smachval.unwrap()(mval, v)
            }

pub unsafe fn bli_dmachval(mval: machval_t, v: *mut c_void) {
                dyload_lib().bli_dmachval.unwrap()(mval, v)
            }

pub unsafe fn bli_cmachval(mval: machval_t, v: *mut c_void) {
                dyload_lib().bli_cmachval.unwrap()(mval, v)
            }

pub unsafe fn bli_zmachval(mval: machval_t, v: *mut c_void) {
                dyload_lib().bli_zmachval.unwrap()(mval, v)
            }

pub unsafe fn bli_getopt_init_state(opterr: c_int, state: *mut getopt_t) {
                dyload_lib().bli_getopt_init_state.unwrap()(opterr, state)
            }

pub unsafe fn bli_getopt(argc: c_int, argv: *const *const c_char, optstring: *const c_char, state: *mut getopt_t) -> c_int {
                dyload_lib().bli_getopt.unwrap()(argc, argv, optstring, state)
            }

pub unsafe fn bli_cntl_init_node(var_func: void_fp, cntl: *mut cntl_t) {
                dyload_lib().bli_cntl_init_node.unwrap()(var_func, cntl)
            }

pub unsafe fn bli_cntl_attach_sub_node(ways: dim_t, sub_node: *mut cntl_t, cntl: *mut cntl_t) {
                dyload_lib().bli_cntl_attach_sub_node.unwrap()(ways, sub_node, cntl)
            }

pub unsafe fn bli_cntl_clear_node(cntl: *mut cntl_t) {
                dyload_lib().bli_cntl_clear_node.unwrap()(cntl)
            }

pub unsafe fn bli_env_get_var(env: *const c_char, fallback: gint_t) -> gint_t {
                dyload_lib().bli_env_get_var.unwrap()(env, fallback)
            }

pub unsafe fn bli_env_get_str(env: *const c_char) -> *mut c_char {
                dyload_lib().bli_env_get_str.unwrap()(env)
            }

pub unsafe fn bli_pack_get_pack_a(pack_a: *mut bool) {
                dyload_lib().bli_pack_get_pack_a.unwrap()(pack_a)
            }

pub unsafe fn bli_pack_get_pack_b(pack_b: *mut bool) {
                dyload_lib().bli_pack_get_pack_b.unwrap()(pack_b)
            }

pub unsafe fn bli_pack_set_pack_a(pack_a: bool) {
                dyload_lib().bli_pack_set_pack_a.unwrap()(pack_a)
            }

pub unsafe fn bli_pack_set_pack_b(pack_b: bool) {
                dyload_lib().bli_pack_set_pack_b.unwrap()(pack_b)
            }

pub unsafe fn bli_info_get_version_str() -> *const c_char {
                dyload_lib().bli_info_get_version_str.unwrap()()
            }

pub unsafe fn bli_info_get_int_type_size_str() -> *const c_char {
                dyload_lib().bli_info_get_int_type_size_str.unwrap()()
            }

pub unsafe fn bli_info_get_int_type_size() -> gint_t {
                dyload_lib().bli_info_get_int_type_size.unwrap()()
            }

pub unsafe fn bli_info_get_num_fp_types() -> gint_t {
                dyload_lib().bli_info_get_num_fp_types.unwrap()()
            }

pub unsafe fn bli_info_get_max_type_size() -> gint_t {
                dyload_lib().bli_info_get_max_type_size.unwrap()()
            }

pub unsafe fn bli_info_get_page_size() -> gint_t {
                dyload_lib().bli_info_get_page_size.unwrap()()
            }

pub unsafe fn bli_info_get_simd_num_registers() -> gint_t {
                dyload_lib().bli_info_get_simd_num_registers.unwrap()()
            }

pub unsafe fn bli_info_get_simd_size() -> gint_t {
                dyload_lib().bli_info_get_simd_size.unwrap()()
            }

pub unsafe fn bli_info_get_simd_align_size() -> gint_t {
                dyload_lib().bli_info_get_simd_align_size.unwrap()()
            }

pub unsafe fn bli_info_get_stack_buf_max_size() -> gint_t {
                dyload_lib().bli_info_get_stack_buf_max_size.unwrap()()
            }

pub unsafe fn bli_info_get_stack_buf_align_size() -> gint_t {
                dyload_lib().bli_info_get_stack_buf_align_size.unwrap()()
            }

pub unsafe fn bli_info_get_heap_addr_align_size() -> gint_t {
                dyload_lib().bli_info_get_heap_addr_align_size.unwrap()()
            }

pub unsafe fn bli_info_get_heap_stride_align_size() -> gint_t {
                dyload_lib().bli_info_get_heap_stride_align_size.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_align_size_a() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_align_size_a.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_align_size_b() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_align_size_b.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_align_size_c() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_align_size_c.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_align_size_gen() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_align_size_gen.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_offset_size_a() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_offset_size_a.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_offset_size_b() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_offset_size_b.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_offset_size_c() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_offset_size_c.unwrap()()
            }

pub unsafe fn bli_info_get_pool_addr_offset_size_gen() -> gint_t {
                dyload_lib().bli_info_get_pool_addr_offset_size_gen.unwrap()()
            }

pub unsafe fn bli_info_get_enable_stay_auto_init() -> gint_t {
                dyload_lib().bli_info_get_enable_stay_auto_init.unwrap()()
            }

pub unsafe fn bli_info_get_enable_blas() -> gint_t {
                dyload_lib().bli_info_get_enable_blas.unwrap()()
            }

pub unsafe fn bli_info_get_enable_cblas() -> gint_t {
                dyload_lib().bli_info_get_enable_cblas.unwrap()()
            }

pub unsafe fn bli_info_get_blas_int_type_size() -> gint_t {
                dyload_lib().bli_info_get_blas_int_type_size.unwrap()()
            }

pub unsafe fn bli_info_get_enable_pba_pools() -> gint_t {
                dyload_lib().bli_info_get_enable_pba_pools.unwrap()()
            }

pub unsafe fn bli_info_get_enable_sba_pools() -> gint_t {
                dyload_lib().bli_info_get_enable_sba_pools.unwrap()()
            }

pub unsafe fn bli_info_get_enable_threading() -> gint_t {
                dyload_lib().bli_info_get_enable_threading.unwrap()()
            }

pub unsafe fn bli_info_get_enable_openmp() -> gint_t {
                dyload_lib().bli_info_get_enable_openmp.unwrap()()
            }

pub unsafe fn bli_info_get_enable_pthreads() -> gint_t {
                dyload_lib().bli_info_get_enable_pthreads.unwrap()()
            }

pub unsafe fn bli_info_get_enable_hpx() -> gint_t {
                dyload_lib().bli_info_get_enable_hpx.unwrap()()
            }

pub unsafe fn bli_info_get_enable_openmp_as_default() -> gint_t {
                dyload_lib().bli_info_get_enable_openmp_as_default.unwrap()()
            }

pub unsafe fn bli_info_get_enable_pthreads_as_default() -> gint_t {
                dyload_lib().bli_info_get_enable_pthreads_as_default.unwrap()()
            }

pub unsafe fn bli_info_get_enable_hpx_as_default() -> gint_t {
                dyload_lib().bli_info_get_enable_hpx_as_default.unwrap()()
            }

pub unsafe fn bli_info_get_thread_jrir_slab() -> gint_t {
                dyload_lib().bli_info_get_thread_jrir_slab.unwrap()()
            }

pub unsafe fn bli_info_get_thread_jrir_rr() -> gint_t {
                dyload_lib().bli_info_get_thread_jrir_rr.unwrap()()
            }

pub unsafe fn bli_info_get_thread_jrir_tlb() -> gint_t {
                dyload_lib().bli_info_get_thread_jrir_tlb.unwrap()()
            }

pub unsafe fn bli_info_get_enable_tls() -> gint_t {
                dyload_lib().bli_info_get_enable_tls.unwrap()()
            }

pub unsafe fn bli_info_get_enable_memkind() -> gint_t {
                dyload_lib().bli_info_get_enable_memkind.unwrap()()
            }

pub unsafe fn bli_info_get_enable_sandbox() -> gint_t {
                dyload_lib().bli_info_get_enable_sandbox.unwrap()()
            }

pub unsafe fn bli_info_get_gemm_ukr_impl_string(method: ind_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_gemm_ukr_impl_string.unwrap()(method, dt)
            }

pub unsafe fn bli_info_get_gemmtrsm_l_ukr_impl_string(method: ind_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_gemmtrsm_l_ukr_impl_string.unwrap()(method, dt)
            }

pub unsafe fn bli_info_get_gemmtrsm_u_ukr_impl_string(method: ind_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_gemmtrsm_u_ukr_impl_string.unwrap()(method, dt)
            }

pub unsafe fn bli_info_get_trsm_l_ukr_impl_string(method: ind_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_trsm_l_ukr_impl_string.unwrap()(method, dt)
            }

pub unsafe fn bli_info_get_trsm_u_ukr_impl_string(method: ind_t, dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_trsm_u_ukr_impl_string.unwrap()(method, dt)
            }

pub unsafe fn bli_info_get_gemm_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_gemm_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_gemmt_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_gemmt_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_hemm_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_hemm_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_herk_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_herk_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_her2k_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_her2k_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_symm_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_symm_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_syrk_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_syrk_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_syr2k_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_syr2k_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_trmm_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_trmm_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_trmm3_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_trmm3_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_info_get_trsm_impl_string(dt: num_t) -> *const c_char {
                dyload_lib().bli_info_get_trsm_impl_string.unwrap()(dt)
            }

pub unsafe fn bli_arch_query_id() -> arch_t {
                dyload_lib().bli_arch_query_id.unwrap()()
            }

pub unsafe fn bli_arch_set_id_once() {
                dyload_lib().bli_arch_set_id_once.unwrap()()
            }

pub unsafe fn bli_arch_set_id() {
                dyload_lib().bli_arch_set_id.unwrap()()
            }

pub unsafe fn bli_arch_query_id_impl() -> arch_t {
                dyload_lib().bli_arch_query_id_impl.unwrap()()
            }

pub unsafe fn bli_arch_string(id: arch_t) -> *const c_char {
                dyload_lib().bli_arch_string.unwrap()(id)
            }

pub unsafe fn bli_arch_set_logging(dolog: bool) {
                dyload_lib().bli_arch_set_logging.unwrap()(dolog)
            }

pub unsafe fn bli_arch_get_logging() -> bool {
                dyload_lib().bli_arch_get_logging.unwrap()()
            }

pub unsafe fn bli_arch_log(arg1: *const c_char) {
                dyload_lib().bli_arch_log.unwrap()(arg1)
            }

pub unsafe fn bli_cpuid_query_id() -> arch_t {
                dyload_lib().bli_cpuid_query_id.unwrap()()
            }

pub unsafe fn bli_cpuid_is_skx(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_skx.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_knl(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_knl.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_haswell(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_haswell.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_sandybridge(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_sandybridge.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_penryn(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_penryn.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_zen3(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_zen3.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_zen2(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_zen2.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_zen(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_zen.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_excavator(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_excavator.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_steamroller(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_steamroller.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_piledriver(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_piledriver.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_bulldozer(family: u32, model: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_bulldozer.unwrap()(family, model, features)
            }

pub unsafe fn bli_cpuid_is_thunderx2(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_thunderx2.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_is_cortexa57(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_cortexa57.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_is_cortexa53(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_cortexa53.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_is_armsve(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_armsve.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_is_a64fx(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_a64fx.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_is_cortexa15(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_cortexa15.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_is_cortexa9(model: u32, part: u32, features: u32) -> bool {
                dyload_lib().bli_cpuid_is_cortexa9.unwrap()(model, part, features)
            }

pub unsafe fn bli_cpuid_query(family: *mut u32, model: *mut u32, features: *mut u32) -> u32 {
                dyload_lib().bli_cpuid_query.unwrap()(family, model, features)
            }

pub unsafe fn get_cpu_name(cpu_name: *mut c_char) {
                dyload_lib().get_cpu_name.unwrap()(cpu_name)
            }

pub unsafe fn vpu_count() -> c_int {
                dyload_lib().vpu_count.unwrap()()
            }

pub unsafe fn bli_string_mkupper(s: *mut c_char) {
                dyload_lib().bli_string_mkupper.unwrap()(s)
            }

pub unsafe fn bli_setijm(ar: f64, ai: f64, i: dim_t, j: dim_t, b: *const obj_t) -> err_t {
                dyload_lib().bli_setijm.unwrap()(ar, ai, i, j, b)
            }

pub unsafe fn bli_ssetijm(ar: f64, ai: f64, i: dim_t, j: dim_t, b: *mut c_void, rs: inc_t, cs: inc_t) {
                dyload_lib().bli_ssetijm.unwrap()(ar, ai, i, j, b, rs, cs)
            }

pub unsafe fn bli_dsetijm(ar: f64, ai: f64, i: dim_t, j: dim_t, b: *mut c_void, rs: inc_t, cs: inc_t) {
                dyload_lib().bli_dsetijm.unwrap()(ar, ai, i, j, b, rs, cs)
            }

pub unsafe fn bli_csetijm(ar: f64, ai: f64, i: dim_t, j: dim_t, b: *mut c_void, rs: inc_t, cs: inc_t) {
                dyload_lib().bli_csetijm.unwrap()(ar, ai, i, j, b, rs, cs)
            }

pub unsafe fn bli_zsetijm(ar: f64, ai: f64, i: dim_t, j: dim_t, b: *mut c_void, rs: inc_t, cs: inc_t) {
                dyload_lib().bli_zsetijm.unwrap()(ar, ai, i, j, b, rs, cs)
            }

pub unsafe fn bli_getijm(i: dim_t, j: dim_t, b: *const obj_t, ar: *mut f64, ai: *mut f64) -> err_t {
                dyload_lib().bli_getijm.unwrap()(i, j, b, ar, ai)
            }

pub unsafe fn bli_sgetijm(i: dim_t, j: dim_t, b: *const c_void, rs: inc_t, cs: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_sgetijm.unwrap()(i, j, b, rs, cs, ar, ai)
            }

pub unsafe fn bli_dgetijm(i: dim_t, j: dim_t, b: *const c_void, rs: inc_t, cs: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_dgetijm.unwrap()(i, j, b, rs, cs, ar, ai)
            }

pub unsafe fn bli_cgetijm(i: dim_t, j: dim_t, b: *const c_void, rs: inc_t, cs: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_cgetijm.unwrap()(i, j, b, rs, cs, ar, ai)
            }

pub unsafe fn bli_zgetijm(i: dim_t, j: dim_t, b: *const c_void, rs: inc_t, cs: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_zgetijm.unwrap()(i, j, b, rs, cs, ar, ai)
            }

pub unsafe fn bli_setijv(ar: f64, ai: f64, i: dim_t, x: *const obj_t) -> err_t {
                dyload_lib().bli_setijv.unwrap()(ar, ai, i, x)
            }

pub unsafe fn bli_ssetijv(ar: f64, ai: f64, i: dim_t, x: *mut c_void, incx: inc_t) {
                dyload_lib().bli_ssetijv.unwrap()(ar, ai, i, x, incx)
            }

pub unsafe fn bli_dsetijv(ar: f64, ai: f64, i: dim_t, x: *mut c_void, incx: inc_t) {
                dyload_lib().bli_dsetijv.unwrap()(ar, ai, i, x, incx)
            }

pub unsafe fn bli_csetijv(ar: f64, ai: f64, i: dim_t, x: *mut c_void, incx: inc_t) {
                dyload_lib().bli_csetijv.unwrap()(ar, ai, i, x, incx)
            }

pub unsafe fn bli_zsetijv(ar: f64, ai: f64, i: dim_t, x: *mut c_void, incx: inc_t) {
                dyload_lib().bli_zsetijv.unwrap()(ar, ai, i, x, incx)
            }

pub unsafe fn bli_getijv(i: dim_t, x: *const obj_t, ar: *mut f64, ai: *mut f64) -> err_t {
                dyload_lib().bli_getijv.unwrap()(i, x, ar, ai)
            }

pub unsafe fn bli_sgetijv(i: dim_t, b: *const c_void, incx: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_sgetijv.unwrap()(i, b, incx, ar, ai)
            }

pub unsafe fn bli_dgetijv(i: dim_t, b: *const c_void, incx: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_dgetijv.unwrap()(i, b, incx, ar, ai)
            }

pub unsafe fn bli_cgetijv(i: dim_t, b: *const c_void, incx: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_cgetijv.unwrap()(i, b, incx, ar, ai)
            }

pub unsafe fn bli_zgetijv(i: dim_t, b: *const c_void, incx: inc_t, ar: *mut f64, ai: *mut f64) {
                dyload_lib().bli_zgetijv.unwrap()(i, b, incx, ar, ai)
            }

pub unsafe fn bli_setrm(alpha: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_setrm.unwrap()(alpha, b)
            }

pub unsafe fn bli_setrv(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setrv.unwrap()(alpha, x)
            }

pub unsafe fn bli_setim(alpha: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_setim.unwrap()(alpha, b)
            }

pub unsafe fn bli_setiv(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setiv.unwrap()(alpha, x)
            }

pub unsafe fn bli_castm(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_castm.unwrap()(a, b)
            }

pub unsafe fn bli_sscastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_sscastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_ddcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_ddcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_cccastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_cccastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zzcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zzcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_sdcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_sdcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_sccastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_sccastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_szcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_szcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dscastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dscastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dccastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dccastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dzcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dzcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_cscastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_cscastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_cdcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_cdcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_czcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_czcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zscastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zscastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zdcastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zdcastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zccastm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zccastm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_castm_check(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_castm_check.unwrap()(a, b)
            }

pub unsafe fn bli_castnzm(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_castnzm.unwrap()(a, b)
            }

pub unsafe fn bli_sscastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_sscastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_ddcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_ddcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_cccastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_cccastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zzcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zzcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_sdcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_sdcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_sccastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_sccastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_szcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_szcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dscastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dscastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dccastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dccastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dzcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dzcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_cscastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_cscastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_cdcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_cdcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_czcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_czcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zscastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zscastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zdcastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zdcastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_zccastnzm(transa: trans_t, m: dim_t, n: dim_t, a: *const c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_zccastnzm.unwrap()(transa, m, n, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_castnzm_check(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_castnzm_check.unwrap()(a, b)
            }

pub unsafe fn bli_castv(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_castv.unwrap()(x, y)
            }

pub unsafe fn bli_sscastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_sscastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_ddcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_ddcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_cccastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_cccastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zzcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_zzcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_sdcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_sdcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_sccastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_sccastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_szcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_szcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_dscastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_dscastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_dccastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_dccastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_dzcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_dzcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_cscastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_cscastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_cdcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_cdcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_czcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_czcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zscastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_zscastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zdcastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_zdcastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zccastv(conjx: conj_t, n: dim_t, x: *const c_void, incx: inc_t, y: *mut c_void, incy: inc_t) {
                dyload_lib().bli_zccastv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_castv_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_castv_check.unwrap()(x, y)
            }

pub unsafe fn bli_projm(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_projm.unwrap()(a, b)
            }

pub unsafe fn bli_projm_check(a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_projm_check.unwrap()(a, b)
            }

pub unsafe fn bli_projv(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_projv.unwrap()(x, y)
            }

pub unsafe fn bli_projv_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_projv_check.unwrap()(x, y)
            }

pub unsafe fn bli_addsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_addsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_copysc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_copysc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_divsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_divsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_mulsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_mulsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_sqrtsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_sqrtsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_sqrtrsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_sqrtrsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_subsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_subsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_invertsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_invertsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_absqsc_check(chi: *const obj_t, absq: *const obj_t) {
                dyload_lib().bli_absqsc_check.unwrap()(chi, absq)
            }

pub unsafe fn bli_normfsc_check(chi: *const obj_t, absq: *const obj_t) {
                dyload_lib().bli_normfsc_check.unwrap()(chi, absq)
            }

pub unsafe fn bli_getsc_check(chi: *const obj_t, zeta_r: *const f64, zeta_i: *const f64) {
                dyload_lib().bli_getsc_check.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_setsc_check(zeta_r: f64, zeta_i: f64, chi: *const obj_t) {
                dyload_lib().bli_setsc_check.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_unzipsc_check(chi: *const obj_t, zeta_r: *const obj_t, zeta_i: *const obj_t) {
                dyload_lib().bli_unzipsc_check.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_zipsc_check(zeta_r: *const obj_t, zeta_i: *const obj_t, chi: *const obj_t) {
                dyload_lib().bli_zipsc_check.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_l0_xsc_check(chi: *const obj_t) {
                dyload_lib().bli_l0_xsc_check.unwrap()(chi)
            }

pub unsafe fn bli_l0_xxsc_check(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_l0_xxsc_check.unwrap()(chi, psi)
            }

pub unsafe fn bli_l0_xx2sc_check(chi: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_l0_xx2sc_check.unwrap()(chi, norm)
            }

pub unsafe fn bli_l0_xxbsc_check(chi: *const obj_t, psi: *const obj_t, is: *const bool) {
                dyload_lib().bli_l0_xxbsc_check.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_absqsc(chi: *const obj_t, absq: *const obj_t) {
                dyload_lib().bli_absqsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_normfsc(chi: *const obj_t, absq: *const obj_t) {
                dyload_lib().bli_normfsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_addsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_addsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_divsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_divsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_mulsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_mulsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_sqrtsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_sqrtsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_sqrtrsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_sqrtrsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_subsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_subsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_invertsc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_invertsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_getsc(chi: *const obj_t, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_getsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_setsc(zeta_r: f64, zeta_i: f64, chi: *const obj_t) {
                dyload_lib().bli_setsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_unzipsc(chi: *const obj_t, zeta_r: *const obj_t, zeta_i: *const obj_t) {
                dyload_lib().bli_unzipsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_zipsc(zeta_r: *const obj_t, zeta_i: *const obj_t, chi: *const obj_t) {
                dyload_lib().bli_zipsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_saddsc(conjchi: conj_t, chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_saddsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_daddsc(conjchi: conj_t, chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_daddsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_caddsc(conjchi: conj_t, chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_caddsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zaddsc(conjchi: conj_t, chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zaddsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_sdivsc(conjchi: conj_t, chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_sdivsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_ddivsc(conjchi: conj_t, chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_ddivsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_cdivsc(conjchi: conj_t, chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_cdivsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zdivsc(conjchi: conj_t, chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zdivsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_smulsc(conjchi: conj_t, chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_smulsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_dmulsc(conjchi: conj_t, chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_dmulsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_cmulsc(conjchi: conj_t, chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_cmulsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zmulsc(conjchi: conj_t, chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zmulsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_ssubsc(conjchi: conj_t, chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_ssubsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_dsubsc(conjchi: conj_t, chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_dsubsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_csubsc(conjchi: conj_t, chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_csubsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zsubsc(conjchi: conj_t, chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zsubsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_sinvertsc(conjchi: conj_t, chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_sinvertsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_dinvertsc(conjchi: conj_t, chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_dinvertsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_cinvertsc(conjchi: conj_t, chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_cinvertsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zinvertsc(conjchi: conj_t, chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zinvertsc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_sabsqsc(chi: *const f32, absq: *mut f32) {
                dyload_lib().bli_sabsqsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_dabsqsc(chi: *const f64, absq: *mut f64) {
                dyload_lib().bli_dabsqsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_cabsqsc(chi: *const scomplex, absq: *mut f32) {
                dyload_lib().bli_cabsqsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_zabsqsc(chi: *const dcomplex, absq: *mut f64) {
                dyload_lib().bli_zabsqsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_snormfsc(chi: *const f32, absq: *mut f32) {
                dyload_lib().bli_snormfsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_dnormfsc(chi: *const f64, absq: *mut f64) {
                dyload_lib().bli_dnormfsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_cnormfsc(chi: *const scomplex, absq: *mut f32) {
                dyload_lib().bli_cnormfsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_znormfsc(chi: *const dcomplex, absq: *mut f64) {
                dyload_lib().bli_znormfsc.unwrap()(chi, absq)
            }

pub unsafe fn bli_ssqrtsc(chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_ssqrtsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_dsqrtsc(chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_dsqrtsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_csqrtsc(chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_csqrtsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_zsqrtsc(chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zsqrtsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_ssqrtrsc(chi: *const f32, psi: *mut f32) {
                dyload_lib().bli_ssqrtrsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_dsqrtrsc(chi: *const f64, psi: *mut f64) {
                dyload_lib().bli_dsqrtrsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_csqrtrsc(chi: *const scomplex, psi: *mut scomplex) {
                dyload_lib().bli_csqrtrsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_zsqrtrsc(chi: *const dcomplex, psi: *mut dcomplex) {
                dyload_lib().bli_zsqrtrsc.unwrap()(chi, psi)
            }

pub unsafe fn bli_sgetsc(chi: *const f32, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_sgetsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_dgetsc(chi: *const f64, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_dgetsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_cgetsc(chi: *const scomplex, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_cgetsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_zgetsc(chi: *const dcomplex, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_zgetsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_ssetsc(zeta_r: f64, zeta_i: f64, chi: *mut f32) {
                dyload_lib().bli_ssetsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_dsetsc(zeta_r: f64, zeta_i: f64, chi: *mut f64) {
                dyload_lib().bli_dsetsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_csetsc(zeta_r: f64, zeta_i: f64, chi: *mut scomplex) {
                dyload_lib().bli_csetsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_zsetsc(zeta_r: f64, zeta_i: f64, chi: *mut dcomplex) {
                dyload_lib().bli_zsetsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_sunzipsc(chi: *const f32, zeta_r: *mut f32, zeta_i: *mut f32) {
                dyload_lib().bli_sunzipsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_dunzipsc(chi: *const f64, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_dunzipsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_cunzipsc(chi: *const scomplex, zeta_r: *mut f32, zeta_i: *mut f32) {
                dyload_lib().bli_cunzipsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_zunzipsc(chi: *const dcomplex, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_zunzipsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_szipsc(zeta_r: *const f32, zeta_i: *const f32, chi: *mut f32) {
                dyload_lib().bli_szipsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_dzipsc(zeta_r: *const f64, zeta_i: *const f64, chi: *mut f64) {
                dyload_lib().bli_dzipsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_czipsc(zeta_r: *const f32, zeta_i: *const f32, chi: *mut scomplex) {
                dyload_lib().bli_czipsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_zzipsc(zeta_r: *const f64, zeta_i: *const f64, chi: *mut dcomplex) {
                dyload_lib().bli_zzipsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_igetsc(chi: *const dim_t, zeta_r: *mut f64, zeta_i: *mut f64) {
                dyload_lib().bli_igetsc.unwrap()(chi, zeta_r, zeta_i)
            }

pub unsafe fn bli_isetsc(zeta_r: f64, zeta_i: f64, chi: *mut dim_t) {
                dyload_lib().bli_isetsc.unwrap()(zeta_r, zeta_i, chi)
            }

pub unsafe fn bli_absqsc_qfp(dt: num_t) -> absqsc_vft {
                dyload_lib().bli_absqsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_normfsc_qfp(dt: num_t) -> normfsc_vft {
                dyload_lib().bli_normfsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_addsc_qfp(dt: num_t) -> addsc_vft {
                dyload_lib().bli_addsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_divsc_qfp(dt: num_t) -> divsc_vft {
                dyload_lib().bli_divsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_mulsc_qfp(dt: num_t) -> mulsc_vft {
                dyload_lib().bli_mulsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_subsc_qfp(dt: num_t) -> subsc_vft {
                dyload_lib().bli_subsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_invertsc_qfp(dt: num_t) -> invertsc_vft {
                dyload_lib().bli_invertsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_sqrtsc_qfp(dt: num_t) -> sqrtsc_vft {
                dyload_lib().bli_sqrtsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_sqrtrsc_qfp(dt: num_t) -> sqrtrsc_vft {
                dyload_lib().bli_sqrtrsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_unzipsc_qfp(dt: num_t) -> unzipsc_vft {
                dyload_lib().bli_unzipsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_zipsc_qfp(dt: num_t) -> zipsc_vft {
                dyload_lib().bli_zipsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_getsc_qfp(dt: num_t) -> getsc_vft {
                dyload_lib().bli_getsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_setsc_qfp(dt: num_t) -> setsc_vft {
                dyload_lib().bli_setsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_copysc(chi: *const obj_t, psi: *const obj_t) {
                dyload_lib().bli_copysc.unwrap()(chi, psi)
            }

pub unsafe fn bli_sscopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_sscopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_ddcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_ddcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_cccopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_cccopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zzcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_zzcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_sccopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_sccopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_cscopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_cscopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_dzcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_dzcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zdcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_zdcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_sdcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_sdcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_szcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_szcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_dscopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_dscopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_dccopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_dccopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_cdcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_cdcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_czcopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_czcopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zscopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_zscopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_zccopysc(conjchi: conj_t, chi: *const c_void, psi: *mut c_void) {
                dyload_lib().bli_zccopysc.unwrap()(conjchi, chi, psi)
            }

pub unsafe fn bli_addv_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_addv_check.unwrap()(x, y)
            }

pub unsafe fn bli_copyv_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_copyv_check.unwrap()(x, y)
            }

pub unsafe fn bli_subv_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_subv_check.unwrap()(x, y)
            }

pub unsafe fn bli_swapv_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_swapv_check.unwrap()(x, y)
            }

pub unsafe fn bli_amaxv_check(x: *const obj_t, index: *const obj_t) {
                dyload_lib().bli_amaxv_check.unwrap()(x, index)
            }

pub unsafe fn bli_axpbyv_check(alpha: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpbyv_check.unwrap()(alpha, x, beta, y)
            }

pub unsafe fn bli_axpyv_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpyv_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_scal2v_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_scal2v_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_dotv_check(x: *const obj_t, y: *const obj_t, rho: *const obj_t) {
                dyload_lib().bli_dotv_check.unwrap()(x, y, rho)
            }

pub unsafe fn bli_dotxv_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, beta: *const obj_t, rho: *const obj_t) {
                dyload_lib().bli_dotxv_check.unwrap()(alpha, x, y, beta, rho)
            }

pub unsafe fn bli_invertv_check(x: *const obj_t) {
                dyload_lib().bli_invertv_check.unwrap()(x)
            }

pub unsafe fn bli_invscalv_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_invscalv_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_scalv_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_scalv_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_setv_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setv_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_xpbyv_check(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbyv_check.unwrap()(x, beta, y)
            }

pub unsafe fn bli_l1v_xy_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1v_xy_check.unwrap()(x, y)
            }

pub unsafe fn bli_l1v_axy_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1v_axy_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_l1v_xby_check(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1v_xby_check.unwrap()(x, beta, y)
            }

pub unsafe fn bli_l1v_axby_check(alpha: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1v_axby_check.unwrap()(alpha, x, beta, y)
            }

pub unsafe fn bli_l1v_dot_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, beta: *const obj_t, rho: *const obj_t) {
                dyload_lib().bli_l1v_dot_check.unwrap()(alpha, x, y, beta, rho)
            }

pub unsafe fn bli_l1v_x_check(x: *const obj_t) {
                dyload_lib().bli_l1v_x_check.unwrap()(x)
            }

pub unsafe fn bli_l1v_ax_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_l1v_ax_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_l1v_xi_check(x: *const obj_t, index: *const obj_t) {
                dyload_lib().bli_l1v_xi_check.unwrap()(x, index)
            }

pub unsafe fn bli_addv_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_addv_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_copyv_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_copyv_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_subv_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_subv_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_amaxv_ex(x: *const obj_t, index: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_amaxv_ex.unwrap()(x, index, cntx, rntm)
            }

pub unsafe fn bli_axpbyv_ex(alpha: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_axpbyv_ex.unwrap()(alpha, x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_axpyv_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_axpyv_ex.unwrap()(alpha, x, y, cntx, rntm)
            }

pub unsafe fn bli_scal2v_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scal2v_ex.unwrap()(alpha, x, y, cntx, rntm)
            }

pub unsafe fn bli_dotv_ex(x: *const obj_t, y: *const obj_t, rho: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dotv_ex.unwrap()(x, y, rho, cntx, rntm)
            }

pub unsafe fn bli_dotxv_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, beta: *const obj_t, rho: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dotxv_ex.unwrap()(alpha, x, y, beta, rho, cntx, rntm)
            }

pub unsafe fn bli_invertv_ex(x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_invertv_ex.unwrap()(x, cntx, rntm)
            }

pub unsafe fn bli_invscalv_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_invscalv_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_scalv_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scalv_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_setv_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_setv_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_swapv_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_swapv_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_xpbyv_ex(x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_xpbyv_ex.unwrap()(x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_addv(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_addv.unwrap()(x, y)
            }

pub unsafe fn bli_copyv(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_copyv.unwrap()(x, y)
            }

pub unsafe fn bli_subv(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_subv.unwrap()(x, y)
            }

pub unsafe fn bli_amaxv(x: *const obj_t, index: *const obj_t) {
                dyload_lib().bli_amaxv.unwrap()(x, index)
            }

pub unsafe fn bli_axpbyv(alpha: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpbyv.unwrap()(alpha, x, beta, y)
            }

pub unsafe fn bli_axpyv(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpyv.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_scal2v(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_scal2v.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_dotv(x: *const obj_t, y: *const obj_t, rho: *const obj_t) {
                dyload_lib().bli_dotv.unwrap()(x, y, rho)
            }

pub unsafe fn bli_dotxv(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, beta: *const obj_t, rho: *const obj_t) {
                dyload_lib().bli_dotxv.unwrap()(alpha, x, y, beta, rho)
            }

pub unsafe fn bli_invertv(x: *const obj_t) {
                dyload_lib().bli_invertv.unwrap()(x)
            }

pub unsafe fn bli_invscalv(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_invscalv.unwrap()(alpha, x)
            }

pub unsafe fn bli_scalv(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_scalv.unwrap()(alpha, x)
            }

pub unsafe fn bli_setv(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setv.unwrap()(alpha, x)
            }

pub unsafe fn bli_swapv(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_swapv.unwrap()(x, y)
            }

pub unsafe fn bli_xpbyv(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbyv.unwrap()(x, beta, y)
            }

pub unsafe fn bli_saddv_ex(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saddv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_daddv_ex(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daddv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_caddv_ex(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caddv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zaddv_ex(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaddv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_scopyv_ex(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scopyv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dcopyv_ex(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dcopyv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_ccopyv_ex(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ccopyv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zcopyv_ex(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zcopyv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_ssubv_ex(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssubv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dsubv_ex(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsubv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_csubv_ex(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csubv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zsubv_ex(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsubv_ex.unwrap()(conjx, n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_samaxv_ex(n: dim_t, x: *const f32, incx: inc_t, index: *mut dim_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_samaxv_ex.unwrap()(n, x, incx, index, cntx, rntm)
            }

pub unsafe fn bli_damaxv_ex(n: dim_t, x: *const f64, incx: inc_t, index: *mut dim_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_damaxv_ex.unwrap()(n, x, incx, index, cntx, rntm)
            }

pub unsafe fn bli_camaxv_ex(n: dim_t, x: *const scomplex, incx: inc_t, index: *mut dim_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_camaxv_ex.unwrap()(n, x, incx, index, cntx, rntm)
            }

pub unsafe fn bli_zamaxv_ex(n: dim_t, x: *const dcomplex, incx: inc_t, index: *mut dim_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zamaxv_ex.unwrap()(n, x, incx, index, cntx, rntm)
            }

pub unsafe fn bli_saxpbyv_ex(conjx: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saxpbyv_ex.unwrap()(conjx, n, alpha, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_daxpbyv_ex(conjx: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daxpbyv_ex.unwrap()(conjx, n, alpha, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_caxpbyv_ex(conjx: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caxpbyv_ex.unwrap()(conjx, n, alpha, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zaxpbyv_ex(conjx: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaxpbyv_ex.unwrap()(conjx, n, alpha, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_saxpyv_ex(conjx: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saxpyv_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_daxpyv_ex(conjx: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daxpyv_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_caxpyv_ex(conjx: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caxpyv_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zaxpyv_ex(conjx: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaxpyv_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_sscal2v_ex(conjx: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sscal2v_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dscal2v_ex(conjx: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dscal2v_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_cscal2v_ex(conjx: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cscal2v_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zscal2v_ex(conjx: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zscal2v_ex.unwrap()(conjx, n, alpha, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_sdotv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, rho: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sdotv_ex.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx, rntm)
            }

pub unsafe fn bli_ddotv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, rho: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ddotv_ex.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx, rntm)
            }

pub unsafe fn bli_cdotv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, rho: *mut scomplex, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cdotv_ex.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx, rntm)
            }

pub unsafe fn bli_zdotv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, rho: *mut dcomplex, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zdotv_ex.unwrap()(conjx, conjy, n, x, incx, y, incy, rho, cntx, rntm)
            }

pub unsafe fn bli_sdotxv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, beta: *const f32, rho: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sdotxv_ex.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho, cntx, rntm)
            }

pub unsafe fn bli_ddotxv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, beta: *const f64, rho: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ddotxv_ex.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho, cntx, rntm)
            }

pub unsafe fn bli_cdotxv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, beta: *const scomplex, rho: *mut scomplex, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cdotxv_ex.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho, cntx, rntm)
            }

pub unsafe fn bli_zdotxv_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, beta: *const dcomplex, rho: *mut dcomplex, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zdotxv_ex.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho, cntx, rntm)
            }

pub unsafe fn bli_sinvertv_ex(n: dim_t, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sinvertv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_dinvertv_ex(n: dim_t, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dinvertv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_cinvertv_ex(n: dim_t, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cinvertv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zinvertv_ex(n: dim_t, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zinvertv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_sinvscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const f32, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sinvscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_dinvscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const f64, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dinvscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_cinvscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cinvscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zinvscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zinvscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_sscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const f32, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_dscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const f64, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_cscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zscalv_ex(conjalpha: conj_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zscalv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_ssetv_ex(conjalpha: conj_t, n: dim_t, alpha: *const f32, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssetv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_dsetv_ex(conjalpha: conj_t, n: dim_t, alpha: *const f64, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsetv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_csetv_ex(conjalpha: conj_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csetv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zsetv_ex(conjalpha: conj_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsetv_ex.unwrap()(conjalpha, n, alpha, x, incx, cntx, rntm)
            }

pub unsafe fn bli_sswapv_ex(n: dim_t, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sswapv_ex.unwrap()(n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dswapv_ex(n: dim_t, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dswapv_ex.unwrap()(n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_cswapv_ex(n: dim_t, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cswapv_ex.unwrap()(n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zswapv_ex(n: dim_t, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zswapv_ex.unwrap()(n, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_sxpbyv_ex(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sxpbyv_ex.unwrap()(conjx, n, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dxpbyv_ex(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dxpbyv_ex.unwrap()(conjx, n, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_cxpbyv_ex(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cxpbyv_ex.unwrap()(conjx, n, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zxpbyv_ex(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zxpbyv_ex.unwrap()(conjx, n, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_saddv(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_saddv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_daddv(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_daddv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_caddv(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_caddv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zaddv(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zaddv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_scopyv(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_scopyv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_dcopyv(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dcopyv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_ccopyv(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_ccopyv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zcopyv(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zcopyv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_ssubv(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_ssubv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_dsubv(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dsubv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_csubv(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_csubv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zsubv(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zsubv.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_samaxv(n: dim_t, x: *const f32, incx: inc_t, index: *mut dim_t) {
                dyload_lib().bli_samaxv.unwrap()(n, x, incx, index)
            }

pub unsafe fn bli_damaxv(n: dim_t, x: *const f64, incx: inc_t, index: *mut dim_t) {
                dyload_lib().bli_damaxv.unwrap()(n, x, incx, index)
            }

pub unsafe fn bli_camaxv(n: dim_t, x: *const scomplex, incx: inc_t, index: *mut dim_t) {
                dyload_lib().bli_camaxv.unwrap()(n, x, incx, index)
            }

pub unsafe fn bli_zamaxv(n: dim_t, x: *const dcomplex, incx: inc_t, index: *mut dim_t) {
                dyload_lib().bli_zamaxv.unwrap()(n, x, incx, index)
            }

pub unsafe fn bli_saxpbyv(conjx: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_saxpbyv.unwrap()(conjx, n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn bli_daxpbyv(conjx: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_daxpbyv.unwrap()(conjx, n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn bli_caxpbyv(conjx: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_caxpbyv.unwrap()(conjx, n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn bli_zaxpbyv(conjx: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zaxpbyv.unwrap()(conjx, n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn bli_saxpyv(conjx: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_saxpyv.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_daxpyv(conjx: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_daxpyv.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_caxpyv(conjx: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_caxpyv.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_zaxpyv(conjx: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zaxpyv.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_sscal2v(conjx: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_sscal2v.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_dscal2v(conjx: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dscal2v.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_cscal2v(conjx: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_cscal2v.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_zscal2v(conjx: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zscal2v.unwrap()(conjx, n, alpha, x, incx, y, incy)
            }

pub unsafe fn bli_sdotv(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, rho: *mut f32) {
                dyload_lib().bli_sdotv.unwrap()(conjx, conjy, n, x, incx, y, incy, rho)
            }

pub unsafe fn bli_ddotv(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, rho: *mut f64) {
                dyload_lib().bli_ddotv.unwrap()(conjx, conjy, n, x, incx, y, incy, rho)
            }

pub unsafe fn bli_cdotv(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, rho: *mut scomplex) {
                dyload_lib().bli_cdotv.unwrap()(conjx, conjy, n, x, incx, y, incy, rho)
            }

pub unsafe fn bli_zdotv(conjx: conj_t, conjy: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, rho: *mut dcomplex) {
                dyload_lib().bli_zdotv.unwrap()(conjx, conjy, n, x, incx, y, incy, rho)
            }

pub unsafe fn bli_sdotxv(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, beta: *const f32, rho: *mut f32) {
                dyload_lib().bli_sdotxv.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho)
            }

pub unsafe fn bli_ddotxv(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, beta: *const f64, rho: *mut f64) {
                dyload_lib().bli_ddotxv.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho)
            }

pub unsafe fn bli_cdotxv(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, beta: *const scomplex, rho: *mut scomplex) {
                dyload_lib().bli_cdotxv.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho)
            }

pub unsafe fn bli_zdotxv(conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, beta: *const dcomplex, rho: *mut dcomplex) {
                dyload_lib().bli_zdotxv.unwrap()(conjx, conjy, n, alpha, x, incx, y, incy, beta, rho)
            }

pub unsafe fn bli_sinvertv(n: dim_t, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_sinvertv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_dinvertv(n: dim_t, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_dinvertv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_cinvertv(n: dim_t, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_cinvertv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_zinvertv(n: dim_t, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_zinvertv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_sinvscalv(conjalpha: conj_t, n: dim_t, alpha: *const f32, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_sinvscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_dinvscalv(conjalpha: conj_t, n: dim_t, alpha: *const f64, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_dinvscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_cinvscalv(conjalpha: conj_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_cinvscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_zinvscalv(conjalpha: conj_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_zinvscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_sscalv(conjalpha: conj_t, n: dim_t, alpha: *const f32, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_sscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_dscalv(conjalpha: conj_t, n: dim_t, alpha: *const f64, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_dscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_cscalv(conjalpha: conj_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_cscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_zscalv(conjalpha: conj_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_zscalv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_ssetv(conjalpha: conj_t, n: dim_t, alpha: *const f32, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_ssetv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_dsetv(conjalpha: conj_t, n: dim_t, alpha: *const f64, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_dsetv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_csetv(conjalpha: conj_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_csetv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_zsetv(conjalpha: conj_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_zsetv.unwrap()(conjalpha, n, alpha, x, incx)
            }

pub unsafe fn bli_sswapv(n: dim_t, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_sswapv.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn bli_dswapv(n: dim_t, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dswapv.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn bli_cswapv(n: dim_t, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_cswapv.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn bli_zswapv(n: dim_t, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zswapv.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn bli_sxpbyv(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_sxpbyv.unwrap()(conjx, n, x, incx, beta, y, incy)
            }

pub unsafe fn bli_dxpbyv(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dxpbyv.unwrap()(conjx, n, x, incx, beta, y, incy)
            }

pub unsafe fn bli_cxpbyv(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_cxpbyv.unwrap()(conjx, n, x, incx, beta, y, incy)
            }

pub unsafe fn bli_zxpbyv(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zxpbyv.unwrap()(conjx, n, x, incx, beta, y, incy)
            }

pub unsafe fn bli_addv_ex_qfp(dt: num_t) -> addv_ex_vft {
                dyload_lib().bli_addv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_copyv_ex_qfp(dt: num_t) -> copyv_ex_vft {
                dyload_lib().bli_copyv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_subv_ex_qfp(dt: num_t) -> subv_ex_vft {
                dyload_lib().bli_subv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_amaxv_ex_qfp(dt: num_t) -> amaxv_ex_vft {
                dyload_lib().bli_amaxv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_axpbyv_ex_qfp(dt: num_t) -> axpbyv_ex_vft {
                dyload_lib().bli_axpbyv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_axpyv_ex_qfp(dt: num_t) -> axpyv_ex_vft {
                dyload_lib().bli_axpyv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_scal2v_ex_qfp(dt: num_t) -> scal2v_ex_vft {
                dyload_lib().bli_scal2v_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_dotv_ex_qfp(dt: num_t) -> dotv_ex_vft {
                dyload_lib().bli_dotv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_dotxv_ex_qfp(dt: num_t) -> dotxv_ex_vft {
                dyload_lib().bli_dotxv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_invertv_ex_qfp(dt: num_t) -> invertv_ex_vft {
                dyload_lib().bli_invertv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_invscalv_ex_qfp(dt: num_t) -> invscalv_ex_vft {
                dyload_lib().bli_invscalv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_scalv_ex_qfp(dt: num_t) -> scalv_ex_vft {
                dyload_lib().bli_scalv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_setv_ex_qfp(dt: num_t) -> setv_ex_vft {
                dyload_lib().bli_setv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_swapv_ex_qfp(dt: num_t) -> swapv_ex_vft {
                dyload_lib().bli_swapv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_xpbyv_ex_qfp(dt: num_t) -> xpbyv_ex_vft {
                dyload_lib().bli_xpbyv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_addd_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_addd_check.unwrap()(x, y)
            }

pub unsafe fn bli_copyd_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_copyd_check.unwrap()(x, y)
            }

pub unsafe fn bli_subd_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_subd_check.unwrap()(x, y)
            }

pub unsafe fn bli_axpyd_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpyd_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_scal2d_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_scal2d_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_invertd_check(x: *const obj_t) {
                dyload_lib().bli_invertd_check.unwrap()(x)
            }

pub unsafe fn bli_invscald_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_invscald_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_scald_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_scald_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_setd_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setd_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_setid_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setid_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_shiftd_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_shiftd_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_xpbyd_check(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbyd_check.unwrap()(x, beta, y)
            }

pub unsafe fn bli_l1d_xy_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1d_xy_check.unwrap()(x, y)
            }

pub unsafe fn bli_l1d_axy_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1d_axy_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_l1d_x_check(x: *const obj_t) {
                dyload_lib().bli_l1d_x_check.unwrap()(x)
            }

pub unsafe fn bli_l1d_ax_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_l1d_ax_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_addd_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_addd_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_copyd_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_copyd_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_subd_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_subd_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_axpyd_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_axpyd_ex.unwrap()(alpha, x, y, cntx, rntm)
            }

pub unsafe fn bli_scal2d_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scal2d_ex.unwrap()(alpha, x, y, cntx, rntm)
            }

pub unsafe fn bli_invertd_ex(x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_invertd_ex.unwrap()(x, cntx, rntm)
            }

pub unsafe fn bli_invscald_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_invscald_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_scald_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scald_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_setd_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_setd_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_setid_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_setid_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_shiftd_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_shiftd_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_xpbyd_ex(x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_xpbyd_ex.unwrap()(x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_addd(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_addd.unwrap()(x, y)
            }

pub unsafe fn bli_copyd(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_copyd.unwrap()(x, y)
            }

pub unsafe fn bli_subd(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_subd.unwrap()(x, y)
            }

pub unsafe fn bli_axpyd(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpyd.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_scal2d(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_scal2d.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_invertd(x: *const obj_t) {
                dyload_lib().bli_invertd.unwrap()(x)
            }

pub unsafe fn bli_invscald(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_invscald.unwrap()(alpha, x)
            }

pub unsafe fn bli_scald(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_scald.unwrap()(alpha, x)
            }

pub unsafe fn bli_setd(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setd.unwrap()(alpha, x)
            }

pub unsafe fn bli_setid(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setid.unwrap()(alpha, x)
            }

pub unsafe fn bli_shiftd(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_shiftd.unwrap()(alpha, x)
            }

pub unsafe fn bli_xpbyd(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbyd.unwrap()(x, beta, y)
            }

pub unsafe fn bli_saddd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saddd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_daddd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daddd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_caddd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caddd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zaddd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaddd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_scopyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scopyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dcopyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dcopyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ccopyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ccopyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zcopyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zcopyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ssubd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssubd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dsubd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsubd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_csubd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csubd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zsubd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsubd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_saxpyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saxpyd_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_daxpyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daxpyd_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_caxpyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caxpyd_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zaxpyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaxpyd_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_sscal2d_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sscal2d_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dscal2d_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dscal2d_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_cscal2d_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cscal2d_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zscal2d_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zscal2d_ex.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_sinvertd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sinvertd_ex.unwrap()(diagoffx, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dinvertd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dinvertd_ex.unwrap()(diagoffx, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_cinvertd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cinvertd_ex.unwrap()(diagoffx, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zinvertd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zinvertd_ex.unwrap()(diagoffx, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_sinvscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sinvscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dinvscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dinvscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_cinvscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cinvscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zinvscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zinvscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_sscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_cscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zscald_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zscald_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_ssetd_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssetd_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dsetd_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsetd_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_csetd_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csetd_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zsetd_ex(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsetd_ex.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_ssetid_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssetid_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dsetid_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsetid_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_csetid_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csetid_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zsetid_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsetid_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_sshiftd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sshiftd_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dshiftd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dshiftd_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_cshiftd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cshiftd_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zshiftd_ex(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zshiftd_ex.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_sxpbyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sxpbyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dxpbyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dxpbyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_cxpbyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cxpbyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zxpbyd_ex(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zxpbyd_ex.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_saddd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_saddd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_daddd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_daddd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_caddd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_caddd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zaddd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zaddd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_scopyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_scopyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_dcopyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dcopyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_ccopyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ccopyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zcopyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zcopyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_ssubd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ssubd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_dsubd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dsubd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_csubd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_csubd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zsubd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zsubd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_saxpyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_saxpyd.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_daxpyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_daxpyd.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_caxpyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_caxpyd.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zaxpyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zaxpyd.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_sscal2d(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_sscal2d.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_dscal2d(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dscal2d.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_cscal2d(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_cscal2d.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zscal2d(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zscal2d.unwrap()(diagoffx, diagx, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_sinvertd(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_sinvertd.unwrap()(diagoffx, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_dinvertd(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dinvertd.unwrap()(diagoffx, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_cinvertd(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_cinvertd.unwrap()(diagoffx, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_zinvertd(diagoffx: doff_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zinvertd.unwrap()(diagoffx, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_sinvscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_sinvscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dinvscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dinvscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_cinvscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_cinvscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zinvscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zinvscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_sscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_sscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_cscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_cscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zscald(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zscald.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_ssetd(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_ssetd.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dsetd(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dsetd.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_csetd(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_csetd.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zsetd(conjalpha: conj_t, diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zsetd.unwrap()(conjalpha, diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_ssetid(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_ssetid.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dsetid(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dsetid.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_csetid(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_csetid.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zsetid(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zsetid.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_sshiftd(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_sshiftd.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dshiftd(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dshiftd.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_cshiftd(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_cshiftd.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zshiftd(diagoffx: doff_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zshiftd.unwrap()(diagoffx, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_sxpbyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_sxpbyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_dxpbyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dxpbyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_cxpbyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_cxpbyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_zxpbyd(diagoffx: doff_t, diagx: diag_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zxpbyd.unwrap()(diagoffx, diagx, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_addd_ex_qfp(dt: num_t) -> addd_ex_vft {
                dyload_lib().bli_addd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_copyd_ex_qfp(dt: num_t) -> copyd_ex_vft {
                dyload_lib().bli_copyd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_subd_ex_qfp(dt: num_t) -> subd_ex_vft {
                dyload_lib().bli_subd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_axpyd_ex_qfp(dt: num_t) -> axpyd_ex_vft {
                dyload_lib().bli_axpyd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_scal2d_ex_qfp(dt: num_t) -> scal2d_ex_vft {
                dyload_lib().bli_scal2d_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_invertd_ex_qfp(dt: num_t) -> invertd_ex_vft {
                dyload_lib().bli_invertd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_invscald_ex_qfp(dt: num_t) -> invscald_ex_vft {
                dyload_lib().bli_invscald_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_scald_ex_qfp(dt: num_t) -> scald_ex_vft {
                dyload_lib().bli_scald_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_setd_ex_qfp(dt: num_t) -> setd_ex_vft {
                dyload_lib().bli_setd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_setid_ex_qfp(dt: num_t) -> setid_ex_vft {
                dyload_lib().bli_setid_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_shiftd_ex_qfp(dt: num_t) -> shiftd_ex_vft {
                dyload_lib().bli_shiftd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_xpbyd_ex_qfp(dt: num_t) -> xpbyd_ex_vft {
                dyload_lib().bli_xpbyd_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_axpy2v_check(alphax: *const obj_t, alphay: *const obj_t, x: *const obj_t, y: *const obj_t, z: *const obj_t) {
                dyload_lib().bli_axpy2v_check.unwrap()(alphax, alphay, x, y, z)
            }

pub unsafe fn bli_axpyf_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpyf_check.unwrap()(alpha, a, x, y)
            }

pub unsafe fn bli_dotaxpyv_check(alpha: *const obj_t, xt: *const obj_t, x: *const obj_t, y: *const obj_t, rho: *const obj_t, z: *const obj_t) {
                dyload_lib().bli_dotaxpyv_check.unwrap()(alpha, xt, x, y, rho, z)
            }

pub unsafe fn bli_dotxaxpyf_check(alpha: *const obj_t, at: *const obj_t, a: *const obj_t, w: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, z: *const obj_t) {
                dyload_lib().bli_dotxaxpyf_check.unwrap()(alpha, at, a, w, x, beta, y, z)
            }

pub unsafe fn bli_dotxf_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_dotxf_check.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_axpy2v_ex(alphax: *const obj_t, alphay: *const obj_t, x: *const obj_t, y: *const obj_t, z: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_axpy2v_ex.unwrap()(alphax, alphay, x, y, z, cntx, rntm)
            }

pub unsafe fn bli_axpyf_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_axpyf_ex.unwrap()(alpha, a, x, y, cntx, rntm)
            }

pub unsafe fn bli_dotaxpyv_ex(alpha: *const obj_t, xt: *const obj_t, x: *const obj_t, y: *const obj_t, rho: *const obj_t, z: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dotaxpyv_ex.unwrap()(alpha, xt, x, y, rho, z, cntx, rntm)
            }

pub unsafe fn bli_dotxaxpyf_ex(alpha: *const obj_t, at: *const obj_t, a: *const obj_t, w: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, z: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dotxaxpyf_ex.unwrap()(alpha, at, a, w, x, beta, y, z, cntx, rntm)
            }

pub unsafe fn bli_dotxf_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dotxf_ex.unwrap()(alpha, a, x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_axpy2v(alphax: *const obj_t, alphay: *const obj_t, x: *const obj_t, y: *const obj_t, z: *const obj_t) {
                dyload_lib().bli_axpy2v.unwrap()(alphax, alphay, x, y, z)
            }

pub unsafe fn bli_axpyf(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpyf.unwrap()(alpha, a, x, y)
            }

pub unsafe fn bli_dotaxpyv(alpha: *const obj_t, xt: *const obj_t, x: *const obj_t, y: *const obj_t, rho: *const obj_t, z: *const obj_t) {
                dyload_lib().bli_dotaxpyv.unwrap()(alpha, xt, x, y, rho, z)
            }

pub unsafe fn bli_dotxaxpyf(alpha: *const obj_t, at: *const obj_t, a: *const obj_t, w: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, z: *const obj_t) {
                dyload_lib().bli_dotxaxpyf.unwrap()(alpha, at, a, w, x, beta, y, z)
            }

pub unsafe fn bli_dotxf(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_dotxf.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_saxpy2v_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const f32, alphay: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, z: *mut f32, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saxpy2v_ex.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_daxpy2v_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const f64, alphay: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, z: *mut f64, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daxpy2v_ex.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_caxpy2v_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const scomplex, alphay: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, z: *mut scomplex, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caxpy2v_ex.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_zaxpy2v_ex(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const dcomplex, alphay: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, z: *mut dcomplex, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaxpy2v_ex.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_saxpyf_ex(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f32, a: *const f32, inca: inc_t, lda: inc_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saxpyf_ex.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_daxpyf_ex(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f64, a: *const f64, inca: inc_t, lda: inc_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daxpyf_ex.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_caxpyf_ex(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const scomplex, a: *const scomplex, inca: inc_t, lda: inc_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caxpyf_ex.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zaxpyf_ex(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const dcomplex, a: *const dcomplex, inca: inc_t, lda: inc_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaxpyf_ex.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy, cntx, rntm)
            }

pub unsafe fn bli_sdotaxpyv_ex(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, rho: *mut f32, z: *mut f32, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sdotaxpyv_ex.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz, cntx, rntm)
            }

pub unsafe fn bli_ddotaxpyv_ex(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, rho: *mut f64, z: *mut f64, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ddotaxpyv_ex.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz, cntx, rntm)
            }

pub unsafe fn bli_cdotaxpyv_ex(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, rho: *mut scomplex, z: *mut scomplex, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cdotaxpyv_ex.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz, cntx, rntm)
            }

pub unsafe fn bli_zdotaxpyv_ex(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, rho: *mut dcomplex, z: *mut dcomplex, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zdotaxpyv_ex.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz, cntx, rntm)
            }

pub unsafe fn bli_sdotxaxpyf_ex(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f32, a: *const f32, inca: inc_t, lda: inc_t, w: *const f32, incw: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, z: *mut f32, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sdotxaxpyf_ex.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_ddotxaxpyf_ex(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f64, a: *const f64, inca: inc_t, lda: inc_t, w: *const f64, incw: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, z: *mut f64, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ddotxaxpyf_ex.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_cdotxaxpyf_ex(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const scomplex, a: *const scomplex, inca: inc_t, lda: inc_t, w: *const scomplex, incw: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, z: *mut scomplex, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cdotxaxpyf_ex.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_zdotxaxpyf_ex(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const dcomplex, a: *const dcomplex, inca: inc_t, lda: inc_t, w: *const dcomplex, incw: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, z: *mut dcomplex, incz: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zdotxaxpyf_ex.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz, cntx, rntm)
            }

pub unsafe fn bli_sdotxf_ex(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f32, a: *const f32, inca: inc_t, lda: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sdotxf_ex.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_ddotxf_ex(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f64, a: *const f64, inca: inc_t, lda: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ddotxf_ex.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_cdotxf_ex(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const scomplex, a: *const scomplex, inca: inc_t, lda: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cdotxf_ex.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zdotxf_ex(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const dcomplex, a: *const dcomplex, inca: inc_t, lda: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zdotxf_ex.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_saxpy2v(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const f32, alphay: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, z: *mut f32, incz: inc_t) {
                dyload_lib().bli_saxpy2v.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz)
            }

pub unsafe fn bli_daxpy2v(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const f64, alphay: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, z: *mut f64, incz: inc_t) {
                dyload_lib().bli_daxpy2v.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz)
            }

pub unsafe fn bli_caxpy2v(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const scomplex, alphay: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, z: *mut scomplex, incz: inc_t) {
                dyload_lib().bli_caxpy2v.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz)
            }

pub unsafe fn bli_zaxpy2v(conjx: conj_t, conjy: conj_t, n: dim_t, alphax: *const dcomplex, alphay: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, z: *mut dcomplex, incz: inc_t) {
                dyload_lib().bli_zaxpy2v.unwrap()(conjx, conjy, n, alphax, alphay, x, incx, y, incy, z, incz)
            }

pub unsafe fn bli_saxpyf(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f32, a: *const f32, inca: inc_t, lda: inc_t, x: *const f32, incx: inc_t, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_saxpyf.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy)
            }

pub unsafe fn bli_daxpyf(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f64, a: *const f64, inca: inc_t, lda: inc_t, x: *const f64, incx: inc_t, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_daxpyf.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy)
            }

pub unsafe fn bli_caxpyf(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const scomplex, a: *const scomplex, inca: inc_t, lda: inc_t, x: *const scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_caxpyf.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy)
            }

pub unsafe fn bli_zaxpyf(conja: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const dcomplex, a: *const dcomplex, inca: inc_t, lda: inc_t, x: *const dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zaxpyf.unwrap()(conja, conjx, m, b_n, alpha, a, inca, lda, x, incx, y, incy)
            }

pub unsafe fn bli_sdotaxpyv(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, rho: *mut f32, z: *mut f32, incz: inc_t) {
                dyload_lib().bli_sdotaxpyv.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz)
            }

pub unsafe fn bli_ddotaxpyv(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, rho: *mut f64, z: *mut f64, incz: inc_t) {
                dyload_lib().bli_ddotaxpyv.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz)
            }

pub unsafe fn bli_cdotaxpyv(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, rho: *mut scomplex, z: *mut scomplex, incz: inc_t) {
                dyload_lib().bli_cdotaxpyv.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz)
            }

pub unsafe fn bli_zdotaxpyv(conjxt: conj_t, conjx: conj_t, conjy: conj_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, rho: *mut dcomplex, z: *mut dcomplex, incz: inc_t) {
                dyload_lib().bli_zdotaxpyv.unwrap()(conjxt, conjx, conjy, n, alpha, x, incx, y, incy, rho, z, incz)
            }

pub unsafe fn bli_sdotxaxpyf(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f32, a: *const f32, inca: inc_t, lda: inc_t, w: *const f32, incw: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, z: *mut f32, incz: inc_t) {
                dyload_lib().bli_sdotxaxpyf.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz)
            }

pub unsafe fn bli_ddotxaxpyf(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f64, a: *const f64, inca: inc_t, lda: inc_t, w: *const f64, incw: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, z: *mut f64, incz: inc_t) {
                dyload_lib().bli_ddotxaxpyf.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz)
            }

pub unsafe fn bli_cdotxaxpyf(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const scomplex, a: *const scomplex, inca: inc_t, lda: inc_t, w: *const scomplex, incw: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, z: *mut scomplex, incz: inc_t) {
                dyload_lib().bli_cdotxaxpyf.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz)
            }

pub unsafe fn bli_zdotxaxpyf(conjat: conj_t, conja: conj_t, conjw: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const dcomplex, a: *const dcomplex, inca: inc_t, lda: inc_t, w: *const dcomplex, incw: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, z: *mut dcomplex, incz: inc_t) {
                dyload_lib().bli_zdotxaxpyf.unwrap()(conjat, conja, conjw, conjx, m, b_n, alpha, a, inca, lda, w, incw, x, incx, beta, y, incy, z, incz)
            }

pub unsafe fn bli_sdotxf(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f32, a: *const f32, inca: inc_t, lda: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_sdotxf.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy)
            }

pub unsafe fn bli_ddotxf(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const f64, a: *const f64, inca: inc_t, lda: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_ddotxf.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy)
            }

pub unsafe fn bli_cdotxf(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const scomplex, a: *const scomplex, inca: inc_t, lda: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_cdotxf.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy)
            }

pub unsafe fn bli_zdotxf(conjat: conj_t, conjx: conj_t, m: dim_t, b_n: dim_t, alpha: *const dcomplex, a: *const dcomplex, inca: inc_t, lda: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zdotxf.unwrap()(conjat, conjx, m, b_n, alpha, a, inca, lda, x, incx, beta, y, incy)
            }

pub unsafe fn bli_axpy2v_ex_qfp(dt: num_t) -> axpy2v_ex_vft {
                dyload_lib().bli_axpy2v_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_axpyf_ex_qfp(dt: num_t) -> axpyf_ex_vft {
                dyload_lib().bli_axpyf_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_dotaxpyv_ex_qfp(dt: num_t) -> dotaxpyv_ex_vft {
                dyload_lib().bli_dotaxpyv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_dotxaxpyf_ex_qfp(dt: num_t) -> dotxaxpyf_ex_vft {
                dyload_lib().bli_dotxaxpyf_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_dotxf_ex_qfp(dt: num_t) -> dotxf_ex_vft {
                dyload_lib().bli_dotxf_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_addm_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_addm_check.unwrap()(x, y)
            }

pub unsafe fn bli_copym_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_copym_check.unwrap()(x, y)
            }

pub unsafe fn bli_subm_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_subm_check.unwrap()(x, y)
            }

pub unsafe fn bli_axpym_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpym_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_scal2m_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_scal2m_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_invscalm_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_invscalm_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_scalm_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_scalm_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_setm_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setm_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_xpbym_check(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbym_check.unwrap()(x, beta, y)
            }

pub unsafe fn bli_l1m_xy_check(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1m_xy_check.unwrap()(x, y)
            }

pub unsafe fn bli_l1m_axy_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_l1m_axy_check.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_l1m_ax_check(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_l1m_ax_check.unwrap()(alpha, x)
            }

pub unsafe fn bli_addm_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_addm_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_copym_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_copym_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_subm_ex(x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_subm_ex.unwrap()(x, y, cntx, rntm)
            }

pub unsafe fn bli_axpym_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_axpym_ex.unwrap()(alpha, x, y, cntx, rntm)
            }

pub unsafe fn bli_scal2m_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scal2m_ex.unwrap()(alpha, x, y, cntx, rntm)
            }

pub unsafe fn bli_invscalm_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_invscalm_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_scalm_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scalm_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_setm_ex(alpha: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_setm_ex.unwrap()(alpha, x, cntx, rntm)
            }

pub unsafe fn bli_xpbym_ex(x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_xpbym_ex.unwrap()(x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_xpbym_md_ex(x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_xpbym_md_ex.unwrap()(x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_addm(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_addm.unwrap()(x, y)
            }

pub unsafe fn bli_copym(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_copym.unwrap()(x, y)
            }

pub unsafe fn bli_subm(x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_subm.unwrap()(x, y)
            }

pub unsafe fn bli_axpym(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_axpym.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_scal2m(alpha: *const obj_t, x: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_scal2m.unwrap()(alpha, x, y)
            }

pub unsafe fn bli_invscalm(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_invscalm.unwrap()(alpha, x)
            }

pub unsafe fn bli_scalm(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_scalm.unwrap()(alpha, x)
            }

pub unsafe fn bli_setm(alpha: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_setm.unwrap()(alpha, x)
            }

pub unsafe fn bli_xpbym(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbym.unwrap()(x, beta, y)
            }

pub unsafe fn bli_xpbym_md(x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xpbym_md.unwrap()(x, beta, y)
            }

pub unsafe fn bli_saddm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saddm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_daddm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daddm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_caddm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caddm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zaddm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaddm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_scopym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scopym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dcopym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dcopym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ccopym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ccopym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zcopym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zcopym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ssubm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssubm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dsubm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsubm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_csubm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csubm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zsubm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsubm_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_saxpym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_saxpym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_daxpym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_daxpym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_caxpym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_caxpym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zaxpym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zaxpym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_sscal2m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sscal2m_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dscal2m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dscal2m_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_cscal2m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cscal2m_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zscal2m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zscal2m_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_sinvscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sinvscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dinvscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dinvscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_cinvscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cinvscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zinvscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zinvscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_sscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_cscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zscalm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zscalm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_ssetm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssetm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_dsetm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsetm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_csetm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csetm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zsetm_ex(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsetm_ex.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_sxpbym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sxpbym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dxpbym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dxpbym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_cxpbym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cxpbym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zxpbym_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zxpbym_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ssxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ddxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ddxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_ccxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ccxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zzxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zzxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_sdxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sdxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_scxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_scxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_szxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_szxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dsxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dcxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dcxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_dzxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dzxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_csxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_cdxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cdxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_czxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_czxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zsxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zdxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zdxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_zcxpbym_md_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zcxpbym_md_ex.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx, rntm)
            }

pub unsafe fn bli_saddm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_saddm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_daddm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_daddm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_caddm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_caddm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zaddm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zaddm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_scopym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_scopym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_dcopym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dcopym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_ccopym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ccopym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zcopym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zcopym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_ssubm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ssubm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_dsubm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dsubm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_csubm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_csubm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zsubm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zsubm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_saxpym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_saxpym.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_daxpym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_daxpym.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_caxpym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_caxpym.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zaxpym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zaxpym.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_sscal2m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_sscal2m.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_dscal2m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dscal2m.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_cscal2m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_cscal2m.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zscal2m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zscal2m.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_sinvscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_sinvscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dinvscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dinvscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_cinvscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_cinvscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zinvscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zinvscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_sscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_sscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_cscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_cscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zscalm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zscalm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_ssetm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_ssetm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_dsetm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_dsetm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_csetm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_csetm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_zsetm(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zsetm.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x)
            }

pub unsafe fn bli_sxpbym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_sxpbym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_dxpbym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dxpbym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_cxpbym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_cxpbym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_zxpbym(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zxpbym.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_ssxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ssxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_ddxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ddxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_ccxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_ccxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_zzxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zzxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_sdxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_sdxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_scxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_scxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_szxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_szxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_dsxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dsxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_dcxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dcxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_dzxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_dzxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_csxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_csxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_cdxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_cdxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_czxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, beta: *const dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_czxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_zsxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zsxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_zdxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zdxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_zcxpbym_md(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *const scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) {
                dyload_lib().bli_zcxpbym_md.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y)
            }

pub unsafe fn bli_addm_ex_qfp(dt: num_t) -> addm_ex_vft {
                dyload_lib().bli_addm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_copym_ex_qfp(dt: num_t) -> copym_ex_vft {
                dyload_lib().bli_copym_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_subm_ex_qfp(dt: num_t) -> subm_ex_vft {
                dyload_lib().bli_subm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_axpym_ex_qfp(dt: num_t) -> axpym_ex_vft {
                dyload_lib().bli_axpym_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_scal2m_ex_qfp(dt: num_t) -> scal2m_ex_vft {
                dyload_lib().bli_scal2m_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_invscalm_ex_qfp(dt: num_t) -> invscalm_ex_vft {
                dyload_lib().bli_invscalm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_scalm_ex_qfp(dt: num_t) -> scalm_ex_vft {
                dyload_lib().bli_scalm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_setm_ex_qfp(dt: num_t) -> setm_ex_vft {
                dyload_lib().bli_setm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_xpbym_ex_qfp(dt: num_t) -> xpbym_ex_vft {
                dyload_lib().bli_xpbym_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_xpbym_md_ex_qfp2(dtx: num_t, dty: num_t) -> xpbym_md_ex_vft {
                dyload_lib().bli_xpbym_md_ex_qfp2.unwrap()(dtx, dty)
            }

pub unsafe fn bli_saddm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_saddm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_daddm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_daddm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_caddm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_caddm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zaddm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zaddm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_scopym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_scopym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dcopym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dcopym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_ccopym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ccopym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zcopym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zcopym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_ssubm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ssubm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dsubm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dsubm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_csubm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_csubm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zsubm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zsubm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_saxpym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_saxpym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_daxpym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_daxpym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_caxpym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_caxpym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zaxpym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zaxpym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_sscal2m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sscal2m_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dscal2m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dscal2m_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_cscal2m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cscal2m_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zscal2m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zscal2m_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, alpha, x, rs_x, cs_x, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_sinvscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sinvscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_dinvscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dinvscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_cinvscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cinvscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_zinvscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zinvscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_sscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_dscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_cscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_zscalm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zscalm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_ssetm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ssetm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_dsetm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dsetm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_csetm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_csetm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_zsetm_unb_var1(conjalpha: conj_t, diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zsetm_unb_var1.unwrap()(conjalpha, diagoffx, diagx, uplox, m, n, alpha, x, rs_x, cs_x, cntx)
            }

pub unsafe fn bli_sxpbym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, beta: *mut f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sxpbym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dxpbym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, beta: *mut f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dxpbym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_cxpbym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cxpbym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zxpbym_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zxpbym_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_ssxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, beta: *mut f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ssxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_ddxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, beta: *mut f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ddxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_ccxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ccxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zzxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zzxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_sdxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, beta: *mut f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sdxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_scxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, beta: *mut scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_scxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_szxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, beta: *mut dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_szxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dsxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, beta: *mut f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dsxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dcxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, beta: *mut scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dcxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_dzxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, beta: *mut dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dzxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_csxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_csxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_cdxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cdxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_czxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut dcomplex, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_czxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zsxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut f32, y: *mut f32, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zsxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zdxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut f64, y: *mut f64, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zdxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_zcxpbym_md_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, beta: *mut scomplex, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zcxpbym_md_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, beta, y, rs_y, cs_y, cntx)
            }

pub unsafe fn bli_packm_alloc(size_needed: siz_t, cntl: *const cntl_t, thread: *mut thrinfo_t) -> *mut c_void {
                dyload_lib().bli_packm_alloc.unwrap()(size_needed, cntl, thread)
            }

pub unsafe fn bli_packm_alloc_ex(size_needed: siz_t, pack_buf_type: packbuf_t, thread: *mut thrinfo_t) -> *mut c_void {
                dyload_lib().bli_packm_alloc_ex.unwrap()(size_needed, pack_buf_type, thread)
            }

pub unsafe fn bli_packm_cntl_init_node(var_func: void_fp, var: packm_var_oft, params: *const c_void, cntl: *mut packm_cntl_t) {
                dyload_lib().bli_packm_cntl_init_node.unwrap()(var_func, var, params, cntl)
            }

pub unsafe fn bli_packm_def_cntl_init_node(var_func: void_fp, dt_orig: num_t, dt_pack: num_t, dt_bmult: num_t, ukr: packm_ker_ft, bmult_m_def: dim_t, bmult_m_pack: dim_t, bmult_m_bcast: dim_t, bmult_m_scale: dim_t, bmult_m_pack_scale: dim_t, bmult_n_def: dim_t, does_invert_diag: bool, rev_iter_if_upper: bool, rev_iter_if_lower: bool, pack_schema: pack_t, pack_buf_type: packbuf_t, cntl: *mut packm_def_cntl_t) {
                dyload_lib().bli_packm_def_cntl_init_node.unwrap()(var_func, dt_orig, dt_pack, dt_bmult, ukr, bmult_m_def, bmult_m_pack, bmult_m_bcast, bmult_m_scale, bmult_m_pack_scale, bmult_n_def, does_invert_diag, rev_iter_if_upper, rev_iter_if_lower, pack_schema, pack_buf_type, cntl)
            }

pub unsafe fn bli_packm_init_check(a: *const obj_t, p: *const obj_t) {
                dyload_lib().bli_packm_init_check.unwrap()(a, p)
            }

pub unsafe fn bli_packm_int_check(a: *const obj_t, p: *const obj_t) {
                dyload_lib().bli_packm_int_check.unwrap()(a, p)
            }

pub unsafe fn bli_packm_init(dt_p: num_t, a: *const obj_t, p: *mut obj_t, cntl: *const cntl_t) -> siz_t {
                dyload_lib().bli_packm_init.unwrap()(dt_p, a, p, cntl)
            }

pub unsafe fn bli_packm_int(a: *const obj_t, p: *mut obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_packm_int.unwrap()(a, p, cntx, cntl, thread)
            }

pub unsafe fn bli_packm_scalar(kappa: *mut obj_t, p: *mut obj_t) -> *mut c_void {
                dyload_lib().bli_packm_scalar.unwrap()(kappa, p)
            }

pub unsafe fn bli_sspackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_sspackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_ddpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_ddpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_ccpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_ccpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_zzpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_zzpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_sdpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_sdpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_szpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_szpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_dspackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dspackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_dcpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_dcpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_cdpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_cdpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_czpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_czpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_zspackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_zspackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_zcpackm_struc_cxk(strucc: struc_t, diagc: diag_t, uploc: uplo_t, conjc: conj_t, schema: pack_t, invdiag: bool, panel_dim: dim_t, panel_len: dim_t, panel_dim_max: dim_t, panel_len_max: dim_t, panel_dim_off: dim_t, panel_len_off: dim_t, panel_bcast: dim_t, kappa: *const c_void, c: *const c_void, incc: inc_t, ldc: inc_t, p: *mut c_void, ldp: inc_t, params: *const c_void, cntx: *const cntx_t) {
                dyload_lib().bli_zcpackm_struc_cxk.unwrap()(strucc, diagc, uploc, conjc, schema, invdiag, panel_dim, panel_len, panel_dim_max, panel_len_max, panel_dim_off, panel_len_off, panel_bcast, kappa, c, incc, ldc, p, ldp, params, cntx)
            }

pub unsafe fn bli_packm_blk_var1(c: *const obj_t, p: *mut obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_packm_blk_var1.unwrap()(c, p, cntx, cntl, thread)
            }

pub unsafe fn bli_unpackm_cntl_init_node(var_func: void_fp, unpackm_var_func: void_fp, cntl: *mut unpackm_cntl_t) {
                dyload_lib().bli_unpackm_cntl_init_node.unwrap()(var_func, unpackm_var_func, cntl)
            }

pub unsafe fn bli_unpackm_int_check(p: *const obj_t, a: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_unpackm_int_check.unwrap()(p, a, cntx)
            }

pub unsafe fn bli_unpackm_int(a: *const obj_t, p: *mut obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_unpackm_int.unwrap()(a, p, cntx, cntl, thread_par)
            }

pub unsafe fn bli_unpackm_blk_var1(p: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *const thrinfo_t) {
                dyload_lib().bli_unpackm_blk_var1.unwrap()(p, c, cntx, cntl, thread)
            }

pub unsafe fn bli_sunpackm_blk_var1(strucc: struc_t, diagoffc: doff_t, diagc: diag_t, uploc: uplo_t, transc: trans_t, m: dim_t, n: dim_t, m_panel: dim_t, n_panel: dim_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sunpackm_blk_var1.unwrap()(strucc, diagoffc, diagc, uploc, transc, m, n, m_panel, n_panel, p, rs_p, cs_p, pd_p, ps_p, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dunpackm_blk_var1(strucc: struc_t, diagoffc: doff_t, diagc: diag_t, uploc: uplo_t, transc: trans_t, m: dim_t, n: dim_t, m_panel: dim_t, n_panel: dim_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dunpackm_blk_var1.unwrap()(strucc, diagoffc, diagc, uploc, transc, m, n, m_panel, n_panel, p, rs_p, cs_p, pd_p, ps_p, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cunpackm_blk_var1(strucc: struc_t, diagoffc: doff_t, diagc: diag_t, uploc: uplo_t, transc: trans_t, m: dim_t, n: dim_t, m_panel: dim_t, n_panel: dim_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cunpackm_blk_var1.unwrap()(strucc, diagoffc, diagc, uploc, transc, m, n, m_panel, n_panel, p, rs_p, cs_p, pd_p, ps_p, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zunpackm_blk_var1(strucc: struc_t, diagoffc: doff_t, diagc: diag_t, uploc: uplo_t, transc: trans_t, m: dim_t, n: dim_t, m_panel: dim_t, n_panel: dim_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zunpackm_blk_var1.unwrap()(strucc, diagoffc, diagc, uploc, transc, m, n, m_panel, n_panel, p, rs_p, cs_p, pd_p, ps_p, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_gemv_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_gemv_check.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_hemv_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_hemv_check.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_symv_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_symv_check.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_ger_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_ger_check.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_her2_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_her2_check.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_syr2_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_syr2_check.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_her_check(alpha: *const obj_t, x: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_her_check.unwrap()(alpha, x, a)
            }

pub unsafe fn bli_syr_check(alpha: *const obj_t, x: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_syr_check.unwrap()(alpha, x, a)
            }

pub unsafe fn bli_trmv_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_trmv_check.unwrap()(alpha, a, x)
            }

pub unsafe fn bli_trsv_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_trsv_check.unwrap()(alpha, a, x)
            }

pub unsafe fn bli_xxmv_check(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_xxmv_check.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_xxr_check(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_xxr_check.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_gemv_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_gemv_ex.unwrap()(alpha, a, x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_hemv_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_hemv_ex.unwrap()(alpha, a, x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_symv_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_symv_ex.unwrap()(alpha, a, x, beta, y, cntx, rntm)
            }

pub unsafe fn bli_ger_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ger_ex.unwrap()(alpha, x, y, a, cntx, rntm)
            }

pub unsafe fn bli_her2_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_her2_ex.unwrap()(alpha, x, y, a, cntx, rntm)
            }

pub unsafe fn bli_syr2_ex(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_syr2_ex.unwrap()(alpha, x, y, a, cntx, rntm)
            }

pub unsafe fn bli_her_ex(alpha: *const obj_t, x: *const obj_t, a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_her_ex.unwrap()(alpha, x, a, cntx, rntm)
            }

pub unsafe fn bli_syr_ex(alpha: *const obj_t, x: *const obj_t, a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_syr_ex.unwrap()(alpha, x, a, cntx, rntm)
            }

pub unsafe fn bli_trmv_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_trmv_ex.unwrap()(alpha, a, x, cntx, rntm)
            }

pub unsafe fn bli_trsv_ex(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_trsv_ex.unwrap()(alpha, a, x, cntx, rntm)
            }

pub unsafe fn bli_gemv(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_gemv.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_hemv(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_hemv.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_symv(alpha: *const obj_t, a: *const obj_t, x: *const obj_t, beta: *const obj_t, y: *const obj_t) {
                dyload_lib().bli_symv.unwrap()(alpha, a, x, beta, y)
            }

pub unsafe fn bli_ger(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_ger.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_her2(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_her2.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_syr2(alpha: *const obj_t, x: *const obj_t, y: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_syr2.unwrap()(alpha, x, y, a)
            }

pub unsafe fn bli_her(alpha: *const obj_t, x: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_her.unwrap()(alpha, x, a)
            }

pub unsafe fn bli_syr(alpha: *const obj_t, x: *const obj_t, a: *const obj_t) {
                dyload_lib().bli_syr.unwrap()(alpha, x, a)
            }

pub unsafe fn bli_trmv(alpha: *const obj_t, a: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_trmv.unwrap()(alpha, a, x)
            }

pub unsafe fn bli_trsv(alpha: *const obj_t, a: *const obj_t, x: *const obj_t) {
                dyload_lib().bli_trsv.unwrap()(alpha, a, x)
            }

pub unsafe fn bli_sgemv_ex(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sgemv_ex.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dgemv_ex(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dgemv_ex.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_cgemv_ex(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cgemv_ex.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zgemv_ex(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zgemv_ex.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_sger_ex(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sger_ex.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dger_ex(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dger_ex.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cger_ex(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cger_ex.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zger_ex(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zger_ex.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_shemv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_shemv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dhemv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dhemv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_chemv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_chemv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zhemv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zhemv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_ssymv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssymv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_dsymv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsymv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_csymv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csymv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_zsymv_ex(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsymv_ex.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx, rntm)
            }

pub unsafe fn bli_sher_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sher_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dher_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dher_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cher_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f32, x: *const scomplex, incx: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cher_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zher_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f64, x: *const dcomplex, incx: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zher_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_ssyr_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssyr_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dsyr_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsyr_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_csyr_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csyr_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zsyr_ex(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsyr_ex.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_sher2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sher2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dher2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dher2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cher2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cher2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zher2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zher2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_ssyr2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssyr2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dsyr2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsyr2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_csyr2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csyr2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zsyr2_ex(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsyr2_ex.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_strmv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_strmv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_dtrmv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dtrmv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_ctrmv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ctrmv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_ztrmv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ztrmv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_strsv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_strsv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_dtrsv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dtrsv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_ctrsv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ctrsv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_ztrsv_ex(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ztrsv_ex.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx, rntm)
            }

pub unsafe fn bli_sgemv(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_sgemv.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_dgemv(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dgemv.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_cgemv(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_cgemv.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_zgemv(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zgemv.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_sger(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_sger.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_dger(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dger.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_cger(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_cger.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_zger(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zger.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_shemv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_shemv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_dhemv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dhemv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_chemv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_chemv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_zhemv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zhemv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_ssymv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *const f32, incx: inc_t, beta: *const f32, y: *mut f32, incy: inc_t) {
                dyload_lib().bli_ssymv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_dsymv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *const f64, incx: inc_t, beta: *const f64, y: *mut f64, incy: inc_t) {
                dyload_lib().bli_dsymv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_csymv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *const scomplex, incx: inc_t, beta: *const scomplex, y: *mut scomplex, incy: inc_t) {
                dyload_lib().bli_csymv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_zsymv(uploa: uplo_t, conja: conj_t, conjx: conj_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *const dcomplex, incx: inc_t, beta: *const dcomplex, y: *mut dcomplex, incy: inc_t) {
                dyload_lib().bli_zsymv.unwrap()(uploa, conja, conjx, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy)
            }

pub unsafe fn bli_sher(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_sher.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_dher(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dher.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_cher(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f32, x: *const scomplex, incx: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_cher.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_zher(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f64, x: *const dcomplex, incx: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zher.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_ssyr(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_ssyr.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_dsyr(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dsyr.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_csyr(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_csyr.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_zsyr(uploa: uplo_t, conjx: conj_t, m: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zsyr.unwrap()(uploa, conjx, m, alpha, x, incx, a, rs_a, cs_a)
            }

pub unsafe fn bli_sher2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_sher2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_dher2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dher2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_cher2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_cher2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_zher2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zher2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_ssyr2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f32, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_ssyr2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_dsyr2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const f64, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dsyr2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_csyr2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const scomplex, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_csyr2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_zsyr2(uploa: uplo_t, conjx: conj_t, conjy: conj_t, m: dim_t, alpha: *const dcomplex, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zsyr2.unwrap()(uploa, conjx, conjy, m, alpha, x, incx, y, incy, a, rs_a, cs_a)
            }

pub unsafe fn bli_strmv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_strmv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_dtrmv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_dtrmv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_ctrmv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_ctrmv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_ztrmv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_ztrmv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_strsv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_strsv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_dtrsv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_dtrsv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_ctrsv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_ctrsv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_ztrsv(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_ztrsv.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx)
            }

pub unsafe fn bli_gemv_ex_qfp(dt: num_t) -> gemv_ex_vft {
                dyload_lib().bli_gemv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_ger_ex_qfp(dt: num_t) -> ger_ex_vft {
                dyload_lib().bli_ger_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_ex_qfp(dt: num_t) -> hemv_ex_vft {
                dyload_lib().bli_hemv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_symv_ex_qfp(dt: num_t) -> symv_ex_vft {
                dyload_lib().bli_symv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her_ex_qfp(dt: num_t) -> her_ex_vft {
                dyload_lib().bli_her_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_syr_ex_qfp(dt: num_t) -> syr_ex_vft {
                dyload_lib().bli_syr_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_ex_qfp(dt: num_t) -> her2_ex_vft {
                dyload_lib().bli_her2_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_syr2_ex_qfp(dt: num_t) -> syr2_ex_vft {
                dyload_lib().bli_syr2_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trmv_ex_qfp(dt: num_t) -> trmv_ex_vft {
                dyload_lib().bli_trmv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsv_ex_qfp(dt: num_t) -> trsv_ex_vft {
                dyload_lib().bli_trsv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemv_unb_var1_qfp(dt: num_t) -> gemv_unb_vft {
                dyload_lib().bli_gemv_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemv_unb_var2_qfp(dt: num_t) -> gemv_unb_vft {
                dyload_lib().bli_gemv_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemv_unf_var1_qfp(dt: num_t) -> gemv_unb_vft {
                dyload_lib().bli_gemv_unf_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemv_unf_var2_qfp(dt: num_t) -> gemv_unb_vft {
                dyload_lib().bli_gemv_unf_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_ger_unb_var1_qfp(dt: num_t) -> ger_unb_vft {
                dyload_lib().bli_ger_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_ger_unb_var2_qfp(dt: num_t) -> ger_unb_vft {
                dyload_lib().bli_ger_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unb_var1_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unb_var2_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unb_var3_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unb_var3_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unb_var4_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unb_var4_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unf_var1_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unf_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unf_var3_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unf_var3_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unf_var1a_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unf_var1a_qfp.unwrap()(dt)
            }

pub unsafe fn bli_hemv_unf_var3a_qfp(dt: num_t) -> hemv_unb_vft {
                dyload_lib().bli_hemv_unf_var3a_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her_unb_var1_qfp(dt: num_t) -> her_unb_vft {
                dyload_lib().bli_her_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her_unb_var2_qfp(dt: num_t) -> her_unb_vft {
                dyload_lib().bli_her_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_unb_var1_qfp(dt: num_t) -> her2_unb_vft {
                dyload_lib().bli_her2_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_unb_var2_qfp(dt: num_t) -> her2_unb_vft {
                dyload_lib().bli_her2_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_unb_var3_qfp(dt: num_t) -> her2_unb_vft {
                dyload_lib().bli_her2_unb_var3_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_unb_var4_qfp(dt: num_t) -> her2_unb_vft {
                dyload_lib().bli_her2_unb_var4_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_unf_var1_qfp(dt: num_t) -> her2_unb_vft {
                dyload_lib().bli_her2_unf_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_her2_unf_var4_qfp(dt: num_t) -> her2_unb_vft {
                dyload_lib().bli_her2_unf_var4_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trmv_unb_var1_qfp(dt: num_t) -> trmv_unb_vft {
                dyload_lib().bli_trmv_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trmv_unb_var2_qfp(dt: num_t) -> trmv_unb_vft {
                dyload_lib().bli_trmv_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trmv_unf_var1_qfp(dt: num_t) -> trmv_unb_vft {
                dyload_lib().bli_trmv_unf_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trmv_unf_var2_qfp(dt: num_t) -> trmv_unb_vft {
                dyload_lib().bli_trmv_unf_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsv_unb_var1_qfp(dt: num_t) -> trsv_unb_vft {
                dyload_lib().bli_trsv_unb_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsv_unb_var2_qfp(dt: num_t) -> trsv_unb_vft {
                dyload_lib().bli_trsv_unb_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsv_unf_var1_qfp(dt: num_t) -> trsv_unb_vft {
                dyload_lib().bli_trsv_unf_var1_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsv_unf_var2_qfp(dt: num_t) -> trsv_unb_vft {
                dyload_lib().bli_trsv_unf_var2_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemv_blk_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_gemv_blk_var1.unwrap()(alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_gemv_blk_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_gemv_blk_var2.unwrap()(alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_gemv_unb_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_gemv_unb_var1.unwrap()(alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_gemv_unb_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_gemv_unb_var2.unwrap()(alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_gemv_unf_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_gemv_unf_var1.unwrap()(alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_gemv_unf_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_gemv_unf_var2.unwrap()(alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_sgemv_unb_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sgemv_unb_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dgemv_unb_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dgemv_unb_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_cgemv_unb_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cgemv_unb_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zgemv_unb_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zgemv_unb_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_sgemv_unb_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sgemv_unb_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dgemv_unb_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dgemv_unb_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_cgemv_unb_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cgemv_unb_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zgemv_unb_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zgemv_unb_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_sgemv_unf_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sgemv_unf_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dgemv_unf_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dgemv_unf_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_cgemv_unf_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cgemv_unf_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zgemv_unf_var1(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zgemv_unf_var1.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_sgemv_unf_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sgemv_unf_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dgemv_unf_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dgemv_unf_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_cgemv_unf_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cgemv_unf_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zgemv_unf_var2(transa: trans_t, conjx: conj_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zgemv_unf_var2.unwrap()(transa, conjx, m, n, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_ger_blk_var1(alpha: *mut obj_t, x: *mut obj_t, y: *mut obj_t, a: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_ger_blk_var1.unwrap()(alpha, x, y, a, cntx, cntl)
            }

pub unsafe fn bli_ger_blk_var2(alpha: *mut obj_t, x: *mut obj_t, y: *mut obj_t, a: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_ger_blk_var2.unwrap()(alpha, x, y, a, cntx, cntl)
            }

pub unsafe fn bli_ger_unb_var1(alpha: *mut obj_t, x: *mut obj_t, y: *mut obj_t, a: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_ger_unb_var1.unwrap()(alpha, x, y, a, cntx, cntl)
            }

pub unsafe fn bli_ger_unb_var2(alpha: *mut obj_t, x: *mut obj_t, y: *mut obj_t, a: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_ger_unb_var2.unwrap()(alpha, x, y, a, cntx, cntl)
            }

pub unsafe fn bli_sger_unb_var1(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sger_unb_var1.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_dger_unb_var1(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dger_unb_var1.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_cger_unb_var1(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cger_unb_var1.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_zger_unb_var1(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zger_unb_var1.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_sger_unb_var2(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sger_unb_var2.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_dger_unb_var2(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dger_unb_var2.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_cger_unb_var2(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cger_unb_var2.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_zger_unb_var2(conjx: conj_t, conjy: conj_t, m: dim_t, n: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zger_unb_var2.unwrap()(conjx, conjy, m, n, alpha, x, incx, y, incy, a, rs_a, cs_a, cntx)
            }

pub unsafe fn bli_hemv_blk_var1(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_blk_var1.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_blk_var2(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_blk_var2.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_blk_var3(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_blk_var3.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_blk_var4(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_blk_var4.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unb_var1(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unb_var1.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unb_var2(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unb_var2.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unb_var3(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unb_var3.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unb_var4(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unb_var4.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unf_var1(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unf_var1.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unf_var3(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unf_var3.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unf_var1a(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unf_var1a.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_hemv_unf_var3a(conjh: conj_t, alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, beta: *mut obj_t, y: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_hemv_unf_var3a.unwrap()(conjh, alpha, a, x, beta, y, cntx, cntl)
            }

pub unsafe fn bli_shemv_unb_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unb_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unb_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unb_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unb_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unb_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unb_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unb_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unb_var2(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unb_var2.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unb_var2(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unb_var2.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unb_var2(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unb_var2.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unb_var2(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unb_var2.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unb_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unb_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unb_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unb_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unb_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unb_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unb_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unb_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unb_var4(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unb_var4.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unb_var4(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unb_var4.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unb_var4(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unb_var4.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unb_var4(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unb_var4.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unf_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unf_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unf_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unf_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unf_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unf_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unf_var1(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unf_var1.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unf_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unf_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unf_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unf_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unf_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unf_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unf_var3(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unf_var3.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unf_var1a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unf_var1a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unf_var1a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unf_var1a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unf_var1a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unf_var1a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unf_var1a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unf_var1a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_shemv_unf_var3a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, beta: *mut f32, y: *mut f32, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_shemv_unf_var3a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_dhemv_unf_var3a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, beta: *mut f64, y: *mut f64, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dhemv_unf_var3a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_chemv_unf_var3a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, beta: *mut scomplex, y: *mut scomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_chemv_unf_var3a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_zhemv_unf_var3a(uplo: uplo_t, conja: conj_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, beta: *mut dcomplex, y: *mut dcomplex, incy: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zhemv_unf_var3a.unwrap()(uplo, conja, conjx, conjh, m, alpha, a, rs_a, cs_a, x, incx, beta, y, incy, cntx)
            }

pub unsafe fn bli_her_blk_var1(conjh: conj_t, alpha: *mut obj_t, x: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her_blk_var1.unwrap()(conjh, alpha, x, c, cntx, cntl)
            }

pub unsafe fn bli_her_blk_var2(conjh: conj_t, alpha: *mut obj_t, x: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her_blk_var2.unwrap()(conjh, alpha, x, c, cntx, cntl)
            }

pub unsafe fn bli_her_unb_var1(conjh: conj_t, alpha: *mut obj_t, x: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her_unb_var1.unwrap()(conjh, alpha, x, c, cntx, cntl)
            }

pub unsafe fn bli_her_unb_var2(conjh: conj_t, alpha: *mut obj_t, x: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her_unb_var2.unwrap()(conjh, alpha, x, c, cntx, cntl)
            }

pub unsafe fn bli_sher_unb_var1(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher_unb_var1.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher_unb_var1(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher_unb_var1.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher_unb_var1(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher_unb_var1.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher_unb_var1(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher_unb_var1.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_sher_unb_var2(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher_unb_var2.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher_unb_var2(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher_unb_var2.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher_unb_var2(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher_unb_var2.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher_unb_var2(uplo: uplo_t, conjx: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher_unb_var2.unwrap()(uplo, conjx, conjh, m, alpha, x, incx, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_her2_blk_var1(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_blk_var1.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_blk_var2(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_blk_var2.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_blk_var3(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_blk_var3.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_blk_var4(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_blk_var4.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_unb_var1(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_unb_var1.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_unb_var2(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_unb_var2.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_unb_var3(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_unb_var3.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_unb_var4(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_unb_var4.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_unf_var1(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_unf_var1.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_her2_unf_var4(conjh: conj_t, alpha: *mut obj_t, alpha_conj: *mut obj_t, x: *mut obj_t, y: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_her2_unf_var4.unwrap()(conjh, alpha, alpha_conj, x, y, c, cntx, cntl)
            }

pub unsafe fn bli_sher2_unb_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher2_unb_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher2_unb_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher2_unb_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher2_unb_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher2_unb_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher2_unb_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher2_unb_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_sher2_unb_var2(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher2_unb_var2.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher2_unb_var2(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher2_unb_var2.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher2_unb_var2(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher2_unb_var2.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher2_unb_var2(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher2_unb_var2.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_sher2_unb_var3(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher2_unb_var3.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher2_unb_var3(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher2_unb_var3.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher2_unb_var3(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher2_unb_var3.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher2_unb_var3(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher2_unb_var3.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_sher2_unb_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher2_unb_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher2_unb_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher2_unb_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher2_unb_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher2_unb_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher2_unb_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher2_unb_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_sher2_unf_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher2_unf_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher2_unf_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher2_unf_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher2_unf_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher2_unf_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher2_unf_var1(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher2_unf_var1.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_sher2_unf_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f32, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_sher2_unf_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_dher2_unf_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut f64, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dher2_unf_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_cher2_unf_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut scomplex, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_cher2_unf_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_zher2_unf_var4(uplo: uplo_t, conjx: conj_t, conjy: conj_t, conjh: conj_t, m: dim_t, alpha: *mut dcomplex, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_zher2_unf_var4.unwrap()(uplo, conjx, conjy, conjh, m, alpha, x, incx, y, incy, c, rs_c, cs_c, cntx)
            }

pub unsafe fn bli_trmv_l_blk_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_l_blk_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_l_blk_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_l_blk_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_u_blk_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_u_blk_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_u_blk_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_u_blk_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_unb_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_unb_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_unb_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_unb_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_unf_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_unf_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trmv_unf_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trmv_unf_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_strmv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strmv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrmv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrmv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrmv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrmv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrmv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrmv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_strmv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strmv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrmv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrmv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrmv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrmv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrmv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrmv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_strmv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strmv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrmv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrmv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrmv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrmv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrmv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrmv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_strmv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strmv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrmv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrmv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrmv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrmv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrmv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrmv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_trsv_l_blk_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_l_blk_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_l_blk_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_l_blk_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_u_blk_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_u_blk_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_u_blk_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_u_blk_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_unb_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_unb_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_unb_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_unb_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_unf_var1(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_unf_var1.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_trsv_unf_var2(alpha: *mut obj_t, a: *mut obj_t, x: *mut obj_t, cntx: *mut cntx_t, cntl: *mut cntl_t) {
                dyload_lib().bli_trsv_unf_var2.unwrap()(alpha, a, x, cntx, cntl)
            }

pub unsafe fn bli_strsv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strsv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrsv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrsv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrsv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrsv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrsv_unb_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrsv_unb_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_strsv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strsv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrsv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrsv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrsv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrsv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrsv_unb_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrsv_unb_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_strsv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strsv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrsv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrsv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrsv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrsv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrsv_unf_var1(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrsv_unf_var1.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_strsv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f32, a: *mut f32, rs_a: inc_t, cs_a: inc_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_strsv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_dtrsv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut f64, a: *mut f64, rs_a: inc_t, cs_a: inc_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_dtrsv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ctrsv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut scomplex, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ctrsv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_ztrsv_unf_var2(uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, alpha: *mut dcomplex, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t) {
                dyload_lib().bli_ztrsv_unf_var2.unwrap()(uploa, transa, diaga, m, alpha, a, rs_a, cs_a, x, incx, cntx)
            }

pub unsafe fn bli_l3_return_early_if_trivial(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) -> err_t {
                dyload_lib().bli_l3_return_early_if_trivial.unwrap()(alpha, a, b, beta, c)
            }

pub unsafe fn bli_l3_thrinfo_create(id: dim_t, gl_comm: *mut thrcomm_t, array: *mut array_t, rntm: *const rntm_t, cntl: *const cntl_t) -> *mut thrinfo_t {
                dyload_lib().bli_l3_thrinfo_create.unwrap()(id, gl_comm, array, rntm, cntl)
            }

pub unsafe fn bli_l3_thrinfo_grow(thread: *mut thrinfo_t, rntm: *const rntm_t, cntl: *const cntl_t) {
                dyload_lib().bli_l3_thrinfo_grow.unwrap()(thread, rntm, cntl)
            }

pub unsafe fn bli_l3_sup_thrinfo_create(id: dim_t, gl_comm: *mut thrcomm_t, pool: *mut pool_t, rntm: *const rntm_t) -> *mut thrinfo_t {
                dyload_lib().bli_l3_sup_thrinfo_create.unwrap()(id, gl_comm, pool, rntm)
            }

pub unsafe fn bli_l3_sup_thrinfo_update(rntm: *const rntm_t, root: *mut *mut thrinfo_t) {
                dyload_lib().bli_l3_sup_thrinfo_update.unwrap()(rntm, root)
            }

pub unsafe fn bli_l3_thrinfo_print_gemm_paths(threads: *mut *mut thrinfo_t) {
                dyload_lib().bli_l3_thrinfo_print_gemm_paths.unwrap()(threads)
            }

pub unsafe fn bli_l3_thrinfo_print_trsm_paths(threads: *mut *mut thrinfo_t) {
                dyload_lib().bli_l3_thrinfo_print_trsm_paths.unwrap()(threads)
            }

pub unsafe fn bli_l3_thrinfo_free_paths(threads: *mut *mut thrinfo_t) {
                dyload_lib().bli_l3_thrinfo_free_paths.unwrap()(threads)
            }

pub unsafe fn bli_l3_thread_decorator(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, rntm: *const rntm_t) {
                dyload_lib().bli_l3_thread_decorator.unwrap()(a, b, c, cntx, cntl, rntm)
            }

pub unsafe fn bli_l3_thread_decorator_check(rntm: *const rntm_t) {
                dyload_lib().bli_l3_thread_decorator_check.unwrap()(rntm)
            }

pub unsafe fn bli_l3_thread_decorator_thread_check(gl_comm: *mut thrcomm_t, rntm: *mut rntm_t) {
                dyload_lib().bli_l3_thread_decorator_thread_check.unwrap()(gl_comm, rntm)
            }

pub unsafe fn bli_l3_sup_thread_decorator(func: l3supint_ft, family: opid_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) -> err_t {
                dyload_lib().bli_l3_sup_thread_decorator.unwrap()(func, family, alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_gemm_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_gemm_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_gemmt_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_gemmt_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_her2k_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_her2k_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_syr2k_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_syr2k_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_hemm_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_hemm_check.unwrap()(side, alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_symm_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_symm_check.unwrap()(side, alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_trmm3_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_trmm3_check.unwrap()(side, alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_herk_check(alpha: *const obj_t, a: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_herk_check.unwrap()(alpha, a, beta, c, cntx)
            }

pub unsafe fn bli_syrk_check(alpha: *const obj_t, a: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_syrk_check.unwrap()(alpha, a, beta, c, cntx)
            }

pub unsafe fn bli_trmm_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_trmm_check.unwrap()(side, alpha, a, b, cntx)
            }

pub unsafe fn bli_trsm_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_trsm_check.unwrap()(side, alpha, a, b, cntx)
            }

pub unsafe fn bli_gemm_basic_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_gemm_basic_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_gemmt_basic_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_gemmt_basic_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_hemm_basic_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_hemm_basic_check.unwrap()(side, alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_trsm_basic_check(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_trsm_basic_check.unwrap()(side, alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_herk_basic_check(alpha: *const obj_t, a: *const obj_t, ah: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_herk_basic_check.unwrap()(alpha, a, ah, beta, c, cntx)
            }

pub unsafe fn bli_her2k_basic_check(alpha: *const obj_t, a: *const obj_t, bh: *const obj_t, b: *const obj_t, ah: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_her2k_basic_check.unwrap()(alpha, a, bh, b, ah, beta, c, cntx)
            }

pub unsafe fn bli_l3_basic_check(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t) {
                dyload_lib().bli_l3_basic_check.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_l3_packa(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_l3_packa.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_l3_packb(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_l3_packb.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_l3_prune_unref_mparts_m(a: *mut obj_t, b: *const obj_t, c: *mut obj_t) {
                dyload_lib().bli_l3_prune_unref_mparts_m.unwrap()(a, b, c)
            }

pub unsafe fn bli_l3_prune_unref_mparts_n(a: *const obj_t, b: *mut obj_t, c: *mut obj_t) {
                dyload_lib().bli_l3_prune_unref_mparts_n.unwrap()(a, b, c)
            }

pub unsafe fn bli_l3_prune_unref_mparts_k(a: *mut obj_t, b: *mut obj_t, c: *const obj_t) {
                dyload_lib().bli_l3_prune_unref_mparts_k.unwrap()(a, b, c)
            }

pub unsafe fn bli_gemm(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_gemm.unwrap()(alpha, a, b, beta, c)
            }

pub unsafe fn bli_gemmt(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_gemmt.unwrap()(alpha, a, b, beta, c)
            }

pub unsafe fn bli_her2k(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_her2k.unwrap()(alpha, a, b, beta, c)
            }

pub unsafe fn bli_syr2k(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_syr2k.unwrap()(alpha, a, b, beta, c)
            }

pub unsafe fn bli_hemm(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_hemm.unwrap()(side, alpha, a, b, beta, c)
            }

pub unsafe fn bli_symm(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_symm.unwrap()(side, alpha, a, b, beta, c)
            }

pub unsafe fn bli_trmm3(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_trmm3.unwrap()(side, alpha, a, b, beta, c)
            }

pub unsafe fn bli_herk(alpha: *const obj_t, a: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_herk.unwrap()(alpha, a, beta, c)
            }

pub unsafe fn bli_syrk(alpha: *const obj_t, a: *const obj_t, beta: *const obj_t, c: *const obj_t) {
                dyload_lib().bli_syrk.unwrap()(alpha, a, beta, c)
            }

pub unsafe fn bli_trmm(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_trmm.unwrap()(side, alpha, a, b)
            }

pub unsafe fn bli_trsm(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t) {
                dyload_lib().bli_trsm.unwrap()(side, alpha, a, b)
            }

pub unsafe fn bli_gemm_ex(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_gemm_ex.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_gemmt_ex(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_gemmt_ex.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_her2k_ex(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_her2k_ex.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_syr2k_ex(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_syr2k_ex.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_hemm_ex(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_hemm_ex.unwrap()(side, alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_symm_ex(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_symm_ex.unwrap()(side, alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_trmm3_ex(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_trmm3_ex.unwrap()(side, alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_herk_ex(alpha: *const obj_t, a: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_herk_ex.unwrap()(alpha, a, beta, c, cntx, rntm)
            }

pub unsafe fn bli_syrk_ex(alpha: *const obj_t, a: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_syrk_ex.unwrap()(alpha, a, beta, c, cntx, rntm)
            }

pub unsafe fn bli_trmm_ex(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_trmm_ex.unwrap()(side, alpha, a, b, cntx, rntm)
            }

pub unsafe fn bli_trsm_ex(side: side_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_trsm_ex.unwrap()(side, alpha, a, b, cntx, rntm)
            }

pub unsafe fn bli_sgemm(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_sgemm.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dgemm(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dgemm.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_cgemm(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_cgemm.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zgemm(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zgemm.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_shemm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_shemm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dhemm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dhemm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_chemm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_chemm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zhemm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zhemm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_ssymm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_ssymm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dsymm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dsymm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_csymm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_csymm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zsymm(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zsymm.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_sherk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_sherk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dherk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dherk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_cherk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, beta: *const f32, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_cherk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zherk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, beta: *const f64, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zherk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_sher2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_sher2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dher2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dher2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_cher2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_cher2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zher2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zher2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_ssyrk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_ssyrk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dsyrk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dsyrk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_csyrk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_csyrk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zsyrk(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zsyrk.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_sgemmt(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_sgemmt.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dgemmt(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dgemmt.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_cgemmt(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_cgemmt.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zgemmt(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zgemmt.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_ssyr2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_ssyr2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dsyr2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dsyr2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_csyr2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_csyr2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_zsyr2k(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_zsyr2k.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_strmm3(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_strmm3.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_dtrmm3(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_dtrmm3.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_ctrmm3(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_ctrmm3.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_ztrmm3(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t) {
                dyload_lib().bli_ztrmm3.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c)
            }

pub unsafe fn bli_strmm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *mut f32, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_strmm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dtrmm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *mut f64, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dtrmm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_ctrmm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *mut scomplex, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_ctrmm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_ztrmm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *mut dcomplex, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_ztrmm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_strsm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *mut f32, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_strsm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_dtrsm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *mut f64, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_dtrsm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_ctrsm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *mut scomplex, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_ctrsm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_ztrsm(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *mut dcomplex, rs_b: inc_t, cs_b: inc_t) {
                dyload_lib().bli_ztrsm.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b)
            }

pub unsafe fn bli_sgemm_ex(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sgemm_ex.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dgemm_ex(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dgemm_ex.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_cgemm_ex(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cgemm_ex.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zgemm_ex(transa: trans_t, transb: trans_t, m: dim_t, n: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zgemm_ex.unwrap()(transa, transb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_shemm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_shemm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dhemm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dhemm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_chemm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_chemm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zhemm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zhemm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_ssymm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssymm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dsymm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsymm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_csymm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csymm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zsymm_ex(side: side_t, uploa: uplo_t, conja: conj_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsymm_ex.unwrap()(side, uploa, conja, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_sherk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sherk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dherk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dherk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_cherk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, beta: *const f32, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cherk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zherk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, beta: *const f64, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zherk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_sher2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sher2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dher2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dher2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_cher2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cher2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zher2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zher2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_ssyrk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssyrk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dsyrk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsyrk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_csyrk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csyrk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zsyrk_ex(uploc: uplo_t, transa: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsyrk_ex.unwrap()(uploc, transa, m, k, alpha, a, rs_a, cs_a, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_sgemmt_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sgemmt_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dgemmt_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dgemmt_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_cgemmt_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cgemmt_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zgemmt_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zgemmt_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_ssyr2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssyr2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dsyr2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsyr2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_csyr2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csyr2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_zsyr2k_ex(uploc: uplo_t, transa: trans_t, transb: trans_t, m: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsyr2k_ex.unwrap()(uploc, transa, transb, m, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_strmm3_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *const f32, rs_b: inc_t, cs_b: inc_t, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_strmm3_ex.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_dtrmm3_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *const f64, rs_b: inc_t, cs_b: inc_t, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dtrmm3_ex.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_ctrmm3_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *const scomplex, rs_b: inc_t, cs_b: inc_t, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ctrmm3_ex.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_ztrmm3_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, transb: trans_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *const dcomplex, rs_b: inc_t, cs_b: inc_t, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ztrmm3_ex.unwrap()(side, uploa, transa, diaga, transb, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, cntx, rntm)
            }

pub unsafe fn bli_strmm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *mut f32, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_strmm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_dtrmm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *mut f64, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dtrmm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_ctrmm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *mut scomplex, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ctrmm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_ztrmm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *mut dcomplex, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ztrmm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_strsm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f32, a: *const f32, rs_a: inc_t, cs_a: inc_t, b: *mut f32, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_strsm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_dtrsm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const f64, a: *const f64, rs_a: inc_t, cs_a: inc_t, b: *mut f64, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dtrsm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_ctrsm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const scomplex, a: *const scomplex, rs_a: inc_t, cs_a: inc_t, b: *mut scomplex, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ctrsm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_ztrsm_ex(side: side_t, uploa: uplo_t, transa: trans_t, diaga: diag_t, m: dim_t, n: dim_t, alpha: *const dcomplex, a: *const dcomplex, rs_a: inc_t, cs_a: inc_t, b: *mut dcomplex, rs_b: inc_t, cs_b: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ztrsm_ex.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, rs_a, cs_a, b, rs_b, cs_b, cntx, rntm)
            }

pub unsafe fn bli_gemmsup(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) -> err_t {
                dyload_lib().bli_gemmsup.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_gemmtsup(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) -> err_t {
                dyload_lib().bli_gemmtsup.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_gemmsup_ref(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *mut rntm_t) -> err_t {
                dyload_lib().bli_gemmsup_ref.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_gemmtsup_ref(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *mut rntm_t) -> err_t {
                dyload_lib().bli_gemmtsup_ref.unwrap()(alpha, a, b, beta, c, cntx, rntm)
            }

pub unsafe fn bli_gemmsup_int(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t, thread: *mut thrinfo_t) -> err_t {
                dyload_lib().bli_gemmsup_int.unwrap()(alpha, a, b, beta, c, cntx, rntm, thread)
            }

pub unsafe fn bli_gemmtsup_int(alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t, thread: *mut thrinfo_t) -> err_t {
                dyload_lib().bli_gemmtsup_int.unwrap()(alpha, a, b, beta, c, cntx, rntm, thread)
            }

pub unsafe fn bli_gemmsup_ref_var1(trans: trans_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, eff_id: stor3_t, cntx: *const cntx_t, rntm: *const rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemmsup_ref_var1.unwrap()(trans, alpha, a, b, beta, c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_gemmsup_ref_var2(trans: trans_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, eff_id: stor3_t, cntx: *const cntx_t, rntm: *const rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemmsup_ref_var2.unwrap()(trans, alpha, a, b, beta, c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_gemmsup_ref_var1n(trans: trans_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, eff_id: stor3_t, cntx: *const cntx_t, rntm: *const rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemmsup_ref_var1n.unwrap()(trans, alpha, a, b, beta, c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_gemmsup_ref_var2m(trans: trans_t, alpha: *const obj_t, a: *const obj_t, b: *const obj_t, beta: *const obj_t, c: *const obj_t, eff_id: stor3_t, cntx: *const cntx_t, rntm: *const rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemmsup_ref_var2m.unwrap()(trans, alpha, a, b, beta, c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_sgemmsup_ref_var1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_sgemmsup_ref_var1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_dgemmsup_ref_var1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_dgemmsup_ref_var1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_cgemmsup_ref_var1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_cgemmsup_ref_var1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_zgemmsup_ref_var1(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_zgemmsup_ref_var1.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_sgemmsup_ref_var2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_sgemmsup_ref_var2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_dgemmsup_ref_var2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_dgemmsup_ref_var2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_cgemmsup_ref_var2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_cgemmsup_ref_var2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_zgemmsup_ref_var2(conja: conj_t, conjb: conj_t, m: dim_t, n: dim_t, k: dim_t, alpha: *mut c_void, a: *mut c_void, rs_a: inc_t, cs_a: inc_t, b: *mut c_void, rs_b: inc_t, cs_b: inc_t, beta: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, eff_id: stor3_t, cntx: *mut cntx_t, rntm: *mut rntm_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_zgemmsup_ref_var2.unwrap()(conja, conjb, m, n, k, alpha, a, rs_a, cs_a, b, rs_b, cs_b, beta, c, rs_c, cs_c, eff_id, cntx, rntm, thread)
            }

pub unsafe fn bli_packm_sup_finalize_mem(did_pack: bool, thread: *mut thrinfo_t) {
                dyload_lib().bli_packm_sup_finalize_mem.unwrap()(did_pack, thread)
            }

pub unsafe fn bli_packm_sup(will_pack: bool, pack_buf_type: packbuf_t, stor_id: stor3_t, dt: num_t, m: dim_t, k: dim_t, mr: dim_t, kappa: *const c_void, a: *const c_void, rs_a: inc_t, cs_a: inc_t, p: *mut *mut c_void, rs_p: *mut inc_t, cs_p: *mut inc_t, ps_p: *mut inc_t, cntx: *const cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_packm_sup.unwrap()(will_pack, pack_buf_type, stor_id, dt, m, k, mr, kappa, a, rs_a, cs_a, p, rs_p, cs_p, ps_p, cntx, thread)
            }

pub unsafe fn bli_spackm_sup_var1(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, m_max: dim_t, n_max: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_spackm_sup_var1.unwrap()(transc, schema, m, n, m_max, n_max, kappa, c, rs_c, cs_c, p, rs_p, cs_p, pd_p, ps_p, cntx, thread)
            }

pub unsafe fn bli_dpackm_sup_var1(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, m_max: dim_t, n_max: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_dpackm_sup_var1.unwrap()(transc, schema, m, n, m_max, n_max, kappa, c, rs_c, cs_c, p, rs_p, cs_p, pd_p, ps_p, cntx, thread)
            }

pub unsafe fn bli_cpackm_sup_var1(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, m_max: dim_t, n_max: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_cpackm_sup_var1.unwrap()(transc, schema, m, n, m_max, n_max, kappa, c, rs_c, cs_c, p, rs_p, cs_p, pd_p, ps_p, cntx, thread)
            }

pub unsafe fn bli_zpackm_sup_var1(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, m_max: dim_t, n_max: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, pd_p: dim_t, ps_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_zpackm_sup_var1.unwrap()(transc, schema, m, n, m_max, n_max, kappa, c, rs_c, cs_c, p, rs_p, cs_p, pd_p, ps_p, cntx, thread)
            }

pub unsafe fn bli_spackm_sup_var2(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_spackm_sup_var2.unwrap()(transc, schema, m, n, kappa, c, rs_c, cs_c, p, rs_p, cs_p, cntx, thread)
            }

pub unsafe fn bli_dpackm_sup_var2(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_dpackm_sup_var2.unwrap()(transc, schema, m, n, kappa, c, rs_c, cs_c, p, rs_p, cs_p, cntx, thread)
            }

pub unsafe fn bli_cpackm_sup_var2(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_cpackm_sup_var2.unwrap()(transc, schema, m, n, kappa, c, rs_c, cs_c, p, rs_p, cs_p, cntx, thread)
            }

pub unsafe fn bli_zpackm_sup_var2(transc: trans_t, schema: pack_t, m: dim_t, n: dim_t, kappa: *mut c_void, c: *mut c_void, rs_c: inc_t, cs_c: inc_t, p: *mut c_void, rs_p: inc_t, cs_p: inc_t, cntx: *mut cntx_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_zpackm_sup_var2.unwrap()(transc, schema, m, n, kappa, c, rs_c, cs_c, p, rs_p, cs_p, cntx, thread)
            }

pub unsafe fn bli_gemm_ukernel(alpha: *mut obj_t, a: *mut obj_t, b: *mut obj_t, beta: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t) {
                dyload_lib().bli_gemm_ukernel.unwrap()(alpha, a, b, beta, c, cntx)
            }

pub unsafe fn bli_gemmtrsm_ukernel(alpha: *mut obj_t, a1x: *mut obj_t, a11: *mut obj_t, bx1: *mut obj_t, b11: *mut obj_t, c11: *mut obj_t, cntx: *mut cntx_t) {
                dyload_lib().bli_gemmtrsm_ukernel.unwrap()(alpha, a1x, a11, bx1, b11, c11, cntx)
            }

pub unsafe fn bli_trsm_ukernel(a: *mut obj_t, b: *mut obj_t, c: *mut obj_t, cntx: *mut cntx_t) {
                dyload_lib().bli_trsm_ukernel.unwrap()(a, b, c, cntx)
            }

pub unsafe fn bli_sgemm_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const f32, a: *const f32, b: *const f32, beta: *const f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemm_ukernel.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemm_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const f64, a: *const f64, b: *const f64, beta: *const f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemm_ukernel.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemm_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const scomplex, a: *const scomplex, b: *const scomplex, beta: *const scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemm_ukernel.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemm_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const dcomplex, a: *const dcomplex, b: *const dcomplex, beta: *const dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemm_ukernel.unwrap()(m, n, k, alpha, a, b, beta, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmtrsm_l_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const f32, a1x: *const f32, a11: *const f32, bx1: *const f32, b11: *mut f32, c11: *mut f32, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmtrsm_l_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmtrsm_l_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const f64, a1x: *const f64, a11: *const f64, bx1: *const f64, b11: *mut f64, c11: *mut f64, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmtrsm_l_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmtrsm_l_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const scomplex, a1x: *const scomplex, a11: *const scomplex, bx1: *const scomplex, b11: *mut scomplex, c11: *mut scomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmtrsm_l_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmtrsm_l_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const dcomplex, a1x: *const dcomplex, a11: *const dcomplex, bx1: *const dcomplex, b11: *mut dcomplex, c11: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmtrsm_l_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_sgemmtrsm_u_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const f32, a1x: *const f32, a11: *const f32, bx1: *const f32, b11: *mut f32, c11: *mut f32, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_sgemmtrsm_u_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dgemmtrsm_u_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const f64, a1x: *const f64, a11: *const f64, bx1: *const f64, b11: *mut f64, c11: *mut f64, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dgemmtrsm_u_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_cgemmtrsm_u_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const scomplex, a1x: *const scomplex, a11: *const scomplex, bx1: *const scomplex, b11: *mut scomplex, c11: *mut scomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_cgemmtrsm_u_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_zgemmtrsm_u_ukernel(m: dim_t, n: dim_t, k: dim_t, alpha: *const dcomplex, a1x: *const dcomplex, a11: *const dcomplex, bx1: *const dcomplex, b11: *mut dcomplex, c11: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_zgemmtrsm_u_ukernel.unwrap()(m, n, k, alpha, a1x, a11, bx1, b11, c11, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_strsm_l_ukernel(a: *const f32, b: *mut f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_strsm_l_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dtrsm_l_ukernel(a: *const f64, b: *mut f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dtrsm_l_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_ctrsm_l_ukernel(a: *const scomplex, b: *mut scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_ctrsm_l_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_ztrsm_l_ukernel(a: *const dcomplex, b: *mut dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_ztrsm_l_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_strsm_u_ukernel(a: *const f32, b: *mut f32, c: *mut f32, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_strsm_u_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_dtrsm_u_ukernel(a: *const f64, b: *mut f64, c: *mut f64, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_dtrsm_u_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_ctrsm_u_ukernel(a: *const scomplex, b: *mut scomplex, c: *mut scomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_ctrsm_u_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_ztrsm_u_ukernel(a: *const dcomplex, b: *mut dcomplex, c: *mut dcomplex, rs_c: inc_t, cs_c: inc_t, data: *const auxinfo_t, cntx: *const cntx_t) {
                dyload_lib().bli_ztrsm_u_ukernel.unwrap()(a, b, c, rs_c, cs_c, data, cntx)
            }

pub unsafe fn bli_gemm_ukernel_qfp(dt: num_t) -> gemm_ukr_ft {
                dyload_lib().bli_gemm_ukernel_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemmtrsm_l_ukernel_qfp(dt: num_t) -> gemmtrsm_ukr_ft {
                dyload_lib().bli_gemmtrsm_l_ukernel_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemmtrsm_u_ukernel_qfp(dt: num_t) -> gemmtrsm_ukr_ft {
                dyload_lib().bli_gemmtrsm_u_ukernel_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsm_l_ukernel_qfp(dt: num_t) -> trsm_ukr_ft {
                dyload_lib().bli_trsm_l_ukernel_qfp.unwrap()(dt)
            }

pub unsafe fn bli_trsm_u_ukernel_qfp(dt: num_t) -> trsm_ukr_ft {
                dyload_lib().bli_trsm_u_ukernel_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gemm_var_cntl_init_node(var_func: void_fp, dt_comp: num_t, dt_out: num_t, ukr: gemm_ukr_ft, real_ukr: gemm_ukr_ft, row_pref: bool, mr: dim_t, nr: dim_t, mr_scale: dim_t, nr_scale: dim_t, cntl: *mut gemm_var_cntl_t) {
                dyload_lib().bli_gemm_var_cntl_init_node.unwrap()(var_func, dt_comp, dt_out, ukr, real_ukr, row_pref, mr, nr, mr_scale, nr_scale, cntl)
            }

pub unsafe fn bli_gemm_cntl_init(im: ind_t, family: opid_t, alpha: *const obj_t, a: *mut obj_t, b: *mut obj_t, beta: *const obj_t, c: *mut obj_t, cntx: *const cntx_t, cntl: *mut gemm_cntl_t) -> bool {
                dyload_lib().bli_gemm_cntl_init.unwrap()(im, family, alpha, a, b, beta, c, cntx, cntl)
            }

pub unsafe fn bli_gemm_cntl_finalize(family: opid_t, a: *const obj_t, b: *const obj_t, c: *const obj_t, cntl: *mut gemm_cntl_t) {
                dyload_lib().bli_gemm_cntl_finalize.unwrap()(family, a, b, c, cntl)
            }

pub unsafe fn bli_gemm_blk_var1(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemm_blk_var1.unwrap()(a, b, c, cntx, cntl, thread)
            }

pub unsafe fn bli_gemm_blk_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemm_blk_var2.unwrap()(a, b, c, cntx, cntl, thread)
            }

pub unsafe fn bli_gemm_blk_var3(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemm_blk_var3.unwrap()(a, b, c, cntx, cntl, thread)
            }

pub unsafe fn bli_gemm_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread: *mut thrinfo_t) {
                dyload_lib().bli_gemm_ker_var2.unwrap()(a, b, c, cntx, cntl, thread)
            }

pub unsafe fn bli_trmm_ll_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_ll_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_lu_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_lu_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_rl_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_rl_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_ru_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_ru_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_ll_ker_var2b(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_ll_ker_var2b.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_lu_ker_var2b(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_lu_ker_var2b.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_rl_ker_var2b(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_rl_ker_var2b.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trmm_ru_ker_var2b(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trmm_ru_ker_var2b.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_var_cntl_init_node(var_func: void_fp, dt_comp: num_t, dt_out: num_t, gemmtrsm_ukr: gemmtrsm_ukr_ft, gemm_ukr: gemm_ukr_ft, real_gemm_ukr: gemm_ukr_ft, row_pref: bool, mr: dim_t, nr: dim_t, mr_pack: dim_t, nr_pack: dim_t, mr_bcast: dim_t, nr_bcast: dim_t, mr_scale: dim_t, nr_scale: dim_t, cntl: *mut trsm_var_cntl_t) {
                dyload_lib().bli_trsm_var_cntl_init_node.unwrap()(var_func, dt_comp, dt_out, gemmtrsm_ukr, gemm_ukr, real_gemm_ukr, row_pref, mr, nr, mr_pack, nr_pack, mr_bcast, nr_bcast, mr_scale, nr_scale, cntl)
            }

pub unsafe fn bli_trsm_cntl_init(im: ind_t, alpha: *const obj_t, a: *mut obj_t, b: *mut obj_t, beta: *const obj_t, c: *mut obj_t, cntx: *const cntx_t, cntl: *mut trsm_cntl_t) {
                dyload_lib().bli_trsm_cntl_init.unwrap()(im, alpha, a, b, beta, c, cntx, cntl)
            }

pub unsafe fn bli_trsm_l_cntl_init(im: ind_t, alpha: *const obj_t, a: *mut obj_t, b: *mut obj_t, beta: *const obj_t, c: *mut obj_t, cntx: *const cntx_t, cntl: *mut trsm_cntl_t) {
                dyload_lib().bli_trsm_l_cntl_init.unwrap()(im, alpha, a, b, beta, c, cntx, cntl)
            }

pub unsafe fn bli_trsm_r_cntl_init(im: ind_t, alpha: *const obj_t, a: *mut obj_t, b: *mut obj_t, beta: *const obj_t, c: *mut obj_t, cntx: *const cntx_t, cntl: *mut trsm_cntl_t) {
                dyload_lib().bli_trsm_r_cntl_init.unwrap()(im, alpha, a, b, beta, c, cntx, cntl)
            }

pub unsafe fn bli_trsm_cntl_finalize(cntl: *mut trsm_cntl_t) {
                dyload_lib().bli_trsm_cntl_finalize.unwrap()(cntl)
            }

pub unsafe fn bli_trsm_blk_var1(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_blk_var1.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_blk_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_blk_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_blk_var3(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_blk_var3.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_ll_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_ll_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_lu_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_lu_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_rl_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_rl_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_trsm_ru_ker_var2(a: *const obj_t, b: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_trsm_ru_ker_var2.unwrap()(a, b, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_gemmt_l_ker_var2(a: *const obj_t, ah: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_gemmt_l_ker_var2.unwrap()(a, ah, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_gemmt_u_ker_var2(a: *const obj_t, ah: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_gemmt_u_ker_var2.unwrap()(a, ah, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_gemmt_l_ker_var2b(a: *const obj_t, ah: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_gemmt_l_ker_var2b.unwrap()(a, ah, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_gemmt_u_ker_var2b(a: *const obj_t, ah: *const obj_t, c: *const obj_t, cntx: *const cntx_t, cntl: *const cntl_t, thread_par: *mut thrinfo_t) {
                dyload_lib().bli_gemmt_u_ker_var2b.unwrap()(a, ah, c, cntx, cntl, thread_par)
            }

pub unsafe fn bli_asumv_check(x: *const obj_t, asum: *const obj_t) {
                dyload_lib().bli_asumv_check.unwrap()(x, asum)
            }

pub unsafe fn bli_mkherm_check(x: *const obj_t) {
                dyload_lib().bli_mkherm_check.unwrap()(x)
            }

pub unsafe fn bli_mksymm_check(x: *const obj_t) {
                dyload_lib().bli_mksymm_check.unwrap()(x)
            }

pub unsafe fn bli_mktrim_check(x: *const obj_t) {
                dyload_lib().bli_mktrim_check.unwrap()(x)
            }

pub unsafe fn bli_norm1v_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_norm1v_check.unwrap()(x, norm)
            }

pub unsafe fn bli_normfv_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normfv_check.unwrap()(x, norm)
            }

pub unsafe fn bli_normiv_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normiv_check.unwrap()(x, norm)
            }

pub unsafe fn bli_norm1m_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_norm1m_check.unwrap()(x, norm)
            }

pub unsafe fn bli_normfm_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normfm_check.unwrap()(x, norm)
            }

pub unsafe fn bli_normim_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normim_check.unwrap()(x, norm)
            }

pub unsafe fn bli_randv_check(x: *const obj_t) {
                dyload_lib().bli_randv_check.unwrap()(x)
            }

pub unsafe fn bli_randnv_check(x: *const obj_t) {
                dyload_lib().bli_randnv_check.unwrap()(x)
            }

pub unsafe fn bli_randm_check(x: *const obj_t) {
                dyload_lib().bli_randm_check.unwrap()(x)
            }

pub unsafe fn bli_randnm_check(x: *const obj_t) {
                dyload_lib().bli_randnm_check.unwrap()(x)
            }

pub unsafe fn bli_sumsqv_check(x: *const obj_t, scale: *const obj_t, sumsq: *const obj_t) {
                dyload_lib().bli_sumsqv_check.unwrap()(x, scale, sumsq)
            }

pub unsafe fn bli_eqsc_check(chi: *const obj_t, psi: *const obj_t, is: *const bool) {
                dyload_lib().bli_eqsc_check.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_ltsc_check(chi: *const obj_t, psi: *const obj_t, is: *const bool) {
                dyload_lib().bli_ltsc_check.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_lesc_check(chi: *const obj_t, psi: *const obj_t, is: *const bool) {
                dyload_lib().bli_lesc_check.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_gtsc_check(chi: *const obj_t, psi: *const obj_t, is: *const bool) {
                dyload_lib().bli_gtsc_check.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_gesc_check(chi: *const obj_t, psi: *const obj_t, is: *const bool) {
                dyload_lib().bli_gesc_check.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_eqv_check(x: *const obj_t, y: *const obj_t, is: *const bool) {
                dyload_lib().bli_eqv_check.unwrap()(x, y, is)
            }

pub unsafe fn bli_eqm_check(x: *const obj_t, y: *const obj_t, is: *const bool) {
                dyload_lib().bli_eqm_check.unwrap()(x, y, is)
            }

pub unsafe fn bli_fprintv_check(file: *const FILE, s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_fprintv_check.unwrap()(file, s1, x, format, s2)
            }

pub unsafe fn bli_fprintm_check(file: *const FILE, s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_fprintm_check.unwrap()(file, s1, x, format, s2)
            }

pub unsafe fn bli_utilv_xi_check(x: *const obj_t, index: *const obj_t) {
                dyload_lib().bli_utilv_xi_check.unwrap()(x, index)
            }

pub unsafe fn bli_utilv_xa_check(x: *const obj_t, asum: *const obj_t) {
                dyload_lib().bli_utilv_xa_check.unwrap()(x, asum)
            }

pub unsafe fn bli_utilm_mkhst_check(a: *const obj_t) {
                dyload_lib().bli_utilm_mkhst_check.unwrap()(a)
            }

pub unsafe fn bli_utilv_norm_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_utilv_norm_check.unwrap()(x, norm)
            }

pub unsafe fn bli_utilm_norm_check(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_utilm_norm_check.unwrap()(x, norm)
            }

pub unsafe fn bli_utilm_fprint_check(file: *const FILE, s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_utilm_fprint_check.unwrap()(file, s1, x, format, s2)
            }

pub unsafe fn bli_utilm_rand_check(x: *const obj_t) {
                dyload_lib().bli_utilm_rand_check.unwrap()(x)
            }

pub unsafe fn bli_utilv_sumsqv_check(x: *const obj_t, scale: *const obj_t, sumsq: *const obj_t) {
                dyload_lib().bli_utilv_sumsqv_check.unwrap()(x, scale, sumsq)
            }

pub unsafe fn bli_asumv_ex(x: *const obj_t, asum: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_asumv_ex.unwrap()(x, asum, cntx, rntm)
            }

pub unsafe fn bli_mkherm_ex(a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_mkherm_ex.unwrap()(a, cntx, rntm)
            }

pub unsafe fn bli_mksymm_ex(a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_mksymm_ex.unwrap()(a, cntx, rntm)
            }

pub unsafe fn bli_mktrim_ex(a: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_mktrim_ex.unwrap()(a, cntx, rntm)
            }

pub unsafe fn bli_norm1v_ex(x: *const obj_t, norm: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_norm1v_ex.unwrap()(x, norm, cntx, rntm)
            }

pub unsafe fn bli_normfv_ex(x: *const obj_t, norm: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_normfv_ex.unwrap()(x, norm, cntx, rntm)
            }

pub unsafe fn bli_normiv_ex(x: *const obj_t, norm: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_normiv_ex.unwrap()(x, norm, cntx, rntm)
            }

pub unsafe fn bli_norm1m_ex(x: *const obj_t, norm: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_norm1m_ex.unwrap()(x, norm, cntx, rntm)
            }

pub unsafe fn bli_normfm_ex(x: *const obj_t, norm: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_normfm_ex.unwrap()(x, norm, cntx, rntm)
            }

pub unsafe fn bli_normim_ex(x: *const obj_t, norm: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_normim_ex.unwrap()(x, norm, cntx, rntm)
            }

pub unsafe fn bli_randv_ex(x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_randv_ex.unwrap()(x, cntx, rntm)
            }

pub unsafe fn bli_randnv_ex(x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_randnv_ex.unwrap()(x, cntx, rntm)
            }

pub unsafe fn bli_randm_ex(x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_randm_ex.unwrap()(x, cntx, rntm)
            }

pub unsafe fn bli_randnm_ex(x: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_randnm_ex.unwrap()(x, cntx, rntm)
            }

pub unsafe fn bli_sumsqv_ex(x: *const obj_t, scale: *const obj_t, sumsq: *const obj_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sumsqv_ex.unwrap()(x, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_asumv(x: *const obj_t, asum: *const obj_t) {
                dyload_lib().bli_asumv.unwrap()(x, asum)
            }

pub unsafe fn bli_mkherm(a: *const obj_t) {
                dyload_lib().bli_mkherm.unwrap()(a)
            }

pub unsafe fn bli_mksymm(a: *const obj_t) {
                dyload_lib().bli_mksymm.unwrap()(a)
            }

pub unsafe fn bli_mktrim(a: *const obj_t) {
                dyload_lib().bli_mktrim.unwrap()(a)
            }

pub unsafe fn bli_norm1v(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_norm1v.unwrap()(x, norm)
            }

pub unsafe fn bli_normfv(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normfv.unwrap()(x, norm)
            }

pub unsafe fn bli_normiv(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normiv.unwrap()(x, norm)
            }

pub unsafe fn bli_norm1m(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_norm1m.unwrap()(x, norm)
            }

pub unsafe fn bli_normfm(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normfm.unwrap()(x, norm)
            }

pub unsafe fn bli_normim(x: *const obj_t, norm: *const obj_t) {
                dyload_lib().bli_normim.unwrap()(x, norm)
            }

pub unsafe fn bli_randv(x: *const obj_t) {
                dyload_lib().bli_randv.unwrap()(x)
            }

pub unsafe fn bli_randnv(x: *const obj_t) {
                dyload_lib().bli_randnv.unwrap()(x)
            }

pub unsafe fn bli_randm(x: *const obj_t) {
                dyload_lib().bli_randm.unwrap()(x)
            }

pub unsafe fn bli_randnm(x: *const obj_t) {
                dyload_lib().bli_randnm.unwrap()(x)
            }

pub unsafe fn bli_sumsqv(x: *const obj_t, scale: *const obj_t, sumsq: *const obj_t) {
                dyload_lib().bli_sumsqv.unwrap()(x, scale, sumsq)
            }

pub unsafe fn bli_eqsc(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_eqsc.unwrap()(x, y, is)
            }

pub unsafe fn bli_eqv(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_eqv.unwrap()(x, y, is)
            }

pub unsafe fn bli_eqm(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_eqm.unwrap()(x, y, is)
            }

pub unsafe fn bli_ltsc(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_ltsc.unwrap()(x, y, is)
            }

pub unsafe fn bli_lesc(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_lesc.unwrap()(x, y, is)
            }

pub unsafe fn bli_gtsc(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_gtsc.unwrap()(x, y, is)
            }

pub unsafe fn bli_gesc(x: *const obj_t, y: *const obj_t, is: *mut bool) {
                dyload_lib().bli_gesc.unwrap()(x, y, is)
            }

pub unsafe fn bli_fprintv(file: *mut FILE, s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_fprintv.unwrap()(file, s1, x, format, s2)
            }

pub unsafe fn bli_fprintm(file: *mut FILE, s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_fprintm.unwrap()(file, s1, x, format, s2)
            }

pub unsafe fn bli_printv(s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_printv.unwrap()(s1, x, format, s2)
            }

pub unsafe fn bli_printm(s1: *const c_char, x: *const obj_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_printm.unwrap()(s1, x, format, s2)
            }

pub unsafe fn bli_sasumv_ex(n: dim_t, x: *const f32, incx: inc_t, asum: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_sasumv_ex.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_dasumv_ex(n: dim_t, x: *const f64, incx: inc_t, asum: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dasumv_ex.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_casumv_ex(n: dim_t, x: *const scomplex, incx: inc_t, asum: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_casumv_ex.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_zasumv_ex(n: dim_t, x: *const dcomplex, incx: inc_t, asum: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zasumv_ex.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_smkherm_ex(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_smkherm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dmkherm_ex(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dmkherm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cmkherm_ex(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cmkherm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zmkherm_ex(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zmkherm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_smksymm_ex(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_smksymm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dmksymm_ex(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dmksymm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cmksymm_ex(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cmksymm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zmksymm_ex(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zmksymm_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_smktrim_ex(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_smktrim_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dmktrim_ex(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dmktrim_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cmktrim_ex(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cmktrim_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zmktrim_ex(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zmktrim_ex.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_snorm1v_ex(n: dim_t, x: *const f32, incx: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_snorm1v_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_dnorm1v_ex(n: dim_t, x: *const f64, incx: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dnorm1v_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_cnorm1v_ex(n: dim_t, x: *const scomplex, incx: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cnorm1v_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_znorm1v_ex(n: dim_t, x: *const dcomplex, incx: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_znorm1v_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_snormfv_ex(n: dim_t, x: *const f32, incx: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_snormfv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormfv_ex(n: dim_t, x: *const f64, incx: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dnormfv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormfv_ex(n: dim_t, x: *const scomplex, incx: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cnormfv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_znormfv_ex(n: dim_t, x: *const dcomplex, incx: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_znormfv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_snormiv_ex(n: dim_t, x: *const f32, incx: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_snormiv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormiv_ex(n: dim_t, x: *const f64, incx: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dnormiv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormiv_ex(n: dim_t, x: *const scomplex, incx: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cnormiv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_znormiv_ex(n: dim_t, x: *const dcomplex, incx: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_znormiv_ex.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_snorm1m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_snorm1m_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_dnorm1m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dnorm1m_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_cnorm1m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cnorm1m_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_znorm1m_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_znorm1m_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_snormfm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_snormfm_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormfm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dnormfm_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormfm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cnormfm_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_znormfm_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_znormfm_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_snormim_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_snormim_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormim_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dnormim_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormim_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_cnormim_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_znormim_ex(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_znormim_ex.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_srandv_ex(n: dim_t, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_srandv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_drandv_ex(n: dim_t, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_drandv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_crandv_ex(n: dim_t, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_crandv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zrandv_ex(n: dim_t, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zrandv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_srandnv_ex(n: dim_t, x: *mut f32, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_srandnv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_drandnv_ex(n: dim_t, x: *mut f64, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_drandnv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_crandnv_ex(n: dim_t, x: *mut scomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_crandnv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zrandnv_ex(n: dim_t, x: *mut dcomplex, incx: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zrandnv_ex.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_srandm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_srandm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_drandm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_drandm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_crandm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_crandm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zrandm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zrandm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_srandnm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_srandnm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_drandnm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_drandnm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_crandnm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_crandnm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zrandnm_ex(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zrandnm_ex.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_ssumsqv_ex(n: dim_t, x: *const f32, incx: inc_t, scale: *mut f32, sumsq: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_ssumsqv_ex.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_dsumsqv_ex(n: dim_t, x: *const f64, incx: inc_t, scale: *mut f64, sumsq: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_dsumsqv_ex.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_csumsqv_ex(n: dim_t, x: *const scomplex, incx: inc_t, scale: *mut f32, sumsq: *mut f32, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_csumsqv_ex.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_zsumsqv_ex(n: dim_t, x: *const dcomplex, incx: inc_t, scale: *mut f64, sumsq: *mut f64, cntx: *const cntx_t, rntm: *const rntm_t) {
                dyload_lib().bli_zsumsqv_ex.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_sasumv(n: dim_t, x: *const f32, incx: inc_t, asum: *mut f32) {
                dyload_lib().bli_sasumv.unwrap()(n, x, incx, asum)
            }

pub unsafe fn bli_dasumv(n: dim_t, x: *const f64, incx: inc_t, asum: *mut f64) {
                dyload_lib().bli_dasumv.unwrap()(n, x, incx, asum)
            }

pub unsafe fn bli_casumv(n: dim_t, x: *const scomplex, incx: inc_t, asum: *mut f32) {
                dyload_lib().bli_casumv.unwrap()(n, x, incx, asum)
            }

pub unsafe fn bli_zasumv(n: dim_t, x: *const dcomplex, incx: inc_t, asum: *mut f64) {
                dyload_lib().bli_zasumv.unwrap()(n, x, incx, asum)
            }

pub unsafe fn bli_smkherm(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_smkherm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_dmkherm(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dmkherm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_cmkherm(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_cmkherm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_zmkherm(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zmkherm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_smksymm(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_smksymm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_dmksymm(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dmksymm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_cmksymm(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_cmksymm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_zmksymm(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zmksymm.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_smktrim(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_smktrim.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_dmktrim(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_dmktrim.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_cmktrim(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_cmktrim.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_zmktrim(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t) {
                dyload_lib().bli_zmktrim.unwrap()(uploa, m, a, rs_a, cs_a)
            }

pub unsafe fn bli_snorm1v(n: dim_t, x: *const f32, incx: inc_t, norm: *mut f32) {
                dyload_lib().bli_snorm1v.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_dnorm1v(n: dim_t, x: *const f64, incx: inc_t, norm: *mut f64) {
                dyload_lib().bli_dnorm1v.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_cnorm1v(n: dim_t, x: *const scomplex, incx: inc_t, norm: *mut f32) {
                dyload_lib().bli_cnorm1v.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_znorm1v(n: dim_t, x: *const dcomplex, incx: inc_t, norm: *mut f64) {
                dyload_lib().bli_znorm1v.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_snormfv(n: dim_t, x: *const f32, incx: inc_t, norm: *mut f32) {
                dyload_lib().bli_snormfv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_dnormfv(n: dim_t, x: *const f64, incx: inc_t, norm: *mut f64) {
                dyload_lib().bli_dnormfv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_cnormfv(n: dim_t, x: *const scomplex, incx: inc_t, norm: *mut f32) {
                dyload_lib().bli_cnormfv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_znormfv(n: dim_t, x: *const dcomplex, incx: inc_t, norm: *mut f64) {
                dyload_lib().bli_znormfv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_snormiv(n: dim_t, x: *const f32, incx: inc_t, norm: *mut f32) {
                dyload_lib().bli_snormiv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_dnormiv(n: dim_t, x: *const f64, incx: inc_t, norm: *mut f64) {
                dyload_lib().bli_dnormiv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_cnormiv(n: dim_t, x: *const scomplex, incx: inc_t, norm: *mut f32) {
                dyload_lib().bli_cnormiv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_znormiv(n: dim_t, x: *const dcomplex, incx: inc_t, norm: *mut f64) {
                dyload_lib().bli_znormiv.unwrap()(n, x, incx, norm)
            }

pub unsafe fn bli_snorm1m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32) {
                dyload_lib().bli_snorm1m.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_dnorm1m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64) {
                dyload_lib().bli_dnorm1m.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_cnorm1m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32) {
                dyload_lib().bli_cnorm1m.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_znorm1m(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64) {
                dyload_lib().bli_znorm1m.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_snormfm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32) {
                dyload_lib().bli_snormfm.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_dnormfm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64) {
                dyload_lib().bli_dnormfm.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_cnormfm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32) {
                dyload_lib().bli_cnormfm.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_znormfm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64) {
                dyload_lib().bli_znormfm.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_snormim(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32) {
                dyload_lib().bli_snormim.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_dnormim(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64) {
                dyload_lib().bli_dnormim.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_cnormim(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32) {
                dyload_lib().bli_cnormim.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_znormim(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64) {
                dyload_lib().bli_znormim.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm)
            }

pub unsafe fn bli_srandv(n: dim_t, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_srandv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_drandv(n: dim_t, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_drandv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_crandv(n: dim_t, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_crandv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_zrandv(n: dim_t, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_zrandv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_srandnv(n: dim_t, x: *mut f32, incx: inc_t) {
                dyload_lib().bli_srandnv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_drandnv(n: dim_t, x: *mut f64, incx: inc_t) {
                dyload_lib().bli_drandnv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_crandnv(n: dim_t, x: *mut scomplex, incx: inc_t) {
                dyload_lib().bli_crandnv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_zrandnv(n: dim_t, x: *mut dcomplex, incx: inc_t) {
                dyload_lib().bli_zrandnv.unwrap()(n, x, incx)
            }

pub unsafe fn bli_srandm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_srandm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_drandm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_drandm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_crandm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_crandm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_zrandm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zrandm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_srandnm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_srandnm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_drandnm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_drandnm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_crandnm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_crandnm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_zrandnm(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t) {
                dyload_lib().bli_zrandnm.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x)
            }

pub unsafe fn bli_ssumsqv(n: dim_t, x: *const f32, incx: inc_t, scale: *mut f32, sumsq: *mut f32) {
                dyload_lib().bli_ssumsqv.unwrap()(n, x, incx, scale, sumsq)
            }

pub unsafe fn bli_dsumsqv(n: dim_t, x: *const f64, incx: inc_t, scale: *mut f64, sumsq: *mut f64) {
                dyload_lib().bli_dsumsqv.unwrap()(n, x, incx, scale, sumsq)
            }

pub unsafe fn bli_csumsqv(n: dim_t, x: *const scomplex, incx: inc_t, scale: *mut f32, sumsq: *mut f32) {
                dyload_lib().bli_csumsqv.unwrap()(n, x, incx, scale, sumsq)
            }

pub unsafe fn bli_zsumsqv(n: dim_t, x: *const dcomplex, incx: inc_t, scale: *mut f64, sumsq: *mut f64) {
                dyload_lib().bli_zsumsqv.unwrap()(n, x, incx, scale, sumsq)
            }

pub unsafe fn bli_seqsc(conjchi: conj_t, chi: *const f32, psi: *const f32, is_eq: *mut bool) {
                dyload_lib().bli_seqsc.unwrap()(conjchi, chi, psi, is_eq)
            }

pub unsafe fn bli_deqsc(conjchi: conj_t, chi: *const f64, psi: *const f64, is_eq: *mut bool) {
                dyload_lib().bli_deqsc.unwrap()(conjchi, chi, psi, is_eq)
            }

pub unsafe fn bli_ceqsc(conjchi: conj_t, chi: *const scomplex, psi: *const scomplex, is_eq: *mut bool) {
                dyload_lib().bli_ceqsc.unwrap()(conjchi, chi, psi, is_eq)
            }

pub unsafe fn bli_zeqsc(conjchi: conj_t, chi: *const dcomplex, psi: *const dcomplex, is_eq: *mut bool) {
                dyload_lib().bli_zeqsc.unwrap()(conjchi, chi, psi, is_eq)
            }

pub unsafe fn bli_seqv(conjx: conj_t, n: dim_t, x: *const f32, incx: inc_t, y: *const f32, incy: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_seqv.unwrap()(conjx, n, x, incx, y, incy, is_eq)
            }

pub unsafe fn bli_deqv(conjx: conj_t, n: dim_t, x: *const f64, incx: inc_t, y: *const f64, incy: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_deqv.unwrap()(conjx, n, x, incx, y, incy, is_eq)
            }

pub unsafe fn bli_ceqv(conjx: conj_t, n: dim_t, x: *const scomplex, incx: inc_t, y: *const scomplex, incy: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_ceqv.unwrap()(conjx, n, x, incx, y, incy, is_eq)
            }

pub unsafe fn bli_zeqv(conjx: conj_t, n: dim_t, x: *const dcomplex, incx: inc_t, y: *const dcomplex, incy: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_zeqv.unwrap()(conjx, n, x, incx, y, incy, is_eq)
            }

pub unsafe fn bli_seqm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, y: *const f32, rs_y: inc_t, cs_y: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_seqm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, is_eq)
            }

pub unsafe fn bli_deqm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, y: *const f64, rs_y: inc_t, cs_y: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_deqm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, is_eq)
            }

pub unsafe fn bli_ceqm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, y: *const scomplex, rs_y: inc_t, cs_y: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_ceqm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, is_eq)
            }

pub unsafe fn bli_zeqm(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, y: *const dcomplex, rs_y: inc_t, cs_y: inc_t, is_eq: *mut bool) {
                dyload_lib().bli_zeqm.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y, is_eq)
            }

pub unsafe fn bli_sltsc(chi: *const f32, psi: *const f32, is: *mut bool) {
                dyload_lib().bli_sltsc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_dltsc(chi: *const f64, psi: *const f64, is: *mut bool) {
                dyload_lib().bli_dltsc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_slesc(chi: *const f32, psi: *const f32, is: *mut bool) {
                dyload_lib().bli_slesc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_dlesc(chi: *const f64, psi: *const f64, is: *mut bool) {
                dyload_lib().bli_dlesc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_sgtsc(chi: *const f32, psi: *const f32, is: *mut bool) {
                dyload_lib().bli_sgtsc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_dgtsc(chi: *const f64, psi: *const f64, is: *mut bool) {
                dyload_lib().bli_dgtsc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_sgesc(chi: *const f32, psi: *const f32, is: *mut bool) {
                dyload_lib().bli_sgesc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_dgesc(chi: *const f64, psi: *const f64, is: *mut bool) {
                dyload_lib().bli_dgesc.unwrap()(chi, psi, is)
            }

pub unsafe fn bli_sprintv(s1: *const c_char, n: dim_t, x: *const c_void, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_sprintv.unwrap()(s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_dprintv(s1: *const c_char, n: dim_t, x: *const c_void, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_dprintv.unwrap()(s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_cprintv(s1: *const c_char, n: dim_t, x: *const c_void, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_cprintv.unwrap()(s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_zprintv(s1: *const c_char, n: dim_t, x: *const c_void, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_zprintv.unwrap()(s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_iprintv(s1: *const c_char, n: dim_t, x: *const c_void, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_iprintv.unwrap()(s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_sprintm(s1: *const c_char, m: dim_t, n: dim_t, x: *const c_void, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_sprintm.unwrap()(s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_dprintm(s1: *const c_char, m: dim_t, n: dim_t, x: *const c_void, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_dprintm.unwrap()(s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_cprintm(s1: *const c_char, m: dim_t, n: dim_t, x: *const c_void, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_cprintm.unwrap()(s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_zprintm(s1: *const c_char, m: dim_t, n: dim_t, x: *const c_void, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_zprintm.unwrap()(s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_iprintm(s1: *const c_char, m: dim_t, n: dim_t, x: *const c_void, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_iprintm.unwrap()(s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_asumv_ex_qfp(dt: num_t) -> asumv_ex_vft {
                dyload_lib().bli_asumv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_mkherm_ex_qfp(dt: num_t) -> mkherm_ex_vft {
                dyload_lib().bli_mkherm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_mksymm_ex_qfp(dt: num_t) -> mksymm_ex_vft {
                dyload_lib().bli_mksymm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_mktrim_ex_qfp(dt: num_t) -> mktrim_ex_vft {
                dyload_lib().bli_mktrim_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_norm1v_ex_qfp(dt: num_t) -> norm1v_ex_vft {
                dyload_lib().bli_norm1v_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_normfv_ex_qfp(dt: num_t) -> normfv_ex_vft {
                dyload_lib().bli_normfv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_normiv_ex_qfp(dt: num_t) -> normiv_ex_vft {
                dyload_lib().bli_normiv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_norm1m_ex_qfp(dt: num_t) -> norm1m_ex_vft {
                dyload_lib().bli_norm1m_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_normfm_ex_qfp(dt: num_t) -> normfm_ex_vft {
                dyload_lib().bli_normfm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_normim_ex_qfp(dt: num_t) -> normim_ex_vft {
                dyload_lib().bli_normim_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_randv_ex_qfp(dt: num_t) -> randv_ex_vft {
                dyload_lib().bli_randv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_randnv_ex_qfp(dt: num_t) -> randnv_ex_vft {
                dyload_lib().bli_randnv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_randm_ex_qfp(dt: num_t) -> randm_ex_vft {
                dyload_lib().bli_randm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_randnm_ex_qfp(dt: num_t) -> randnm_ex_vft {
                dyload_lib().bli_randnm_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_sumsqv_ex_qfp(dt: num_t) -> sumsqv_ex_vft {
                dyload_lib().bli_sumsqv_ex_qfp.unwrap()(dt)
            }

pub unsafe fn bli_eqsc_qfp(dt: num_t) -> eqsc_vft {
                dyload_lib().bli_eqsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_eqv_qfp(dt: num_t) -> eqv_vft {
                dyload_lib().bli_eqv_qfp.unwrap()(dt)
            }

pub unsafe fn bli_eqm_qfp(dt: num_t) -> eqm_vft {
                dyload_lib().bli_eqm_qfp.unwrap()(dt)
            }

pub unsafe fn bli_ltsc_qfp(dt: num_t) -> ltsc_vft {
                dyload_lib().bli_ltsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_lesc_qfp(dt: num_t) -> lesc_vft {
                dyload_lib().bli_lesc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gtsc_qfp(dt: num_t) -> gtsc_vft {
                dyload_lib().bli_gtsc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_gesc_qfp(dt: num_t) -> gesc_vft {
                dyload_lib().bli_gesc_qfp.unwrap()(dt)
            }

pub unsafe fn bli_fprintv_qfp(dt: num_t) -> fprintv_vft {
                dyload_lib().bli_fprintv_qfp.unwrap()(dt)
            }

pub unsafe fn bli_fprintm_qfp(dt: num_t) -> fprintm_vft {
                dyload_lib().bli_fprintm_qfp.unwrap()(dt)
            }

pub unsafe fn bli_sasumv_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, asum: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_sasumv_unb_var1.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_dasumv_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, asum: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dasumv_unb_var1.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_casumv_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, asum: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_casumv_unb_var1.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_zasumv_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, asum: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zasumv_unb_var1.unwrap()(n, x, incx, asum, cntx, rntm)
            }

pub unsafe fn bli_smkherm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_smkherm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dmkherm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dmkherm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cmkherm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cmkherm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zmkherm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zmkherm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_smksymm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_smksymm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dmksymm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dmksymm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cmksymm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cmksymm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zmksymm_unb_var1(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zmksymm_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_smktrim_unb_var1(uploa: uplo_t, m: dim_t, a: *mut f32, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_smktrim_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_dmktrim_unb_var1(uploa: uplo_t, m: dim_t, a: *mut f64, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dmktrim_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_cmktrim_unb_var1(uploa: uplo_t, m: dim_t, a: *mut scomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cmktrim_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_zmktrim_unb_var1(uploa: uplo_t, m: dim_t, a: *mut dcomplex, rs_a: inc_t, cs_a: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zmktrim_unb_var1.unwrap()(uploa, m, a, rs_a, cs_a, cntx, rntm)
            }

pub unsafe fn bli_snorm1v_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_snorm1v_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_dnorm1v_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dnorm1v_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_cnorm1v_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cnorm1v_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_znorm1v_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_znorm1v_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_snormfv_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_snormfv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormfv_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dnormfv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormfv_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cnormfv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_znormfv_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_znormfv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_snormiv_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_snormiv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormiv_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dnormiv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormiv_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cnormiv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_znormiv_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_znormiv_unb_var1.unwrap()(n, x, incx, norm, cntx, rntm)
            }

pub unsafe fn bli_snorm1m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_snorm1m_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_dnorm1m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dnorm1m_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_cnorm1m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cnorm1m_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_znorm1m_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_znorm1m_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_snormfm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_snormfm_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormfm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dnormfm_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormfm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cnormfm_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_znormfm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_znormfm_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_snormim_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_snormim_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_dnormim_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dnormim_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_cnormim_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_cnormim_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_znormim_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, norm: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_znormim_unb_var1.unwrap()(diagoffx, diagx, uplox, m, n, x, rs_x, cs_x, norm, cntx, rntm)
            }

pub unsafe fn bli_srandv_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_srandv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_drandv_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_drandv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_crandv_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_crandv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zrandv_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zrandv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_srandnv_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_srandnv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_drandnv_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_drandnv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_crandnv_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_crandnv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_zrandnv_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zrandnv_unb_var1.unwrap()(n, x, incx, cntx, rntm)
            }

pub unsafe fn bli_srandm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_srandm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_drandm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_drandm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_crandm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_crandm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zrandm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zrandm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_srandnm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_srandnm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_drandnm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_drandnm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_crandnm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_crandnm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_zrandnm_unb_var1(diagoffx: doff_t, uplox: uplo_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zrandnm_unb_var1.unwrap()(diagoffx, uplox, m, n, x, rs_x, cs_x, cntx, rntm)
            }

pub unsafe fn bli_ssumsqv_unb_var1(n: dim_t, x: *mut f32, incx: inc_t, scale: *mut f32, sumsq: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_ssumsqv_unb_var1.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_dsumsqv_unb_var1(n: dim_t, x: *mut f64, incx: inc_t, scale: *mut f64, sumsq: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_dsumsqv_unb_var1.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_csumsqv_unb_var1(n: dim_t, x: *mut scomplex, incx: inc_t, scale: *mut f32, sumsq: *mut f32, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_csumsqv_unb_var1.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_zsumsqv_unb_var1(n: dim_t, x: *mut dcomplex, incx: inc_t, scale: *mut f64, sumsq: *mut f64, cntx: *mut cntx_t, rntm: *mut rntm_t) {
                dyload_lib().bli_zsumsqv_unb_var1.unwrap()(n, x, incx, scale, sumsq, cntx, rntm)
            }

pub unsafe fn bli_seqv_unb_var1(conjx: conj_t, n: dim_t, x: *mut f32, incx: inc_t, y: *mut f32, incy: inc_t) -> bool {
                dyload_lib().bli_seqv_unb_var1.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_deqv_unb_var1(conjx: conj_t, n: dim_t, x: *mut f64, incx: inc_t, y: *mut f64, incy: inc_t) -> bool {
                dyload_lib().bli_deqv_unb_var1.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_ceqv_unb_var1(conjx: conj_t, n: dim_t, x: *mut scomplex, incx: inc_t, y: *mut scomplex, incy: inc_t) -> bool {
                dyload_lib().bli_ceqv_unb_var1.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_zeqv_unb_var1(conjx: conj_t, n: dim_t, x: *mut dcomplex, incx: inc_t, y: *mut dcomplex, incy: inc_t) -> bool {
                dyload_lib().bli_zeqv_unb_var1.unwrap()(conjx, n, x, incx, y, incy)
            }

pub unsafe fn bli_seqm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f32, rs_x: inc_t, cs_x: inc_t, y: *mut f32, rs_y: inc_t, cs_y: inc_t) -> bool {
                dyload_lib().bli_seqm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_deqm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut f64, rs_x: inc_t, cs_x: inc_t, y: *mut f64, rs_y: inc_t, cs_y: inc_t) -> bool {
                dyload_lib().bli_deqm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_ceqm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut scomplex, rs_x: inc_t, cs_x: inc_t, y: *mut scomplex, rs_y: inc_t, cs_y: inc_t) -> bool {
                dyload_lib().bli_ceqm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_zeqm_unb_var1(diagoffx: doff_t, diagx: diag_t, uplox: uplo_t, transx: trans_t, m: dim_t, n: dim_t, x: *mut dcomplex, rs_x: inc_t, cs_x: inc_t, y: *mut dcomplex, rs_y: inc_t, cs_y: inc_t) -> bool {
                dyload_lib().bli_zeqm_unb_var1.unwrap()(diagoffx, diagx, uplox, transx, m, n, x, rs_x, cs_x, y, rs_y, cs_y)
            }

pub unsafe fn bli_sfprintv(file: *mut FILE, s1: *const c_char, n: dim_t, x: *const f32, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_sfprintv.unwrap()(file, s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_dfprintv(file: *mut FILE, s1: *const c_char, n: dim_t, x: *const f64, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_dfprintv.unwrap()(file, s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_cfprintv(file: *mut FILE, s1: *const c_char, n: dim_t, x: *const scomplex, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_cfprintv.unwrap()(file, s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_zfprintv(file: *mut FILE, s1: *const c_char, n: dim_t, x: *const dcomplex, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_zfprintv.unwrap()(file, s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_ifprintv(file: *mut FILE, s1: *const c_char, n: dim_t, x: *const gint_t, incx: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_ifprintv.unwrap()(file, s1, n, x, incx, format, s2)
            }

pub unsafe fn bli_sfprintm(file: *mut FILE, s1: *const c_char, m: dim_t, n: dim_t, x: *const f32, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_sfprintm.unwrap()(file, s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_dfprintm(file: *mut FILE, s1: *const c_char, m: dim_t, n: dim_t, x: *const f64, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_dfprintm.unwrap()(file, s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_cfprintm(file: *mut FILE, s1: *const c_char, m: dim_t, n: dim_t, x: *const scomplex, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_cfprintm.unwrap()(file, s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_zfprintm(file: *mut FILE, s1: *const c_char, m: dim_t, n: dim_t, x: *const dcomplex, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_zfprintm.unwrap()(file, s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bli_ifprintm(file: *mut FILE, s1: *const c_char, m: dim_t, n: dim_t, x: *const gint_t, rs_x: inc_t, cs_x: inc_t, format: *const c_char, s2: *const c_char) {
                dyload_lib().bli_ifprintm.unwrap()(file, s1, m, n, x, rs_x, cs_x, format, s2)
            }

pub unsafe fn bla_r_sign(a: *const bla_real, b: *const bla_real) -> f64 {
                dyload_lib().bla_r_sign.unwrap()(a, b)
            }

pub unsafe fn bla_d_sign(a: *const bla_double, b: *const bla_double) -> f64 {
                dyload_lib().bla_d_sign.unwrap()(a, b)
            }

pub unsafe fn bla_r_cnjg(dest: *mut bla_scomplex, src: *const bla_scomplex) {
                dyload_lib().bla_r_cnjg.unwrap()(dest, src)
            }

pub unsafe fn bla_d_cnjg(dest: *mut bla_dcomplex, src: *const bla_dcomplex) {
                dyload_lib().bla_d_cnjg.unwrap()(dest, src)
            }

pub unsafe fn bla_r_imag(z: *const bla_scomplex) -> bla_real {
                dyload_lib().bla_r_imag.unwrap()(z)
            }

pub unsafe fn bla_d_imag(z: *const bla_dcomplex) -> f64 {
                dyload_lib().bla_d_imag.unwrap()(z)
            }

pub unsafe fn bla_c_div(cp: *mut bla_scomplex, ap: *const bla_scomplex, bp: *const bla_scomplex) {
                dyload_lib().bla_c_div.unwrap()(cp, ap, bp)
            }

pub unsafe fn bla_z_div(cp: *mut bla_dcomplex, ap: *const bla_dcomplex, bp: *const bla_dcomplex) {
                dyload_lib().bla_z_div.unwrap()(cp, ap, bp)
            }

pub unsafe fn bla_f__cabs(real: f64, imag: f64) -> f64 {
                dyload_lib().bla_f__cabs.unwrap()(real, imag)
            }

pub unsafe fn bla_r_abs(x: *const bla_real) -> f64 {
                dyload_lib().bla_r_abs.unwrap()(x)
            }

pub unsafe fn bla_d_abs(x: *const bla_double) -> f64 {
                dyload_lib().bla_d_abs.unwrap()(x)
            }

pub unsafe fn bla_c_abs(z: *const bla_scomplex) -> f64 {
                dyload_lib().bla_c_abs.unwrap()(z)
            }

pub unsafe fn bla_z_abs(z: *const bla_dcomplex) -> f64 {
                dyload_lib().bla_z_abs.unwrap()(z)
            }

pub unsafe fn lsame_(ca: *const c_char, cb: *const c_char, ca_len: c_int, cb_len: c_int) -> c_int {
                dyload_lib().lsame_.unwrap()(ca, cb, ca_len, cb_len)
            }

pub unsafe fn xerbla_(srname: *const bla_character, info: *const bla_integer, srname_len: ftnlen) -> c_int {
                dyload_lib().xerbla_.unwrap()(srname, info, srname_len)
            }

pub unsafe fn xerbla_array_(srname: *const bla_character, srname_len: bla_integer, info: *const bla_integer) -> c_int {
                dyload_lib().xerbla_array_.unwrap()(srname, srname_len, info)
            }

pub unsafe fn scabs1_(z: *mut bla_scomplex) -> bla_real {
                dyload_lib().scabs1_.unwrap()(z)
            }

pub unsafe fn dcabs1_(z: *mut bla_dcomplex) -> bla_double {
                dyload_lib().dcabs1_.unwrap()(z)
            }

pub unsafe fn isamax_(n: *const f77_int, x: *const f32, incx: *const f77_int) -> f77_int {
                dyload_lib().isamax_.unwrap()(n, x, incx)
            }

pub unsafe fn idamax_(n: *const f77_int, x: *const f64, incx: *const f77_int) -> f77_int {
                dyload_lib().idamax_.unwrap()(n, x, incx)
            }

pub unsafe fn icamax_(n: *const f77_int, x: *const scomplex, incx: *const f77_int) -> f77_int {
                dyload_lib().icamax_.unwrap()(n, x, incx)
            }

pub unsafe fn izamax_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int) -> f77_int {
                dyload_lib().izamax_.unwrap()(n, x, incx)
            }

pub unsafe fn sasum_(n: *const f77_int, x: *const f32, incx: *const f77_int) -> f32 {
                dyload_lib().sasum_.unwrap()(n, x, incx)
            }

pub unsafe fn dasum_(n: *const f77_int, x: *const f64, incx: *const f77_int) -> f64 {
                dyload_lib().dasum_.unwrap()(n, x, incx)
            }

pub unsafe fn scasum_(n: *const f77_int, x: *const scomplex, incx: *const f77_int) -> f32 {
                dyload_lib().scasum_.unwrap()(n, x, incx)
            }

pub unsafe fn dzasum_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int) -> f64 {
                dyload_lib().dzasum_.unwrap()(n, x, incx)
            }

pub unsafe fn saxpy_(n: *const f77_int, alpha: *const f32, x: *const f32, incx: *const f77_int, y: *mut f32, incy: *const f77_int) {
                dyload_lib().saxpy_.unwrap()(n, alpha, x, incx, y, incy)
            }

pub unsafe fn daxpy_(n: *const f77_int, alpha: *const f64, x: *const f64, incx: *const f77_int, y: *mut f64, incy: *const f77_int) {
                dyload_lib().daxpy_.unwrap()(n, alpha, x, incx, y, incy)
            }

pub unsafe fn caxpy_(n: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().caxpy_.unwrap()(n, alpha, x, incx, y, incy)
            }

pub unsafe fn zaxpy_(n: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zaxpy_.unwrap()(n, alpha, x, incx, y, incy)
            }

pub unsafe fn scopy_(n: *const f77_int, x: *const f32, incx: *const f77_int, y: *mut f32, incy: *const f77_int) {
                dyload_lib().scopy_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn dcopy_(n: *const f77_int, x: *const f64, incx: *const f77_int, y: *mut f64, incy: *const f77_int) {
                dyload_lib().dcopy_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn ccopy_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().ccopy_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn zcopy_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zcopy_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn sdot_(n: *const f77_int, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int) -> f32 {
                dyload_lib().sdot_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn ddot_(n: *const f77_int, x: *const f64, incx: *const f77_int, y: *const f64, incy: *const f77_int) -> f64 {
                dyload_lib().ddot_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn cdotc_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int) -> scomplex {
                dyload_lib().cdotc_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn cdotu_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int) -> scomplex {
                dyload_lib().cdotu_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn zdotc_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int) -> dcomplex {
                dyload_lib().zdotc_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn zdotu_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int) -> dcomplex {
                dyload_lib().zdotu_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn sdsdot_(n: *const f77_int, sb: *const f32, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int) -> f32 {
                dyload_lib().sdsdot_.unwrap()(n, sb, x, incx, y, incy)
            }

pub unsafe fn dsdot_(n: *const f77_int, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int) -> f64 {
                dyload_lib().dsdot_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn snrm2_(n: *const f77_int, x: *const f32, incx: *const f77_int) -> f32 {
                dyload_lib().snrm2_.unwrap()(n, x, incx)
            }

pub unsafe fn dnrm2_(n: *const f77_int, x: *const f64, incx: *const f77_int) -> f64 {
                dyload_lib().dnrm2_.unwrap()(n, x, incx)
            }

pub unsafe fn scnrm2_(n: *const f77_int, x: *const scomplex, incx: *const f77_int) -> f32 {
                dyload_lib().scnrm2_.unwrap()(n, x, incx)
            }

pub unsafe fn dznrm2_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int) -> f64 {
                dyload_lib().dznrm2_.unwrap()(n, x, incx)
            }

pub unsafe fn srot_(n: *const bla_integer, sx: *mut bla_real, incx: *const bla_integer, sy: *mut bla_real, incy: *const bla_integer, c__: *const bla_real, s: *const bla_real) -> c_int {
                dyload_lib().srot_.unwrap()(n, sx, incx, sy, incy, c__, s)
            }

pub unsafe fn drot_(n: *const bla_integer, dx: *mut bla_double, incx: *const bla_integer, dy: *mut bla_double, incy: *const bla_integer, c__: *const bla_double, s: *const bla_double) -> c_int {
                dyload_lib().drot_.unwrap()(n, dx, incx, dy, incy, c__, s)
            }

pub unsafe fn csrot_(n: *const bla_integer, cx: *mut bla_scomplex, incx: *const bla_integer, cy: *mut bla_scomplex, incy: *const bla_integer, c__: *const bla_real, s: *const bla_real) -> c_int {
                dyload_lib().csrot_.unwrap()(n, cx, incx, cy, incy, c__, s)
            }

pub unsafe fn zdrot_(n: *const bla_integer, zx: *mut bla_dcomplex, incx: *const bla_integer, zy: *mut bla_dcomplex, incy: *const bla_integer, c__: *const bla_double, s: *const bla_double) -> c_int {
                dyload_lib().zdrot_.unwrap()(n, zx, incx, zy, incy, c__, s)
            }

pub unsafe fn crot_(n: *const bla_integer, cx: *mut bla_scomplex, incx: *const bla_integer, cy: *mut bla_scomplex, incy: *const bla_integer, c__: *const bla_real, s: *const bla_scomplex) -> c_int {
                dyload_lib().crot_.unwrap()(n, cx, incx, cy, incy, c__, s)
            }

pub unsafe fn zrot_(n: *const bla_integer, cx: *mut bla_dcomplex, incx: *const bla_integer, cy: *mut bla_dcomplex, incy: *const bla_integer, c__: *const bla_double, s: *const bla_dcomplex) -> c_int {
                dyload_lib().zrot_.unwrap()(n, cx, incx, cy, incy, c__, s)
            }

pub unsafe fn srotg_(sa: *mut bla_real, sb: *mut bla_real, c__: *mut bla_real, s: *mut bla_real) -> c_int {
                dyload_lib().srotg_.unwrap()(sa, sb, c__, s)
            }

pub unsafe fn drotg_(da: *mut bla_double, db: *mut bla_double, c__: *mut bla_double, s: *mut bla_double) -> c_int {
                dyload_lib().drotg_.unwrap()(da, db, c__, s)
            }

pub unsafe fn crotg_(ca: *mut bla_scomplex, cb: *mut bla_scomplex, c__: *mut bla_real, s: *mut bla_scomplex) -> c_int {
                dyload_lib().crotg_.unwrap()(ca, cb, c__, s)
            }

pub unsafe fn zrotg_(ca: *mut bla_dcomplex, cb: *mut bla_dcomplex, c__: *mut bla_double, s: *mut bla_dcomplex) -> c_int {
                dyload_lib().zrotg_.unwrap()(ca, cb, c__, s)
            }

pub unsafe fn srotm_(n: *const bla_integer, sx: *mut bla_real, incx: *const bla_integer, sy: *mut bla_real, incy: *const bla_integer, sparam: *const bla_real) -> c_int {
                dyload_lib().srotm_.unwrap()(n, sx, incx, sy, incy, sparam)
            }

pub unsafe fn drotm_(n: *const bla_integer, dx: *mut bla_double, incx: *const bla_integer, dy: *mut bla_double, incy: *const bla_integer, dparam: *const bla_double) -> c_int {
                dyload_lib().drotm_.unwrap()(n, dx, incx, dy, incy, dparam)
            }

pub unsafe fn srotmg_(sd1: *mut bla_real, sd2: *mut bla_real, sx1: *mut bla_real, sy1: *const bla_real, sparam: *mut bla_real) -> c_int {
                dyload_lib().srotmg_.unwrap()(sd1, sd2, sx1, sy1, sparam)
            }

pub unsafe fn drotmg_(dd1: *mut bla_double, dd2: *mut bla_double, dx1: *mut bla_double, dy1: *const bla_double, dparam: *mut bla_double) -> c_int {
                dyload_lib().drotmg_.unwrap()(dd1, dd2, dx1, dy1, dparam)
            }

pub unsafe fn sscal_(n: *const f77_int, alpha: *const f32, x: *mut f32, incx: *const f77_int) {
                dyload_lib().sscal_.unwrap()(n, alpha, x, incx)
            }

pub unsafe fn dscal_(n: *const f77_int, alpha: *const f64, x: *mut f64, incx: *const f77_int) {
                dyload_lib().dscal_.unwrap()(n, alpha, x, incx)
            }

pub unsafe fn cscal_(n: *const f77_int, alpha: *const scomplex, x: *mut scomplex, incx: *const f77_int) {
                dyload_lib().cscal_.unwrap()(n, alpha, x, incx)
            }

pub unsafe fn zscal_(n: *const f77_int, alpha: *const dcomplex, x: *mut dcomplex, incx: *const f77_int) {
                dyload_lib().zscal_.unwrap()(n, alpha, x, incx)
            }

pub unsafe fn csscal_(n: *const f77_int, alpha: *const f32, x: *mut scomplex, incx: *const f77_int) {
                dyload_lib().csscal_.unwrap()(n, alpha, x, incx)
            }

pub unsafe fn zdscal_(n: *const f77_int, alpha: *const f64, x: *mut dcomplex, incx: *const f77_int) {
                dyload_lib().zdscal_.unwrap()(n, alpha, x, incx)
            }

pub unsafe fn sswap_(n: *const f77_int, x: *mut f32, incx: *const f77_int, y: *mut f32, incy: *const f77_int) {
                dyload_lib().sswap_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn dswap_(n: *const f77_int, x: *mut f64, incx: *const f77_int, y: *mut f64, incy: *const f77_int) {
                dyload_lib().dswap_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn cswap_(n: *const f77_int, x: *mut scomplex, incx: *const f77_int, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().cswap_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn zswap_(n: *const f77_int, x: *mut dcomplex, incx: *const f77_int, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zswap_.unwrap()(n, x, incx, y, incy)
            }

pub unsafe fn isamaxsub_(n: *const f77_int, x: *const f32, incx: *const f77_int, rval: *mut f77_int) {
                dyload_lib().isamaxsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn idamaxsub_(n: *const f77_int, x: *const f64, incx: *const f77_int, rval: *mut f77_int) {
                dyload_lib().idamaxsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn icamaxsub_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, rval: *mut f77_int) {
                dyload_lib().icamaxsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn izamaxsub_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, rval: *mut f77_int) {
                dyload_lib().izamaxsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn sasumsub_(n: *const f77_int, x: *const f32, incx: *const f77_int, rval: *mut f32) {
                dyload_lib().sasumsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn dasumsub_(n: *const f77_int, x: *const f64, incx: *const f77_int, rval: *mut f64) {
                dyload_lib().dasumsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn scasumsub_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, rval: *mut f32) {
                dyload_lib().scasumsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn dzasumsub_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, rval: *mut f64) {
                dyload_lib().dzasumsub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn sdotsub_(n: *const f77_int, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int, rval: *mut f32) {
                dyload_lib().sdotsub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn ddotsub_(n: *const f77_int, x: *const f64, incx: *const f77_int, y: *const f64, incy: *const f77_int, rval: *mut f64) {
                dyload_lib().ddotsub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn cdotcsub_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int, rval: *mut scomplex) {
                dyload_lib().cdotcsub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn cdotusub_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int, rval: *mut scomplex) {
                dyload_lib().cdotusub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn zdotcsub_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int, rval: *mut dcomplex) {
                dyload_lib().zdotcsub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn zdotusub_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int, rval: *mut dcomplex) {
                dyload_lib().zdotusub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn sdsdotsub_(n: *const f77_int, sb: *const f32, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int, rval: *mut f32) {
                dyload_lib().sdsdotsub_.unwrap()(n, sb, x, incx, y, incy, rval)
            }

pub unsafe fn dsdotsub_(n: *const f77_int, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int, rval: *mut f64) {
                dyload_lib().dsdotsub_.unwrap()(n, x, incx, y, incy, rval)
            }

pub unsafe fn snrm2sub_(n: *const f77_int, x: *const f32, incx: *const f77_int, rval: *mut f32) {
                dyload_lib().snrm2sub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn dnrm2sub_(n: *const f77_int, x: *const f64, incx: *const f77_int, rval: *mut f64) {
                dyload_lib().dnrm2sub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn scnrm2sub_(n: *const f77_int, x: *const scomplex, incx: *const f77_int, rval: *mut f32) {
                dyload_lib().scnrm2sub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn dznrm2sub_(n: *const f77_int, x: *const dcomplex, incx: *const f77_int, rval: *mut f64) {
                dyload_lib().dznrm2sub_.unwrap()(n, x, incx, rval)
            }

pub unsafe fn sgemv_(transa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, x: *const f32, incx: *const f77_int, beta: *const f32, y: *mut f32, incy: *const f77_int) {
                dyload_lib().sgemv_.unwrap()(transa, m, n, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn dgemv_(transa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, x: *const f64, incx: *const f77_int, beta: *const f64, y: *mut f64, incy: *const f77_int) {
                dyload_lib().dgemv_.unwrap()(transa, m, n, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn cgemv_(transa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, x: *const scomplex, incx: *const f77_int, beta: *const scomplex, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().cgemv_.unwrap()(transa, m, n, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn zgemv_(transa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, x: *const dcomplex, incx: *const f77_int, beta: *const dcomplex, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zgemv_.unwrap()(transa, m, n, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn sger_(m: *const f77_int, n: *const f77_int, alpha: *const f32, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int, a: *mut f32, lda: *const f77_int) {
                dyload_lib().sger_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn dger_(m: *const f77_int, n: *const f77_int, alpha: *const f64, x: *const f64, incx: *const f77_int, y: *const f64, incy: *const f77_int, a: *mut f64, lda: *const f77_int) {
                dyload_lib().dger_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn cgerc_(m: *const f77_int, n: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int, a: *mut scomplex, lda: *const f77_int) {
                dyload_lib().cgerc_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn cgeru_(m: *const f77_int, n: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int, a: *mut scomplex, lda: *const f77_int) {
                dyload_lib().cgeru_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn zgerc_(m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int, a: *mut dcomplex, lda: *const f77_int) {
                dyload_lib().zgerc_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn zgeru_(m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int, a: *mut dcomplex, lda: *const f77_int) {
                dyload_lib().zgeru_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn chemv_(uploa: *const f77_char, m: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, x: *const scomplex, incx: *const f77_int, beta: *const scomplex, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().chemv_.unwrap()(uploa, m, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn zhemv_(uploa: *const f77_char, m: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, x: *const dcomplex, incx: *const f77_int, beta: *const dcomplex, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zhemv_.unwrap()(uploa, m, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn cher_(uploa: *const f77_char, m: *const f77_int, alpha: *const f32, x: *const scomplex, incx: *const f77_int, a: *mut scomplex, lda: *const f77_int) {
                dyload_lib().cher_.unwrap()(uploa, m, alpha, x, incx, a, lda)
            }

pub unsafe fn zher_(uploa: *const f77_char, m: *const f77_int, alpha: *const f64, x: *const dcomplex, incx: *const f77_int, a: *mut dcomplex, lda: *const f77_int) {
                dyload_lib().zher_.unwrap()(uploa, m, alpha, x, incx, a, lda)
            }

pub unsafe fn cher2_(uploa: *const f77_char, m: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int, a: *mut scomplex, lda: *const f77_int) {
                dyload_lib().cher2_.unwrap()(uploa, m, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn zher2_(uploa: *const f77_char, m: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int, a: *mut dcomplex, lda: *const f77_int) {
                dyload_lib().zher2_.unwrap()(uploa, m, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn ssymv_(uploa: *const f77_char, m: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, x: *const f32, incx: *const f77_int, beta: *const f32, y: *mut f32, incy: *const f77_int) {
                dyload_lib().ssymv_.unwrap()(uploa, m, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn dsymv_(uploa: *const f77_char, m: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, x: *const f64, incx: *const f77_int, beta: *const f64, y: *mut f64, incy: *const f77_int) {
                dyload_lib().dsymv_.unwrap()(uploa, m, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn csymv_(uploa: *const f77_char, m: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, x: *const scomplex, incx: *const f77_int, beta: *const scomplex, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().csymv_.unwrap()(uploa, m, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn zsymv_(uploa: *const f77_char, m: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, x: *const dcomplex, incx: *const f77_int, beta: *const dcomplex, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zsymv_.unwrap()(uploa, m, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn ssyr_(uploa: *const f77_char, m: *const f77_int, alpha: *const f32, x: *const f32, incx: *const f77_int, a: *mut f32, lda: *const f77_int) {
                dyload_lib().ssyr_.unwrap()(uploa, m, alpha, x, incx, a, lda)
            }

pub unsafe fn dsyr_(uploa: *const f77_char, m: *const f77_int, alpha: *const f64, x: *const f64, incx: *const f77_int, a: *mut f64, lda: *const f77_int) {
                dyload_lib().dsyr_.unwrap()(uploa, m, alpha, x, incx, a, lda)
            }

pub unsafe fn csyr_(uploa: *const f77_char, m: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, a: *mut scomplex, lda: *const f77_int) {
                dyload_lib().csyr_.unwrap()(uploa, m, alpha, x, incx, a, lda)
            }

pub unsafe fn zsyr_(uploa: *const f77_char, m: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, a: *mut dcomplex, lda: *const f77_int) {
                dyload_lib().zsyr_.unwrap()(uploa, m, alpha, x, incx, a, lda)
            }

pub unsafe fn ssyr2_(uploa: *const f77_char, m: *const f77_int, alpha: *const f32, x: *const f32, incx: *const f77_int, y: *const f32, incy: *const f77_int, a: *mut f32, lda: *const f77_int) {
                dyload_lib().ssyr2_.unwrap()(uploa, m, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn dsyr2_(uploa: *const f77_char, m: *const f77_int, alpha: *const f64, x: *const f64, incx: *const f77_int, y: *const f64, incy: *const f77_int, a: *mut f64, lda: *const f77_int) {
                dyload_lib().dsyr2_.unwrap()(uploa, m, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn csyr2_(uploa: *const f77_char, m: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, y: *const scomplex, incy: *const f77_int, a: *mut scomplex, lda: *const f77_int) {
                dyload_lib().csyr2_.unwrap()(uploa, m, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn zsyr2_(uploa: *const f77_char, m: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, y: *const dcomplex, incy: *const f77_int, a: *mut dcomplex, lda: *const f77_int) {
                dyload_lib().zsyr2_.unwrap()(uploa, m, alpha, x, incx, y, incy, a, lda)
            }

pub unsafe fn strmv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const f32, lda: *const f77_int, x: *mut f32, incx: *const f77_int) {
                dyload_lib().strmv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn dtrmv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const f64, lda: *const f77_int, x: *mut f64, incx: *const f77_int) {
                dyload_lib().dtrmv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn ctrmv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const scomplex, lda: *const f77_int, x: *mut scomplex, incx: *const f77_int) {
                dyload_lib().ctrmv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn ztrmv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const dcomplex, lda: *const f77_int, x: *mut dcomplex, incx: *const f77_int) {
                dyload_lib().ztrmv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn strsv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const f32, lda: *const f77_int, x: *mut f32, incx: *const f77_int) {
                dyload_lib().strsv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn dtrsv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const f64, lda: *const f77_int, x: *mut f64, incx: *const f77_int) {
                dyload_lib().dtrsv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn ctrsv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const scomplex, lda: *const f77_int, x: *mut scomplex, incx: *const f77_int) {
                dyload_lib().ctrsv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn ztrsv_(uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, a: *const dcomplex, lda: *const f77_int, x: *mut dcomplex, incx: *const f77_int) {
                dyload_lib().ztrsv_.unwrap()(uploa, transa, diaga, m, a, lda, x, incx)
            }

pub unsafe fn chpmv_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_scomplex, ap: *const bla_scomplex, x: *const bla_scomplex, incx: *const bla_integer, beta: *const bla_scomplex, y: *mut bla_scomplex, incy: *const bla_integer) -> c_int {
                dyload_lib().chpmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
            }

pub unsafe fn zhpmv_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_dcomplex, ap: *const bla_dcomplex, x: *const bla_dcomplex, incx: *const bla_integer, beta: *const bla_dcomplex, y: *mut bla_dcomplex, incy: *const bla_integer) -> c_int {
                dyload_lib().zhpmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
            }

pub unsafe fn chpr_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_real, x: *const bla_scomplex, incx: *const bla_integer, ap: *mut bla_scomplex) -> c_int {
                dyload_lib().chpr_.unwrap()(uplo, n, alpha, x, incx, ap)
            }

pub unsafe fn zhpr_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_double, x: *const bla_dcomplex, incx: *const bla_integer, ap: *mut bla_dcomplex) -> c_int {
                dyload_lib().zhpr_.unwrap()(uplo, n, alpha, x, incx, ap)
            }

pub unsafe fn chpr2_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_scomplex, x: *const bla_scomplex, incx: *const bla_integer, y: *const bla_scomplex, incy: *const bla_integer, ap: *mut bla_scomplex) -> c_int {
                dyload_lib().chpr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
            }

pub unsafe fn zhpr2_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_dcomplex, x: *const bla_dcomplex, incx: *const bla_integer, y: *const bla_dcomplex, incy: *const bla_integer, ap: *mut bla_dcomplex) -> c_int {
                dyload_lib().zhpr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
            }

pub unsafe fn dspmv_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_double, ap: *const bla_double, x: *const bla_double, incx: *const bla_integer, beta: *const bla_double, y: *mut bla_double, incy: *const bla_integer) -> c_int {
                dyload_lib().dspmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
            }

pub unsafe fn sspmv_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_real, ap: *const bla_real, x: *const bla_real, incx: *const bla_integer, beta: *const bla_real, y: *mut bla_real, incy: *const bla_integer) -> c_int {
                dyload_lib().sspmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
            }

pub unsafe fn dspr_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_double, x: *const bla_double, incx: *const bla_integer, ap: *mut bla_double) -> c_int {
                dyload_lib().dspr_.unwrap()(uplo, n, alpha, x, incx, ap)
            }

pub unsafe fn sspr_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_real, x: *const bla_real, incx: *const bla_integer, ap: *mut bla_real) -> c_int {
                dyload_lib().sspr_.unwrap()(uplo, n, alpha, x, incx, ap)
            }

pub unsafe fn dspr2_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_double, x: *const bla_double, incx: *const bla_integer, y: *const bla_double, incy: *const bla_integer, ap: *mut bla_double) -> c_int {
                dyload_lib().dspr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
            }

pub unsafe fn sspr2_(uplo: *const bla_character, n: *const bla_integer, alpha: *const bla_real, x: *const bla_real, incx: *const bla_integer, y: *const bla_real, incy: *const bla_integer, ap: *mut bla_real) -> c_int {
                dyload_lib().sspr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
            }

pub unsafe fn ctpmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_scomplex, x: *mut bla_scomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ctpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn dtpmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_double, x: *mut bla_double, incx: *const bla_integer) -> c_int {
                dyload_lib().dtpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn stpmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_real, x: *mut bla_real, incx: *const bla_integer) -> c_int {
                dyload_lib().stpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn ztpmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_dcomplex, x: *mut bla_dcomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ztpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn ctpsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_scomplex, x: *mut bla_scomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ctpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn dtpsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_double, x: *mut bla_double, incx: *const bla_integer) -> c_int {
                dyload_lib().dtpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn stpsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_real, x: *mut bla_real, incx: *const bla_integer) -> c_int {
                dyload_lib().stpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn ztpsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, ap: *const bla_dcomplex, x: *mut bla_dcomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ztpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
            }

pub unsafe fn cgbmv_(trans: *const bla_character, m: *const bla_integer, n: *const bla_integer, kl: *const bla_integer, ku: *const bla_integer, alpha: *const bla_scomplex, a: *const bla_scomplex, lda: *const bla_integer, x: *const bla_scomplex, incx: *const bla_integer, beta: *const bla_scomplex, y: *mut bla_scomplex, incy: *const bla_integer) -> c_int {
                dyload_lib().cgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn dgbmv_(trans: *const bla_character, m: *const bla_integer, n: *const bla_integer, kl: *const bla_integer, ku: *const bla_integer, alpha: *const bla_double, a: *const bla_double, lda: *const bla_integer, x: *const bla_double, incx: *const bla_integer, beta: *const bla_double, y: *mut bla_double, incy: *const bla_integer) -> c_int {
                dyload_lib().dgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn sgbmv_(trans: *const bla_character, m: *const bla_integer, n: *const bla_integer, kl: *const bla_integer, ku: *const bla_integer, alpha: *const bla_real, a: *const bla_real, lda: *const bla_integer, x: *const bla_real, incx: *const bla_integer, beta: *const bla_real, y: *mut bla_real, incy: *const bla_integer) -> c_int {
                dyload_lib().sgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn zgbmv_(trans: *const bla_character, m: *const bla_integer, n: *const bla_integer, kl: *const bla_integer, ku: *const bla_integer, alpha: *const bla_dcomplex, a: *const bla_dcomplex, lda: *const bla_integer, x: *const bla_dcomplex, incx: *const bla_integer, beta: *const bla_dcomplex, y: *mut bla_dcomplex, incy: *const bla_integer) -> c_int {
                dyload_lib().zgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn chbmv_(uplo: *const bla_character, n: *const bla_integer, k: *const bla_integer, alpha: *const bla_scomplex, a: *const bla_scomplex, lda: *const bla_integer, x: *const bla_scomplex, incx: *const bla_integer, beta: *const bla_scomplex, y: *mut bla_scomplex, incy: *const bla_integer) -> c_int {
                dyload_lib().chbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn zhbmv_(uplo: *const bla_character, n: *const bla_integer, k: *const bla_integer, alpha: *const bla_dcomplex, a: *const bla_dcomplex, lda: *const bla_integer, x: *const bla_dcomplex, incx: *const bla_integer, beta: *const bla_dcomplex, y: *mut bla_dcomplex, incy: *const bla_integer) -> c_int {
                dyload_lib().zhbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn dsbmv_(uplo: *const bla_character, n: *const bla_integer, k: *const bla_integer, alpha: *const bla_double, a: *const bla_double, lda: *const bla_integer, x: *const bla_double, incx: *const bla_integer, beta: *const bla_double, y: *mut bla_double, incy: *const bla_integer) -> c_int {
                dyload_lib().dsbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn ssbmv_(uplo: *const bla_character, n: *const bla_integer, k: *const bla_integer, alpha: *const bla_real, a: *const bla_real, lda: *const bla_integer, x: *const bla_real, incx: *const bla_integer, beta: *const bla_real, y: *mut bla_real, incy: *const bla_integer) -> c_int {
                dyload_lib().ssbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
            }

pub unsafe fn ctbmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_scomplex, lda: *const bla_integer, x: *mut bla_scomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ctbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn dtbmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_double, lda: *const bla_integer, x: *mut bla_double, incx: *const bla_integer) -> c_int {
                dyload_lib().dtbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn stbmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_real, lda: *const bla_integer, x: *mut bla_real, incx: *const bla_integer) -> c_int {
                dyload_lib().stbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn ztbmv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_dcomplex, lda: *const bla_integer, x: *mut bla_dcomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ztbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn ctbsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_scomplex, lda: *const bla_integer, x: *mut bla_scomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ctbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn dtbsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_double, lda: *const bla_integer, x: *mut bla_double, incx: *const bla_integer) -> c_int {
                dyload_lib().dtbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn stbsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_real, lda: *const bla_integer, x: *mut bla_real, incx: *const bla_integer) -> c_int {
                dyload_lib().stbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn ztbsv_(uplo: *const bla_character, trans: *const bla_character, diag: *const bla_character, n: *const bla_integer, k: *const bla_integer, a: *const bla_dcomplex, lda: *const bla_integer, x: *mut bla_dcomplex, incx: *const bla_integer) -> c_int {
                dyload_lib().ztbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
            }

pub unsafe fn sgemm_(transa: *const f77_char, transb: *const f77_char, m: *const f77_int, n: *const f77_int, k: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *const f32, ldb: *const f77_int, beta: *const f32, c: *mut f32, ldc: *const f77_int) {
                dyload_lib().sgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn dgemm_(transa: *const f77_char, transb: *const f77_char, m: *const f77_int, n: *const f77_int, k: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *const f64, ldb: *const f77_int, beta: *const f64, c: *mut f64, ldc: *const f77_int) {
                dyload_lib().dgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn cgemm_(transa: *const f77_char, transb: *const f77_char, m: *const f77_int, n: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().cgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zgemm_(transa: *const f77_char, transb: *const f77_char, m: *const f77_int, n: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn chemm_(side: *const f77_char, uploa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().chemm_.unwrap()(side, uploa, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zhemm_(side: *const f77_char, uploa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zhemm_.unwrap()(side, uploa, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn cherk_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f32, a: *const scomplex, lda: *const f77_int, beta: *const f32, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().cherk_.unwrap()(uploc, transa, m, k, alpha, a, lda, beta, c, ldc)
            }

pub unsafe fn zherk_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f64, a: *const dcomplex, lda: *const f77_int, beta: *const f64, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zherk_.unwrap()(uploc, transa, m, k, alpha, a, lda, beta, c, ldc)
            }

pub unsafe fn cher2k_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const f32, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().cher2k_.unwrap()(uploc, transa, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zher2k_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const f64, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zher2k_.unwrap()(uploc, transa, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn ssymm_(side: *const f77_char, uploa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *const f32, ldb: *const f77_int, beta: *const f32, c: *mut f32, ldc: *const f77_int) {
                dyload_lib().ssymm_.unwrap()(side, uploa, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn dsymm_(side: *const f77_char, uploa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *const f64, ldb: *const f77_int, beta: *const f64, c: *mut f64, ldc: *const f77_int) {
                dyload_lib().dsymm_.unwrap()(side, uploa, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn csymm_(side: *const f77_char, uploa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().csymm_.unwrap()(side, uploa, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zsymm_(side: *const f77_char, uploa: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zsymm_.unwrap()(side, uploa, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn ssyrk_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, beta: *const f32, c: *mut f32, ldc: *const f77_int) {
                dyload_lib().ssyrk_.unwrap()(uploc, transa, m, k, alpha, a, lda, beta, c, ldc)
            }

pub unsafe fn dsyrk_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, beta: *const f64, c: *mut f64, ldc: *const f77_int) {
                dyload_lib().dsyrk_.unwrap()(uploc, transa, m, k, alpha, a, lda, beta, c, ldc)
            }

pub unsafe fn csyrk_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().csyrk_.unwrap()(uploc, transa, m, k, alpha, a, lda, beta, c, ldc)
            }

pub unsafe fn zsyrk_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zsyrk_.unwrap()(uploc, transa, m, k, alpha, a, lda, beta, c, ldc)
            }

pub unsafe fn ssyr2k_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *const f32, ldb: *const f77_int, beta: *const f32, c: *mut f32, ldc: *const f77_int) {
                dyload_lib().ssyr2k_.unwrap()(uploc, transa, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn dsyr2k_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *const f64, ldb: *const f77_int, beta: *const f64, c: *mut f64, ldc: *const f77_int) {
                dyload_lib().dsyr2k_.unwrap()(uploc, transa, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn csyr2k_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().csyr2k_.unwrap()(uploc, transa, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zsyr2k_(uploc: *const f77_char, transa: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zsyr2k_.unwrap()(uploc, transa, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn strmm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *mut f32, ldb: *const f77_int) {
                dyload_lib().strmm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn dtrmm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *mut f64, ldb: *const f77_int) {
                dyload_lib().dtrmm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn ctrmm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *mut scomplex, ldb: *const f77_int) {
                dyload_lib().ctrmm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn ztrmm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *mut dcomplex, ldb: *const f77_int) {
                dyload_lib().ztrmm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn strsm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *mut f32, ldb: *const f77_int) {
                dyload_lib().strsm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn dtrsm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *mut f64, ldb: *const f77_int) {
                dyload_lib().dtrsm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn ctrsm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *mut scomplex, ldb: *const f77_int) {
                dyload_lib().ctrsm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn ztrsm_(side: *const f77_char, uploa: *const f77_char, transa: *const f77_char, diaga: *const f77_char, m: *const f77_int, n: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *mut dcomplex, ldb: *const f77_int) {
                dyload_lib().ztrsm_.unwrap()(side, uploa, transa, diaga, m, n, alpha, a, lda, b, ldb)
            }

pub unsafe fn saxpby_(n: *const f77_int, alpha: *const f32, x: *const f32, incx: *const f77_int, beta: *const f32, y: *mut f32, incy: *const f77_int) {
                dyload_lib().saxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn daxpby_(n: *const f77_int, alpha: *const f64, x: *const f64, incx: *const f77_int, beta: *const f64, y: *mut f64, incy: *const f77_int) {
                dyload_lib().daxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn caxpby_(n: *const f77_int, alpha: *const scomplex, x: *const scomplex, incx: *const f77_int, beta: *const scomplex, y: *mut scomplex, incy: *const f77_int) {
                dyload_lib().caxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn zaxpby_(n: *const f77_int, alpha: *const dcomplex, x: *const dcomplex, incx: *const f77_int, beta: *const dcomplex, y: *mut dcomplex, incy: *const f77_int) {
                dyload_lib().zaxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
            }

pub unsafe fn sgemmt_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *const f32, ldb: *const f77_int, beta: *const f32, c: *mut f32, ldc: *const f77_int) {
                dyload_lib().sgemmt_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn dgemmt_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *const f64, ldb: *const f77_int, beta: *const f64, c: *mut f64, ldc: *const f77_int) {
                dyload_lib().dgemmt_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn cgemmt_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().cgemmt_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zgemmt_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zgemmt_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn sgemmtr_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f32, a: *const f32, lda: *const f77_int, b: *const f32, ldb: *const f77_int, beta: *const f32, c: *mut f32, ldc: *const f77_int) {
                dyload_lib().sgemmtr_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn dgemmtr_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const f64, a: *const f64, lda: *const f77_int, b: *const f64, ldb: *const f77_int, beta: *const f64, c: *mut f64, ldc: *const f77_int) {
                dyload_lib().dgemmtr_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn cgemmtr_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().cgemmtr_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zgemmtr_(uploc: *const f77_char, transa: *const f77_char, transb: *const f77_char, m: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zgemmtr_.unwrap()(uploc, transa, transb, m, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn sgemm_batch_(transa_array: *const f77_char, transb_array: *const f77_char, m_array: *const f77_int, n_array: *const f77_int, k_array: *const f77_int, alpha_array: *const f32, a_array: *mut *const f32, lda_array: *const f77_int, b_array: *mut *const f32, ldb_array: *const f77_int, beta_array: *const f32, c_array: *mut *mut f32, ldc_array: *const f77_int, group_count: *const f77_int, group_size: *const f77_int) {
                dyload_lib().sgemm_batch_.unwrap()(transa_array, transb_array, m_array, n_array, k_array, alpha_array, a_array, lda_array, b_array, ldb_array, beta_array, c_array, ldc_array, group_count, group_size)
            }

pub unsafe fn dgemm_batch_(transa_array: *const f77_char, transb_array: *const f77_char, m_array: *const f77_int, n_array: *const f77_int, k_array: *const f77_int, alpha_array: *const f64, a_array: *mut *const f64, lda_array: *const f77_int, b_array: *mut *const f64, ldb_array: *const f77_int, beta_array: *const f64, c_array: *mut *mut f64, ldc_array: *const f77_int, group_count: *const f77_int, group_size: *const f77_int) {
                dyload_lib().dgemm_batch_.unwrap()(transa_array, transb_array, m_array, n_array, k_array, alpha_array, a_array, lda_array, b_array, ldb_array, beta_array, c_array, ldc_array, group_count, group_size)
            }

pub unsafe fn cgemm_batch_(transa_array: *const f77_char, transb_array: *const f77_char, m_array: *const f77_int, n_array: *const f77_int, k_array: *const f77_int, alpha_array: *const scomplex, a_array: *mut *const scomplex, lda_array: *const f77_int, b_array: *mut *const scomplex, ldb_array: *const f77_int, beta_array: *const scomplex, c_array: *mut *mut scomplex, ldc_array: *const f77_int, group_count: *const f77_int, group_size: *const f77_int) {
                dyload_lib().cgemm_batch_.unwrap()(transa_array, transb_array, m_array, n_array, k_array, alpha_array, a_array, lda_array, b_array, ldb_array, beta_array, c_array, ldc_array, group_count, group_size)
            }

pub unsafe fn zgemm_batch_(transa_array: *const f77_char, transb_array: *const f77_char, m_array: *const f77_int, n_array: *const f77_int, k_array: *const f77_int, alpha_array: *const dcomplex, a_array: *mut *const dcomplex, lda_array: *const f77_int, b_array: *mut *const dcomplex, ldb_array: *const f77_int, beta_array: *const dcomplex, c_array: *mut *mut dcomplex, ldc_array: *const f77_int, group_count: *const f77_int, group_size: *const f77_int) {
                dyload_lib().zgemm_batch_.unwrap()(transa_array, transb_array, m_array, n_array, k_array, alpha_array, a_array, lda_array, b_array, ldb_array, beta_array, c_array, ldc_array, group_count, group_size)
            }

pub unsafe fn cgemm3m_(transa: *const f77_char, transb: *const f77_char, m: *const f77_int, n: *const f77_int, k: *const f77_int, alpha: *const scomplex, a: *const scomplex, lda: *const f77_int, b: *const scomplex, ldb: *const f77_int, beta: *const scomplex, c: *mut scomplex, ldc: *const f77_int) {
                dyload_lib().cgemm3m_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn zgemm3m_(transa: *const f77_char, transb: *const f77_char, m: *const f77_int, n: *const f77_int, k: *const f77_int, alpha: *const dcomplex, a: *const dcomplex, lda: *const f77_int, b: *const dcomplex, ldb: *const f77_int, beta: *const dcomplex, c: *mut dcomplex, ldc: *const f77_int) {
                dyload_lib().zgemm3m_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
            }

pub unsafe fn bli_thread_set_ways_(jc: *const f77_int, pc: *const f77_int, ic: *const f77_int, jr: *const f77_int, ir: *const f77_int) {
                dyload_lib().bli_thread_set_ways_.unwrap()(jc, pc, ic, jr, ir)
            }

pub unsafe fn bli_thread_set_num_threads_(nt: *const f77_int) {
                dyload_lib().bli_thread_set_num_threads_.unwrap()(nt)
            }

pub unsafe fn cblas_sdsdot(N: f77_int, alpha: f32, X: *const f32, incX: f77_int, Y: *const f32, incY: f77_int) -> f32 {
                dyload_lib().cblas_sdsdot.unwrap()(N, alpha, X, incX, Y, incY)
            }

pub unsafe fn cblas_dsdot(N: f77_int, X: *const f32, incX: f77_int, Y: *const f32, incY: f77_int) -> f64 {
                dyload_lib().cblas_dsdot.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_sdot(N: f77_int, X: *const f32, incX: f77_int, Y: *const f32, incY: f77_int) -> f32 {
                dyload_lib().cblas_sdot.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_ddot(N: f77_int, X: *const f64, incX: f77_int, Y: *const f64, incY: f77_int) -> f64 {
                dyload_lib().cblas_ddot.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_cdotu_sub(N: f77_int, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, dotu: *mut c_void) {
                dyload_lib().cblas_cdotu_sub.unwrap()(N, X, incX, Y, incY, dotu)
            }

pub unsafe fn cblas_cdotc_sub(N: f77_int, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, dotc: *mut c_void) {
                dyload_lib().cblas_cdotc_sub.unwrap()(N, X, incX, Y, incY, dotc)
            }

pub unsafe fn cblas_zdotu_sub(N: f77_int, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, dotu: *mut c_void) {
                dyload_lib().cblas_zdotu_sub.unwrap()(N, X, incX, Y, incY, dotu)
            }

pub unsafe fn cblas_zdotc_sub(N: f77_int, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, dotc: *mut c_void) {
                dyload_lib().cblas_zdotc_sub.unwrap()(N, X, incX, Y, incY, dotc)
            }

pub unsafe fn cblas_snrm2(N: f77_int, X: *const f32, incX: f77_int) -> f32 {
                dyload_lib().cblas_snrm2.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_sasum(N: f77_int, X: *const f32, incX: f77_int) -> f32 {
                dyload_lib().cblas_sasum.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_dnrm2(N: f77_int, X: *const f64, incX: f77_int) -> f64 {
                dyload_lib().cblas_dnrm2.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_dasum(N: f77_int, X: *const f64, incX: f77_int) -> f64 {
                dyload_lib().cblas_dasum.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_scnrm2(N: f77_int, X: *const c_void, incX: f77_int) -> f32 {
                dyload_lib().cblas_scnrm2.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_scasum(N: f77_int, X: *const c_void, incX: f77_int) -> f32 {
                dyload_lib().cblas_scasum.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_dznrm2(N: f77_int, X: *const c_void, incX: f77_int) -> f64 {
                dyload_lib().cblas_dznrm2.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_dzasum(N: f77_int, X: *const c_void, incX: f77_int) -> f64 {
                dyload_lib().cblas_dzasum.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_isamax(N: f77_int, X: *const f32, incX: f77_int) -> f77_int {
                dyload_lib().cblas_isamax.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_idamax(N: f77_int, X: *const f64, incX: f77_int) -> f77_int {
                dyload_lib().cblas_idamax.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_icamax(N: f77_int, X: *const c_void, incX: f77_int) -> f77_int {
                dyload_lib().cblas_icamax.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_izamax(N: f77_int, X: *const c_void, incX: f77_int) -> f77_int {
                dyload_lib().cblas_izamax.unwrap()(N, X, incX)
            }

pub unsafe fn cblas_sswap(N: f77_int, X: *mut f32, incX: f77_int, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_sswap.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_scopy(N: f77_int, X: *const f32, incX: f77_int, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_scopy.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_saxpy(N: f77_int, alpha: f32, X: *const f32, incX: f77_int, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_saxpy.unwrap()(N, alpha, X, incX, Y, incY)
            }

pub unsafe fn cblas_dswap(N: f77_int, X: *mut f64, incX: f77_int, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dswap.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_dcopy(N: f77_int, X: *const f64, incX: f77_int, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dcopy.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_daxpy(N: f77_int, alpha: f64, X: *const f64, incX: f77_int, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_daxpy.unwrap()(N, alpha, X, incX, Y, incY)
            }

pub unsafe fn cblas_cswap(N: f77_int, X: *mut c_void, incX: f77_int, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_cswap.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_ccopy(N: f77_int, X: *const c_void, incX: f77_int, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_ccopy.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_caxpy(N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_caxpy.unwrap()(N, alpha, X, incX, Y, incY)
            }

pub unsafe fn cblas_zswap(N: f77_int, X: *mut c_void, incX: f77_int, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zswap.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_zcopy(N: f77_int, X: *const c_void, incX: f77_int, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zcopy.unwrap()(N, X, incX, Y, incY)
            }

pub unsafe fn cblas_zaxpy(N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zaxpy.unwrap()(N, alpha, X, incX, Y, incY)
            }

pub unsafe fn cblas_srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) {
                dyload_lib().cblas_srotg.unwrap()(a, b, c, s)
            }

pub unsafe fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32) {
                dyload_lib().cblas_srotmg.unwrap()(d1, d2, b1, b2, P)
            }

pub unsafe fn cblas_srot(N: f77_int, X: *mut f32, incX: f77_int, Y: *mut f32, incY: f77_int, c: f32, s: f32) {
                dyload_lib().cblas_srot.unwrap()(N, X, incX, Y, incY, c, s)
            }

pub unsafe fn cblas_srotm(N: f77_int, X: *mut f32, incX: f77_int, Y: *mut f32, incY: f77_int, P: *const f32) {
                dyload_lib().cblas_srotm.unwrap()(N, X, incX, Y, incY, P)
            }

pub unsafe fn cblas_drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) {
                dyload_lib().cblas_drotg.unwrap()(a, b, c, s)
            }

pub unsafe fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64) {
                dyload_lib().cblas_drotmg.unwrap()(d1, d2, b1, b2, P)
            }

pub unsafe fn cblas_drot(N: f77_int, X: *mut f64, incX: f77_int, Y: *mut f64, incY: f77_int, c: f64, s: f64) {
                dyload_lib().cblas_drot.unwrap()(N, X, incX, Y, incY, c, s)
            }

pub unsafe fn cblas_drotm(N: f77_int, X: *mut f64, incX: f77_int, Y: *mut f64, incY: f77_int, P: *const f64) {
                dyload_lib().cblas_drotm.unwrap()(N, X, incX, Y, incY, P)
            }

pub unsafe fn cblas_sscal(N: f77_int, alpha: f32, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_sscal.unwrap()(N, alpha, X, incX)
            }

pub unsafe fn cblas_dscal(N: f77_int, alpha: f64, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dscal.unwrap()(N, alpha, X, incX)
            }

pub unsafe fn cblas_cscal(N: f77_int, alpha: *const c_void, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_cscal.unwrap()(N, alpha, X, incX)
            }

pub unsafe fn cblas_zscal(N: f77_int, alpha: *const c_void, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_zscal.unwrap()(N, alpha, X, incX)
            }

pub unsafe fn cblas_csscal(N: f77_int, alpha: f32, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_csscal.unwrap()(N, alpha, X, incX)
            }

pub unsafe fn cblas_zdscal(N: f77_int, alpha: f64, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_zdscal.unwrap()(N, alpha, X, incX)
            }

pub unsafe fn cblas_sgemv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, alpha: f32, A: *const f32, lda: f77_int, X: *const f32, incX: f77_int, beta: f32, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_sgemv.unwrap()(order, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_sgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, KL: f77_int, KU: f77_int, alpha: f32, A: *const f32, lda: f77_int, X: *const f32, incX: f77_int, beta: f32, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_sgbmv.unwrap()(order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_strmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const f32, lda: f77_int, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_strmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_stbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const f32, lda: f77_int, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_stbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_stpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const f32, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_stpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_strsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const f32, lda: f77_int, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_strsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_stbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const f32, lda: f77_int, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_stbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_stpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const f32, X: *mut f32, incX: f77_int) {
                dyload_lib().cblas_stpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_dgemv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, alpha: f64, A: *const f64, lda: f77_int, X: *const f64, incX: f77_int, beta: f64, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dgemv.unwrap()(order, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_dgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, KL: f77_int, KU: f77_int, alpha: f64, A: *const f64, lda: f77_int, X: *const f64, incX: f77_int, beta: f64, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dgbmv.unwrap()(order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_dtrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const f64, lda: f77_int, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dtrmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_dtbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const f64, lda: f77_int, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dtbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_dtpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const f64, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dtpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_dtrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const f64, lda: f77_int, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dtrsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_dtbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const f64, lda: f77_int, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dtbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_dtpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const f64, X: *mut f64, incX: f77_int) {
                dyload_lib().cblas_dtpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_cgemv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_cgemv.unwrap()(order, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_cgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, KL: f77_int, KU: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_cgbmv.unwrap()(order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_ctrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ctrmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_ctbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ctbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_ctpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const c_void, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ctpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_ctrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ctrsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_ctbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ctbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_ctpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const c_void, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ctpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_zgemv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zgemv.unwrap()(order, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_zgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, KL: f77_int, KU: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zgbmv.unwrap()(order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_ztrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ztrmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_ztbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ztbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_ztpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const c_void, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ztpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_ztrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ztrsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
            }

pub unsafe fn cblas_ztbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, K: f77_int, A: *const c_void, lda: f77_int, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ztbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
            }

pub unsafe fn cblas_ztpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: f77_int, Ap: *const c_void, X: *mut c_void, incX: f77_int) {
                dyload_lib().cblas_ztpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
            }

pub unsafe fn cblas_ssymv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, A: *const f32, lda: f77_int, X: *const f32, incX: f77_int, beta: f32, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_ssymv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_ssbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, K: f77_int, alpha: f32, A: *const f32, lda: f77_int, X: *const f32, incX: f77_int, beta: f32, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_ssbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_sspmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, Ap: *const f32, X: *const f32, incX: f77_int, beta: f32, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_sspmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_sger(order: CBLAS_ORDER, M: f77_int, N: f77_int, alpha: f32, X: *const f32, incX: f77_int, Y: *const f32, incY: f77_int, A: *mut f32, lda: f77_int) {
                dyload_lib().cblas_sger.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_ssyr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, X: *const f32, incX: f77_int, A: *mut f32, lda: f77_int) {
                dyload_lib().cblas_ssyr.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
            }

pub unsafe fn cblas_sspr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, X: *const f32, incX: f77_int, Ap: *mut f32) {
                dyload_lib().cblas_sspr.unwrap()(order, Uplo, N, alpha, X, incX, Ap)
            }

pub unsafe fn cblas_ssyr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, X: *const f32, incX: f77_int, Y: *const f32, incY: f77_int, A: *mut f32, lda: f77_int) {
                dyload_lib().cblas_ssyr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_sspr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, X: *const f32, incX: f77_int, Y: *const f32, incY: f77_int, A: *mut f32) {
                dyload_lib().cblas_sspr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A)
            }

pub unsafe fn cblas_dsymv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, A: *const f64, lda: f77_int, X: *const f64, incX: f77_int, beta: f64, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dsymv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_dsbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, K: f77_int, alpha: f64, A: *const f64, lda: f77_int, X: *const f64, incX: f77_int, beta: f64, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dsbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_dspmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, Ap: *const f64, X: *const f64, incX: f77_int, beta: f64, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_dspmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_dger(order: CBLAS_ORDER, M: f77_int, N: f77_int, alpha: f64, X: *const f64, incX: f77_int, Y: *const f64, incY: f77_int, A: *mut f64, lda: f77_int) {
                dyload_lib().cblas_dger.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_dsyr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, X: *const f64, incX: f77_int, A: *mut f64, lda: f77_int) {
                dyload_lib().cblas_dsyr.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
            }

pub unsafe fn cblas_dspr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, X: *const f64, incX: f77_int, Ap: *mut f64) {
                dyload_lib().cblas_dspr.unwrap()(order, Uplo, N, alpha, X, incX, Ap)
            }

pub unsafe fn cblas_dsyr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, X: *const f64, incX: f77_int, Y: *const f64, incY: f77_int, A: *mut f64, lda: f77_int) {
                dyload_lib().cblas_dsyr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_dspr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, X: *const f64, incX: f77_int, Y: *const f64, incY: f77_int, A: *mut f64) {
                dyload_lib().cblas_dspr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A)
            }

pub unsafe fn cblas_chemv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_chemv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_chbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_chbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_chpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, Ap: *const c_void, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_chpmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_cgeru(order: CBLAS_ORDER, M: f77_int, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_cgeru.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_cgerc(order: CBLAS_ORDER, M: f77_int, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_cgerc.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_cher(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, X: *const c_void, incX: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_cher.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
            }

pub unsafe fn cblas_chpr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f32, X: *const c_void, incX: f77_int, A: *mut c_void) {
                dyload_lib().cblas_chpr.unwrap()(order, Uplo, N, alpha, X, incX, A)
            }

pub unsafe fn cblas_cher2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_cher2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_chpr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, Ap: *mut c_void) {
                dyload_lib().cblas_chpr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, Ap)
            }

pub unsafe fn cblas_zhemv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zhemv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_zhbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zhbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_zhpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, Ap: *const c_void, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zhpmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_zgeru(order: CBLAS_ORDER, M: f77_int, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_zgeru.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_zgerc(order: CBLAS_ORDER, M: f77_int, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_zgerc.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_zher(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, X: *const c_void, incX: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_zher.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
            }

pub unsafe fn cblas_zhpr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: f64, X: *const c_void, incX: f77_int, A: *mut c_void) {
                dyload_lib().cblas_zhpr.unwrap()(order, Uplo, N, alpha, X, incX, A)
            }

pub unsafe fn cblas_zher2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, A: *mut c_void, lda: f77_int) {
                dyload_lib().cblas_zher2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
            }

pub unsafe fn cblas_zhpr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, Y: *const c_void, incY: f77_int, Ap: *mut c_void) {
                dyload_lib().cblas_zhpr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, Ap)
            }

pub unsafe fn cblas_sgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, K: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *const f32, ldb: f77_int, beta: f32, C: *mut f32, ldc: f77_int) {
                dyload_lib().cblas_sgemm.unwrap()(Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_ssymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: f77_int, N: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *const f32, ldb: f77_int, beta: f32, C: *mut f32, ldc: f77_int) {
                dyload_lib().cblas_ssymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_ssyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f32, A: *const f32, lda: f77_int, beta: f32, C: *mut f32, ldc: f77_int) {
                dyload_lib().cblas_ssyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
            }

pub unsafe fn cblas_ssyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *const f32, ldb: f77_int, beta: f32, C: *mut f32, ldc: f77_int) {
                dyload_lib().cblas_ssyr2k.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_strmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *mut f32, ldb: f77_int) {
                dyload_lib().cblas_strmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_strsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *mut f32, ldb: f77_int) {
                dyload_lib().cblas_strsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_dgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, K: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *const f64, ldb: f77_int, beta: f64, C: *mut f64, ldc: f77_int) {
                dyload_lib().cblas_dgemm.unwrap()(Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_dsymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: f77_int, N: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *const f64, ldb: f77_int, beta: f64, C: *mut f64, ldc: f77_int) {
                dyload_lib().cblas_dsymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_dsyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f64, A: *const f64, lda: f77_int, beta: f64, C: *mut f64, ldc: f77_int) {
                dyload_lib().cblas_dsyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
            }

pub unsafe fn cblas_dsyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *const f64, ldb: f77_int, beta: f64, C: *mut f64, ldc: f77_int) {
                dyload_lib().cblas_dsyr2k.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_dtrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *mut f64, ldb: f77_int) {
                dyload_lib().cblas_dtrmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_dtrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *mut f64, ldb: f77_int) {
                dyload_lib().cblas_dtrsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_cgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_cgemm.unwrap()(Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_csymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_csymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_csyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_csyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
            }

pub unsafe fn cblas_csyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_csyr2k.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_ctrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *mut c_void, ldb: f77_int) {
                dyload_lib().cblas_ctrmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_ctrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *mut c_void, ldb: f77_int) {
                dyload_lib().cblas_ctrsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_zgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zgemm.unwrap()(Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zsymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zsymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zsyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zsyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
            }

pub unsafe fn cblas_zsyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zsyr2k.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_ztrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *mut c_void, ldb: f77_int) {
                dyload_lib().cblas_ztrmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_ztrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *mut c_void, ldb: f77_int) {
                dyload_lib().cblas_ztrsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
            }

pub unsafe fn cblas_chemm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_chemm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_cherk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f32, A: *const c_void, lda: f77_int, beta: f32, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_cherk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
            }

pub unsafe fn cblas_cher2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: f32, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_cher2k.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zhemm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: f77_int, N: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zhemm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zherk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f64, A: *const c_void, lda: f77_int, beta: f64, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zherk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
            }

pub unsafe fn cblas_zher2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: f64, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zher2k.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_xerbla(p: f77_int, rout: *const c_char, form: *const c_char) {
                dyload_lib().cblas_xerbla.unwrap()(p, rout, form)
            }

pub unsafe fn cblas_saxpby(N: f77_int, alpha: f32, X: *const f32, incX: f77_int, beta: f32, Y: *mut f32, incY: f77_int) {
                dyload_lib().cblas_saxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_daxpby(N: f77_int, alpha: f64, X: *const f64, incX: f77_int, beta: f64, Y: *mut f64, incY: f77_int) {
                dyload_lib().cblas_daxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_caxpby(N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_caxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_zaxpby(N: f77_int, alpha: *const c_void, X: *const c_void, incX: f77_int, beta: *const c_void, Y: *mut c_void, incY: f77_int) {
                dyload_lib().cblas_zaxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
            }

pub unsafe fn cblas_sgemmt(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *const f32, ldb: f77_int, beta: f32, C: *mut f32, ldc: f77_int) {
                dyload_lib().cblas_sgemmt.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_sgemmtr(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f32, A: *const f32, lda: f77_int, B: *const f32, ldb: f77_int, beta: f32, C: *mut f32, ldc: f77_int) {
                dyload_lib().cblas_sgemmtr.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_dgemmt(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *const f64, ldb: f77_int, beta: f64, C: *mut f64, ldc: f77_int) {
                dyload_lib().cblas_dgemmt.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_dgemmtr(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: f64, A: *const f64, lda: f77_int, B: *const f64, ldb: f77_int, beta: f64, C: *mut f64, ldc: f77_int) {
                dyload_lib().cblas_dgemmtr.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_cgemmt(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_cgemmt.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_cgemmtr(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_cgemmtr.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zgemmt(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zgemmt.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zgemmtr(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zgemmtr.unwrap()(Order, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_sgemm_batch(Order: CBLAS_ORDER, TransA_array: *mut CBLAS_TRANSPOSE, TransB_array: *mut CBLAS_TRANSPOSE, M_array: *mut f77_int, N_array: *mut f77_int, K_array: *mut f77_int, alpha_array: *const f32, A: *mut *const f32, lda_array: *mut f77_int, B: *mut *const f32, ldb_array: *mut f77_int, beta_array: *const f32, C: *mut *mut f32, ldc_array: *mut f77_int, group_count: f77_int, group_size: *mut f77_int) {
                dyload_lib().cblas_sgemm_batch.unwrap()(Order, TransA_array, TransB_array, M_array, N_array, K_array, alpha_array, A, lda_array, B, ldb_array, beta_array, C, ldc_array, group_count, group_size)
            }

pub unsafe fn cblas_dgemm_batch(Order: CBLAS_ORDER, TransA_array: *mut CBLAS_TRANSPOSE, TransB_array: *mut CBLAS_TRANSPOSE, M_array: *mut f77_int, N_array: *mut f77_int, K_array: *mut f77_int, alpha_array: *const f64, A: *mut *const f64, lda_array: *mut f77_int, B: *mut *const f64, ldb_array: *mut f77_int, beta_array: *const f64, C: *mut *mut f64, ldc_array: *mut f77_int, group_count: f77_int, group_size: *mut f77_int) {
                dyload_lib().cblas_dgemm_batch.unwrap()(Order, TransA_array, TransB_array, M_array, N_array, K_array, alpha_array, A, lda_array, B, ldb_array, beta_array, C, ldc_array, group_count, group_size)
            }

pub unsafe fn cblas_cgemm_batch(Order: CBLAS_ORDER, TransA_array: *mut CBLAS_TRANSPOSE, TransB_array: *mut CBLAS_TRANSPOSE, M_array: *mut f77_int, N_array: *mut f77_int, K_array: *mut f77_int, alpha_array: *const c_void, A: *mut *const c_void, lda_array: *mut f77_int, B: *mut *const c_void, ldb_array: *mut f77_int, beta_array: *const c_void, C: *mut *mut c_void, ldc_array: *mut f77_int, group_count: f77_int, group_size: *mut f77_int) {
                dyload_lib().cblas_cgemm_batch.unwrap()(Order, TransA_array, TransB_array, M_array, N_array, K_array, alpha_array, A, lda_array, B, ldb_array, beta_array, C, ldc_array, group_count, group_size)
            }

pub unsafe fn cblas_zgemm_batch(Order: CBLAS_ORDER, TransA_array: *mut CBLAS_TRANSPOSE, TransB_array: *mut CBLAS_TRANSPOSE, M_array: *mut f77_int, N_array: *mut f77_int, K_array: *mut f77_int, alpha_array: *const c_void, A: *mut *const c_void, lda_array: *mut f77_int, B: *mut *const c_void, ldb_array: *mut f77_int, beta_array: *const c_void, C: *mut *mut c_void, ldc_array: *mut f77_int, group_count: f77_int, group_size: *mut f77_int) {
                dyload_lib().cblas_zgemm_batch.unwrap()(Order, TransA_array, TransB_array, M_array, N_array, K_array, alpha_array, A, lda_array, B, ldb_array, beta_array, C, ldc_array, group_count, group_size)
            }

pub unsafe fn cblas_cgemm3m(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_cgemm3m.unwrap()(Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn cblas_zgemm3m(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: f77_int, N: f77_int, K: f77_int, alpha: *const c_void, A: *const c_void, lda: f77_int, B: *const c_void, ldb: f77_int, beta: *const c_void, C: *mut c_void, ldc: f77_int) {
                dyload_lib().cblas_zgemm3m.unwrap()(Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc)
            }

pub unsafe fn bli_sleep(secs: c_uint) {
                dyload_lib().bli_sleep.unwrap()(secs)
            }


    