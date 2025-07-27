//! Library struct definition for dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

pub struct DyLoadLib {
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub MKL_Get_Version: Option<unsafe extern "C" fn(ver: *mut MKLVersion)>,
    pub MKL_Get_Version_String: Option<unsafe extern "C" fn(buffer: *mut c_char, len: c_int)>,
    pub MKL_Free_Buffers: Option<unsafe extern "C" fn()>,
    pub MKL_Thread_Free_Buffers: Option<unsafe extern "C" fn()>,
    pub MKL_Mem_Stat: Option<unsafe extern "C" fn(nbuffers: *mut c_int) -> MKL_INT64>,
    pub MKL_Peak_Mem_Usage: Option<unsafe extern "C" fn(reset: c_int) -> MKL_INT64>,
    pub MKL_malloc: Option<unsafe extern "C" fn(size: usize, align: c_int) -> *mut c_void>,
    pub MKL_calloc:
        Option<unsafe extern "C" fn(num: usize, size: usize, align: c_int) -> *mut c_void>,
    pub MKL_realloc: Option<unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void>,
    pub MKL_free: Option<unsafe extern "C" fn(ptr: *mut c_void)>,
    pub MKL_Disable_Fast_MM: Option<unsafe extern "C" fn() -> c_int>,
    pub MKL_Get_Cpu_Clocks: Option<unsafe extern "C" fn(arg1: *mut MKL_UINT64)>,
    pub MKL_Get_Cpu_Frequency: Option<unsafe extern "C" fn() -> f64>,
    pub MKL_Get_Max_Cpu_Frequency: Option<unsafe extern "C" fn() -> f64>,
    pub MKL_Get_Clocks_Frequency: Option<unsafe extern "C" fn() -> f64>,
    pub MKL_Set_Num_Threads_Local: Option<unsafe extern "C" fn(nth: c_int) -> c_int>,
    pub MKL_Set_Num_Threads: Option<unsafe extern "C" fn(nth: c_int)>,
    pub MKL_Get_Max_Threads: Option<unsafe extern "C" fn() -> c_int>,
    pub MKL_Set_Num_Stripes: Option<unsafe extern "C" fn(nstripes: c_int)>,
    pub MKL_Get_Num_Stripes: Option<unsafe extern "C" fn() -> c_int>,
    pub MKL_Domain_Set_Num_Threads:
        Option<unsafe extern "C" fn(nth: c_int, MKL_DOMAIN: c_int) -> c_int>,
    pub MKL_Domain_Get_Max_Threads: Option<unsafe extern "C" fn(MKL_DOMAIN: c_int) -> c_int>,
    pub MKL_Set_Dynamic: Option<unsafe extern "C" fn(bool_MKL_DYNAMIC: c_int)>,
    pub MKL_Get_Dynamic: Option<unsafe extern "C" fn() -> c_int>,
    pub MKL_PROGRESS: Option<
        unsafe extern "C" fn(
            thread: *mut c_int,
            step: *mut c_int,
            stage: *mut c_char,
            lstage: c_int,
        ) -> c_int,
    >,
    pub MKL_PROGRESS_: Option<
        unsafe extern "C" fn(
            thread: *mut c_int,
            step: *mut c_int,
            stage: *mut c_char,
            lstage: c_int,
        ) -> c_int,
    >,
    pub mkl_progress: Option<
        unsafe extern "C" fn(
            thread: *mut c_int,
            step: *mut c_int,
            stage: *mut c_char,
            lstage: c_int,
        ) -> c_int,
    >,
    pub mkl_progress_: Option<
        unsafe extern "C" fn(
            thread: *mut c_int,
            step: *mut c_int,
            stage: *mut c_char,
            lstage: c_int,
        ) -> c_int,
    >,
    pub MKL_Enable_Instructions: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub MKL_Set_Interface_Layer: Option<unsafe extern "C" fn(code: c_int) -> c_int>,
    pub MKL_Set_Threading_Layer: Option<unsafe extern "C" fn(code: c_int) -> c_int>,
    pub mkl_set_xerbla: Option<unsafe extern "C" fn(xerbla: XerblaEntry) -> XerblaEntry>,
    pub mkl_set_progress: Option<unsafe extern "C" fn(progress: ProgressEntry) -> ProgressEntry>,
    pub mkl_set_pardiso_pivot:
        Option<unsafe extern "C" fn(pardiso_pivot: PardisopivotEntry) -> PardisopivotEntry>,
    pub MKL_CBWR_Get: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub MKL_CBWR_Set: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub MKL_CBWR_Get_Auto_Branch: Option<unsafe extern "C" fn() -> c_int>,
    pub MKL_Set_Env_Mode: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub MKL_Verbose: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub MKL_Verbose_Output_File: Option<unsafe extern "C" fn(fname: *const c_char) -> c_int>,
    pub MKL_Set_Exit_Handler: Option<unsafe extern "C" fn(h: MKLExitHandler)>,
    pub MKL_Set_mpi:
        Option<unsafe extern "C" fn(vendor: c_int, custom_library_name: *const c_char) -> c_int>,
    pub MKL_Set_Memory_Limit: Option<unsafe extern "C" fn(mem_type: c_int, limit: usize) -> c_int>,
    pub MKL_Finalize: Option<unsafe extern "C" fn()>,
}
