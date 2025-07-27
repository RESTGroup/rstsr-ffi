//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn MKL_Get_Version(ver: *mut MKLVersion) {
    dyload_lib().MKL_Get_Version.unwrap()(ver)
}

pub unsafe fn MKL_Get_Version_String(buffer: *mut c_char, len: c_int) {
    dyload_lib().MKL_Get_Version_String.unwrap()(buffer, len)
}

pub unsafe fn MKL_Free_Buffers() {
    dyload_lib().MKL_Free_Buffers.unwrap()()
}

pub unsafe fn MKL_Thread_Free_Buffers() {
    dyload_lib().MKL_Thread_Free_Buffers.unwrap()()
}

pub unsafe fn MKL_Mem_Stat(nbuffers: *mut c_int) -> MKL_INT64 {
    dyload_lib().MKL_Mem_Stat.unwrap()(nbuffers)
}

pub unsafe fn MKL_Peak_Mem_Usage(reset: c_int) -> MKL_INT64 {
    dyload_lib().MKL_Peak_Mem_Usage.unwrap()(reset)
}

pub unsafe fn MKL_malloc(size: usize, align: c_int) -> *mut c_void {
    dyload_lib().MKL_malloc.unwrap()(size, align)
}

pub unsafe fn MKL_calloc(num: usize, size: usize, align: c_int) -> *mut c_void {
    dyload_lib().MKL_calloc.unwrap()(num, size, align)
}

pub unsafe fn MKL_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    dyload_lib().MKL_realloc.unwrap()(ptr, size)
}

pub unsafe fn MKL_free(ptr: *mut c_void) {
    dyload_lib().MKL_free.unwrap()(ptr)
}

pub unsafe fn MKL_Disable_Fast_MM() -> c_int {
    dyload_lib().MKL_Disable_Fast_MM.unwrap()()
}

pub unsafe fn MKL_Get_Cpu_Clocks(arg1: *mut MKL_UINT64) {
    dyload_lib().MKL_Get_Cpu_Clocks.unwrap()(arg1)
}

pub unsafe fn MKL_Get_Cpu_Frequency() -> f64 {
    dyload_lib().MKL_Get_Cpu_Frequency.unwrap()()
}

pub unsafe fn MKL_Get_Max_Cpu_Frequency() -> f64 {
    dyload_lib().MKL_Get_Max_Cpu_Frequency.unwrap()()
}

pub unsafe fn MKL_Get_Clocks_Frequency() -> f64 {
    dyload_lib().MKL_Get_Clocks_Frequency.unwrap()()
}

pub unsafe fn MKL_Set_Num_Threads_Local(nth: c_int) -> c_int {
    dyload_lib().MKL_Set_Num_Threads_Local.unwrap()(nth)
}

pub unsafe fn MKL_Set_Num_Threads(nth: c_int) {
    dyload_lib().MKL_Set_Num_Threads.unwrap()(nth)
}

pub unsafe fn MKL_Get_Max_Threads() -> c_int {
    dyload_lib().MKL_Get_Max_Threads.unwrap()()
}

pub unsafe fn MKL_Set_Num_Stripes(nstripes: c_int) {
    dyload_lib().MKL_Set_Num_Stripes.unwrap()(nstripes)
}

pub unsafe fn MKL_Get_Num_Stripes() -> c_int {
    dyload_lib().MKL_Get_Num_Stripes.unwrap()()
}

pub unsafe fn MKL_Domain_Set_Num_Threads(nth: c_int, MKL_DOMAIN: c_int) -> c_int {
    dyload_lib().MKL_Domain_Set_Num_Threads.unwrap()(nth, MKL_DOMAIN)
}

pub unsafe fn MKL_Domain_Get_Max_Threads(MKL_DOMAIN: c_int) -> c_int {
    dyload_lib().MKL_Domain_Get_Max_Threads.unwrap()(MKL_DOMAIN)
}

pub unsafe fn MKL_Set_Dynamic(bool_MKL_DYNAMIC: c_int) {
    dyload_lib().MKL_Set_Dynamic.unwrap()(bool_MKL_DYNAMIC)
}

