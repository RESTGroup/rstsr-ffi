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
            MKL_Get_Version: get_symbol(&libs, b"MKL_Get_Version\0").map(|sym| *sym),
            MKL_Get_Version_String: get_symbol(&libs, b"MKL_Get_Version_String\0").map(|sym| *sym),
            MKL_Free_Buffers: get_symbol(&libs, b"MKL_Free_Buffers\0").map(|sym| *sym),
            MKL_Thread_Free_Buffers: get_symbol(&libs, b"MKL_Thread_Free_Buffers\0")
                .map(|sym| *sym),
            MKL_Mem_Stat: get_symbol(&libs, b"MKL_Mem_Stat\0").map(|sym| *sym),
            MKL_Peak_Mem_Usage: get_symbol(&libs, b"MKL_Peak_Mem_Usage\0").map(|sym| *sym),
            MKL_malloc: get_symbol(&libs, b"MKL_malloc\0").map(|sym| *sym),
            MKL_calloc: get_symbol(&libs, b"MKL_calloc\0").map(|sym| *sym),
            MKL_realloc: get_symbol(&libs, b"MKL_realloc\0").map(|sym| *sym),
            MKL_free: get_symbol(&libs, b"MKL_free\0").map(|sym| *sym),
            MKL_Disable_Fast_MM: get_symbol(&libs, b"MKL_Disable_Fast_MM\0").map(|sym| *sym),
            MKL_Get_Cpu_Clocks: get_symbol(&libs, b"MKL_Get_Cpu_Clocks\0").map(|sym| *sym),
            MKL_Get_Cpu_Frequency: get_symbol(&libs, b"MKL_Get_Cpu_Frequency\0").map(|sym| *sym),
            MKL_Get_Max_Cpu_Frequency: get_symbol(&libs, b"MKL_Get_Max_Cpu_Frequency\0")
                .map(|sym| *sym),
            MKL_Get_Clocks_Frequency: get_symbol(&libs, b"MKL_Get_Clocks_Frequency\0")
                .map(|sym| *sym),
            MKL_Set_Num_Threads_Local: get_symbol(&libs, b"MKL_Set_Num_Threads_Local\0")
                .map(|sym| *sym),
            MKL_Set_Num_Threads: get_symbol(&libs, b"MKL_Set_Num_Threads\0").map(|sym| *sym),
            MKL_Get_Max_Threads: get_symbol(&libs, b"MKL_Get_Max_Threads\0").map(|sym| *sym),
            MKL_Set_Num_Stripes: get_symbol(&libs, b"MKL_Set_Num_Stripes\0").map(|sym| *sym),
            MKL_Get_Num_Stripes: get_symbol(&libs, b"MKL_Get_Num_Stripes\0").map(|sym| *sym),
            MKL_Domain_Set_Num_Threads: get_symbol(&libs, b"MKL_Domain_Set_Num_Threads\0")
                .map(|sym| *sym),
            MKL_Domain_Get_Max_Threads: get_symbol(&libs, b"MKL_Domain_Get_Max_Threads\0")
                .map(|sym| *sym),
            MKL_Set_Dynamic: get_symbol(&libs, b"MKL_Set_Dynamic\0").map(|sym| *sym),
            MKL_Get_Dynamic: get_symbol(&libs, b"MKL_Get_Dynamic\0").map(|sym| *sym),
            MKL_PROGRESS: get_symbol(&libs, b"MKL_PROGRESS\0").map(|sym| *sym),
            MKL_PROGRESS_: get_symbol(&libs, b"MKL_PROGRESS_\0").map(|sym| *sym),
            mkl_progress: get_symbol(&libs, b"mkl_progress\0").map(|sym| *sym),
            mkl_progress_: get_symbol(&libs, b"mkl_progress_\0").map(|sym| *sym),
            MKL_Enable_Instructions: get_symbol(&libs, b"MKL_Enable_Instructions\0")
                .map(|sym| *sym),
            MKL_Set_Interface_Layer: get_symbol(&libs, b"MKL_Set_Interface_Layer\0")
                .map(|sym| *sym),
            MKL_Set_Threading_Layer: get_symbol(&libs, b"MKL_Set_Threading_Layer\0")
                .map(|sym| *sym),
            mkl_set_xerbla: get_symbol(&libs, b"mkl_set_xerbla\0").map(|sym| *sym),
            mkl_set_progress: get_symbol(&libs, b"mkl_set_progress\0").map(|sym| *sym),
            mkl_set_pardiso_pivot: get_symbol(&libs, b"mkl_set_pardiso_pivot\0").map(|sym| *sym),
            MKL_CBWR_Get: get_symbol(&libs, b"MKL_CBWR_Get\0").map(|sym| *sym),
            MKL_CBWR_Set: get_symbol(&libs, b"MKL_CBWR_Set\0").map(|sym| *sym),
            MKL_CBWR_Get_Auto_Branch: get_symbol(&libs, b"MKL_CBWR_Get_Auto_Branch\0")
                .map(|sym| *sym),
            MKL_Set_Env_Mode: get_symbol(&libs, b"MKL_Set_Env_Mode\0").map(|sym| *sym),
            MKL_Verbose: get_symbol(&libs, b"MKL_Verbose\0").map(|sym| *sym),
            MKL_Verbose_Output_File: get_symbol(&libs, b"MKL_Verbose_Output_File\0")
                .map(|sym| *sym),
            MKL_Set_Exit_Handler: get_symbol(&libs, b"MKL_Set_Exit_Handler\0").map(|sym| *sym),
            MKL_Set_mpi: get_symbol(&libs, b"MKL_Set_mpi\0").map(|sym| *sym),
            MKL_Set_Memory_Limit: get_symbol(&libs, b"MKL_Set_Memory_Limit\0").map(|sym| *sym),
            MKL_Finalize: get_symbol(&libs, b"MKL_Finalize\0").map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
