//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn MKL_Get_Version(ver: *mut MKLVersion);
    pub fn MKL_Get_Version_String(buffer: *mut c_char, len: c_int);
    pub fn MKL_Free_Buffers();
    pub fn MKL_Thread_Free_Buffers();
    pub fn MKL_Mem_Stat(nbuffers: *mut c_int) -> MKL_INT64;
    pub fn MKL_Peak_Mem_Usage(reset: c_int) -> MKL_INT64;
    pub fn MKL_malloc(size: usize, align: c_int) -> *mut c_void;
    pub fn MKL_calloc(num: usize, size: usize, align: c_int) -> *mut c_void;
    pub fn MKL_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn MKL_free(ptr: *mut c_void);
    pub fn MKL_Disable_Fast_MM() -> c_int;
    pub fn MKL_Get_Cpu_Clocks(arg1: *mut MKL_UINT64);
    pub fn MKL_Get_Cpu_Frequency() -> f64;
    pub fn MKL_Get_Max_Cpu_Frequency() -> f64;
    pub fn MKL_Get_Clocks_Frequency() -> f64;
    pub fn MKL_Set_Num_Threads_Local(nth: c_int) -> c_int;
    pub fn MKL_Set_Num_Threads(nth: c_int);
    pub fn MKL_Get_Max_Threads() -> c_int;
    pub fn MKL_Set_Num_Stripes(nstripes: c_int);
    pub fn MKL_Get_Num_Stripes() -> c_int;
    pub fn MKL_Domain_Set_Num_Threads(nth: c_int, MKL_DOMAIN: c_int) -> c_int;
    pub fn MKL_Domain_Get_Max_Threads(MKL_DOMAIN: c_int) -> c_int;
    pub fn MKL_Set_Dynamic(bool_MKL_DYNAMIC: c_int);
    pub fn MKL_Get_Dynamic() -> c_int;
    pub fn MKL_PROGRESS(
        thread: *mut c_int,
        step: *mut c_int,
        stage: *mut c_char,
        lstage: c_int,
    ) -> c_int;
    pub fn MKL_PROGRESS_(
        thread: *mut c_int,
        step: *mut c_int,
        stage: *mut c_char,
        lstage: c_int,
    ) -> c_int;
    pub fn mkl_progress(
        thread: *mut c_int,
        step: *mut c_int,
        stage: *mut c_char,
        lstage: c_int,
    ) -> c_int;
    pub fn mkl_progress_(
        thread: *mut c_int,
        step: *mut c_int,
        stage: *mut c_char,
        lstage: c_int,
    ) -> c_int;
    pub fn MKL_Enable_Instructions(arg1: c_int) -> c_int;
    pub fn MKL_Set_Interface_Layer(code: c_int) -> c_int;
    pub fn MKL_Set_Threading_Layer(code: c_int) -> c_int;
    pub fn mkl_set_xerbla(xerbla: XerblaEntry) -> XerblaEntry;
    pub fn mkl_set_progress(progress: ProgressEntry) -> ProgressEntry;
    pub fn mkl_set_pardiso_pivot(pardiso_pivot: PardisopivotEntry) -> PardisopivotEntry;
    pub fn MKL_CBWR_Get(arg1: c_int) -> c_int;
    pub fn MKL_CBWR_Set(arg1: c_int) -> c_int;
    pub fn MKL_CBWR_Get_Auto_Branch() -> c_int;
    pub fn MKL_Set_Env_Mode(arg1: c_int) -> c_int;
    pub fn MKL_Verbose(arg1: c_int) -> c_int;
    pub fn MKL_Verbose_Output_File(fname: *const c_char) -> c_int;
    pub fn MKL_Set_Exit_Handler(h: MKLExitHandler);
    pub fn MKL_Set_mpi(vendor: c_int, custom_library_name: *const c_char) -> c_int;
    pub fn MKL_Set_Memory_Limit(mem_type: c_int, limit: usize) -> c_int;
    pub fn MKL_Finalize();
}