pub unsafe fn MKL_Get_Dynamic() -> c_int {
    dyload_lib().MKL_Get_Dynamic.unwrap()()
}

pub unsafe fn MKL_PROGRESS(
    thread: *mut c_int,
    step: *mut c_int,
    stage: *mut c_char,
    lstage: c_int,
) -> c_int {
    dyload_lib().MKL_PROGRESS.unwrap()(thread, step, stage, lstage)
}

pub unsafe fn MKL_PROGRESS_(
    thread: *mut c_int,
    step: *mut c_int,
    stage: *mut c_char,
    lstage: c_int,
) -> c_int {
    dyload_lib().MKL_PROGRESS_.unwrap()(thread, step, stage, lstage)
}

pub unsafe fn mkl_progress(
    thread: *mut c_int,
    step: *mut c_int,
    stage: *mut c_char,
    lstage: c_int,
) -> c_int {
    dyload_lib().mkl_progress.unwrap()(thread, step, stage, lstage)
}

pub unsafe fn mkl_progress_(
    thread: *mut c_int,
    step: *mut c_int,
    stage: *mut c_char,
    lstage: c_int,
) -> c_int {
    dyload_lib().mkl_progress_.unwrap()(thread, step, stage, lstage)
}

pub unsafe fn MKL_Enable_Instructions(arg1: c_int) -> c_int {
    dyload_lib().MKL_Enable_Instructions.unwrap()(arg1)
}

pub unsafe fn MKL_Set_Interface_Layer(code: c_int) -> c_int {
    dyload_lib().MKL_Set_Interface_Layer.unwrap()(code)
}

pub unsafe fn MKL_Set_Threading_Layer(code: c_int) -> c_int {
    dyload_lib().MKL_Set_Threading_Layer.unwrap()(code)
}

pub unsafe fn mkl_set_xerbla(xerbla: XerblaEntry) -> XerblaEntry {
    dyload_lib().mkl_set_xerbla.unwrap()(xerbla)
}

pub unsafe fn mkl_set_progress(progress: ProgressEntry) -> ProgressEntry {
    dyload_lib().mkl_set_progress.unwrap()(progress)
}

pub unsafe fn mkl_set_pardiso_pivot(pardiso_pivot: PardisopivotEntry) -> PardisopivotEntry {
    dyload_lib().mkl_set_pardiso_pivot.unwrap()(pardiso_pivot)
}

pub unsafe fn MKL_CBWR_Get(arg1: c_int) -> c_int {
    dyload_lib().MKL_CBWR_Get.unwrap()(arg1)
}

pub unsafe fn MKL_CBWR_Set(arg1: c_int) -> c_int {
    dyload_lib().MKL_CBWR_Set.unwrap()(arg1)
}

pub unsafe fn MKL_CBWR_Get_Auto_Branch() -> c_int {
    dyload_lib().MKL_CBWR_Get_Auto_Branch.unwrap()()
}

pub unsafe fn MKL_Set_Env_Mode(arg1: c_int) -> c_int {
    dyload_lib().MKL_Set_Env_Mode.unwrap()(arg1)
}

pub unsafe fn MKL_Verbose(arg1: c_int) -> c_int {
    dyload_lib().MKL_Verbose.unwrap()(arg1)
}

pub unsafe fn MKL_Verbose_Output_File(fname: *const c_char) -> c_int {
    dyload_lib().MKL_Verbose_Output_File.unwrap()(fname)
}

pub unsafe fn MKL_Set_Exit_Handler(h: MKLExitHandler) {
    dyload_lib().MKL_Set_Exit_Handler.unwrap()(h)
}

pub unsafe fn MKL_Set_mpi(vendor: c_int, custom_library_name: *const c_char) -> c_int {
    dyload_lib().MKL_Set_mpi.unwrap()(vendor, custom_library_name)
}

pub unsafe fn MKL_Set_Memory_Limit(mem_type: c_int, limit: usize) -> c_int {
    dyload_lib().MKL_Set_Memory_Limit.unwrap()(mem_type, limit)
}

pub unsafe fn MKL_Finalize() {
    dyload_lib().MKL_Finalize.unwrap()()
}
