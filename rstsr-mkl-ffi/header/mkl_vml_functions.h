/* file: mkl_vml_functions.h */
/*******************************************************************************
* Copyright (C) 2006 Intel Corporation
*
* This software and the related documents are Intel copyrighted  materials,  and
* your use of  them is  governed by the  express license  under which  they were
* provided to you (License).  Unless the License provides otherwise, you may not
* use, modify, copy, publish, distribute,  disclose or transmit this software or
* the related documents without Intel's prior written permission.
*
* This software and the related documents  are provided as  is,  with no express
* or implied  warranties,  other  than those  that are  expressly stated  in the
* License.
*******************************************************************************/

/*
//++
//  User-level VML function declarations
//--
*/

#ifndef __MKL_VML_FUNCTIONS_H__
#define __MKL_VML_FUNCTIONS_H__

#include "mkl_vml_types.h"

#ifdef __cplusplus
#if __cplusplus > 199711L
#define NOTHROW noexcept
#else
#define NOTHROW throw()
#endif
#else
#define NOTHROW
#endif


#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/*
//++
//  EXTERNAL API MACROS.
//  Used to construct VML function declaration. Change them if you are going to
//  provide different API for VML functions.
//--
*/

#if defined( _WIN32 ) || defined( _WIN64 )
  #if  !defined(_Mkl_Vml_Api)
  #define _Mkl_Vml_Api(rtype,name,arg)   extern rtype name  arg
  #endif

  #if  !defined(_mkl_vml_api)
  #define _mkl_vml_api(rtype,name,arg)   extern rtype name  arg
  #endif

  #if  !defined(_MKL_VML_API)
  #define _MKL_VML_API(rtype,name,arg)   extern rtype name  arg
  #endif
#else
  #if  !defined(_Mkl_Vml_Api)
  #define _Mkl_Vml_Api(rtype,name,arg)   extern rtype name    arg NOTHROW
  #endif

  #if  !defined(_mkl_vml_api)
  #define _mkl_vml_api(rtype,name,arg)   extern rtype name##_ arg NOTHROW
  #endif

  #if  !defined(_MKL_VML_API)
  #define _MKL_VML_API(rtype,name,arg)   extern rtype name##_ arg NOTHROW
  #endif
#endif


/*
//++
// VML ELEMENTARY FUNCTION DECLARATIONS.
//--
*/
/* Absolute value: r[i] = |a[i]| */
_MKL_VML_API(void,VSABS,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDABS,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsabs,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdabs,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAbs,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAbs,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSABS,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDABS,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsabs,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdabs,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAbs,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAbs,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex absolute value: r[i] = |a[i]| */
_MKL_VML_API(void,VCABS,(const MKL_INT *n, const MKL_Complex8 a[], float r[]));
_MKL_VML_API(void,VZABS,(const MKL_INT *n, const MKL_Complex16 a[], double r[]));
_mkl_vml_api(void,vcabs,(const MKL_INT *n, const MKL_Complex8 a[], float r[]));
_mkl_vml_api(void,vzabs,(const MKL_INT *n, const MKL_Complex16 a[], double r[]));
_Mkl_Vml_Api(void,vcAbs,(const MKL_INT n, const MKL_Complex8 a[], float r[]));
_Mkl_Vml_Api(void,vzAbs,(const MKL_INT n, const MKL_Complex16 a[], double r[]));

_MKL_VML_API(void,VMCABS,(const MKL_INT *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZABS,(const MKL_INT *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcabs,(const MKL_INT *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzabs,(const MKL_INT *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAbs,(const MKL_INT n, const MKL_Complex8 a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAbs,(const MKL_INT n, const MKL_Complex16 a[], double r[], MKL_INT64 mode));

/* Argument of complex value: r[i] = carg(a[i]) */
_MKL_VML_API(void,VCARG,(const MKL_INT *n, const MKL_Complex8 a[], float r[]));
_MKL_VML_API(void,VZARG,(const MKL_INT *n, const MKL_Complex16 a[], double r[]));
_mkl_vml_api(void,vcarg,(const MKL_INT *n, const MKL_Complex8 a[], float r[]));
_mkl_vml_api(void,vzarg,(const MKL_INT *n, const MKL_Complex16 a[], double r[]));
_Mkl_Vml_Api(void,vcArg,(const MKL_INT n, const MKL_Complex8 a[], float r[]));
_Mkl_Vml_Api(void,vzArg,(const MKL_INT n, const MKL_Complex16 a[], double r[]));

_MKL_VML_API(void,VMCARG,(const MKL_INT *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZARG,(const MKL_INT *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcarg,(const MKL_INT *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzarg,(const MKL_INT *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcArg,(const MKL_INT n, const MKL_Complex8 a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzArg,(const MKL_INT n, const MKL_Complex16 a[], double r[], MKL_INT64 mode));

/* Addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void,VSADD,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDADD,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsadd,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdadd,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsAdd,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdAdd,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSADD,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDADD,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsadd,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdadd,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAdd,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAdd,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void,VCADD,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZADD,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcadd,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzadd,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAdd,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAdd,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCADD,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZADD,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcadd,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzadd,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAdd,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAdd,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void,VSSUB,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDSUB,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vssub,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdsub,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsSub,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdSub,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSSUB,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSUB,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssub,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsub,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSub,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSub,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void,VCSUB,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSUB,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsub,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsub,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSub,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSub,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSUB,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSUB,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsub,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsub,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSub,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSub,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Reciprocal: r[i] = 1.0 / a[i] */
_MKL_VML_API(void,VSINV,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDINV,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsinv,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdinv,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsInv,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdInv,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSINV,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDINV,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsinv,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdinv,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsInv,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdInv,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void,VSSQRT,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDSQRT,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vssqrt,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdsqrt,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSqrt,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSqrt,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSSQRT,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSQRT,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssqrt,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsqrt,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSqrt,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSqrt,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void,VCSQRT,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSQRT,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsqrt,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsqrt,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSqrt,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSqrt,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSQRT,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSQRT,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsqrt,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsqrt,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSqrt,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSqrt,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Reciprocal square root: r[i] = 1/a[i]^0.5 */
_MKL_VML_API(void,VSINVSQRT,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDINVSQRT,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsinvsqrt,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdinvsqrt,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsInvSqrt,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdInvSqrt,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSINVSQRT,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDINVSQRT,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsinvsqrt,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdinvsqrt,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsInvSqrt,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdInvSqrt,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Cube root: r[i] = a[i]^(1/3) */
_MKL_VML_API(void,VSCBRT,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDCBRT,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vscbrt,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdcbrt,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCbrt,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCbrt,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSCBRT,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCBRT,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscbrt,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcbrt,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCbrt,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCbrt,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Reciprocal cube root: r[i] = 1/a[i]^(1/3) */
_MKL_VML_API(void,VSINVCBRT,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDINVCBRT,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsinvcbrt,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdinvcbrt,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsInvCbrt,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdInvCbrt,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSINVCBRT,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDINVCBRT,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsinvcbrt,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdinvcbrt,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsInvCbrt,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdInvCbrt,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Squaring: r[i] = a[i]^2 */
_MKL_VML_API(void,VSSQR,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDSQR,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vssqr,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdsqr,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSqr,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSqr,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSSQR,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSQR,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssqr,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsqr,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSqr,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSqr,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Exponential function: r[i] = e^a[i] */
_MKL_VML_API(void,VSEXP,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDEXP,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsexp,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdexp,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsExp,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdExp,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSEXP,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDEXP,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsexp,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdexp,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsExp,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdExp,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex exponential function: r[i] = e^a[i] */
_MKL_VML_API(void, VCEXP, (const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void, VZEXP, (const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void, vcexp, (const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void, vzexp, (const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void, vcExp, (const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void, vzExp, (const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void, VMCEXP, (const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMZEXP, (const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmcexp, (const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmzexp, (const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcExp, (const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzExp, (const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Exponential function (base 2): r[i] = 2^a[i] */
_MKL_VML_API(void, VSEXP2, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDEXP2, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vsexp2, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdexp2, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsExp2, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdExp2, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSEXP2, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP2, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp2, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp2, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp2, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp2, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Exponential function (base 10): r[i] = 10^a[i] */
_MKL_VML_API(void, VSEXP10, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDEXP10, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vsexp10, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdexp10, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsExp10, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdExp10, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSEXP10, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP10, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp10, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp10, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp10, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp10, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Exponential of arguments decreased by 1: r[i] = e^(a[i]-1) */
_MKL_VML_API(void,VSEXPM1,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDEXPM1,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsexpm1,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdexpm1,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsExpm1,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdExpm1,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSEXPM1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDEXPM1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsexpm1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdexpm1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsExpm1,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdExpm1,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void,VSLN,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDLN,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsln,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdln,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLn,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLn,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSLN,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLN,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsln,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdln,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLn,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLn,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void,VCLN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZLN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcln,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzln,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcLn,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzLn,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCLN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZLN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcln,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzln,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcLn,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzLn,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Logarithm (base 2): r[i] = lb(a[i]) */
_MKL_VML_API(void, VSLOG2, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDLOG2, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vslog2, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdlog2, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsLog2, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdLog2, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSLOG2, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG2, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmslog2, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog2, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog2, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog2, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void,VSLOG10,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDLOG10,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vslog10,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdlog10,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLog10,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLog10,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSLOG10,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLOG10,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslog10,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlog10,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLog10,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLog10,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void,VCLOG10,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZLOG10,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vclog10,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzlog10,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcLog10,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzLog10,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCLOG10,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZLOG10,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmclog10,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzlog10,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcLog10,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzLog10,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Logarithm (base e) of arguments increased by 1: r[i] = log(1+a[i]) */
_MKL_VML_API(void,VSLOG1P,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDLOG1P,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vslog1p,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdlog1p,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLog1p,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLog1p,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSLOG1P,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLOG1P,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslog1p,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlog1p,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLog1p,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLog1p,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Computes the exponent: r[i] = logb(a[i]) */
_MKL_VML_API(void, VSLOGB, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDLOGB, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vslogb, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdlogb, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsLogb, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdLogb, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSLOGB, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOGB, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmslogb, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdlogb, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLogb, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLogb, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Cosine: r[i] = cos(a[i]) */
_MKL_VML_API(void,VSCOS,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDCOS,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vscos,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdcos,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCos,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCos,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSCOS,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCOS,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscos,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcos,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCos,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCos,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex cosine: r[i] = ccos(a[i]) */
_MKL_VML_API(void,VCCOS,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCOS,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vccos,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzcos,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcCos,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzCos,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCOS,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCOS,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmccos,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzcos,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcCos,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzCos,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Sine: r[i] = sin(a[i]) */
_MKL_VML_API(void,VSSIN,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDSIN,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vssin,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdsin,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSin,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSin,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSSIN,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSIN,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssin,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsin,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSin,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSin,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex sine: r[i] = sin(a[i]) */
_MKL_VML_API(void,VCSIN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSIN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsin,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsin,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSin,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSin,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSIN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSIN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsin,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsin,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSin,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSin,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void,VSTAN,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDTAN,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vstan,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdtan,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTan,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTan,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSTAN,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTAN,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstan,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtan,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTan,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTan,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void,VCTAN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZTAN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vctan,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vztan,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcTan,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzTan,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCTAN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZTAN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmctan,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmztan,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcTan,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzTan,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Cosine PI: r[i] = cos(a[i]*PI) */
_MKL_VML_API(void, VSCOSPI, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDCOSPI, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vscospi, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdcospi, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsCospi, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdCospi, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSCOSPI, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSPI, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmscospi, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdcospi, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCospi, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCospi, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Sine PI: r[i] = sin(a[i]*PI) */
_MKL_VML_API(void, VSSINPI, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDSINPI, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vssinpi, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdsinpi, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsSinpi, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdSinpi, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSSINPI, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINPI, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmssinpi, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdsinpi, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinpi, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinpi, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Tangent PI: r[i] = tan(a[i]*PI) */
_MKL_VML_API(void, VSTANPI, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDTANPI, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vstanpi, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdtanpi, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsTanpi, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdTanpi, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSTANPI, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANPI, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmstanpi, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdtanpi, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanpi, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanpi, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Cosine degree: r[i] = cos(a[i]*PI/180) */
_MKL_VML_API(void, VSCOSD, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDCOSD, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vscosd, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdcosd, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsCosd, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdCosd, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSCOSD, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSD, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmscosd, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdcosd, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCosd, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCosd, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Sine degree: r[i] = sin(a[i]*PI/180) */
_MKL_VML_API(void, VSSIND, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDSIND, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vssind, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdsind, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsSind, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdSind, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSSIND, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDSIND, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmssind, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdsind, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSind, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSind, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Tangent degree: r[i] = tan(a[i]*PI/180) */
_MKL_VML_API(void, VSTAND, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDTAND, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vstand, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdtand, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsTand, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdTand, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSTAND, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDTAND, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmstand, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdtand, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTand, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTand, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void,VSCOSH,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDCOSH,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vscosh,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdcosh,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCosh,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCosh,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSCOSH,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCOSH,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscosh,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcosh,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCosh,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCosh,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void,VCCOSH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCOSH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vccosh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzcosh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcCosh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzCosh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCOSH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCOSH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmccosh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzcosh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcCosh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzCosh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void,VSSINH,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDSINH,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vssinh,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdsinh,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSinh,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSinh,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSSINH,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSINH,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssinh,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsinh,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSinh,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSinh,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void,VCSINH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSINH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsinh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsinh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSinh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSinh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSINH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSINH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsinh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsinh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSinh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSinh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void,VSTANH,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDTANH,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vstanh,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdtanh,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTanh,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTanh,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSTANH,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTANH,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstanh,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtanh,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTanh,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTanh,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void,VCTANH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZTANH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vctanh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vztanh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcTanh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzTanh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCTANH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZTANH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmctanh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmztanh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcTanh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzTanh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void,VSACOS,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDACOS,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsacos,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdacos,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAcos,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAcos,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSACOS,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDACOS,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsacos,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdacos,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAcos,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAcos,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void,VCACOS,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZACOS,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcacos,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzacos,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAcos,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAcos,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCACOS,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZACOS,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcacos,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzacos,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAcos,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAcos,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void,VSASIN,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDASIN,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsasin,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdasin,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAsin,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAsin,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSASIN,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDASIN,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsasin,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdasin,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAsin,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAsin,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void,VCASIN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZASIN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcasin,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzasin,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAsin,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAsin,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCASIN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZASIN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcasin,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzasin,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAsin,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAsin,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void,VSATAN,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDATAN,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsatan,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdatan,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAtan,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAtan,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSATAN,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDATAN,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsatan,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdatan,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAtan,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAtan,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void,VCATAN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZATAN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcatan,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzatan,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAtan,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAtan,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCATAN,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZATAN,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcatan,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzatan,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAtan,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAtan,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc cosine PI: r[i] = arccos(a[i])/PI */
_MKL_VML_API(void, VSACOSPI, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDACOSPI, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vsacospi, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdacospi, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsAcospi, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdAcospi, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSACOSPI, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSPI, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsacospi, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdacospi, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcospi, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcospi, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Arc sine PI: r[i] = arcsin(a[i])/PI */
_MKL_VML_API(void, VSASINPI, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDASINPI, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vsasinpi, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdasinpi, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsAsinpi, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdAsinpi, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSASINPI, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINPI, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsasinpi, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdasinpi, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinpi, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinpi, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Arc tangent PI: r[i] = arctan(a[i])/PI */
_MKL_VML_API(void, VSATANPI, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDATANPI, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vsatanpi, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdatanpi, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsAtanpi, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdAtanpi, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSATANPI, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANPI, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsatanpi, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdatanpi, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanpi, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanpi, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void,VSACOSH,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDACOSH,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsacosh,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdacosh,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAcosh,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAcosh,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSACOSH,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDACOSH,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsacosh,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdacosh,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAcosh,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAcosh,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void,VCACOSH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZACOSH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcacosh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzacosh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAcosh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAcosh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCACOSH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZACOSH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcacosh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzacosh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAcosh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAcosh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void,VSASINH,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDASINH,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsasinh,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdasinh,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAsinh,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAsinh,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSASINH,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDASINH,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsasinh,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdasinh,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAsinh,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAsinh,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void,VCASINH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZASINH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcasinh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzasinh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAsinh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAsinh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCASINH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZASINH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcasinh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzasinh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAsinh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAsinh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void,VSATANH,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDATANH,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsatanh,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdatanh,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAtanh,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAtanh,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSATANH,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDATANH,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsatanh,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdatanh,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAtanh,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAtanh,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void,VCATANH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZATANH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcatanh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzatanh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAtanh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAtanh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCATANH,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZATANH,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcatanh,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzatanh,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAtanh,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAtanh,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Error function: r[i] = erf(a[i]) */
_MKL_VML_API(void,VSERF,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDERF,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vserf,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vderf,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErf,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErf,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSERF,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERF,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserf,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderf,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErf,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErf,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Inverse error function: r[i] = erfinv(a[i]) */
_MKL_VML_API(void,VSERFINV,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFINV,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vserfinv,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vderfinv,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfInv,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfInv,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFINV,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFINV,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfinv,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfinv,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfInv,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfInv,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Square root of the sum of the squares: r[i] = hypot(a[i],b[i]) */
_MKL_VML_API(void,VSHYPOT,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDHYPOT,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vshypot,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdhypot,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsHypot,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdHypot,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSHYPOT,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDHYPOT,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmshypot,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdhypot,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsHypot,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdHypot,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complementary error function: r[i] = 1 - erf(a[i]) */
_MKL_VML_API(void,VSERFC,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFC,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vserfc,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vderfc,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfc,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfc,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFC,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFC,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfc,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfc,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfc,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfc,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Inverse complementary error function: r[i] = erfcinv(a[i]) */
_MKL_VML_API(void,VSERFCINV,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFCINV,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vserfcinv,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vderfcinv,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfcInv,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfcInv,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFCINV,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFCINV,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfcinv,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfcinv,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfcInv,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfcInv,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Scaled complementary error function: r[i] = erfcx(a[i]) */
_MKL_VML_API(void,VSERFCX,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFCX,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vserfcx,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vderfcx,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfcx,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfcx,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFCX,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFCX,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfcx,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfcx,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfcx,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfcx,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Cumulative normal distribution function: r[i] = cdfnorm(a[i]) */
_MKL_VML_API(void,VSCDFNORM,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDCDFNORM,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vscdfnorm,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdcdfnorm,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCdfNorm,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCdfNorm,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSCDFNORM,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCDFNORM,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscdfnorm,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcdfnorm,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCdfNorm,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCdfNorm,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Inverse cumulative normal distribution function: r[i] = cdfnorminv(a[i]) */
_MKL_VML_API(void,VSCDFNORMINV,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDCDFNORMINV,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vscdfnorminv,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdcdfnorminv,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCdfNormInv,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCdfNormInv,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSCDFNORMINV,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCDFNORMINV,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscdfnorminv,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcdfnorminv,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCdfNormInv,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCdfNormInv,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Logarithm (base e) of the absolute value of gamma function: r[i] = lgamma(a[i]) */
_MKL_VML_API(void,VSLGAMMA,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDLGAMMA,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vslgamma,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdlgamma,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLGamma,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLGamma,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSLGAMMA,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLGAMMA,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslgamma,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlgamma,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLGamma,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLGamma,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Gamma function: r[i] = tgamma(a[i]) */
_MKL_VML_API(void,VSTGAMMA,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDTGAMMA,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vstgamma,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdtgamma,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTGamma,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTGamma,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSTGAMMA,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTGAMMA,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstgamma,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtgamma,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTGamma,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTGamma,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* I0 Bessel function: r[i] = i0(a[i]) */
_MKL_VML_API(void,VSI0,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDI0,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsi0,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdi0,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsI0,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdI0,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSI0,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDI0,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsi0,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdi0,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsI0,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdI0,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* I1 Bessel function: r[i] = i1(a[i]) */
_MKL_VML_API(void,VSI1,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDI1,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsi1,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdi1,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsI1,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdI1,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSI1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDI1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsi1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdi1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsI1,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdI1,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* J0 Bessel function: r[i] = j0(a[i]) */
_MKL_VML_API(void,VSJ0,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDJ0,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsj0,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdj0,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsJ0,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdJ0,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSJ0,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDJ0,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsj0,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdj0,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsJ0,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdJ0,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* J1 Bessel function: r[i] = j1(a[i]) */
_MKL_VML_API(void,VSJ1,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDJ1,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsj1,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdj1,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsJ1,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdJ1,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSJ1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDJ1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsj1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdj1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsJ1,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdJ1,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Y0 Bessel function: r[i] = y0(a[i]) */
_MKL_VML_API(void,VSY0,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDY0,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsy0,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdy0,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsY0,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdY0,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSY0,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDY0,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsy0,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdy0,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsY0,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdY0,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Y1 Bessel function: r[i] = y1(a[i]) */
_MKL_VML_API(void,VSY1,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDY1,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsy1,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdy1,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsY1,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdY1,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSY1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDY1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsy1,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdy1,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsY1,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdY1,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Jn Bessel function: r[i] = jn(a[i],n) */
_MKL_VML_API(void,VSJN,(const MKL_INT *n, const float a[], const float *b, float r[]));
_MKL_VML_API(void,VDJN,(const MKL_INT *n, const double a[], const double *b, double r[]));
_mkl_vml_api(void,vsjn,(const MKL_INT *n, const float a[], const float *b, float r[]));
_mkl_vml_api(void,vdjn,(const MKL_INT *n, const double a[], const double *b, double r[]));
_Mkl_Vml_Api(void,vsJn,(const MKL_INT n, const float a[], const float b, float r[]));
_Mkl_Vml_Api(void,vdJn,(const MKL_INT n, const double a[], const double b, double r[]));

_MKL_VML_API(void,VMSJN,(const MKL_INT *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDJN,(const MKL_INT *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsjn,(const MKL_INT *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdjn,(const MKL_INT *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsJn,(const MKL_INT n, const float a[], const float b, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdJn,(const MKL_INT n, const double a[], const double b, double r[], MKL_INT64 mode));

/* Yn Bessel function: r[i] = yn(a[i],n) */
_MKL_VML_API(void,VSYN,(const MKL_INT *n, const float a[], const float *b, float r[]));
_MKL_VML_API(void,VDYN,(const MKL_INT *n, const double a[], const double *b, double r[]));
_mkl_vml_api(void,vsyn,(const MKL_INT *n, const float a[], const float *b, float r[]));
_mkl_vml_api(void,vdyn,(const MKL_INT *n, const double a[], const double *b, double r[]));
_Mkl_Vml_Api(void,vsYn,(const MKL_INT n, const float a[], const float b, float r[]));
_Mkl_Vml_Api(void,vdYn,(const MKL_INT n, const double a[], const double b, double r[]));

_MKL_VML_API(void,VMSYN,(const MKL_INT *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDYN,(const MKL_INT *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsyn,(const MKL_INT *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdyn,(const MKL_INT *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsYn,(const MKL_INT n, const float a[], const float b, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdYn,(const MKL_INT n, const double a[], const double b, double r[], MKL_INT64 mode));

/* Arc tangent of a/b: r[i] = arctan(a[i]/b[i]) */
_MKL_VML_API(void,VSATAN2,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDATAN2,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsatan2,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdatan2,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsAtan2,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdAtan2,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSATAN2,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDATAN2,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsatan2,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdatan2,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAtan2,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAtan2,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Arc tangent of a/b divided by PI: r[i] = arctan(a[i]/b[i])/PI */
_MKL_VML_API(void, VSATAN2PI, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDATAN2PI, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsatan2pi, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdatan2pi, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsAtan2pi, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdAtan2pi, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSATAN2PI, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDATAN2PI, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsatan2pi, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdatan2pi, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtan2pi, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtan2pi, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Multiplicaton: r[i] = a[i] * b[i] */
_MKL_VML_API(void,VSMUL,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDMUL,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsmul,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdmul,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsMul,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdMul,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSMUL,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDMUL,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsmul,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdmul,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsMul,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdMul,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex multiplication: r[i] = a[i] * b[i] */
_MKL_VML_API(void,VCMUL,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZMUL,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcmul,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzmul,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcMul,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzMul,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCMUL,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZMUL,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcmul,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzmul,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcMul,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzMul,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Division: r[i] = a[i] / b[i] */
_MKL_VML_API(void,VSDIV,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDDIV,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsdiv,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vddiv,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsDiv,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdDiv,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSDIV,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDDIV,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsdiv,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmddiv,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsDiv,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdDiv,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex division: r[i] = a[i] / b[i] */
_MKL_VML_API(void,VCDIV,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZDIV,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcdiv,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzdiv,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcDiv,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzDiv,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCDIV,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZDIV,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcdiv,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzdiv,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcDiv,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzDiv,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void,VSPOW,(const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDPOW,(const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vspow,(const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdpow,(const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsPow,(const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdPow,(const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSPOW,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOW,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspow,(const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpow,(const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPow,(const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPow,(const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void,VCPOW,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZPOW,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcpow,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzpow,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcPow,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzPow,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCPOW,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZPOW,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcpow,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzpow,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcPow,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzPow,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Power function: r[i] = a[i]^(3/2) */
_MKL_VML_API(void,VSPOW3O2,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDPOW3O2,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vspow3o2,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdpow3o2,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsPow3o2,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdPow3o2,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSPOW3O2,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOW3O2,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspow3o2,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpow3o2,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPow3o2,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPow3o2,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Power function: r[i] = a[i]^(2/3) */
_MKL_VML_API(void,VSPOW2O3,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDPOW2O3,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vspow2o3,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdpow2o3,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsPow2o3,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdPow2o3,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSPOW2O3,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOW2O3,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspow2o3,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpow2o3,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPow2o3,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPow2o3,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void,VSPOWX,(const MKL_INT *n, const float a[], const float *b, float r[]));
_MKL_VML_API(void,VDPOWX,(const MKL_INT *n, const double a[], const double *b, double r[]));
_mkl_vml_api(void,vspowx,(const MKL_INT *n, const float a[], const float *b, float r[]));
_mkl_vml_api(void,vdpowx,(const MKL_INT *n, const double a[], const double *b, double r[]));
_Mkl_Vml_Api(void,vsPowx,(const MKL_INT n, const float a[], const float b, float r[]));
_Mkl_Vml_Api(void,vdPowx,(const MKL_INT n, const double a[], const double b, double r[]));

_MKL_VML_API(void,VMSPOWX,(const MKL_INT *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOWX,(const MKL_INT *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspowx,(const MKL_INT *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpowx,(const MKL_INT *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPowx,(const MKL_INT n, const float a[], const float b, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPowx,(const MKL_INT n, const double a[], const double b, double r[], MKL_INT64 mode));

/* Complex power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void,VCPOWX,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[]));
_MKL_VML_API(void,VZPOWX,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[]));
_mkl_vml_api(void,vcpowx,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[]));
_mkl_vml_api(void,vzpowx,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcPowx,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b, MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzPowx,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b, MKL_Complex16 r[]));

_MKL_VML_API(void,VMCPOWX,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZPOWX,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcpowx,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzpowx,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcPowx,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b, MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzPowx,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b, MKL_Complex16 r[], MKL_INT64 mode));

/* Power function with a[i]>=0: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VSPOWR, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDPOWR, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vspowr, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdpowr, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsPowr, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdPowr, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSPOWR, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWR, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmspowr, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowr, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowr, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowr, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Sine & cosine: r1[i] = sin(a[i]), r2[i]=cos(a[i]) */
_MKL_VML_API(void,VSSINCOS,(const MKL_INT *n, const float a[], float r1[], float r2[]));
_MKL_VML_API(void,VDSINCOS,(const MKL_INT *n, const double a[], double r1[], double r2[]));
_mkl_vml_api(void,vssincos,(const MKL_INT *n, const float a[], float r1[], float r2[]));
_mkl_vml_api(void,vdsincos,(const MKL_INT *n, const double a[], double r1[], double r2[]));
_Mkl_Vml_Api(void,vsSinCos,(const MKL_INT n, const float a[], float r1[], float r2[]));
_Mkl_Vml_Api(void,vdSinCos,(const MKL_INT n, const double a[], double r1[], double r2[]));

_MKL_VML_API(void,VMSSINCOS,(const MKL_INT *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSINCOS,(const MKL_INT *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssincos,(const MKL_INT *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsincos,(const MKL_INT *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSinCos,(const MKL_INT n, const float a[], float r1[], float r2[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSinCos,(const MKL_INT n, const double a[], double r1[], double r2[], MKL_INT64 mode));

/* Sine & cosine PI: r1[i] = sinpi(a[i]), r2[i]=cospi(a[i]) */
_MKL_VML_API(void,VSSINCOSPI,(const MKL_INT *n, const float a[], float r1[], float r2[]));
_MKL_VML_API(void,VDSINCOSPI,(const MKL_INT *n, const double a[], double r1[], double r2[]));
_mkl_vml_api(void,vssincospi,(const MKL_INT *n, const float a[], float r1[], float r2[]));
_mkl_vml_api(void,vdsincospi,(const MKL_INT *n, const double a[], double r1[], double r2[]));
_Mkl_Vml_Api(void,vsSinCospi,(const MKL_INT n, const float a[], float r1[], float r2[]));
_Mkl_Vml_Api(void,vdSinCospi,(const MKL_INT n, const double a[], double r1[], double r2[]));

_MKL_VML_API(void,VMSSINCOSPI,(const MKL_INT *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSINCOSPI,(const MKL_INT *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssincospi,(const MKL_INT *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsincospi,(const MKL_INT *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSinCospi,(const MKL_INT n, const float a[], float r1[], float r2[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSinCospi,(const MKL_INT n, const double a[], double r1[], double r2[], MKL_INT64 mode));

/* Linear fraction: r[i] = (a[i]*scalea + shifta)/(b[i]*scaleb + shiftb) */
_MKL_VML_API(void,VSLINEARFRAC,(const MKL_INT *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[]));
_MKL_VML_API(void,VDLINEARFRAC,(const MKL_INT *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[]));
_mkl_vml_api(void,vslinearfrac,(const MKL_INT *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[]));
_mkl_vml_api(void,vdlinearfrac,(const MKL_INT *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[]));
_Mkl_Vml_Api(void,vsLinearFrac,(const MKL_INT n, const float a[], const float b[], const float scalea, const float shifta, const float scaleb, const float shiftb, float r[]));
_Mkl_Vml_Api(void,vdLinearFrac,(const MKL_INT n, const double a[], const double b[], const double scalea, const double shifta, const double scaleb, const double shiftb, double r[]));

_MKL_VML_API(void,VMSLINEARFRAC,(const MKL_INT *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLINEARFRAC,(const MKL_INT *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslinearfrac,(const MKL_INT *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlinearfrac,(const MKL_INT *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLinearFrac,(const MKL_INT n, const float a[], const float b[], const float scalea, const float shifta, const float scaleb, const float shiftb, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLinearFrac,(const MKL_INT n, const double a[], const double b[], const double scalea, const double shifta, const double scaleb, const double shiftb, double r[], MKL_INT64 mode));

/* Integer value rounded towards plus infinity: r[i] = ceil(a[i]) */
_MKL_VML_API(void,VSCEIL,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDCEIL,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsceil,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdceil,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCeil,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCeil,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSCEIL,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCEIL,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsceil,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdceil,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCeil,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCeil,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Integer value rounded towards minus infinity: r[i] = floor(a[i]) */
_MKL_VML_API(void,VSFLOOR,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDFLOOR,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsfloor,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdfloor,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsFloor,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdFloor,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSFLOOR,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDFLOOR,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsfloor,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdfloor,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsFloor,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdFloor,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Signed fraction part: r[i] = a[i] - |a[i]| */
_MKL_VML_API(void,VSFRAC,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDFRAC,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsfrac,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdfrac,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsFrac,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdFrac,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSFRAC,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDFRAC,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsfrac,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdfrac,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsFrac,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdFrac,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Truncated integer value and the remaining fraction part: r1[i] = |a[i]|, r2[i] = a[i] - |a[i]| */
_MKL_VML_API(void,VSMODF,(const MKL_INT *n, const float a[], float r1[], float r2[]));
_MKL_VML_API(void,VDMODF,(const MKL_INT *n, const double a[], double r1[], double r2[]));
_mkl_vml_api(void,vsmodf,(const MKL_INT *n, const float a[], float r1[], float r2[]));
_mkl_vml_api(void,vdmodf,(const MKL_INT *n, const double a[], double r1[], double r2[]));
_Mkl_Vml_Api(void,vsModf,(const MKL_INT n, const float a[], float r1[], float r2[]));
_Mkl_Vml_Api(void,vdModf,(const MKL_INT n, const double a[], double r1[], double r2[]));

_MKL_VML_API(void,VMSMODF,(const MKL_INT *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDMODF,(const MKL_INT *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsmodf,(const MKL_INT *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdmodf,(const MKL_INT *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsModf,(const MKL_INT n, const float a[], float r1[], float r2[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdModf,(const MKL_INT n, const double a[], double r1[], double r2[], MKL_INT64 mode));

/* Modulus function: r[i] = fmod(a[i], b[i]) */
_MKL_VML_API(void, VSFMOD, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFMOD, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfmod, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfmod, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFmod, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFmod, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFMOD, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMOD, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmod, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmod, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmod, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmod, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Remainder function: r[i] = remainder(a[i], b[i]) */
_MKL_VML_API(void, VSREMAINDER, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDREMAINDER, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsremainder, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdremainder, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsRemainder, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdRemainder, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSREMAINDER, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDREMAINDER, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsremainder, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdremainder, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRemainder, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRemainder, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Next after function: r[i] = nextafter(a[i], b[i]) */
_MKL_VML_API(void, VSNEXTAFTER, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDNEXTAFTER, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsnextafter, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdnextafter, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsNextAfter, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdNextAfter, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSNEXTAFTER, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDNEXTAFTER, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsnextafter, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdnextafter, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsNextAfter, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdNextAfter, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Copy sign function: r[i] = copysign(a[i], b[i]) */
_MKL_VML_API(void, VSCOPYSIGN, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDCOPYSIGN, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vscopysign, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdcopysign, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsCopySign, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdCopySign, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSCOPYSIGN, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOPYSIGN, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmscopysign, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdcopysign, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCopySign, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCopySign, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Positive difference function: r[i] = fdim(a[i], b[i]) */
_MKL_VML_API(void, VSFDIM, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFDIM, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfdim, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfdim, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFdim, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFdim, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFDIM, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFDIM, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfdim, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfdim, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFdim, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFdim, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Maximum function: r[i] = fmax(a[i], b[i]) */
_MKL_VML_API(void, VSFMAX, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFMAX, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfmax, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfmax, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFmax, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFmax, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFMAX, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMAX, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmax, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmax, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmax, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmax, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Minimum function: r[i] = fmin(a[i], b[i]) */
_MKL_VML_API(void, VSFMIN, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFMIN, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfmin, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfmin, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFmin, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFmin, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFMIN, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMIN, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmin, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmin, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmin, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmin, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Maximum magnitude function: r[i] = maxmag(a[i], b[i]) */
_MKL_VML_API(void, VSMAXMAG, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDMAXMAG, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsmaxmag, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdmaxmag, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsMaxMag, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdMaxMag, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSMAXMAG, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDMAXMAG, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsmaxmag, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdmaxmag, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMaxMag, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMaxMag, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Minimum magnitude function: r[i] = minmag(a[i], b[i]) */
_MKL_VML_API(void, VSMINMAG, (const MKL_INT *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDMINMAG, (const MKL_INT *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsminmag, (const MKL_INT *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdminmag, (const MKL_INT *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsMinMag, (const MKL_INT n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdMinMag, (const MKL_INT n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSMINMAG, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDMINMAG, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsminmag, (const MKL_INT *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdminmag, (const MKL_INT *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMinMag, (const MKL_INT n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMinMag, (const MKL_INT n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Rounded integer value in the current rounding mode: r[i] = nearbyint(a[i]) */
_MKL_VML_API(void,VSNEARBYINT,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDNEARBYINT,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsnearbyint,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdnearbyint,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsNearbyInt,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdNearbyInt,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSNEARBYINT,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDNEARBYINT,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsnearbyint,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdnearbyint,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsNearbyInt,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdNearbyInt,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Rounded integer value in the current rounding mode with inexact result exception raised for rach changed value: r[i] = rint(a[i]) */
_MKL_VML_API(void,VSRINT,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDRINT,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsrint,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdrint,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsRint,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdRint,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSRINT,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDRINT,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsrint,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdrint,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsRint,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdRint,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Value rounded to the nearest integer: r[i] = round(a[i]) */
_MKL_VML_API(void,VSROUND,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDROUND,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsround,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdround,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsRound,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdRound,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSROUND,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDROUND,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsround,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdround,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsRound,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdRound,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Integer value rounded towards zero: r[i] = trunc(a[i]) */
_MKL_VML_API(void,VSTRUNC,(const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDTRUNC,(const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vstrunc,(const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdtrunc,(const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTrunc,(const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTrunc,(const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSTRUNC,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTRUNC,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstrunc,(const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtrunc,(const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTrunc,(const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTrunc,(const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Element by element conjugation: r[i] = conj(a[i]) */
_MKL_VML_API(void,VCCONJ,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCONJ,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcconj,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzconj,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcConj,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzConj,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCONJ,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCONJ,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcconj,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzconj,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcConj,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzConj,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Element by element multiplication of vector A element and conjugated vector B element: r[i] = mulbyconj(a[i],b[i]) */
_MKL_VML_API(void,VCMULBYCONJ,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZMULBYCONJ,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcmulbyconj,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzmulbyconj,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcMulByConj,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzMulByConj,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCMULBYCONJ,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZMULBYCONJ,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcmulbyconj,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzmulbyconj,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcMulByConj,(const MKL_INT n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzMulByConj,(const MKL_INT n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Complex exponent of real vector elements: r[i] = CIS(a[i]) */
_MKL_VML_API(void,VCCIS,(const MKL_INT *n, const float a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCIS,(const MKL_INT *n, const double a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vccis,(const MKL_INT *n, const float a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzcis,(const MKL_INT *n, const double a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcCIS,(const MKL_INT n, const float a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzCIS,(const MKL_INT n, const double a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCIS,(const MKL_INT *n, const float a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCIS,(const MKL_INT *n, const double a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmccis,(const MKL_INT *n, const float a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzcis,(const MKL_INT *n, const double a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcCIS,(const MKL_INT n, const float a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzCIS,(const MKL_INT n, const double a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Exponential integral of real vector elements: r[i] = E1(a[i]) */
_MKL_VML_API(void, VSEXPINT1, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void, VDEXPINT1, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void, vsexpint1, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void, vdexpint1, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsExpInt1, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdExpInt1, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void, VMSEXPINT1, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPINT1, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpint1, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpint1, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpInt1, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpInt1, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* Absolute value: r[i] = |a[i]| */
_MKL_VML_API(void,VSABS_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDABS_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsabs_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdabs_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAbs_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAbs_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSABS_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDABS_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsabs_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdabs_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAbs_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAbs_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex absolute value: r[i] = |a[i]| */
_MKL_VML_API(void,VCABS_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[]));
_MKL_VML_API(void,VZABS_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[]));
_mkl_vml_api(void,vcabs_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[]));
_mkl_vml_api(void,vzabs_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[]));
_Mkl_Vml_Api(void,vcAbs_64, (const MKL_INT64 n, const MKL_Complex8 a[], float r[]));
_Mkl_Vml_Api(void,vzAbs_64, (const MKL_INT64 n, const MKL_Complex16 a[], double r[]));

_MKL_VML_API(void,VMCABS_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZABS_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcabs_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzabs_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAbs_64, (const MKL_INT64 n, const MKL_Complex8 a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAbs_64, (const MKL_INT64 n, const MKL_Complex16 a[], double r[], MKL_INT64 mode));

/* Argument of complex value: r[i] = carg(a[i]) */
_MKL_VML_API(void,VCARG_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[]));
_MKL_VML_API(void,VZARG_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[]));
_mkl_vml_api(void,vcarg_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[]));
_mkl_vml_api(void,vzarg_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[]));
_Mkl_Vml_Api(void,vcArg_64, (const MKL_INT64 n, const MKL_Complex8 a[], float r[]));
_Mkl_Vml_Api(void,vzArg_64, (const MKL_INT64 n, const MKL_Complex16 a[], double r[]));

_MKL_VML_API(void,VMCARG_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZARG_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcarg_64, (const MKL_INT64 *n, const MKL_Complex8 a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzarg_64, (const MKL_INT64 *n, const MKL_Complex16 a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcArg_64, (const MKL_INT64 n, const MKL_Complex8 a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzArg_64, (const MKL_INT64 n, const MKL_Complex16 a[], double r[], MKL_INT64 mode));

/* Addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void,VSADD_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDADD_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsadd_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdadd_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsAdd_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdAdd_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSADD_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDADD_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsadd_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdadd_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAdd_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAdd_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void,VCADD_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZADD_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcadd_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzadd_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAdd_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAdd_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCADD_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZADD_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcadd_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzadd_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAdd_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAdd_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void,VSSUB_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDSUB_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vssub_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdsub_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsSub_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdSub_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSSUB_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSUB_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssub_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsub_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSub_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSub_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void,VCSUB_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSUB_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsub_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsub_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSub_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSub_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSUB_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSUB_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsub_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsub_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSub_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSub_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Reciprocal: r[i] = 1.0 / a[i] */
_MKL_VML_API(void,VSINV_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDINV_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsinv_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdinv_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsInv_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdInv_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSINV_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDINV_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsinv_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdinv_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsInv_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdInv_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void,VSSQRT_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDSQRT_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vssqrt_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdsqrt_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSqrt_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSqrt_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSSQRT_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSQRT_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssqrt_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsqrt_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSqrt_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSqrt_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void,VCSQRT_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSQRT_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsqrt_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsqrt_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSqrt_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSqrt_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSQRT_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSQRT_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsqrt_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsqrt_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSqrt_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSqrt_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Reciprocal square root: r[i] = 1/a[i]^0.5 */
_MKL_VML_API(void,VSINVSQRT_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDINVSQRT_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsinvsqrt_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdinvsqrt_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsInvSqrt_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdInvSqrt_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSINVSQRT_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDINVSQRT_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsinvsqrt_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdinvsqrt_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsInvSqrt_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdInvSqrt_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Cube root: r[i] = a[i]^(1/3) */
_MKL_VML_API(void,VSCBRT_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDCBRT_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vscbrt_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdcbrt_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCbrt_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCbrt_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSCBRT_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCBRT_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscbrt_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcbrt_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCbrt_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCbrt_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Reciprocal cube root: r[i] = 1/a[i]^(1/3) */
_MKL_VML_API(void,VSINVCBRT_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDINVCBRT_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsinvcbrt_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdinvcbrt_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsInvCbrt_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdInvCbrt_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSINVCBRT_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDINVCBRT_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsinvcbrt_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdinvcbrt_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsInvCbrt_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdInvCbrt_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Squaring: r[i] = a[i]^2 */
_MKL_VML_API(void,VSSQR_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDSQR_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vssqr_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdsqr_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSqr_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSqr_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSSQR_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSQR_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssqr_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsqr_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSqr_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSqr_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Exponential function: r[i] = e^a[i] */
_MKL_VML_API(void,VSEXP_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDEXP_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsexp_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdexp_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsExp_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdExp_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSEXP_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDEXP_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsexp_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdexp_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsExp_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdExp_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex exponential function: r[i] = e^a[i] */
_MKL_VML_API(void, VCEXP_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void, VZEXP_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void, vcexp_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void, vzexp_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void, vcExp_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void, vzExp_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void, VMCEXP_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMZEXP_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmcexp_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmzexp_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcExp_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzExp_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Exponential function (base 2): r[i] = 2^a[i] */
_MKL_VML_API(void, VSEXP2_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDEXP2_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vsexp2_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdexp2_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsExp2_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdExp2_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSEXP2_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP2_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp2_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp2_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp2_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp2_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Exponential function (base 10): r[i] = 10^a[i] */
_MKL_VML_API(void, VSEXP10_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDEXP10_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vsexp10_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdexp10_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsExp10_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdExp10_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSEXP10_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP10_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp10_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp10_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp10_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp10_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Exponential of arguments decreased by 1: r[i] = e^(a[i]-1) */
_MKL_VML_API(void,VSEXPM1_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDEXPM1_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsexpm1_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdexpm1_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsExpm1_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdExpm1_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSEXPM1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDEXPM1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsexpm1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdexpm1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsExpm1_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdExpm1_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void,VSLN_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDLN_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsln_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdln_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLn_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLn_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSLN_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLN_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsln_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdln_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLn_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLn_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void,VCLN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZLN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcln_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzln_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcLn_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzLn_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCLN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZLN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcln_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzln_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcLn_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzLn_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Logarithm (base 2): r[i] = lb(a[i]) */
_MKL_VML_API(void, VSLOG2_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDLOG2_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vslog2_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdlog2_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsLog2_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdLog2_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSLOG2_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG2_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmslog2_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog2_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog2_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog2_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void,VSLOG10_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDLOG10_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vslog10_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdlog10_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLog10_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLog10_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSLOG10_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLOG10_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslog10_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlog10_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLog10_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLog10_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void,VCLOG10_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZLOG10_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vclog10_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzlog10_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcLog10_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzLog10_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCLOG10_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZLOG10_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmclog10_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzlog10_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcLog10_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzLog10_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Logarithm (base e) of arguments increased by 1: r[i] = log(1+a[i]) */
_MKL_VML_API(void,VSLOG1P_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDLOG1P_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vslog1p_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdlog1p_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLog1p_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLog1p_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSLOG1P_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLOG1P_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslog1p_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlog1p_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLog1p_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLog1p_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Computes the exponent: r[i] = logb(a[i]) */
_MKL_VML_API(void, VSLOGB_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDLOGB_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vslogb_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdlogb_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsLogb_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdLogb_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSLOGB_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOGB_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmslogb_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdlogb_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLogb_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLogb_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Cosine: r[i] = cos(a[i]) */
_MKL_VML_API(void,VSCOS_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDCOS_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vscos_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdcos_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCos_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCos_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSCOS_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCOS_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscos_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcos_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCos_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCos_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex cosine: r[i] = ccos(a[i]) */
_MKL_VML_API(void,VCCOS_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCOS_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vccos_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzcos_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcCos_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzCos_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCOS_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCOS_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmccos_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzcos_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcCos_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzCos_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Sine: r[i] = sin(a[i]) */
_MKL_VML_API(void,VSSIN_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDSIN_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vssin_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdsin_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSin_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSin_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSSIN_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSIN_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssin_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsin_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSin_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSin_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex sine: r[i] = sin(a[i]) */
_MKL_VML_API(void,VCSIN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSIN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsin_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsin_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSin_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSin_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSIN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSIN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsin_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsin_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSin_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSin_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void,VSTAN_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDTAN_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vstan_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdtan_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTan_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTan_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSTAN_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTAN_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstan_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtan_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTan_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTan_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void,VCTAN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZTAN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vctan_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vztan_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcTan_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzTan_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCTAN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZTAN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmctan_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmztan_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcTan_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzTan_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Cosine PI: r[i] = cos(a[i]*PI) */
_MKL_VML_API(void, VSCOSPI_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDCOSPI_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vscospi_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdcospi_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsCospi_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdCospi_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSCOSPI_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSPI_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmscospi_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdcospi_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCospi_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCospi_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Sine PI: r[i] = sin(a[i]*PI) */
_MKL_VML_API(void, VSSINPI_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDSINPI_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vssinpi_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdsinpi_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsSinpi_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdSinpi_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSSINPI_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINPI_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmssinpi_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdsinpi_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinpi_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinpi_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Tangent PI: r[i] = tan(a[i]*PI) */
_MKL_VML_API(void, VSTANPI_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDTANPI_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vstanpi_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdtanpi_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsTanpi_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdTanpi_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSTANPI_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANPI_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmstanpi_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdtanpi_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanpi_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanpi_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Cosine degree: r[i] = cos(a[i]*PI/180) */
_MKL_VML_API(void, VSCOSD_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDCOSD_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vscosd_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdcosd_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsCosd_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdCosd_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSCOSD_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSD_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmscosd_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdcosd_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCosd_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCosd_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Sine degree: r[i] = sin(a[i]*PI/180) */
_MKL_VML_API(void, VSSIND_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDSIND_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vssind_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdsind_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsSind_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdSind_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSSIND_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDSIND_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmssind_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdsind_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSind_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSind_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Tangent degree: r[i] = tan(a[i]*PI/180) */
_MKL_VML_API(void, VSTAND_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDTAND_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vstand_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdtand_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsTand_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdTand_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSTAND_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDTAND_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmstand_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdtand_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTand_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTand_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void,VSCOSH_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDCOSH_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vscosh_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdcosh_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCosh_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCosh_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSCOSH_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCOSH_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscosh_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcosh_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCosh_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCosh_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void,VCCOSH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCOSH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vccosh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzcosh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcCosh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzCosh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCOSH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCOSH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmccosh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzcosh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcCosh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzCosh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void,VSSINH_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDSINH_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vssinh_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdsinh_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsSinh_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdSinh_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSSINH_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSINH_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssinh_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsinh_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSinh_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSinh_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void,VCSINH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZSINH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcsinh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzsinh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcSinh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzSinh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCSINH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZSINH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcsinh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzsinh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcSinh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzSinh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void,VSTANH_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDTANH_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vstanh_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdtanh_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTanh_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTanh_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSTANH_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTANH_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstanh_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtanh_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTanh_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTanh_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void,VCTANH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZTANH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vctanh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vztanh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcTanh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzTanh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCTANH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZTANH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmctanh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmztanh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcTanh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzTanh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void,VSACOS_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDACOS_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsacos_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdacos_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAcos_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAcos_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSACOS_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDACOS_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsacos_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdacos_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAcos_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAcos_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void,VCACOS_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZACOS_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcacos_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzacos_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAcos_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAcos_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCACOS_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZACOS_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcacos_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzacos_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAcos_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAcos_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void,VSASIN_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDASIN_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsasin_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdasin_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAsin_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAsin_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSASIN_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDASIN_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsasin_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdasin_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAsin_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAsin_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void,VCASIN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZASIN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcasin_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzasin_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAsin_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAsin_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCASIN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZASIN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcasin_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzasin_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAsin_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAsin_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void,VSATAN_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDATAN_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsatan_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdatan_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAtan_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAtan_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSATAN_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDATAN_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsatan_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdatan_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAtan_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAtan_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void,VCATAN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZATAN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcatan_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzatan_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAtan_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAtan_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCATAN_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZATAN_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcatan_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzatan_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAtan_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAtan_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Arc cosine PI: r[i] = arccos(a[i])/PI */
_MKL_VML_API(void, VSACOSPI_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDACOSPI_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vsacospi_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdacospi_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsAcospi_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdAcospi_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSACOSPI_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSPI_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsacospi_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdacospi_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcospi_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcospi_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Arc sine PI: r[i] = arcsin(a[i])/PI */
_MKL_VML_API(void, VSASINPI_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDASINPI_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vsasinpi_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdasinpi_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsAsinpi_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdAsinpi_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSASINPI_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINPI_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsasinpi_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdasinpi_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinpi_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinpi_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Arc tangent PI: r[i] = arctan(a[i])/PI */
_MKL_VML_API(void, VSATANPI_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDATANPI_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vsatanpi_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdatanpi_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsAtanpi_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdAtanpi_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSATANPI_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANPI_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsatanpi_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdatanpi_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanpi_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanpi_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void,VSACOSH_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDACOSH_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsacosh_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdacosh_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAcosh_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAcosh_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSACOSH_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDACOSH_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsacosh_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdacosh_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAcosh_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAcosh_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void,VCACOSH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZACOSH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcacosh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzacosh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAcosh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAcosh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCACOSH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZACOSH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcacosh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzacosh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAcosh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAcosh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void,VSASINH_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDASINH_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsasinh_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdasinh_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAsinh_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAsinh_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSASINH_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDASINH_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsasinh_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdasinh_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAsinh_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAsinh_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void,VCASINH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZASINH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcasinh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzasinh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAsinh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAsinh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCASINH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZASINH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcasinh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzasinh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAsinh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAsinh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void,VSATANH_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDATANH_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsatanh_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdatanh_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsAtanh_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdAtanh_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSATANH_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDATANH_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsatanh_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdatanh_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAtanh_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAtanh_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Complex hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void,VCATANH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZATANH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcatanh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzatanh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcAtanh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzAtanh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCATANH_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZATANH_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcatanh_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzatanh_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcAtanh_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzAtanh_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Error function: r[i] = erf(a[i]) */
_MKL_VML_API(void,VSERF_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDERF_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vserf_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vderf_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErf_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErf_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSERF_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERF_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserf_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderf_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErf_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErf_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Inverse error function: r[i] = erfinv(a[i]) */
_MKL_VML_API(void,VSERFINV_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFINV_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vserfinv_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vderfinv_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfInv_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfInv_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFINV_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFINV_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfinv_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfinv_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfInv_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfInv_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Square root of the sum of the squares: r[i] = hypot(a[i],b[i]) */
_MKL_VML_API(void,VSHYPOT_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDHYPOT_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vshypot_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdhypot_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsHypot_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdHypot_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSHYPOT_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDHYPOT_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmshypot_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdhypot_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsHypot_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdHypot_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complementary error function: r[i] = 1 - erf(a[i]) */
_MKL_VML_API(void,VSERFC_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFC_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vserfc_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vderfc_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfc_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfc_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFC_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFC_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfc_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfc_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfc_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfc_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Inverse complementary error function: r[i] = erfcinv(a[i]) */
_MKL_VML_API(void,VSERFCINV_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFCINV_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vserfcinv_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vderfcinv_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfcInv_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfcInv_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFCINV_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFCINV_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfcinv_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfcinv_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfcInv_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfcInv_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Scaled complementary error function: r[i] = erfcx(a[i]) */
_MKL_VML_API(void,VSERFCX_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDERFCX_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vserfcx_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vderfcx_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsErfcx_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdErfcx_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSERFCX_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDERFCX_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmserfcx_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmderfcx_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsErfcx_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdErfcx_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Cumulative normal distribution function: r[i] = cdfnorm(a[i]) */
_MKL_VML_API(void,VSCDFNORM_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDCDFNORM_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vscdfnorm_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdcdfnorm_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCdfNorm_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCdfNorm_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSCDFNORM_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCDFNORM_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscdfnorm_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcdfnorm_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCdfNorm_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCdfNorm_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Inverse cumulative normal distribution function: r[i] = cdfnorminv(a[i]) */
_MKL_VML_API(void,VSCDFNORMINV_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDCDFNORMINV_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vscdfnorminv_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdcdfnorminv_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCdfNormInv_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCdfNormInv_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSCDFNORMINV_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCDFNORMINV_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmscdfnorminv_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdcdfnorminv_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCdfNormInv_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCdfNormInv_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Logarithm (base e) of the absolute value of gamma function: r[i] = lgamma(a[i]) */
_MKL_VML_API(void,VSLGAMMA_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDLGAMMA_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vslgamma_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdlgamma_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsLGamma_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdLGamma_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSLGAMMA_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLGAMMA_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslgamma_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlgamma_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLGamma_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLGamma_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Gamma function: r[i] = tgamma(a[i]) */
_MKL_VML_API(void,VSTGAMMA_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDTGAMMA_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vstgamma_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdtgamma_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTGamma_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTGamma_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSTGAMMA_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTGAMMA_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstgamma_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtgamma_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTGamma_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTGamma_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* I0 Bessel function: r[i] = i0(a[i]) */
_MKL_VML_API(void,VSI0_64, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDI0_64, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsi0_64, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdi0_64, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsI0_64, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdI0_64, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSI0_64, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDI0_64, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsi0_64, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdi0_64, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsI0_64, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdI0_64, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* I1 Bessel function: r[i] = i1(a[i]) */
_MKL_VML_API(void,VSI1_64, (const MKL_INT *n, const float a[], float r[]));
_MKL_VML_API(void,VDI1_64, (const MKL_INT *n, const double a[], double r[]));
_mkl_vml_api(void,vsi1_64, (const MKL_INT *n, const float a[], float r[]));
_mkl_vml_api(void,vdi1_64, (const MKL_INT *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsI1_64, (const MKL_INT n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdI1_64, (const MKL_INT n, const double a[], double r[]));

_MKL_VML_API(void,VMSI1_64, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDI1_64, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsi1_64, (const MKL_INT *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdi1_64, (const MKL_INT *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsI1_64, (const MKL_INT n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdI1_64, (const MKL_INT n, const double a[], double r[], MKL_INT64 mode));

/* J0 Bessel function: r[i] = j0(a[i]) */
_MKL_VML_API(void,VSJ0_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDJ0_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsj0_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdj0_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsJ0_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdJ0_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSJ0_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDJ0_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsj0_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdj0_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsJ0_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdJ0_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* J1 Bessel function: r[i] = j1(a[i]) */
_MKL_VML_API(void,VSJ1_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDJ1_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsj1_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdj1_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsJ1_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdJ1_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSJ1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDJ1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsj1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdj1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsJ1_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdJ1_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Y0 Bessel function: r[i] = y0(a[i]) */
_MKL_VML_API(void,VSY0_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDY0_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsy0_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdy0_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsY0_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdY0_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSY0_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDY0_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsy0_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdy0_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsY0_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdY0_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Y1 Bessel function: r[i] = y1(a[i]) */
_MKL_VML_API(void,VSY1_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDY1_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsy1_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdy1_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsY1_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdY1_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSY1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDY1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsy1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdy1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsY1_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdY1_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Jn Bessel function: r[i] = jn(a[i],n) */
_MKL_VML_API(void,VSJN_64, (const MKL_INT64 *n, const float a[], const float *b, float r[]));
_MKL_VML_API(void,VDJN_64, (const MKL_INT64 *n, const double a[], const double *b, double r[]));
_mkl_vml_api(void,vsjn_64, (const MKL_INT64 *n, const float a[], const float *b, float r[]));
_mkl_vml_api(void,vdjn_64, (const MKL_INT64 *n, const double a[], const double *b, double r[]));
_Mkl_Vml_Api(void,vsJn_64, (const MKL_INT64 n, const float a[], const float b, float r[]));
_Mkl_Vml_Api(void,vdJn_64, (const MKL_INT64 n, const double a[], const double b, double r[]));

_MKL_VML_API(void,VMSJN_64, (const MKL_INT64 *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDJN_64, (const MKL_INT64 *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsjn_64, (const MKL_INT64 *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdjn_64, (const MKL_INT64 *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsJn_64, (const MKL_INT64 n, const float a[], const float b, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdJn_64, (const MKL_INT64 n, const double a[], const double b, double r[], MKL_INT64 mode));

/* Yn Bessel function: r[i] = yn(a[i],n) */
_MKL_VML_API(void,VSYN_64, (const MKL_INT64 *n, const float a[], const float *b, float r[]));
_MKL_VML_API(void,VDYN_64, (const MKL_INT64 *n, const double a[], const double *b, double r[]));
_mkl_vml_api(void,vsyn_64, (const MKL_INT64 *n, const float a[], const float *b, float r[]));
_mkl_vml_api(void,vdyn_64, (const MKL_INT64 *n, const double a[], const double *b, double r[]));
_Mkl_Vml_Api(void,vsYn_64, (const MKL_INT64 n, const float a[], const float b, float r[]));
_Mkl_Vml_Api(void,vdYn_64, (const MKL_INT64 n, const double a[], const double b, double r[]));

_MKL_VML_API(void,VMSYN_64, (const MKL_INT64 *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDYN_64, (const MKL_INT64 *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsyn_64, (const MKL_INT64 *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdyn_64, (const MKL_INT64 *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsYn_64, (const MKL_INT64 n, const float a[], const float b, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdYn_64, (const MKL_INT64 n, const double a[], const double b, double r[], MKL_INT64 mode));

/* Arc tangent of a/b: r[i] = arctan(a[i]/b[i]) */
_MKL_VML_API(void,VSATAN2_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDATAN2_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsatan2_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdatan2_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsAtan2_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdAtan2_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSATAN2_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDATAN2_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsatan2_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdatan2_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsAtan2_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdAtan2_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Arc tangent of a/b divided by PI: r[i] = arctan(a[i]/b[i])/PI */
_MKL_VML_API(void, VSATAN2PI_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDATAN2PI_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsatan2pi_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdatan2pi_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsAtan2pi_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdAtan2pi_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSATAN2PI_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDATAN2PI_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsatan2pi_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdatan2pi_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtan2pi_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtan2pi_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Multiplicaton: r[i] = a[i] * b[i] */
_MKL_VML_API(void,VSMUL_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDMUL_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsmul_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdmul_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsMul_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdMul_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSMUL_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDMUL_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsmul_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdmul_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsMul_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdMul_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex multiplication: r[i] = a[i] * b[i] */
_MKL_VML_API(void,VCMUL_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZMUL_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcmul_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzmul_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcMul_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzMul_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCMUL_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZMUL_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcmul_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzmul_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcMul_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzMul_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Division: r[i] = a[i] / b[i] */
_MKL_VML_API(void,VSDIV_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDDIV_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vsdiv_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vddiv_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsDiv_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdDiv_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSDIV_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDDIV_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsdiv_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmddiv_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsDiv_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdDiv_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex division: r[i] = a[i] / b[i] */
_MKL_VML_API(void,VCDIV_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZDIV_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcdiv_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzdiv_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcDiv_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzDiv_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCDIV_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZDIV_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcdiv_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzdiv_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcDiv_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzDiv_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void,VSPOW_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void,VDPOW_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void,vspow_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void,vdpow_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void,vsPow_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void,vdPow_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void,VMSPOW_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOW_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspow_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpow_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPow_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPow_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Complex power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void,VCPOW_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZPOW_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcpow_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzpow_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcPow_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzPow_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCPOW_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZPOW_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcpow_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzpow_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcPow_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzPow_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Power function: r[i] = a[i]^(3/2) */
_MKL_VML_API(void,VSPOW3O2_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDPOW3O2_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vspow3o2_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdpow3o2_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsPow3o2_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdPow3o2_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSPOW3O2_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOW3O2_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspow3o2_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpow3o2_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPow3o2_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPow3o2_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Power function: r[i] = a[i]^(2/3) */
_MKL_VML_API(void,VSPOW2O3_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDPOW2O3_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vspow2o3_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdpow2o3_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsPow2o3_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdPow2o3_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSPOW2O3_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOW2O3_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspow2o3_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpow2o3_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPow2o3_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPow2o3_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void,VSPOWX_64, (const MKL_INT64 *n, const float a[], const float *b, float r[]));
_MKL_VML_API(void,VDPOWX_64, (const MKL_INT64 *n, const double a[], const double *b, double r[]));
_mkl_vml_api(void,vspowx_64, (const MKL_INT64 *n, const float a[], const float *b, float r[]));
_mkl_vml_api(void,vdpowx_64, (const MKL_INT64 *n, const double a[], const double *b, double r[]));
_Mkl_Vml_Api(void,vsPowx_64, (const MKL_INT64 n, const float a[], const float b, float r[]));
_Mkl_Vml_Api(void,vdPowx_64, (const MKL_INT64 n, const double a[], const double b, double r[]));

_MKL_VML_API(void,VMSPOWX_64, (const MKL_INT64 *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDPOWX_64, (const MKL_INT64 *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmspowx_64, (const MKL_INT64 *n, const float a[], const float *b, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdpowx_64, (const MKL_INT64 *n, const double a[], const double *b, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsPowx_64, (const MKL_INT64 n, const float a[], const float b, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdPowx_64, (const MKL_INT64 n, const double a[], const double b, double r[], MKL_INT64 mode));

/* Complex power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void,VCPOWX_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[]));
_MKL_VML_API(void,VZPOWX_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[]));
_mkl_vml_api(void,vcpowx_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[]));
_mkl_vml_api(void,vzpowx_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcPowx_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b, MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzPowx_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b, MKL_Complex16 r[]));

_MKL_VML_API(void,VMCPOWX_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZPOWX_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcpowx_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 *b, MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzpowx_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 *b, MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcPowx_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b, MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzPowx_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b, MKL_Complex16 r[], MKL_INT64 mode));

/* Power function with a[i]>=0: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VSPOWR_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDPOWR_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vspowr_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdpowr_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsPowr_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdPowr_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSPOWR_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWR_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmspowr_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowr_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowr_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowr_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Sine & cosine: r1[i] = sin(a[i]), r2[i]=cos(a[i]) */
_MKL_VML_API(void,VSSINCOS_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[]));
_MKL_VML_API(void,VDSINCOS_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[]));
_mkl_vml_api(void,vssincos_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[]));
_mkl_vml_api(void,vdsincos_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[]));
_Mkl_Vml_Api(void,vsSinCos_64, (const MKL_INT64 n, const float a[], float r1[], float r2[]));
_Mkl_Vml_Api(void,vdSinCos_64, (const MKL_INT64 n, const double a[], double r1[], double r2[]));

_MKL_VML_API(void,VMSSINCOS_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSINCOS_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssincos_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsincos_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSinCos_64, (const MKL_INT64 n, const float a[], float r1[], float r2[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSinCos_64, (const MKL_INT64 n, const double a[], double r1[], double r2[], MKL_INT64 mode));

/* Sine & cosine PI: r1[i] = sinpi(a[i]), r2[i]=cospi(a[i]) */
_MKL_VML_API(void,VSSINCOSPI_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[]));
_MKL_VML_API(void,VDSINCOSPI_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[]));
_mkl_vml_api(void,vssincospi_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[]));
_mkl_vml_api(void,vdsincospi_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[]));
_Mkl_Vml_Api(void,vsSinCospi_64, (const MKL_INT64 n, const float a[], float r1[], float r2[]));
_Mkl_Vml_Api(void,vdSinCospi_64, (const MKL_INT64 n, const double a[], double r1[], double r2[]));

_MKL_VML_API(void,VMSSINCOSPI_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDSINCOSPI_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmssincospi_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdsincospi_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsSinCospi_64, (const MKL_INT64 n, const float a[], float r1[], float r2[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdSinCospi_64, (const MKL_INT64 n, const double a[], double r1[], double r2[], MKL_INT64 mode));

/* Linear fraction: r[i] = (a[i]*scalea + shifta)/(b[i]*scaleb + shiftb) */
_MKL_VML_API(void,VSLINEARFRAC_64, (const MKL_INT64 *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[]));
_MKL_VML_API(void,VDLINEARFRAC_64, (const MKL_INT64 *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[]));
_mkl_vml_api(void,vslinearfrac_64, (const MKL_INT64 *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[]));
_mkl_vml_api(void,vdlinearfrac_64, (const MKL_INT64 *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[]));
_Mkl_Vml_Api(void,vsLinearFrac_64, (const MKL_INT64 n, const float a[], const float b[], const float scalea, const float shifta, const float scaleb, const float shiftb, float r[]));
_Mkl_Vml_Api(void,vdLinearFrac_64, (const MKL_INT64 n, const double a[], const double b[], const double scalea, const double shifta, const double scaleb, const double shiftb, double r[]));

_MKL_VML_API(void,VMSLINEARFRAC_64, (const MKL_INT64 *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDLINEARFRAC_64, (const MKL_INT64 *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmslinearfrac_64, (const MKL_INT64 *n, const float a[], const float b[], const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdlinearfrac_64, (const MKL_INT64 *n, const double a[], const double b[], const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsLinearFrac_64, (const MKL_INT64 n, const float a[], const float b[], const float scalea, const float shifta, const float scaleb, const float shiftb, float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdLinearFrac_64, (const MKL_INT64 n, const double a[], const double b[], const double scalea, const double shifta, const double scaleb, const double shiftb, double r[], MKL_INT64 mode));

/* Integer value rounded towards plus infinity: r[i] = ceil(a[i]) */
_MKL_VML_API(void,VSCEIL_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDCEIL_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsceil_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdceil_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsCeil_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdCeil_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSCEIL_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDCEIL_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsceil_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdceil_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsCeil_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdCeil_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Integer value rounded towards minus infinity: r[i] = floor(a[i]) */
_MKL_VML_API(void,VSFLOOR_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDFLOOR_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsfloor_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdfloor_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsFloor_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdFloor_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSFLOOR_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDFLOOR_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsfloor_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdfloor_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsFloor_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdFloor_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Signed fraction part: r[i] = a[i] - |a[i]| */
_MKL_VML_API(void,VSFRAC_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDFRAC_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsfrac_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdfrac_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsFrac_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdFrac_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSFRAC_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDFRAC_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsfrac_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdfrac_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsFrac_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdFrac_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Truncated integer value and the remaining fraction part: r1[i] = |a[i]|, r2[i] = a[i] - |a[i]| */
_MKL_VML_API(void,VSMODF_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[]));
_MKL_VML_API(void,VDMODF_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[]));
_mkl_vml_api(void,vsmodf_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[]));
_mkl_vml_api(void,vdmodf_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[]));
_Mkl_Vml_Api(void,vsModf_64, (const MKL_INT64 n, const float a[], float r1[], float r2[]));
_Mkl_Vml_Api(void,vdModf_64, (const MKL_INT64 n, const double a[], double r1[], double r2[]));

_MKL_VML_API(void,VMSMODF_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDMODF_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsmodf_64, (const MKL_INT64 *n, const float a[], float r1[], float r2[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdmodf_64, (const MKL_INT64 *n, const double a[], double r1[], double r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsModf_64, (const MKL_INT64 n, const float a[], float r1[], float r2[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdModf_64, (const MKL_INT64 n, const double a[], double r1[], double r2[], MKL_INT64 mode));

/* Modulus function: r[i] = fmod(a[i], b[i]) */
_MKL_VML_API(void, VSFMOD_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFMOD_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfmod_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfmod_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFmod_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFmod_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFMOD_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMOD_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmod_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmod_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmod_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmod_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Remainder function: r[i] = remainder(a[i], b[i]) */
_MKL_VML_API(void, VSREMAINDER_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDREMAINDER_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsremainder_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdremainder_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsRemainder_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdRemainder_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSREMAINDER_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDREMAINDER_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsremainder_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdremainder_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRemainder_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRemainder_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Next after function: r[i] = nextafter(a[i], b[i]) */
_MKL_VML_API(void, VSNEXTAFTER_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDNEXTAFTER_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsnextafter_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdnextafter_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsNextAfter_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdNextAfter_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSNEXTAFTER_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDNEXTAFTER_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsnextafter_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdnextafter_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsNextAfter_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdNextAfter_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Copy sign function: r[i] = copysign(a[i], b[i]) */
_MKL_VML_API(void, VSCOPYSIGN_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDCOPYSIGN_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vscopysign_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdcopysign_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsCopySign_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdCopySign_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSCOPYSIGN_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOPYSIGN_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmscopysign_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdcopysign_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCopySign_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCopySign_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Positive difference function: r[i] = fdim(a[i], b[i]) */
_MKL_VML_API(void, VSFDIM_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFDIM_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfdim_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfdim_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFdim_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFdim_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFDIM_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFDIM_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfdim_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfdim_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFdim_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFdim_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Maximum function: r[i] = fmax(a[i], b[i]) */
_MKL_VML_API(void, VSFMAX_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFMAX_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfmax_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfmax_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFmax_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFmax_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFMAX_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMAX_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmax_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmax_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmax_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmax_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Minimum function: r[i] = fmin(a[i], b[i]) */
_MKL_VML_API(void, VSFMIN_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDFMIN_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsfmin_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdfmin_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsFmin_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdFmin_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSFMIN_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMIN_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmin_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmin_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmin_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmin_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Maximum magnitude function: r[i] = maxmag(a[i], b[i]) */
_MKL_VML_API(void, VSMAXMAG_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDMAXMAG_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsmaxmag_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdmaxmag_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsMaxMag_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdMaxMag_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSMAXMAG_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDMAXMAG_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsmaxmag_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdmaxmag_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMaxMag_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMaxMag_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Minimum magnitude function: r[i] = minmag(a[i], b[i]) */
_MKL_VML_API(void, VSMINMAG_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_MKL_VML_API(void, VDMINMAG_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_mkl_vml_api(void, vsminmag_64, (const MKL_INT64 *n, const float a[], const float b[], float r[]));
_mkl_vml_api(void, vdminmag_64, (const MKL_INT64 *n, const double a[], const double b[], double r[]));
_Mkl_Vml_Api(void, vsMinMag_64, (const MKL_INT64 n, const float a[], const float b[], float r[]));
_Mkl_Vml_Api(void, vdMinMag_64, (const MKL_INT64 n, const double a[], const double b[], double r[]));

_MKL_VML_API(void, VMSMINMAG_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDMINMAG_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsminmag_64, (const MKL_INT64 *n, const float a[], const float b[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdminmag_64, (const MKL_INT64 *n, const double a[], const double b[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMinMag_64, (const MKL_INT64 n, const float a[], const float b[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMinMag_64, (const MKL_INT64 n, const double a[], const double b[], double r[], MKL_INT64 mode));

/* Rounded integer value in the current rounding mode: r[i] = nearbyint(a[i]) */
_MKL_VML_API(void,VSNEARBYINT_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDNEARBYINT_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsnearbyint_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdnearbyint_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsNearbyInt_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdNearbyInt_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSNEARBYINT_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDNEARBYINT_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsnearbyint_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdnearbyint_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsNearbyInt_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdNearbyInt_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Rounded integer value in the current rounding mode with inexact result exception raised for rach changed value: r[i] = rint(a[i]) */
_MKL_VML_API(void,VSRINT_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDRINT_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsrint_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdrint_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsRint_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdRint_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSRINT_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDRINT_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsrint_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdrint_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsRint_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdRint_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Value rounded to the nearest integer: r[i] = round(a[i]) */
_MKL_VML_API(void,VSROUND_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDROUND_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vsround_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdround_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsRound_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdRound_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSROUND_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDROUND_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmsround_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdround_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsRound_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdRound_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Integer value rounded towards zero: r[i] = trunc(a[i]) */
_MKL_VML_API(void,VSTRUNC_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void,VDTRUNC_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void,vstrunc_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void,vdtrunc_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void,vsTrunc_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void,vdTrunc_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void,VMSTRUNC_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMDTRUNC_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmstrunc_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmdtrunc_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmsTrunc_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmdTrunc_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/* Element by element conjugation: r[i] = conj(a[i]) */
_MKL_VML_API(void,VCCONJ_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCONJ_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcconj_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzconj_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcConj_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzConj_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCONJ_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCONJ_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcconj_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzconj_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcConj_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzConj_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Element by element multiplication of vector A element and conjugated vector B element: r[i] = mulbyconj(a[i],b[i]) */
_MKL_VML_API(void,VCMULBYCONJ_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZMULBYCONJ_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_mkl_vml_api(void,vcmulbyconj_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzmulbyconj_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcMulByConj_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzMulByConj_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCMULBYCONJ_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZMULBYCONJ_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmcmulbyconj_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzmulbyconj_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcMulByConj_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_Complex8 b[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzMulByConj_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_Complex16 b[], MKL_Complex16 r[], MKL_INT64 mode));

/* Complex exponent of real vector elements: r[i] = CIS(a[i]) */
_MKL_VML_API(void,VCCIS_64, (const MKL_INT64 *n, const float a[], MKL_Complex8 r[]));
_MKL_VML_API(void,VZCIS_64, (const MKL_INT64 *n, const double a[], MKL_Complex16 r[]));
_mkl_vml_api(void,vccis_64, (const MKL_INT64 *n, const float a[], MKL_Complex8 r[]));
_mkl_vml_api(void,vzcis_64, (const MKL_INT64 *n, const double a[], MKL_Complex16 r[]));
_Mkl_Vml_Api(void,vcCIS_64, (const MKL_INT64 n, const float a[], MKL_Complex8 r[]));
_Mkl_Vml_Api(void,vzCIS_64, (const MKL_INT64 n, const double a[], MKL_Complex16 r[]));

_MKL_VML_API(void,VMCCIS_64, (const MKL_INT64 *n, const float a[], MKL_Complex8 r[], MKL_INT64 *mode));
_MKL_VML_API(void,VMZCIS_64, (const MKL_INT64 *n, const double a[], MKL_Complex16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmccis_64, (const MKL_INT64 *n, const float a[], MKL_Complex8 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmzcis_64, (const MKL_INT64 *n, const double a[], MKL_Complex16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmcCIS_64, (const MKL_INT64 n, const float a[], MKL_Complex8 r[], MKL_INT64 mode));
_Mkl_Vml_Api(void,vmzCIS_64, (const MKL_INT64 n, const double a[], MKL_Complex16 r[], MKL_INT64 mode));

/* Exponential integral of real vector elements: r[i] = E1(a[i]) */
_MKL_VML_API(void, VSEXPINT1_64, (const MKL_INT64 *n, const float a[], float r[]));
_MKL_VML_API(void, VDEXPINT1_64, (const MKL_INT64 *n, const double a[], double r[]));
_mkl_vml_api(void, vsexpint1_64, (const MKL_INT64 *n, const float a[], float r[]));
_mkl_vml_api(void, vdexpint1_64, (const MKL_INT64 *n, const double a[], double r[]));
_Mkl_Vml_Api(void, vsExpInt1_64, (const MKL_INT64 n, const float a[], float r[]));
_Mkl_Vml_Api(void, vdExpInt1_64, (const MKL_INT64 n, const double a[], double r[]));

_MKL_VML_API(void, VMSEXPINT1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPINT1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpint1_64, (const MKL_INT64 *n, const float a[], float r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpint1_64, (const MKL_INT64 *n, const double a[], double r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpInt1_64, (const MKL_INT64 n, const float a[], float r[], MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpInt1_64, (const MKL_INT64 n, const double a[], double r[], MKL_INT64 mode));

/*
//++
// VML ELEMENTARY FUNCTION DECLARATIONS: API WITH STRIDES
//--
*/
/* Absolute value: r[i] = |a[i]| */
_MKL_VML_API(void, VSABSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDABSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsabsi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdabsi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAbsI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAbsI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSABSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr,MKL_INT64 *mode));
_MKL_VML_API(void, VMDABSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr,MKL_INT64 *mode));
_mkl_vml_api(void, vmsabsi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr,MKL_INT64 *mode));
_mkl_vml_api(void, vmdabsi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr,MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAbsI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAbsI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex absolute value: r[i] = |a[i]| */
_MKL_VML_API(void, VCABSI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VZABSI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vcabsi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vzabsi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAbsI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAbsI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMCABSI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZABSI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcabsi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzabsi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAbsI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAbsI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Argument of complex value: r[i] = carg(a[i]) */
_MKL_VML_API(void, VCARGI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VZARGI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vcargi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vzargi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcArgI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzArgI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMCARGI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZARGI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcargi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzargi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcArgI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzArgI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void, VSADDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDADDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsaddi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdaddi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAddI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAddI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSADDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDADDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsaddi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdaddi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAddI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAddI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void, VCADDI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 [], const MKL_INT *));
_MKL_VML_API(void, VZADDI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 [], const MKL_INT *));
_mkl_vml_api(void, vcaddi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 [], const MKL_INT *));
_mkl_vml_api(void, vzaddi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 [], const MKL_INT *));
_Mkl_Vml_Api(void, vcAddI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 [], const MKL_INT));
_Mkl_Vml_Api(void, vzAddI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 [], const MKL_INT));

_MKL_VML_API(void, VMCADDI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZADDI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcaddi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzaddi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAddI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAddI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void, VSSUBI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSUBI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssubi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsubi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSubI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSubI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSUBI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSUBI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssubi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsubi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSubI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSubI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void, VCSUBI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZSUBI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcsubi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzsubi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcSubI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzSubI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCSUBI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSUBI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsubi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsubi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSubI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSubI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Reciprocal: r[i] = 1.0 / a[i] */
_MKL_VML_API(void, VSINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsinvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdinvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsinvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdinvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void, VSSQRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSQRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssqrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsqrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSqrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSqrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSQRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSQRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssqrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsqrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSqrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSqrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void, VCSQRTI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZSQRTI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcsqrti, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzsqrti, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcSqrtI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzSqrtI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCSQRTI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSQRTI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsqrti, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsqrti, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSqrtI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSqrtI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Reciprocal square root: r[i] = 1/a[i]^0.5 */
_MKL_VML_API(void, VSINVSQRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDINVSQRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsinvsqrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdinvsqrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsInvSqrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdInvSqrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSINVSQRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDINVSQRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsinvsqrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdinvsqrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsInvSqrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdInvSqrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Cube root: r[i] = a[i]^(1/3) */
_MKL_VML_API(void, VSCBRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCBRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscbrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcbrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCbrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCbrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCBRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCBRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscbrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcbrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCbrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCbrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Reciprocal cube root: r[i] = 1/a[i]^(1/3) */
_MKL_VML_API(void, VSINVCBRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDINVCBRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsinvcbrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdinvcbrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsInvCbrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdInvCbrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSINVCBRTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDINVCBRTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsinvcbrti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdinvcbrti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsInvCbrtI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdInvCbrtI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Squaring: r[i] = a[i]^2 */
_MKL_VML_API(void, VSSQRI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSQRI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssqri, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsqri, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSqrI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSqrI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSQRI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSQRI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssqri, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsqri, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSqrI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSqrI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Exponential function: r[i] = e^a[i] */
_MKL_VML_API(void, VSEXPI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDEXPI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsexpi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdexpi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsExpI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdExpI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSEXPI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Exponential function (base 2): r[i] = 2^a[i] */
_MKL_VML_API(void, VSEXP2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDEXP2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsexp2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdexp2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsExp2I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdExp2I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSEXP2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp2I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp2I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Exponential function (base 10): r[i] = 10^a[i] */
_MKL_VML_API(void, VSEXP10I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDEXP10I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsexp10i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdexp10i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsExp10I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdExp10I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSEXP10I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP10I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp10i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp10i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp10I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp10I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Exponential of arguments decreased by 1: r[i] = e^(a[i]-1) */
_MKL_VML_API(void, VSEXPM1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDEXPM1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsexpm1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdexpm1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsExpm1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdExpm1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSEXPM1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPM1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpm1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpm1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpm1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpm1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex exponential function: r[i] = e^a[i] */
_MKL_VML_API(void, VCEXPI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZEXPI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcexpi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzexpi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcExpI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzExpI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCEXPI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZEXPI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcexpi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzexpi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcExpI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzExpI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void, VSLNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslni, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlni, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLnI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLnI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslni, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlni, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLnI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLnI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void, VCLNI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZLNI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vclni, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzlni, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcLnI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzLnI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCLNI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZLNI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmclni, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzlni, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcLnI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzLnI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void, VSLOG10I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLOG10I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslog10i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlog10i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLog10I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLog10I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLOG10I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG10I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslog10i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog10i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog10I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog10I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void, VCLOG10I, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZLOG10I, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vclog10i, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzlog10i, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcLog10I, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzLog10I, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCLOG10I, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZLOG10I, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmclog10i, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzlog10i, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcLog10I, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzLog10I, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Logarithm (base 2): r[i] = log2(a[i]) */
_MKL_VML_API(void, VSLOG2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLOG2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslog2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlog2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLog2I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLog2I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLOG2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslog2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog2I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog2I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Logarithm (base e) of arguments increased by 1: r[i] = log(1+a[i]) */
_MKL_VML_API(void, VSLOG1PI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLOG1PI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslog1pi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlog1pi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLog1pI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLog1pI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLOG1PI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG1PI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslog1pi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog1pi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog1pI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog1pI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Computes the exponent: r[i] = logb(a[i]) */
_MKL_VML_API(void, VSLOGBI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLOGBI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslogbi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlogbi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLogbI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLogbI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLOGBI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOGBI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslogbi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlogbi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLogbI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLogbI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Cosine: r[i] = cos(a[i]) */
_MKL_VML_API(void, VSCOSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCOSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscosi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcosi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCosI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCosI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCOSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscosi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcosi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCosI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCosI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex cosine: r[i] = ccos(a[i]) */
_MKL_VML_API(void, VCCOSI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZCOSI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vccosi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzcosi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcCosI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzCosI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCCOSI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCOSI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmccosi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzcosi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcCosI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzCosI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Sine: r[i] = sin(a[i]) */
_MKL_VML_API(void, VSSINI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSINI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssini, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsini, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSinI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSinI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSINI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssini, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsini, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex sine: r[i] = sin(a[i]) */
_MKL_VML_API(void, VCSINI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZSINI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcsini, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzsini, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcSinI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzSinI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCSINI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSINI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsini, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsini, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSinI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSinI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void, VSTANI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDTANI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vstani, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdtani, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsTanI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdTanI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSTANI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstani, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtani, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void, VCTANI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZTANI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vctani, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vztani, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcTanI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzTanI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCTANI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZTANI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmctani, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmztani, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcTanI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzTanI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void, VSCOSHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCOSHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscoshi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcoshi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCoshI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCoshI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCOSHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscoshi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcoshi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCoshI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCoshI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void, VCCOSHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZCOSHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vccoshi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzcoshi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcCoshI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzCoshI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCCOSHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCOSHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmccoshi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzcoshi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcCoshI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzCoshI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Cosine degree: r[i] = cos(a[i]*PI/180) */
_MKL_VML_API(void, VSCOSDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCOSDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscosdi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcosdi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCosdI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCosdI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCOSDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscosdi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcosdi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCosdI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCosdI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Cosine PI: r[i] = cos(a[i]*PI) */
_MKL_VML_API(void, VSCOSPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCOSPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscospii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcospii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCospiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCospiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCOSPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscospii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcospii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCospiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCospiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void, VSSINHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSINHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssinhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsinhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSinhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSinhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSINHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssinhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsinhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void, VCSINHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZSINHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcsinhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzsinhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcSinhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzSinhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCSINHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSINHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsinhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsinhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSinhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSinhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Sine degree: r[i] = sin(a[i]*PI/180) */
_MKL_VML_API(void, VSSINDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSINDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssindi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsindi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSindI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSindI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSINDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssindi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsindi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSindI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSindI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Sine PI: r[i] = sin(a[i]*PI) */
_MKL_VML_API(void, VSSINPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDSINPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vssinpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdsinpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsSinpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdSinpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSSINPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssinpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsinpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void, VSTANHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDTANHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vstanhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdtanhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsTanhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdTanhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSTANHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstanhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtanhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void, VCTANHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZTANHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vctanhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vztanhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcTanhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzTanhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCTANHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZTANHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmctanhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmztanhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcTanhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzTanhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Tangent degree: r[i] = tan(a[i]*PI/180) */
_MKL_VML_API(void, VSTANDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDTANDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vstandi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdtandi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsTandI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdTandI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSTANDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstandi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtandi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTandI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTandI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Tangent PI: r[i] = tan(a[i]*PI) */
_MKL_VML_API(void, VSTANPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDTANPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vstanpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdtanpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsTanpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdTanpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSTANPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstanpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtanpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void, VSACOSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDACOSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsacosi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdacosi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAcosI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAcosI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSACOSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsacosi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdacosi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcosI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcosI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void, VCACOSI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZACOSI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcacosi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzacosi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAcosI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAcosI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCACOSI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZACOSI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcacosi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzacosi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAcosI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAcosI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc cosine PI: r[i] = arccos(a[i])/PI */
_MKL_VML_API(void, VSACOSPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDACOSPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsacospii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdacospii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAcospiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAcospiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSACOSPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsacospii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdacospii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcospiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcospiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void, VSASINI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDASINI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsasini, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdasini, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAsinI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAsinI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSASINI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsasini, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdasini, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void, VCASINI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZASINI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcasini, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzasini, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAsinI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAsinI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCASINI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZASINI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcasini, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzasini, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAsinI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAsinI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc sine PI: r[i] = arcsin(a[i])/PI */
_MKL_VML_API(void, VSASINPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDASINPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsasinpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdasinpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAsinpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAsinpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSASINPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsasinpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdasinpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void, VSATANI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDATANI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsatani, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdatani, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAtanI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAtanI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSATANI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatani, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatani, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void, VCATANI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZATANI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcatani, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzatani, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAtanI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAtanI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCATANI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZATANI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcatani, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzatani, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAtanI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAtanI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc tangent PI: r[i] = arctan(a[i])/PI */
_MKL_VML_API(void, VSATANPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDATANPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsatanpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdatanpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAtanpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAtanpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSATANPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatanpii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatanpii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanpiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanpiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void, VSACOSHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDACOSHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsacoshi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdacoshi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAcoshI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAcoshI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSACOSHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsacoshi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdacoshi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcoshI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcoshI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void, VCACOSHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZACOSHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcacoshi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzacoshi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAcoshI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAcoshI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCACOSHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZACOSHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcacoshi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzacoshi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAcoshI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAcoshI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void, VSASINHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDASINHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsasinhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdasinhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAsinhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAsinhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSASINHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsasinhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdasinhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void, VCASINHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZASINHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcasinhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzasinhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAsinhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAsinhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCASINHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZASINHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcasinhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzasinhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAsinhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAsinhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void, VSATANHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDATANHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsatanhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdatanhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAtanhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAtanhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSATANHI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANHI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatanhi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatanhi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanhI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanhI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void, VCATANHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZATANHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcatanhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzatanhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcAtanhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzAtanhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCATANHI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZATANHI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcatanhi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzatanhi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAtanhI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAtanhI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Error function: r[i] = erf(a[i]) */
_MKL_VML_API(void, VSERFI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDERFI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vserfi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vderfi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsErfI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdErfI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSERFI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Inverse error function: r[i] = erfinv(a[i]) */
_MKL_VML_API(void, VSERFINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDERFINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vserfinvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vderfinvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsErfInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdErfInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSERFINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfinvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfinvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Square root of the sum of the squares: r[i] = hypot(a[i],b[i]) */
_MKL_VML_API(void, VSHYPOTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDHYPOTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vshypoti, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdhypoti, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsHypotI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdHypotI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSHYPOTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDHYPOTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmshypoti, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdhypoti, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsHypotI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdHypotI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complementary error function: r[i] = 1 - erf(a[i]) */
_MKL_VML_API(void, VSERFCI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDERFCI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vserfci, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vderfci, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsErfcI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdErfcI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSERFCI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFCI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfci, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfci, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfcI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfcI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Inverse complementary error function: r[i] = erfcinv(a[i]) */
_MKL_VML_API(void, VSERFCINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDERFCINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vserfcinvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vderfcinvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsErfcInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdErfcInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSERFCINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFCINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfcinvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfcinvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfcInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfcInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Scaled complementary error function: r[i] = erfcx(a[i]) */
_MKL_VML_API(void, VSERFCXI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDERFCXI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vserfcxi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vderfcxi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsErfcxI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdErfcxI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSERFCXI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFCXI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfcxi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfcxi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfcxI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfcxI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Cumulative normal distribution function: r[i] = cdfnorm(a[i]) */
_MKL_VML_API(void, VSCDFNORMI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCDFNORMI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscdfnormi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcdfnormi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCdfNormI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCdfNormI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCDFNORMI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCDFNORMI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscdfnormi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcdfnormi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCdfNormI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCdfNormI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Inverse cumulative normal distribution function: r[i] = cdfnorminv(a[i]) */
_MKL_VML_API(void, VSCDFNORMINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCDFNORMINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscdfnorminvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcdfnorminvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCdfNormInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCdfNormInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCDFNORMINVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCDFNORMINVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscdfnorminvi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcdfnorminvi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCdfNormInvI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCdfNormInvI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Logarithm (base e) of the absolute value of gamma function: r[i] = lgamma(a[i]) */
_MKL_VML_API(void, VSLGAMMAI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLGAMMAI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslgammai, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlgammai, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLGammaI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLGammaI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLGAMMAI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLGAMMAI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslgammai, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlgammai, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLGammaI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLGammaI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Gamma function: r[i] = tgamma(a[i]) */
_MKL_VML_API(void, VSTGAMMAI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDTGAMMAI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vstgammai, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdtgammai, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsTGammaI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdTGammaI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSTGAMMAI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTGAMMAI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstgammai, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtgammai, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTGammaI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTGammaI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* I0 Bessel function: r[i] = i0(a[i]) */
_MKL_VML_API(void, VSI0I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDI0I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsi0i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdi0i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsI0I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdI0I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSI0I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDI0I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsi0i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdi0i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsI0I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdI0I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* I1 Bessel function: r[i] = i1(a[i]) */
_MKL_VML_API(void, VSI1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDI1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsi1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdi1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsI1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdI1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSI1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDI1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsi1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdi1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsI1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdI1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* J0 Bessel function: r[i] = j0(a[i]) */
_MKL_VML_API(void, VSJ0I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDJ0I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsj0i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdj0i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsJ0I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdJ0I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSJ0I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDJ0I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsj0i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdj0i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsJ0I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdJ0I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* J1 Bessel function: r[i] = j1(a[i]) */
_MKL_VML_API(void, VSJ1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDJ1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsj1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdj1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsJ1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdJ1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSJ1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDJ1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsj1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdj1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsJ1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdJ1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Y0 Bessel function: r[i] = y0(a[i]) */
_MKL_VML_API(void, VSY0I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDY0I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsy0i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdy0i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsY0I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdY0I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSY0I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDY0I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsy0i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdy0i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsY0I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdY0I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Y1 Bessel function: r[i] = y1(a[i]) */
_MKL_VML_API(void, VSY1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDY1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsy1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdy1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsY1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdY1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSY1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDY1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsy1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdy1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsY1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdY1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Jn Bessel function: r[i] = jn(a[i],n) */
_MKL_VML_API(void, VSJNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDJNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsjni, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdjni, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsJnI, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdJnI, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSJNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDJNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsjni, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdjni, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsJnI, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdJnI, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Yn Bessel function: r[i] = yn(a[i],n) */
_MKL_VML_API(void, VSYNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDYNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsyni, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdyni, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsYnI, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdYnI, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSYNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDYNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsyni, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdyni, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsYnI, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdYnI, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc tangent of a/b: r[i] = arctan(a[i]/b[i]) */
_MKL_VML_API(void, VSATAN2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDATAN2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsatan2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdatan2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAtan2I, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAtan2I, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSATAN2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATAN2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatan2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatan2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtan2I, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtan2I, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc tangent of a/b divided by PI: r[i] = arctan(a[i]/b[i])/PI */
_MKL_VML_API(void, VSATAN2PII, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDATAN2PII, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsatan2pii, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdatan2pii, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsAtan2piI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdAtan2piI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSATAN2PII, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATAN2PII, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatan2pii, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatan2pii, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtan2piI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtan2piI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Multiplicaton: r[i] = a[i] * b[i] */
_MKL_VML_API(void, VSMULI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDMULI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsmuli, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdmuli, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsMulI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdMulI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSMULI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMULI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsmuli, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdmuli, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMulI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMulI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex multiplication: r[i] = a[i] * b[i] */
_MKL_VML_API(void, VCMULI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZMULI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcmuli, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzmuli, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcMulI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzMulI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCMULI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZMULI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcmuli, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzmuli, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcMulI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzMulI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Division: r[i] = a[i] / b[i] */
_MKL_VML_API(void, VSDIVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDDIVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsdivi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vddivi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsDivI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdDivI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSDIVI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDDIVI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsdivi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmddivi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsDivI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdDivI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex division: r[i] = a[i] / b[i] */
_MKL_VML_API(void, VCDIVI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZDIVI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcdivi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzdivi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcDivI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzDivI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCDIVI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZDIVI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcdivi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzdivi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcDivI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzDivI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Positive difference function: r[i] = fdim(a[i], b[i]) */
_MKL_VML_API(void, VSFDIMI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDFDIMI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsfdimi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdfdimi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsFdimI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdFdimI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSFDIMI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFDIMI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfdimi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfdimi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFdimI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFdimI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Modulus function: r[i] = fmod(a[i], b[i]) */
_MKL_VML_API(void, VSFMODI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDFMODI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsfmodi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdfmodi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsFmodI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdFmodI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSFMODI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMODI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmodi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmodi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmodI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmodI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Maximum function: r[i] = fmax(a[i], b[i]) */
_MKL_VML_API(void, VSFMAXI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDFMAXI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsfmaxi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdfmaxi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsFmaxI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdFmaxI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSFMAXI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMAXI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmaxi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmaxi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmaxI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmaxI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Minimum function: r[i] = fmin(a[i], b[i]) */
_MKL_VML_API(void, VSFMINI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDFMINI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsfmini, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdfmini, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsFminI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdFminI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSFMINI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMINI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmini, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmini, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFminI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFminI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VSPOWI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDPOWI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vspowi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdpowi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsPowI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdPowI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSPOWI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspowi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VCPOWI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZPOWI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcpowi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzpowi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcPowI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzPowI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCPOWI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZPOWI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcpowi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzpowi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcPowI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzPowI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Power function with a[i]>=0: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VSPOWRI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDPOWRI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vspowri, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdpowri, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsPowrI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdPowrI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSPOWRI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWRI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspowri, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowri, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowrI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowrI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Power function: r[i] = a[i]^(3/2) */
_MKL_VML_API(void, VSPOW3O2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDPOW3O2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vspow3o2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdpow3o2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsPow3o2I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdPow3o2I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSPOW3O2I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOW3O2I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspow3o2i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpow3o2i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPow3o2I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPow3o2I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Power function: r[i] = a[i]^(2/3) */
_MKL_VML_API(void, VSPOW2O3I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDPOW2O3I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vspow2o3i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdpow2o3i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsPow2o3I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdPow2o3I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSPOW2O3I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOW2O3I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspow2o3i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpow2o3i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPow2o3I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPow2o3I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void, VSPOWXI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDPOWXI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vspowxi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdpowxi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsPowxI, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdPowxI, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSPOWXI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWXI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspowxi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowxi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowxI, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowxI, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void, VCPOWXI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZPOWXI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcpowxi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzpowxi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcPowxI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzPowxI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCPOWXI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZPOWXI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcpowxi, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzpowxi, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcPowxI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzPowxI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Sine & cosine: r1[i] = sin(a[i]), r2[i]=cos(a[i]) */
_MKL_VML_API(void, VSSINCOSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2));
_MKL_VML_API(void, VDSINCOSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vssincosi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vdsincosi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2));
_Mkl_Vml_Api(void, vsSinCosI, (const MKL_INT n, const float a[], const MKL_INT inca, float r1[], const MKL_INT incr1, float r2[], const MKL_INT incr2));
_Mkl_Vml_Api(void, vdSinCosI, (const MKL_INT n, const double a[], const MKL_INT inca, double r1[], const MKL_INT incr1, double r2[], const MKL_INT incr2));

_MKL_VML_API(void, VMSSINCOSI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINCOSI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmssincosi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsincosi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinCosI, (const MKL_INT n, const float a[], const MKL_INT inca, float r1[], const MKL_INT incr1, float r2[], const MKL_INT incr2, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinCosI, (const MKL_INT n, const double a[], const MKL_INT inca, double r1[], const MKL_INT incr1, double r2[], const MKL_INT incr2, MKL_INT64 mode));

/* Sine & cosine PI: r1[i] = sinpi(a[i]), r2[i]=cospi(a[i]) */
_MKL_VML_API(void, VSSINCOSPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2));
_MKL_VML_API(void, VDSINCOSPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vssincospii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vdsincospii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2));
_Mkl_Vml_Api(void, vsSinCospiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r1[], const MKL_INT incr1, float r2[], const MKL_INT incr2));
_Mkl_Vml_Api(void, vdSinCospiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r1[], const MKL_INT incr1, double r2[], const MKL_INT incr2));

_MKL_VML_API(void, VMSSINCOSPII, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINCOSPII, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmssincospii, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsincospii, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinCospiI, (const MKL_INT n, const float a[], const MKL_INT inca, float r1[], const MKL_INT incr1, float r2[], const MKL_INT incr2, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinCospiI, (const MKL_INT n, const double a[], const MKL_INT inca, double r1[], const MKL_INT incr1, double r2[], const MKL_INT incr2, MKL_INT64 mode));

/* Linear fraction: r[i] = (a[i]*scalea + shifta)/(b[i]*scaleb + shiftb) */
_MKL_VML_API(void, VSLINEARFRACI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDLINEARFRACI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vslinearfraci, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdlinearfraci, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsLinearFracI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, const float scalea, const float shifta, const float scaleb, const float shiftb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdLinearFracI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, const double scalea, const double shifta, const double scaleb, const double shiftb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSLINEARFRACI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLINEARFRACI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslinearfraci, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlinearfraci, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLinearFracI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, const float scalea, const float shifta, const float scaleb, const float shiftb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLinearFracI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, const double scalea, const double shifta, const double scaleb, const double shiftb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Integer value rounded towards plus infinity: r[i] = ceil(a[i]) */
_MKL_VML_API(void, VSCEILI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCEILI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsceili, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdceili, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCeilI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCeilI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCEILI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCEILI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsceili, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdceili, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCeilI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCeilI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Integer value rounded towards minus infinity: r[i] = floor(a[i]) */
_MKL_VML_API(void, VSFLOORI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDFLOORI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsfloori, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdfloori, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsFloorI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdFloorI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSFLOORI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFLOORI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfloori, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfloori, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFloorI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFloorI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Signed fraction part */
_MKL_VML_API(void, VSFRACI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDFRACI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsfraci, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdfraci, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsFracI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdFracI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSFRACI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFRACI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfraci, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfraci, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFracI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFracI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Truncated integer value and the remaining fraction part */
_MKL_VML_API(void, VSMODFI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2));
_MKL_VML_API(void, VDMODFI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vsmodfi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vdmodfi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2));
_Mkl_Vml_Api(void, vsModfI, (const MKL_INT n, const float a[], const MKL_INT inca, float r1[], const MKL_INT incr1, float r2[], const MKL_INT incr2));
_Mkl_Vml_Api(void, vdModfI, (const MKL_INT n, const double a[], const MKL_INT inca, double r1[], const MKL_INT incr1, double r2[], const MKL_INT incr2));

_MKL_VML_API(void, VMSMODFI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMODFI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmsmodfi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r1[], const MKL_INT *incr1, float r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmdmodfi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r1[], const MKL_INT *incr1, double r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsModfI, (const MKL_INT n, const float a[], const MKL_INT inca, float r1[], const MKL_INT incr1, float r2[], const MKL_INT incr2, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdModfI, (const MKL_INT n, const double a[], const MKL_INT inca, double r1[], const MKL_INT incr1, double r2[], const MKL_INT incr2, MKL_INT64 mode));

/* Rounded integer value in the current rounding mode: r[i] = nearbyint(a[i]) */
_MKL_VML_API(void, VSNEARBYINTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDNEARBYINTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsnearbyinti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdnearbyinti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsNearbyIntI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdNearbyIntI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSNEARBYINTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDNEARBYINTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsnearbyinti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdnearbyinti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsNearbyIntI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdNearbyIntI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Next after function: r[i] = nextafter(a[i], b[i]) */
_MKL_VML_API(void, VSNEXTAFTERI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDNEXTAFTERI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsnextafteri, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdnextafteri, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsNextAfterI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdNextAfterI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSNEXTAFTERI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDNEXTAFTERI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsnextafteri, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdnextafteri, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsNextAfterI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdNextAfterI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Minimum magnitude function: r[i] = minmag(a[i], b[i]) */
_MKL_VML_API(void, VSMINMAGI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDMINMAGI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsminmagi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdminmagi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsMinMagI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdMinMagI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSMINMAGI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMINMAGI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsminmagi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdminmagi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMinMagI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMinMagI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Maximum magnitude function: r[i] = maxmag(a[i], b[i]) */
_MKL_VML_API(void, VSMAXMAGI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDMAXMAGI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsmaxmagi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdmaxmagi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsMaxMagI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdMaxMagI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSMAXMAGI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMAXMAGI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsmaxmagi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdmaxmagi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMaxMagI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMaxMagI, (const MKL_INT n, const double a[], const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Rounded integer value in the current rounding mode with inexact result exception raised for rach changed value: r[i] = rint(a[i]) */
_MKL_VML_API(void, VSRINTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDRINTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsrinti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdrinti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsRintI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdRintI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSRINTI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDRINTI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsrinti, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdrinti, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRintI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRintI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Value rounded to the nearest integer: r[i] = round(a[i]) */
_MKL_VML_API(void, VSROUNDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDROUNDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsroundi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdroundi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsRoundI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdRoundI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSROUNDI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDROUNDI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsroundi, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdroundi, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRoundI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRoundI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Integer value rounded towards zero: r[i] = trunc(a[i]) */
_MKL_VML_API(void, VSTRUNCI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDTRUNCI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vstrunci, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdtrunci, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsTruncI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdTruncI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSTRUNCI, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTRUNCI, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstrunci, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtrunci, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTruncI, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTruncI, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Element by element conjugation: r[i] = conj(a[i]) */
_MKL_VML_API(void, VCCONJI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZCONJI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcconji, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzconji, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcConjI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzConjI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCCONJI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCONJI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcconji, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzconji, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcConjI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzConjI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Element by element multiplication of vector A element and conjugated vector B element: r[i] = mulbyconj(a[i],b[i]) */
_MKL_VML_API(void, VCMULBYCONJI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZMULBYCONJI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vcmulbyconji, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzmulbyconji, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcMulByConjI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzMulByConjI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCMULBYCONJI, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZMULBYCONJI, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcmulbyconji, (const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT *inca, const MKL_Complex8 b[], const MKL_INT *incb, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzmulbyconji, (const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT *inca, const MKL_Complex16 b[], const MKL_INT *incb, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcMulByConjI, (const MKL_INT n, const MKL_Complex8 a[], const MKL_INT inca, const MKL_Complex8 b[], const MKL_INT incb, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzMulByConjI, (const MKL_INT n, const MKL_Complex16 a[], const MKL_INT inca, const MKL_Complex16 b[], const MKL_INT incb, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Complex exponent of real vector elements: r[i] = CIS(a[i]) */
_MKL_VML_API(void, VCCISI, (const MKL_INT *n, const float a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_MKL_VML_API(void, VZCISI, (const MKL_INT *n, const double a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vccisi, (const MKL_INT *n, const float a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr));
_mkl_vml_api(void, vzcisi, (const MKL_INT *n, const double a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vcCISI, (const MKL_INT n, const float a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vzCISI, (const MKL_INT n, const double a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMCCISI, (const MKL_INT *n, const float a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCISI, (const MKL_INT *n, const double a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmccisi, (const MKL_INT *n, const float a[], const MKL_INT *inca, MKL_Complex8 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzcisi, (const MKL_INT *n, const double a[], const MKL_INT *inca, MKL_Complex16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcCISI, (const MKL_INT n, const float a[], const MKL_INT inca, MKL_Complex8 r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzCISI, (const MKL_INT n, const double a[], const MKL_INT inca, MKL_Complex16 r[], const MKL_INT incr, MKL_INT64 mode));

/* Exponential integral of real vector elements: r[i] = E1(a[i]) */
_MKL_VML_API(void, VSEXPINT1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDEXPINT1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsexpint1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdexpint1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsExpInt1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdExpInt1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSEXPINT1I, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPINT1I, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpint1i, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpint1i, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpInt1I, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpInt1I, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Copy sign function: r[i] = copysign(a[i], b[i]) */
_MKL_VML_API(void, VSCOPYSIGNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDCOPYSIGNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vscopysigni, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdcopysigni, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsCopySignI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdCopySignI, (const MKL_INT n, const double a[],const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSCOPYSIGNI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOPYSIGNI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscopysigni, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcopysigni, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCopySignI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCopySignI, (const MKL_INT n, const double a[],const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Remainder function: r[i] = remainder(a[i], b[i]) */
_MKL_VML_API(void, VSREMAINDERI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDREMAINDERI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsremainderi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdremainderi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsRemainderI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdRemainderI, (const MKL_INT n, const double a[],const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSREMAINDERI, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDREMAINDERI, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsremainderi, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float b[], const MKL_INT *incb, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdremainderi, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double b[], const MKL_INT *incb, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRemainderI, (const MKL_INT n, const float a[], const MKL_INT inca, const float b[], const MKL_INT incb, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRemainderI, (const MKL_INT n, const double a[],const MKL_INT inca, const double b[], const MKL_INT incb, double r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VSABSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDABSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsabsi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdabsi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAbsI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAbsI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSABSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr,MKL_INT64 *mode));
_MKL_VML_API(void, VMDABSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr,MKL_INT64 *mode));
_mkl_vml_api(void, vmsabsi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr,MKL_INT64 *mode));
_mkl_vml_api(void, vmdabsi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr,MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAbsI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAbsI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex absolute value: r[i] = |a[i]| */
_MKL_VML_API(void, VCABSI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZABSI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcabsi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzabsi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAbsI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAbsI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCABSI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZABSI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcabsi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzabsi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAbsI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAbsI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Argument of complex value: r[i] = carg(a[i]) */
_MKL_VML_API(void, VCARGI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZARGI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcargi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzargi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcArgI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzArgI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCARGI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZARGI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcargi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzargi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcArgI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzArgI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void, VSADDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDADDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsaddi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdaddi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAddI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAddI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSADDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDADDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsaddi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdaddi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAddI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAddI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex addition: r[i] = a[i] + b[i] */
_MKL_VML_API(void, VCADDI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 [], const MKL_INT64 *));
_MKL_VML_API(void, VZADDI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 [], const MKL_INT64 *));
_mkl_vml_api(void, vcaddi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 [], const MKL_INT64 *));
_mkl_vml_api(void, vzaddi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 [], const MKL_INT64 *));
_Mkl_Vml_Api(void, vcAddI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 [], const MKL_INT64));
_Mkl_Vml_Api(void, vzAddI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 [], const MKL_INT64));

_MKL_VML_API(void, VMCADDI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZADDI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcaddi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzaddi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAddI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAddI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void, VSSUBI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSUBI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssubi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsubi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSubI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSubI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSUBI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSUBI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssubi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsubi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSubI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSubI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex subtraction: r[i] = a[i] - b[i] */
_MKL_VML_API(void, VCSUBI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZSUBI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcsubi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzsubi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcSubI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzSubI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCSUBI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSUBI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsubi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsubi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSubI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSubI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Reciprocal: r[i] = 1.0 / a[i] */
_MKL_VML_API(void, VSINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsinvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdinvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsinvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdinvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void, VSSQRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSQRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssqrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsqrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSqrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSqrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSQRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSQRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssqrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsqrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSqrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSqrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex square root: r[i] = a[i]^0.5 */
_MKL_VML_API(void, VCSQRTI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZSQRTI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcsqrti_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzsqrti_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcSqrtI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzSqrtI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCSQRTI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSQRTI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsqrti_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsqrti_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSqrtI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSqrtI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Reciprocal square root: r[i] = 1/a[i]^0.5 */
_MKL_VML_API(void, VSINVSQRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDINVSQRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsinvsqrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdinvsqrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsInvSqrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdInvSqrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSINVSQRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDINVSQRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsinvsqrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdinvsqrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsInvSqrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdInvSqrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Cube root: r[i] = a[i]^(1/3) */
_MKL_VML_API(void, VSCBRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCBRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscbrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcbrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCbrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCbrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCBRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCBRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscbrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcbrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCbrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCbrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Reciprocal cube root: r[i] = 1/a[i]^(1/3) */
_MKL_VML_API(void, VSINVCBRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDINVCBRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsinvcbrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdinvcbrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsInvCbrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdInvCbrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSINVCBRTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDINVCBRTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsinvcbrti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdinvcbrti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsInvCbrtI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdInvCbrtI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Squaring: r[i] = a[i]^2 */
_MKL_VML_API(void, VSSQRI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSQRI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssqri_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsqri_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSqrI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSqrI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSQRI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSQRI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssqri_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsqri_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSqrI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSqrI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Exponential function: r[i] = e^a[i] */
_MKL_VML_API(void, VSEXPI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDEXPI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsexpi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdexpi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsExpI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdExpI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSEXPI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Exponential function (base 2): r[i] = 2^a[i] */
_MKL_VML_API(void, VSEXP2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDEXP2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsexp2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdexp2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsExp2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdExp2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSEXP2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Exponential function (base 10): r[i] = 10^a[i] */
_MKL_VML_API(void, VSEXP10I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDEXP10I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsexp10i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdexp10i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsExp10I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdExp10I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSEXP10I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXP10I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexp10i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexp10i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExp10I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExp10I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Exponential of arguments decreased by 1: r[i] = e^(a[i]-1) */
_MKL_VML_API(void, VSEXPM1I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDEXPM1I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsexpm1i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdexpm1i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsExpm1I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdExpm1I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSEXPM1I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPM1I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpm1i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpm1i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpm1I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpm1I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex exponential function: r[i] = e^a[i] */
_MKL_VML_API(void, VCEXPI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZEXPI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcexpi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzexpi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcExpI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzExpI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCEXPI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZEXPI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcexpi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzexpi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcExpI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzExpI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void, VSLNI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLNI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslni_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlni_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLnI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLnI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLNI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLNI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslni_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlni_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLnI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLnI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex logarithm (base e): r[i] = ln(a[i]) */
_MKL_VML_API(void, VCLNI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZLNI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vclni_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzlni_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcLnI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzLnI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCLNI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZLNI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmclni_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzlni_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcLnI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzLnI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void, VSLOG10I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLOG10I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslog10i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlog10i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLog10I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLog10I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLOG10I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG10I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslog10i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog10i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog10I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog10I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex logarithm (base 10): r[i] = lg(a[i]) */
_MKL_VML_API(void, VCLOG10I_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZLOG10I_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vclog10i_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzlog10i_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcLog10I_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzLog10I_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCLOG10I_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZLOG10I_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmclog10i_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzlog10i_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcLog10I_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzLog10I_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Logarithm (base 2): r[i] = log2(a[i]) */
_MKL_VML_API(void, VSLOG2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLOG2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslog2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlog2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLog2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLog2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLOG2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslog2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Logarithm (base e) of arguments increased by 1: r[i] = log(1+a[i]) */
_MKL_VML_API(void, VSLOG1PI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLOG1PI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslog1pi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlog1pi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLog1pI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLog1pI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLOG1PI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOG1PI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslog1pi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlog1pi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLog1pI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLog1pI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Computes the exponent: r[i] = logb(a[i]) */
_MKL_VML_API(void, VSLOGBI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLOGBI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslogbi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlogbi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLogbI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLogbI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLOGBI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLOGBI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslogbi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlogbi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLogbI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLogbI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Cosine: r[i] = cos(a[i]) */
_MKL_VML_API(void, VSCOSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCOSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscosi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcosi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCosI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCosI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCOSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscosi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcosi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCosI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCosI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex cosine: r[i] = ccos(a[i]) */
_MKL_VML_API(void, VCCOSI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZCOSI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vccosi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzcosi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcCosI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzCosI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCCOSI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCOSI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmccosi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzcosi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcCosI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzCosI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Sine: r[i] = sin(a[i]) */
_MKL_VML_API(void, VSSINI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSINI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssini_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsini_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSinI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSinI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSINI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssini_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsini_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex sine: r[i] = sin(a[i]) */
_MKL_VML_API(void, VCSINI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZSINI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcsini_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzsini_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcSinI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzSinI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCSINI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSINI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsini_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsini_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSinI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSinI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void, VSTANI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDTANI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vstani_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdtani_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsTanI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdTanI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSTANI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstani_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtani_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex tangent: r[i] = tan(a[i]) */
_MKL_VML_API(void, VCTANI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZTANI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vctani_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vztani_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcTanI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzTanI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCTANI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZTANI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmctani_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmztani_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcTanI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzTanI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void, VSCOSHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCOSHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscoshi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcoshi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCoshI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCoshI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCOSHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscoshi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcoshi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCoshI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCoshI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex hyperbolic cosine: r[i] = ch(a[i]) */
_MKL_VML_API(void, VCCOSHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZCOSHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vccoshi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzcoshi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcCoshI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzCoshI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCCOSHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCOSHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmccoshi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzcoshi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcCoshI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzCoshI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Cosine degree: r[i] = cos(a[i]*PI/180) */
_MKL_VML_API(void, VSCOSDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCOSDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscosdi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcosdi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCosdI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCosdI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCOSDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscosdi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcosdi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCosdI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCosdI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Cosine PI: r[i] = cos(a[i]*PI) */
_MKL_VML_API(void, VSCOSPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCOSPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscospii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcospii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCospiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCospiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCOSPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOSPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscospii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcospii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCospiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCospiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void, VSSINHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSINHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssinhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsinhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSinhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSinhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSINHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssinhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsinhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex hyperbolic sine: r[i] = sh(a[i]) */
_MKL_VML_API(void, VCSINHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZSINHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcsinhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzsinhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcSinhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzSinhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCSINHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZSINHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcsinhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzsinhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcSinhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzSinhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Sine degree: r[i] = sin(a[i]*PI/180) */
_MKL_VML_API(void, VSSINDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSINDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssindi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsindi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSindI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSindI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSINDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssindi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsindi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSindI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSindI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Sine PI: r[i] = sin(a[i]*PI) */
_MKL_VML_API(void, VSSINPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDSINPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vssinpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdsinpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsSinpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdSinpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSSINPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmssinpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsinpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void, VSTANHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDTANHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vstanhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdtanhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsTanhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdTanhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSTANHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstanhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtanhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex hyperbolic tangent: r[i] = th(a[i]) */
_MKL_VML_API(void, VCTANHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZTANHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vctanhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vztanhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcTanhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzTanhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCTANHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZTANHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmctanhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmztanhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcTanhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzTanhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Tangent degree: r[i] = tan(a[i]*PI/180) */
_MKL_VML_API(void, VSTANDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDTANDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vstandi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdtandi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsTandI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdTandI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSTANDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstandi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtandi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTandI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTandI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Tangent PI: r[i] = tan(a[i]*PI) */
_MKL_VML_API(void, VSTANPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDTANPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vstanpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdtanpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsTanpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdTanpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSTANPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTANPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstanpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtanpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTanpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTanpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void, VSACOSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDACOSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsacosi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdacosi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAcosI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAcosI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSACOSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsacosi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdacosi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcosI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcosI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex arc cosine: r[i] = arccos(a[i]) */
_MKL_VML_API(void, VCACOSI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZACOSI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcacosi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzacosi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAcosI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAcosI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCACOSI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZACOSI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcacosi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzacosi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAcosI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAcosI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc cosine PI: r[i] = arccos(a[i])/PI */
_MKL_VML_API(void, VSACOSPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDACOSPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsacospii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdacospii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAcospiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAcospiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSACOSPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsacospii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdacospii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcospiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcospiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void, VSASINI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDASINI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsasini_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdasini_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAsinI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAsinI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSASINI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsasini_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdasini_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex arc sine: r[i] = arcsin(a[i]) */
_MKL_VML_API(void, VCASINI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZASINI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcasini_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzasini_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAsinI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAsinI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCASINI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZASINI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcasini_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzasini_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAsinI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAsinI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc sine PI: r[i] = arcsin(a[i])/PI */
_MKL_VML_API(void, VSASINPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDASINPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsasinpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdasinpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAsinpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAsinpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSASINPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsasinpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdasinpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void, VSATANI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDATANI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsatani_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdatani_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAtanI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAtanI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSATANI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatani_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatani_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex arc tangent: r[i] = arctan(a[i]) */
_MKL_VML_API(void, VCATANI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZATANI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcatani_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzatani_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAtanI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAtanI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCATANI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZATANI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcatani_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzatani_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAtanI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAtanI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc tangent PI: r[i] = arctan(a[i])/PI */
_MKL_VML_API(void, VSATANPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDATANPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsatanpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdatanpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAtanpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAtanpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSATANPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatanpii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatanpii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanpiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanpiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void, VSACOSHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDACOSHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsacoshi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdacoshi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAcoshI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAcoshI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSACOSHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDACOSHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsacoshi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdacoshi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAcoshI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAcoshI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex hyperbolic arc cosine: r[i] = arcch(a[i]) */
_MKL_VML_API(void, VCACOSHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZACOSHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcacoshi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzacoshi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAcoshI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAcoshI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCACOSHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZACOSHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcacoshi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzacoshi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAcoshI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAcoshI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void, VSASINHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDASINHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsasinhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdasinhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAsinhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAsinhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSASINHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDASINHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsasinhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdasinhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAsinhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAsinhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex hyperbolic arc sine: r[i] = arcsh(a[i]) */
_MKL_VML_API(void, VCASINHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZASINHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcasinhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzasinhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAsinhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAsinhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCASINHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZASINHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcasinhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzasinhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAsinhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAsinhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void, VSATANHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDATANHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsatanhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdatanhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAtanhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAtanhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSATANHI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATANHI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatanhi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatanhi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtanhI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtanhI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex hyperbolic arc tangent: r[i] = arcth(a[i]) */
_MKL_VML_API(void, VCATANHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZATANHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcatanhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzatanhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcAtanhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzAtanhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCATANHI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZATANHI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcatanhi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzatanhi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcAtanhI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzAtanhI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Error function: r[i] = erf(a[i]) */
_MKL_VML_API(void, VSERFI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDERFI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vserfi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vderfi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsErfI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdErfI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSERFI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Inverse error function: r[i] = erfinv(a[i]) */
_MKL_VML_API(void, VSERFINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDERFINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vserfinvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vderfinvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsErfInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdErfInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSERFINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfinvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfinvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Square root of the sum of the squares: r[i] = hypot(a[i],b[i]) */
_MKL_VML_API(void, VSHYPOTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDHYPOTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vshypoti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdhypoti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsHypotI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdHypotI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSHYPOTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDHYPOTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmshypoti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdhypoti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsHypotI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdHypotI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complementary error function: r[i] = 1 - erf(a[i]) */
_MKL_VML_API(void, VSERFCI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDERFCI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vserfci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vderfci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsErfcI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdErfcI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSERFCI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFCI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfcI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfcI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Inverse complementary error function: r[i] = erfcinv(a[i]) */
_MKL_VML_API(void, VSERFCINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDERFCINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vserfcinvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vderfcinvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsErfcInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdErfcInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSERFCINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFCINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfcinvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfcinvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfcInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfcInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Scaled complementary error function: r[i] = erfcx(a[i]) */
_MKL_VML_API(void, VSERFCXI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDERFCXI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vserfcxi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vderfcxi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsErfcxI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdErfcxI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSERFCXI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDERFCXI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmserfcxi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmderfcxi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsErfcxI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdErfcxI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Cumulative normal distribution function: r[i] = cdfnorm(a[i]) */
_MKL_VML_API(void, VSCDFNORMI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCDFNORMI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscdfnormi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcdfnormi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCdfNormI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCdfNormI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCDFNORMI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCDFNORMI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscdfnormi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcdfnormi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCdfNormI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCdfNormI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Inverse cumulative normal distribution function: r[i] = cdfnorminv(a[i]) */
_MKL_VML_API(void, VSCDFNORMINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCDFNORMINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscdfnorminvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcdfnorminvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCdfNormInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCdfNormInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCDFNORMINVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCDFNORMINVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscdfnorminvi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcdfnorminvi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCdfNormInvI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCdfNormInvI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Logarithm (base e) of the absolute value of gamma function: r[i] = lgamma(a[i]) */
_MKL_VML_API(void, VSLGAMMAI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLGAMMAI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslgammai_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlgammai_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLGammaI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLGammaI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLGAMMAI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLGAMMAI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslgammai_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlgammai_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLGammaI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLGammaI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Gamma function: r[i] = tgamma(a[i]) */
_MKL_VML_API(void, VSTGAMMAI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDTGAMMAI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vstgammai_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdtgammai_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsTGammaI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdTGammaI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSTGAMMAI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTGAMMAI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstgammai_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtgammai_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTGammaI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTGammaI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* I0 Bessel function: r[i] = i0(a[i]) */
_MKL_VML_API(void, VSI0I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDI0I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsi0i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdi0i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsI0I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdI0I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSI0I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDI0I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsi0i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdi0i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsI0I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdI0I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* I1 Bessel function: r[i] = i1(a[i]) */
_MKL_VML_API(void, VSI1I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDI1I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsi1i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdi1i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsI1I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdI1I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSI1I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDI1I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsi1i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdi1i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsI1I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdI1I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* J0 Bessel function: r[i] = j0(a[i]) */
_MKL_VML_API(void, VSJ0I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDJ0I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsj0i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdj0i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsJ0I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdJ0I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSJ0I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDJ0I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsj0i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdj0i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsJ0I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdJ0I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* J1 Bessel function: r[i] = j1(a[i]) */
_MKL_VML_API(void, VSJ1I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDJ1I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsj1i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdj1i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsJ1I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdJ1I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSJ1I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDJ1I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsj1i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdj1i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsJ1I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdJ1I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Y0 Bessel function: r[i] = y0(a[i]) */
_MKL_VML_API(void, VSY0I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDY0I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsy0i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdy0i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsY0I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdY0I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSY0I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDY0I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsy0i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdy0i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsY0I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdY0I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Y1 Bessel function: r[i] = y1(a[i]) */
_MKL_VML_API(void, VSY1I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDY1I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsy1i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdy1i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsY1I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdY1I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSY1I_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDY1I_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsy1i_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdy1i_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsY1I_64, (const MKL_INT n, const float a[], const MKL_INT inca, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdY1I_64, (const MKL_INT n, const double a[], const MKL_INT inca, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Jn Bessel function: r[i] = jn(a[i],n) */
_MKL_VML_API(void, VSJNI_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDJNI_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsjni_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdjni_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsJnI_64, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdJnI_64, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSJNI_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDJNI_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsjni_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdjni_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsJnI_64, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdJnI_64, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Yn Bessel function: r[i] = yn(a[i],n) */
_MKL_VML_API(void, VSYNI_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_MKL_VML_API(void, VDYNI_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_mkl_vml_api(void, vsyni_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr));
_mkl_vml_api(void, vdyni_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vsYnI_64, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr));
_Mkl_Vml_Api(void, vdYnI_64, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr));

_MKL_VML_API(void, VMSYNI_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDYNI_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsyni_64, (const MKL_INT *n, const float a[], const MKL_INT *inca, const float *, float r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdyni_64, (const MKL_INT *n, const double a[], const MKL_INT *inca, const double *, double r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsYnI_64, (const MKL_INT n, const float a[], const MKL_INT inca, const float, float r[], const MKL_INT incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdYnI_64, (const MKL_INT n, const double a[], const MKL_INT inca, const double, double r[], const MKL_INT incr, MKL_INT64 mode));

/* Arc tangent of a/b: r[i] = arctan(a[i]/b[i]) */
_MKL_VML_API(void, VSATAN2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDATAN2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsatan2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdatan2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAtan2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAtan2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSATAN2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATAN2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatan2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatan2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtan2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtan2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Arc tangent of a/b divided by PI: r[i] = arctan(a[i]/b[i])/PI */
_MKL_VML_API(void, VSATAN2PII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDATAN2PII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsatan2pii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdatan2pii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsAtan2piI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdAtan2piI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSATAN2PII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDATAN2PII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsatan2pii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdatan2pii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsAtan2piI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdAtan2piI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Multiplicaton: r[i] = a[i] * b[i] */
_MKL_VML_API(void, VSMULI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDMULI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsmuli_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdmuli_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsMulI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdMulI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSMULI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMULI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsmuli_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdmuli_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMulI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMulI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex multiplication: r[i] = a[i] * b[i] */
_MKL_VML_API(void, VCMULI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZMULI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcmuli_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzmuli_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcMulI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzMulI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCMULI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZMULI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcmuli_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzmuli_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcMulI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzMulI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Division: r[i] = a[i] / b[i] */
_MKL_VML_API(void, VSDIVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDDIVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsdivi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vddivi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsDivI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdDivI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSDIVI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDDIVI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsdivi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmddivi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsDivI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdDivI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex division: r[i] = a[i] / b[i] */
_MKL_VML_API(void, VCDIVI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZDIVI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcdivi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzdivi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcDivI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzDivI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCDIVI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZDIVI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcdivi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzdivi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcDivI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzDivI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Positive difference function: r[i] = fdim(a[i], b[i]) */
_MKL_VML_API(void, VSFDIMI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDFDIMI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsfdimi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdfdimi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsFdimI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdFdimI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSFDIMI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFDIMI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfdimi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfdimi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFdimI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFdimI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Modulus function: r[i] = fmod(a[i], b[i]) */
_MKL_VML_API(void, VSFMODI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDFMODI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsfmodi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdfmodi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsFmodI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdFmodI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSFMODI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMODI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmodi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmodi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmodI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmodI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Maximum function: r[i] = fmax(a[i], b[i]) */
_MKL_VML_API(void, VSFMAXI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDFMAXI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsfmaxi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdfmaxi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsFmaxI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdFmaxI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSFMAXI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMAXI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmaxi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmaxi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFmaxI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFmaxI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Minimum function: r[i] = fmin(a[i], b[i]) */
_MKL_VML_API(void, VSFMINI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDFMINI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsfmini_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdfmini_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsFminI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdFminI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSFMINI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFMINI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfmini_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfmini_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFminI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFminI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VSPOWI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDPOWI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vspowi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdpowi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsPowI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdPowI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSPOWI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspowi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex power function: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VCPOWI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZPOWI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcpowi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzpowi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcPowI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzPowI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCPOWI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZPOWI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcpowi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzpowi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcPowI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzPowI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Power function with a[i]>=0: r[i] = a[i]^b[i] */
_MKL_VML_API(void, VSPOWRI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDPOWRI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vspowri_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdpowri_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsPowrI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdPowrI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSPOWRI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWRI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspowri_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowri_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowrI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowrI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Power function: r[i] = a[i]^(3/2) */
_MKL_VML_API(void, VSPOW3O2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDPOW3O2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vspow3o2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdpow3o2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsPow3o2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdPow3o2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSPOW3O2I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOW3O2I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspow3o2i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpow3o2i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPow3o2I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPow3o2I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Power function: r[i] = a[i]^(2/3) */
_MKL_VML_API(void, VSPOW2O3I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDPOW2O3I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vspow2o3i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdpow2o3i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsPow2o3I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdPow2o3I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSPOW2O3I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOW2O3I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspow2o3i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpow2o3i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPow2o3I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPow2o3I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void, VSPOWXI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float *, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDPOWXI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double *, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vspowxi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float *, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdpowxi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double *, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsPowxI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdPowxI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSPOWXI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float *, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDPOWXI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double *, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmspowxi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float *, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdpowxi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double *, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsPowxI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdPowxI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex power function with fixed degree: r[i] = a[i]^b */
_MKL_VML_API(void, VCPOWXI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZPOWXI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcpowxi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzpowxi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcPowxI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzPowxI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCPOWXI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZPOWXI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcpowxi_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 *b, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzpowxi_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 *b, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcPowxI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzPowxI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Sine & cosine: r1[i] = sin(a[i]), r2[i]=cos(a[i]) */
_MKL_VML_API(void, VSSINCOSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2));
_MKL_VML_API(void, VDSINCOSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vssincosi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vdsincosi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2));
_Mkl_Vml_Api(void, vsSinCosI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r1[], const MKL_INT64 incr1, float r2[], const MKL_INT64 incr2));
_Mkl_Vml_Api(void, vdSinCosI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r1[], const MKL_INT64 incr1, double r2[], const MKL_INT64 incr2));

_MKL_VML_API(void, VMSSINCOSI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINCOSI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmssincosi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsincosi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinCosI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r1[], const MKL_INT64 incr1, float r2[], const MKL_INT64 incr2, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinCosI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r1[], const MKL_INT64 incr1, double r2[], const MKL_INT64 incr2, MKL_INT64 mode));

/* Sine & cosine PI: r1[i] = sinpi(a[i]), r2[i]=cospi(a[i]) */
_MKL_VML_API(void, VSSINCOSPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2));
_MKL_VML_API(void, VDSINCOSPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vssincospii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vdsincospii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2));
_Mkl_Vml_Api(void, vsSinCospiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r1[], const MKL_INT64 incr1, float r2[], const MKL_INT64 incr2));
_Mkl_Vml_Api(void, vdSinCospiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r1[], const MKL_INT64 incr1, double r2[], const MKL_INT64 incr2));

_MKL_VML_API(void, VMSSINCOSPII_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_MKL_VML_API(void, VMDSINCOSPII_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmssincospii_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmdsincospii_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsSinCospiI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r1[], const MKL_INT64 incr1, float r2[], const MKL_INT64 incr2, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdSinCospiI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r1[], const MKL_INT64 incr1, double r2[], const MKL_INT64 incr2, MKL_INT64 mode));

/* Linear fraction: r[i] = (a[i]*scalea + shifta)/(b[i]*scaleb + shiftb) */
_MKL_VML_API(void, VSLINEARFRACI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDLINEARFRACI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vslinearfraci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdlinearfraci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsLinearFracI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, const float scalea, const float shifta, const float scaleb, const float shiftb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdLinearFracI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, const double scalea, const double shifta, const double scaleb, const double shiftb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSLINEARFRACI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDLINEARFRACI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmslinearfraci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, const float *scalea, const float *shifta, const float *scaleb, const float *shiftb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdlinearfraci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, const double *scalea, const double *shifta, const double *scaleb, const double *shiftb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsLinearFracI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, const float scalea, const float shifta, const float scaleb, const float shiftb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdLinearFracI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, const double scalea, const double shifta, const double scaleb, const double shiftb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Integer value rounded towards plus infinity: r[i] = ceil(a[i]) */
_MKL_VML_API(void, VSCEILI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCEILI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsceili_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdceili_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCeilI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCeilI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCEILI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCEILI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsceili_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdceili_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCeilI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCeilI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Integer value rounded towards minus infinity: r[i] = floor(a[i]) */
_MKL_VML_API(void, VSFLOORI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDFLOORI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsfloori_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdfloori_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsFloorI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdFloorI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSFLOORI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFLOORI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfloori_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfloori_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFloorI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFloorI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Signed fraction part */
_MKL_VML_API(void, VSFRACI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDFRACI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsfraci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdfraci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsFracI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdFracI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSFRACI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDFRACI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsfraci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdfraci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsFracI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdFracI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Truncated integer value and the remaining fraction part */
_MKL_VML_API(void, VSMODFI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2));
_MKL_VML_API(void, VDMODFI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vsmodfi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vdmodfi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2));
_Mkl_Vml_Api(void, vsModfI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r1[], const MKL_INT64 incr1, float r2[], const MKL_INT64 incr2));
_Mkl_Vml_Api(void, vdModfI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r1[], const MKL_INT64 incr1, double r2[], const MKL_INT64 incr2));

_MKL_VML_API(void, VMSMODFI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMODFI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmsmodfi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r1[], const MKL_INT64 *incr1, float r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmdmodfi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r1[], const MKL_INT64 *incr1, double r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsModfI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r1[], const MKL_INT64 incr1, float r2[], const MKL_INT64 incr2, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdModfI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r1[], const MKL_INT64 incr1, double r2[], const MKL_INT64 incr2, MKL_INT64 mode));

/* Rounded integer value in the current rounding mode: r[i] = nearbyint(a[i]) */
_MKL_VML_API(void, VSNEARBYINTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDNEARBYINTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsnearbyinti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdnearbyinti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsNearbyIntI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdNearbyIntI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSNEARBYINTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDNEARBYINTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsnearbyinti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdnearbyinti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsNearbyIntI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdNearbyIntI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Next after function: r[i] = nextafter(a[i], b[i]) */
_MKL_VML_API(void, VSNEXTAFTERI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDNEXTAFTERI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsnextafteri_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdnextafteri_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsNextAfterI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdNextAfterI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSNEXTAFTERI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDNEXTAFTERI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsnextafteri_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdnextafteri_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsNextAfterI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdNextAfterI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Minimum magnitude function: r[i] = minmag(a[i], b[i]) */
_MKL_VML_API(void, VSMINMAGI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDMINMAGI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsminmagi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdminmagi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsMinMagI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdMinMagI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSMINMAGI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMINMAGI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsminmagi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdminmagi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMinMagI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMinMagI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Maximum magnitude function: r[i] = maxmag(a[i], b[i]) */
_MKL_VML_API(void, VSMAXMAGI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDMAXMAGI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsmaxmagi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdmaxmagi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsMaxMagI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdMaxMagI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSMAXMAGI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDMAXMAGI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsmaxmagi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdmaxmagi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsMaxMagI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdMaxMagI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Rounded integer value in the current rounding mode with inexact result exception raised for rach changed value: r[i] = rint(a[i]) */
_MKL_VML_API(void, VSRINTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDRINTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsrinti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdrinti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsRintI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdRintI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSRINTI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDRINTI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsrinti_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdrinti_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRintI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRintI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Value rounded to the nearest integer: r[i] = round(a[i]) */
_MKL_VML_API(void, VSROUNDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDROUNDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsroundi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdroundi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsRoundI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdRoundI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSROUNDI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDROUNDI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsroundi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdroundi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRoundI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRoundI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Integer value rounded towards zero: r[i] = trunc(a[i]) */
_MKL_VML_API(void, VSTRUNCI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDTRUNCI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vstrunci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdtrunci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsTruncI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdTruncI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSTRUNCI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDTRUNCI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmstrunci_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdtrunci_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsTruncI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdTruncI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Element by element conjugation: r[i] = conj(a[i]) */
_MKL_VML_API(void, VCCONJI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZCONJI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcconji_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzconji_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcConjI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzConjI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCCONJI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCONJI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcconji_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzconji_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcConjI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzConjI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Element by element multiplication of vector A element and conjugated vector B element: r[i] = mulbyconj(a[i],b[i]) */
_MKL_VML_API(void, VCMULBYCONJI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZMULBYCONJI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vcmulbyconji_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzmulbyconji_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcMulByConjI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzMulByConjI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCMULBYCONJI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZMULBYCONJI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmcmulbyconji_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 *inca, const MKL_Complex8 b[], const MKL_INT64 *incb, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzmulbyconji_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 *inca, const MKL_Complex16 b[], const MKL_INT64 *incb, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcMulByConjI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 inca, const MKL_Complex8 b[], const MKL_INT64 incb, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzMulByConjI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 inca, const MKL_Complex16 b[], const MKL_INT64 incb, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Complex exponent of real vector elements: r[i] = CIS(a[i]) */
_MKL_VML_API(void, VCCISI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VZCISI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vccisi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vzcisi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vcCISI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vzCISI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMCCISI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMZCISI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmccisi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, MKL_Complex8 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmzcisi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, MKL_Complex16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmcCISI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, MKL_Complex8 r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmzCISI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, MKL_Complex16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Exponential integral of real vector elements: r[i] = E1(a[i]) */
_MKL_VML_API(void, VSEXPINT1I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDEXPINT1I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsexpint1i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdexpint1i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsExpInt1I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdExpInt1I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSEXPINT1I_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDEXPINT1I_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsexpint1i_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdexpint1i_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsExpInt1I_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdExpInt1I_64, (const MKL_INT64 n, const double a[], const MKL_INT64 inca, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Copy sign function: r[i] = copysign(a[i], b[i]) */
_MKL_VML_API(void, VSCOPYSIGNI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDCOPYSIGNI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vscopysigni_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdcopysigni_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsCopySignI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdCopySignI_64, (const MKL_INT64 n, const double a[],const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSCOPYSIGNI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDCOPYSIGNI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmscopysigni_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdcopysigni_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsCopySignI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdCopySignI_64, (const MKL_INT64 n, const double a[],const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/* Remainder function: r[i] = remainder(a[i], b[i]) */
_MKL_VML_API(void, VSREMAINDERI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_MKL_VML_API(void, VDREMAINDERI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vsremainderi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vdremainderi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vsRemainderI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr));
_Mkl_Vml_Api(void, vdRemainderI_64, (const MKL_INT64 n, const double a[],const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMSREMAINDERI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_MKL_VML_API(void, VMDREMAINDERI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmsremainderi_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 *inca, const float b[], const MKL_INT64 *incb, float r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmdremainderi_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 *inca, const double b[], const MKL_INT64 *incb, double r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmsRemainderI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 inca, const float b[], const MKL_INT64 incb, float r[], const MKL_INT64 incr, MKL_INT64 mode));
_Mkl_Vml_Api(void, vmdRemainderI_64, (const MKL_INT64 n, const double a[],const MKL_INT64 inca, const double b[], const MKL_INT64 incb, double r[], const MKL_INT64 incr, MKL_INT64 mode));

/*
//++
// FLOAT16 functions
//--
*/

_MKL_VML_API(void, VHABS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhabs,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAbs,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHABS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhabs,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAbs,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHADD,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhadd,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAdd,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHADD,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhadd,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAdd,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSUB,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhsub,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSub,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSUB,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsub,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSub,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhinv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhinv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSQRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsqrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSqrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSQRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHINVSQRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhinvsqrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhInvSqrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHINVSQRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvsqrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvSqrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCBRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcbrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCbrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCBRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcbrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCbrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHINVCBRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhinvcbrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhInvCbrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHINVCBRT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvcbrt,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvCbrt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSQR,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsqr,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSqr,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSQR,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqr,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqr,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXP,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexp,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExp,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXP,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXP2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexp2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExp2, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXP2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp2, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXP10, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexp10, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExp10, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXP10, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp10, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp10, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXPM1,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexpm1,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExpm1,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXPM1,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpm1,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpm1,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhln,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLn,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhln,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLn,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOG2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlog2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLog2, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOG2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog2, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog2, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOG10,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlog10,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLog10,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOG10,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog10,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog10,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOG1P,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlog1p,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLog1p,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOG1P,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog1p,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog1p,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOGB, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlogb, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLogb, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOGB, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlogb, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLogb, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcos,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCos,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcos,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCos,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSIN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsin,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSin,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSIN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsin,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSin,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTAN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtan,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTan,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTAN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtan,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTan,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOSPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcospi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCospi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOSPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcospi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCospi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSINPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsinpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSinpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSINPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTANPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtanpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTanpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTANPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOSD, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcosd, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCosd, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOSD, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosd, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosd, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSIND, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsind, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSind, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSIND, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsind, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSind, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTAND, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtand, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTand, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTAND, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtand, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTand, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOSH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcosh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCosh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOSH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSINH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsinh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSinh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSINH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTANH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtanh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTanh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTANH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHACOS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhacos,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAcos,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHACOS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhacos,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcos,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHASIN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhasin,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAsin,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHASIN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhasin,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsin,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATAN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhatan,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtan,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATAN,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHACOSPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhacospi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAcospi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHACOSPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhacospi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcospi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHASINPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhasinpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAsinpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHASINPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATANPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhatanpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtanpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATANPI, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanpi, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanpi, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHACOSH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhacosh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAcosh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHACOSH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhacosh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcosh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHASINH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhasinh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAsinh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHASINH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATANH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhatanh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtanh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATANH,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanh,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanh,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERF,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherf,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErf,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERF,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherf,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErf,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERFINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherfinv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErfInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERFINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherfinv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHHYPOT,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhhypot,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhHypot,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHHYPOT,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhhypot,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhHypot,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERFC,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherfc,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErfc,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERFC,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherfc,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfc,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERFCINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherfcinv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErfcInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERFCINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherfcinv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORM,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcdfnorm,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCdfNorm,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCDFNORM,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnorm,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNorm,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORMINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcdfnorminv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCdfNormInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCDFNORMINV,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnorminv,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNormInv,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLGAMMA,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlgamma,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLGamma,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLGAMMA,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlgamma,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLGamma,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTGAMMA,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtgamma,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTGamma,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTGAMMA,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtgamma,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTGamma,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhatan2,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtan2,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATAN2,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2PI, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhatan2pi, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtan2pi, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATAN2PI, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2pi, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2pi, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMUL,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhmul,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhMul,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHMUL,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhmul,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMul,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHDIV,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhdiv,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhDiv,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHDIV,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhdiv,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhDiv,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOW,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhpow,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPow,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOW,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOW3O2,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhpow3o2,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPow3o2,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOW3O2,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow3o2,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow3o2,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOW2O3,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhpow2o3,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPow2o3,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOW2O3,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow2o3,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow2o3,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOWX,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_mkl_vml_api(void, vhpowx,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPowx,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[]));
_MKL_VML_API(void, VMHPOWX,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowx,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowx,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOWR, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhpowr, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPowr, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOWR, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowr, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowr, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSINCOS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_mkl_vml_api(void, vhsincos,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_Mkl_Vml_Api(void, vhSinCos,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_MKL_VML_API(void, VMHSINCOS,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsincos,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinCos,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 mode));

_MKL_VML_API(void, VHLINEARFRAC,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[]));
_mkl_vml_api(void, vhlinearfrac,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLinearFrac,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[]));
_MKL_VML_API(void, VMHLINEARFRAC,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlinearfrac,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLinearFrac,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCEIL,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhceil,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCeil,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCEIL,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhceil,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCeil,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFLOOR,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhfloor,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFloor,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFLOOR,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfloor,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFloor,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFRAC,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhfrac,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFrac,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFRAC,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfrac,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFrac,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMODF,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_mkl_vml_api(void, vhmodf,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_Mkl_Vml_Api(void, vhModf,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_MKL_VML_API(void, VMHMODF,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhmodf,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhModf,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 mode));

_MKL_VML_API(void, VHFMOD, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfmod, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFmod, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFMOD, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmod, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmod, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHREMAINDER, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhremainder, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhRemainder, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHREMAINDER, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhremainder, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRemainder, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHNEXTAFTER, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhnextafter, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhNextAfter, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHNEXTAFTER, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhnextafter, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNextAfter, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOPYSIGN, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhcopysign, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCopySign, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOPYSIGN, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcopysign, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCopySign, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFDIM, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfdim, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFdim, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFDIM, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfdim, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFdim, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFMAX, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfmax, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFmax, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFMAX, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmax, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmax, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFMIN, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfmin, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFmin, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFMIN, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmin, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmin, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMAXMAG, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhmaxmag, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhMaxMag, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHMAXMAG, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhmaxmag, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMaxMag, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMINMAG, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhminmag, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhMinMag, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHMINMAG, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhminmag, (const MKL_INT *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMinMag, (const MKL_INT n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHNEARBYINT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhnearbyint,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhNearbyInt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHNEARBYINT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhnearbyint,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNearbyInt,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHRINT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhrint,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhRint,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHRINT,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhrint,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRint,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHROUND,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhround,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhRound,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHROUND,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhround,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRound,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTRUNC,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtrunc,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTrunc,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTRUNC,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtrunc,(const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTrunc,(const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXPINT1, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexpint1, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExpInt1, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXPINT1, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpint1, (const MKL_INT *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpInt1, (const MKL_INT n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHERFCX,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vherfcx,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhErfcx,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHERFCX,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmherfcx,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhErfcx,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHI0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhi0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhI0,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHI0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhi0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhI0,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHI1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhi1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhI1,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHI1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhi1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhI1,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHJ0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhj0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhJ0,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHJ0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhj0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhJ0,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHJ1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhj1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhJ1,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHJ1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhj1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhJ1,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHY0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhy0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhY0,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHY0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhy0,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhY0,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHY1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhy1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhY1,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHY1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhy1,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhY1,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHJN,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_mkl_vml_api(void,vhjn,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_Mkl_Vml_Api(void,vhJn,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[]));

_MKL_VML_API(void,VMHJN,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhjn,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhJn,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHYN,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_mkl_vml_api(void,vhyn,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_Mkl_Vml_Api(void,vhYn,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[]));

_MKL_VML_API(void,VMHYN,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhyn,(const MKL_INT *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhYn,(const MKL_INT n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[], MKL_INT64 mode));


_MKL_VML_API(void, VHABS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhabs_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAbs_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHABS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhabs_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAbs_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHADD_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhadd_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAdd_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHADD_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhadd_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAdd_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSUB_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhsub_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSub_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSUB_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsub_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSub_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhinv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhinv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSQRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsqrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSqrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSQRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHINVSQRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhinvsqrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhInvSqrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHINVSQRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvsqrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvSqrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCBRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcbrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCbrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCBRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcbrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCbrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHINVCBRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhinvcbrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhInvCbrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHINVCBRT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvcbrt_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvCbrt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSQR_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsqr_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSqr_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSQR_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqr_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqr_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXP_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexp_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExp_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXP_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXP2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexp2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExp2_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXP2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp2_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXP10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexp10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExp10_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXP10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp10_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXPM1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexpm1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExpm1_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXPM1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpm1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpm1_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhln_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLn_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhln_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLn_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOG2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlog2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLog2_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOG2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog2_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOG10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlog10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLog10_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOG10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog10_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog10_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOG1P_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlog1p_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLog1p_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOG1P_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog1p_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog1p_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLOGB_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlogb_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLogb_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLOGB_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlogb_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLogb_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcos_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCos_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcos_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCos_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSIN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsin_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSin_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSIN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsin_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSin_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTAN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtan_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTan_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTAN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtan_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTan_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOSPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcospi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCospi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOSPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcospi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCospi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSINPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsinpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSinpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSINPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTANPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtanpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTanpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTANPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOSD_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcosd_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCosd_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOSD_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosd_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosd_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSIND_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsind_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSind_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSIND_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsind_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSind_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTAND_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtand_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTand_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTAND_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtand_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTand_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOSH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcosh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCosh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOSH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSINH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhsinh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhSinh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHSINH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTANH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtanh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTanh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTANH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHACOS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhacos_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAcos_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHACOS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhacos_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcos_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHASIN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhasin_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAsin_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHASIN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhasin_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsin_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATAN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhatan_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtan_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATAN_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHACOSPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhacospi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAcospi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHACOSPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhacospi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcospi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHASINPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhasinpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAsinpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHASINPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATANPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhatanpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtanpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATANPI_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanpi_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanpi_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHACOSH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhacosh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAcosh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHACOSH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhacosh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcosh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHASINH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhasinh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAsinh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHASINH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATANH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhatanh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtanh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATANH_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanh_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanh_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERF_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherf_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErf_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERF_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherf_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErf_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERFINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherfinv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErfInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERFINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherfinv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHHYPOT_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhhypot_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhHypot_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHHYPOT_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhhypot_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhHypot_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERFC_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherfc_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErfc_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERFC_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherfc_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfc_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHERFCINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vherfcinv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhErfcInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHERFCINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmherfcinv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORM_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcdfnorm_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCdfNorm_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCDFNORM_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnorm_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNorm_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORMINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhcdfnorminv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCdfNormInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCDFNORMINV_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnorminv_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNormInv_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHLGAMMA_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhlgamma_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLGamma_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHLGAMMA_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlgamma_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLGamma_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTGAMMA_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtgamma_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTGamma_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTGAMMA_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtgamma_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTGamma_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhatan2_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtan2_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATAN2_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2PI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhatan2pi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhAtan2pi_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHATAN2PI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2pi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2pi_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMUL_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhmul_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhMul_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHMUL_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhmul_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMul_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHDIV_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhdiv_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhDiv_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHDIV_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhdiv_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhDiv_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOW_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhpow_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPow_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOW_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOW3O2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhpow3o2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPow3o2_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOW3O2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow3o2_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow3o2_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOW2O3_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhpow2o3_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPow2o3_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOW2O3_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow2o3_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow2o3_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOWX_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_mkl_vml_api(void, vhpowx_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPowx_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[]));
_MKL_VML_API(void, VMHPOWX_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowx_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowx_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHPOWR_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhpowr_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhPowr_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHPOWR_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowr_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowr_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHSINCOS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_mkl_vml_api(void, vhsincos_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_Mkl_Vml_Api(void, vhSinCos_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_MKL_VML_API(void, VMHSINCOS_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhsincos_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinCos_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 mode));

_MKL_VML_API(void, VHLINEARFRAC_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[]));
_mkl_vml_api(void, vhlinearfrac_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[]));
_Mkl_Vml_Api(void, vhLinearFrac_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[]));
_MKL_VML_API(void, VMHLINEARFRAC_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhlinearfrac_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLinearFrac_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCEIL_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhceil_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCeil_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCEIL_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhceil_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCeil_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFLOOR_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhfloor_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFloor_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFLOOR_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfloor_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFloor_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFRAC_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhfrac_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFrac_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFRAC_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfrac_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFrac_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMODF_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_mkl_vml_api(void, vhmodf_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_Mkl_Vml_Api(void, vhModf_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[]));
_MKL_VML_API(void, VMHMODF_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhmodf_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhModf_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r1[], MKL_F16 r2[], MKL_INT64 mode));

_MKL_VML_API(void, VHFMOD_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfmod_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFmod_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFMOD_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmod_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmod_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHREMAINDER_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhremainder_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhRemainder_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHREMAINDER_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhremainder_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRemainder_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHNEXTAFTER_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhnextafter_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhNextAfter_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHNEXTAFTER_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhnextafter_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNextAfter_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHCOPYSIGN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhcopysign_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhCopySign_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHCOPYSIGN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhcopysign_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCopySign_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFDIM_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfdim_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFdim_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFDIM_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfdim_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFdim_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFMAX_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfmax_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFmax_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFMAX_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmax_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmax_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHFMIN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhfmin_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhFmin_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHFMIN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmin_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmin_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMAXMAG_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhmaxmag_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhMaxMag_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHMAXMAG_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhmaxmag_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMaxMag_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHMINMAG_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_mkl_vml_api(void, vhminmag_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhMinMag_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[]));
_MKL_VML_API(void, VMHMINMAG_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhminmag_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMinMag_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHNEARBYINT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhnearbyint_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhNearbyInt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHNEARBYINT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhnearbyint_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNearbyInt_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHRINT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhrint_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhRint_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHRINT_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhrint_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRint_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHROUND_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhround_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhRound_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHROUND_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhround_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRound_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHTRUNC_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhtrunc_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhTrunc_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHTRUNC_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhtrunc_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTrunc_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void, VHEXPINT1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void, vhexpint1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void, vhExpInt1_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[]));
_MKL_VML_API(void, VMHEXPINT1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpint1_64, (const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpInt1_64, (const MKL_INT64 n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHERFCX_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vherfcx_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhErfcx_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHERFCX_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmherfcx_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhErfcx_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHI0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhi0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhI0_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHI0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhi0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhI0_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHI1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhi1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhI1_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHI1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhi1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhI1_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHJ0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhj0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhJ0_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHJ0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhj0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhJ0_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHJ1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhj1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhJ1_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHJ1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhj1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhJ1_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHY0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhy0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhY0_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHY0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhy0_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhY0_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHY1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_mkl_vml_api(void,vhy1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[]));
_Mkl_Vml_Api(void,vhY1_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[]));

_MKL_VML_API(void,VMHY1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhy1_64,(const MKL_INT64 *n, const MKL_F16 a[], MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhY1_64,(const MKL_INT64 n,  const MKL_F16 a[], MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHJN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_mkl_vml_api(void,vhjn_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_Mkl_Vml_Api(void,vhJn_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[]));

_MKL_VML_API(void,VMHJN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhjn_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhJn_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[], MKL_INT64 mode));

_MKL_VML_API(void,VHYN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_mkl_vml_api(void,vhyn_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[]));
_Mkl_Vml_Api(void,vhYn_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[]));

_MKL_VML_API(void,VMHYN_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_mkl_vml_api(void,vmhyn_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_F16 *b, MKL_F16 r[], MKL_INT64 *mode));
_Mkl_Vml_Api(void,vmhYn_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_F16 b, MKL_F16 r[], MKL_INT64 mode));


_MKL_VML_API(void, VHABSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhabsi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAbsI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHABSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr,MKL_INT64 *mode));
_mkl_vml_api(void, vmhabsi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr,MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAbsI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHADDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhaddi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAddI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHADDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhaddi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAddI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSUBI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsubi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSubI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSUBI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsubi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSubI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhinvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSQRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsqrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSqrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSQRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHINVSQRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhinvsqrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhInvSqrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHINVSQRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvsqrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvSqrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCBRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcbrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCbrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCBRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcbrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCbrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHINVCBRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhinvcbrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhInvCbrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHINVCBRTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvcbrti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvCbrtI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSQRI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsqri, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSqrI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSQRI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqri, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqrI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXPI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhexpi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhExpI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHEXPI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXP2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhexp2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhExp2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHEXP2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXP10I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhexp10i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhExp10I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHEXP10I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp10i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp10I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXPM1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhexpm1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhExpm1I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHEXPM1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpm1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpm1I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLnI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLnI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOG10I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlog10i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLog10I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLOG10I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog10i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog10I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOG2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlog2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLog2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLOG2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOG1PI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlog1pi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLog1pI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLOG1PI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog1pi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog1pI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOGBI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlogbi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLogbI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLOGBI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlogbi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLogbI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcosi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCosI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCOSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsini, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSinI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSINI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsini, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhtani, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhTanI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHTANI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtani, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcoshi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCoshI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCOSHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcoshi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCoshI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcosdi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCosdI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCOSDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosdi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosdI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcospii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCospiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCOSPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcospii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCospiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsinhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSinhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSINHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsindi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSindI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSINDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsindi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSindI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhsinpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhSinpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHSINPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhtanhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhTanhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHTANHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhtandi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhTandI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHTANDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtandi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTandI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhtanpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhTanpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHTANPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHACOSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhacosi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAcosI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHACOSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhacosi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcosI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHACOSPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhacospii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAcospiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHACOSPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhacospii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcospiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHASINI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhasini, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAsinI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHASINI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhasini, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHASINPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhasinpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAsinpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHASINPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATANI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhatani, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAtanI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHATANI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatani, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATANPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhatanpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAtanpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHATANPII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanpii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanpiI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHACOSHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhacoshi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAcoshI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHACOSHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhacoshi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcoshI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHASINHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhasinhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAsinhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHASINHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATANHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhatanhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAtanhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHATANHI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanhi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanhI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vherfi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhErfI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHERFI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vherfinvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhErfInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHERFINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfinvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHHYPOTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhhypoti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhHypotI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHHYPOTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhhypoti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhHypotI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFCI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vherfci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhErfcI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHERFCI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFCINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vherfcinvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhErfcInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHERFCINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfcinvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORMI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcdfnormi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCdfNormI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCDFNORMI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnormi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNormI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORMINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcdfnorminvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCdfNormInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCDFNORMINVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnorminvi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNormInvI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLGAMMAI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlgammai, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLGammaI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLGAMMAI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlgammai, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLGammaI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTGAMMAI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhtgammai, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhTGammaI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHTGAMMAI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtgammai, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTGammaI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhatan2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAtan2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHATAN2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2PII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhatan2pii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhAtan2piI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHATAN2PII, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2pii, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2piI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMULI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhmuli, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhMulI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHMULI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhmuli, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMulI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHDIVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhdivi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhDivI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHDIVI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhdivi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhDivI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFDIMI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhfdimi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhFdimI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHFDIMI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfdimi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFdimI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFMODI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhfmodi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhFmodI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHFMODI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmodi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmodI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFMAXI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhfmaxi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhFmaxI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHFMAXI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmaxi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmaxI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFMINI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhfmini, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhFminI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHFMINI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmini, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFminI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOWI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhpowi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhPowI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHPOWI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOWRI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhpowri, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhPowrI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHPOWRI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowri, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowrI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOW3O2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhpow3o2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhPow3o2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHPOW3O2I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow3o2i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow3o2I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOW2O3I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhpow2o3i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhPow2o3I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHPOW2O3I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow2o3i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow2o3I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOWXI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhpowxi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhPowxI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHPOWXI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowxi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowxI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINCOSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vhsincosi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2));
_Mkl_Vml_Api(void, vhSinCosI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r1[], const MKL_INT incr1, MKL_F16 r2[], const MKL_INT incr2));
_MKL_VML_API(void, VMHSINCOSI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsincosi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinCosI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r1[], const MKL_INT incr1, MKL_F16 r2[], const MKL_INT incr2, MKL_INT64 mode));

_MKL_VML_API(void, VHLINEARFRACI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhlinearfraci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhLinearFracI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHLINEARFRACI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlinearfraci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLinearFracI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCEILI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhceili, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCeilI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCEILI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhceili, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCeilI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFLOORI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhfloori, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhFloorI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHFLOORI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfloori, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFloorI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFRACI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhfraci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhFracI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHFRACI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfraci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFracI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMODFI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2));
_mkl_vml_api(void, vhmodfi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2));
_Mkl_Vml_Api(void, vhModfI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r1[], const MKL_INT incr1, MKL_F16 r2[], const MKL_INT incr2));
_MKL_VML_API(void, VMHMODFI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmhmodfi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r1[], const MKL_INT *incr1, MKL_F16 r2[], const MKL_INT *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhModfI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r1[], const MKL_INT incr1, MKL_F16 r2[], const MKL_INT incr2, MKL_INT64 mode));

_MKL_VML_API(void, VHNEARBYINTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhnearbyinti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhNearbyIntI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHNEARBYINTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhnearbyinti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNearbyIntI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHNEXTAFTERI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhnextafteri, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhNextAfterI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHNEXTAFTERI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhnextafteri, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNextAfterI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMINMAGI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhminmagi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhMinMagI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHMINMAGI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhminmagi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMinMagI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMAXMAGI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhmaxmagi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhMaxMagI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHMAXMAGI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhmaxmagi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMaxMagI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHRINTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhrinti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhRintI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHRINTI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhrinti, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRintI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHROUNDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhroundi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhRoundI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHROUNDI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhroundi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRoundI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTRUNCI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhtrunci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhTruncI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHTRUNCI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtrunci, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTruncI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXPINT1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhexpint1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhExpInt1I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHEXPINT1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpint1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpInt1I, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOPYSIGNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhcopysigni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhCopySignI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHCOPYSIGNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcopysigni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCopySignI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHREMAINDERI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhremainderi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhRemainderI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr));
_MKL_VML_API(void, VMHREMAINDERI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhremainderi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 b[], const MKL_INT *incb, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRemainderI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16 b[], const MKL_INT incb, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFCXI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vherfcxi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vErfcxI, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHERFCXI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfcxi, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcxI, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHI0I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhi0i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhI0I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHI0I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhi0i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhI0I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHI1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhi1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhI1I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHI1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhi1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhI1I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHJ0I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhj0i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhJ0I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHJ0I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhj0i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhJ0I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHJ1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhj1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhJ1I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHJ1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhj1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhJ1I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHY0I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhy0i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhY0I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHY0I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhy0i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhY0I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHY1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhy1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhY1I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr));

_MKL_VML_API(void, VMHY1I, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhy1i, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhY1I, (const MKL_INT n,  const MKL_F16 a[], const MKL_INT inca, MKL_F16 r[],  const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHJNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhjni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhJnI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16, MKL_F16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMHJNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhjni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhJnI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));

_MKL_VML_API(void, VHYNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr));
_mkl_vml_api(void, vhyni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr));
_Mkl_Vml_Api(void, vhYnI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16, MKL_F16 r[], const MKL_INT incr));

_MKL_VML_API(void, VMHYNI, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhyni, (const MKL_INT *n, const MKL_F16 a[], const MKL_INT *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhYnI, (const MKL_INT n, const MKL_F16 a[], const MKL_INT inca, const MKL_F16, MKL_F16 r[], const MKL_INT incr, MKL_INT64 mode));


_MKL_VML_API(void, VHABSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhabsi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAbsI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHABSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr,MKL_INT64 *mode));
_mkl_vml_api(void, vmhabsi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr,MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAbsI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHADDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhaddi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAddI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHADDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhaddi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAddI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSUBI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsubi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSubI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSUBI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsubi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSubI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhinvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSQRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsqrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSqrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSQRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHINVSQRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhinvsqrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhInvSqrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHINVSQRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvsqrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvSqrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCBRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcbrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCbrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCBRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcbrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCbrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHINVCBRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhinvcbrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhInvCbrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHINVCBRTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhinvcbrti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhInvCbrtI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSQRI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsqri_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSqrI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSQRI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsqri_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSqrI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXPI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhexpi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhExpI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHEXPI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXP2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhexp2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhExp2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHEXP2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXP10I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhexp10i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhExp10I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHEXP10I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexp10i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExp10I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXPM1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhexpm1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhExpm1I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHEXPM1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpm1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpm1I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLnI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLnI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOG10I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlog10i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLog10I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLOG10I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog10i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog10I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOG2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlog2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLog2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLOG2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOG1PI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlog1pi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLog1pI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLOG1PI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlog1pi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLog1pI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLOGBI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlogbi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLogbI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLOGBI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlogbi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLogbI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcosi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCosI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCOSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsini_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSinI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSINI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsini_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhtani_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhTanI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHTANI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtani_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcoshi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCoshI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCOSHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcoshi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCoshI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcosdi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCosdI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCOSDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcosdi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCosdI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOSPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcospii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCospiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCOSPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcospii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCospiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsinhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSinhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSINHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsindi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSindI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSINDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsindi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSindI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhsinpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhSinpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHSINPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsinpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhtanhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhTanhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHTANHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhtandi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhTandI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHTANDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtandi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTandI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTANPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhtanpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhTanpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHTANPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtanpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTanpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHACOSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhacosi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAcosI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHACOSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhacosi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcosI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHACOSPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhacospii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAcospiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHACOSPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhacospii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcospiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHASINI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhasini_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAsinI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHASINI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhasini_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHASINPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhasinpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAsinpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHASINPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATANI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhatani_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAtanI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHATANI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatani_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATANPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhatanpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAtanpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHATANPII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanpii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanpiI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHACOSHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhacoshi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAcoshI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHACOSHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhacoshi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAcoshI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHASINHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhasinhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAsinhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHASINHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhasinhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAsinhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATANHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhatanhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAtanhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHATANHI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatanhi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtanhI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vherfi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhErfI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHERFI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vherfinvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhErfInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHERFINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfinvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHHYPOTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhhypoti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhHypotI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHHYPOTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhhypoti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhHypotI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFCI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vherfci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhErfcI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHERFCI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFCINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vherfcinvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhErfcInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHERFCINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfcinvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORMI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcdfnormi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCdfNormI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCDFNORMI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnormi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNormI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCDFNORMINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcdfnorminvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCdfNormInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCDFNORMINVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcdfnorminvi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCdfNormInvI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHLGAMMAI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlgammai_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLGammaI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLGAMMAI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlgammai_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLGammaI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTGAMMAI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhtgammai_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhTGammaI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHTGAMMAI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtgammai_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTGammaI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhatan2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAtan2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHATAN2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHATAN2PII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhatan2pii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhAtan2piI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHATAN2PII_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhatan2pii_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhAtan2piI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMULI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhmuli_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhMulI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHMULI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhmuli_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMulI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHDIVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhdivi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhDivI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHDIVI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhdivi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhDivI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFDIMI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhfdimi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhFdimI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHFDIMI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfdimi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFdimI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFMODI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhfmodi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhFmodI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHFMODI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmodi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmodI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFMAXI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhfmaxi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhFmaxI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHFMAXI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmaxi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFmaxI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFMINI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhfmini_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhFminI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHFMINI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfmini_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFminI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOWI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhpowi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhPowI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHPOWI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOWRI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhpowri_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhPowrI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHPOWRI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowri_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowrI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOW3O2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhpow3o2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhPow3o2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHPOW3O2I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow3o2i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow3o2I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOW2O3I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhpow2o3i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhPow2o3I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHPOW2O3I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpow2o3i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPow2o3I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHPOWXI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhpowxi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhPowxI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHPOWXI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhpowxi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhPowxI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHSINCOSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vhsincosi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2));
_Mkl_Vml_Api(void, vhSinCosI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r1[], const MKL_INT64 incr1, MKL_F16 r2[], const MKL_INT64 incr2));
_MKL_VML_API(void, VMHSINCOSI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmhsincosi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhSinCosI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r1[], const MKL_INT64 incr1, MKL_F16 r2[], const MKL_INT64 incr2, MKL_INT64 mode));

_MKL_VML_API(void, VHLINEARFRACI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhlinearfraci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhLinearFracI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHLINEARFRACI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhlinearfraci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, const MKL_F16 *scalea, const MKL_F16 *shifta, const MKL_F16 *scaleb, const MKL_F16 *shiftb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhLinearFracI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, const MKL_F16 scalea, const MKL_F16 shifta, const MKL_F16 scaleb, const MKL_F16 shiftb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCEILI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhceili_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCeilI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCEILI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhceili_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCeilI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFLOORI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhfloori_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhFloorI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHFLOORI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfloori_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFloorI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHFRACI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhfraci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhFracI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHFRACI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhfraci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhFracI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMODFI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2));
_mkl_vml_api(void, vhmodfi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2));
_Mkl_Vml_Api(void, vhModfI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r1[], const MKL_INT64 incr1, MKL_F16 r2[], const MKL_INT64 incr2));
_MKL_VML_API(void, VMHMODFI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_mkl_vml_api(void, vmhmodfi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r1[], const MKL_INT64 *incr1, MKL_F16 r2[], const MKL_INT64 *incr2, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhModfI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r1[], const MKL_INT64 incr1, MKL_F16 r2[], const MKL_INT64 incr2, MKL_INT64 mode));

_MKL_VML_API(void, VHNEARBYINTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhnearbyinti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhNearbyIntI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHNEARBYINTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhnearbyinti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNearbyIntI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHNEXTAFTERI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhnextafteri_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhNextAfterI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHNEXTAFTERI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhnextafteri_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhNextAfterI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMINMAGI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhminmagi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhMinMagI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHMINMAGI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhminmagi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMinMagI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHMAXMAGI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhmaxmagi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhMaxMagI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHMAXMAGI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhmaxmagi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhMaxMagI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHRINTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhrinti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhRintI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHRINTI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhrinti_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRintI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHROUNDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhroundi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhRoundI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHROUNDI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhroundi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRoundI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHTRUNCI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhtrunci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhTruncI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHTRUNCI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhtrunci_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhTruncI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHEXPINT1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhexpint1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhExpInt1I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHEXPINT1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhexpint1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhExpInt1I_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHCOPYSIGNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhcopysigni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhCopySignI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHCOPYSIGNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhcopysigni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhCopySignI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHREMAINDERI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhremainderi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhRemainderI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr));
_MKL_VML_API(void, VMHREMAINDERI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhremainderi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 b[], const MKL_INT64 *incb, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhRemainderI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16 b[], const MKL_INT64 incb, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHERFCXI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vherfcxi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhErfcxI_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHERFCXI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmherfcxi_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhErfcxI_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHI0I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhi0i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhI0I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHI0I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhi0i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhI0I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHI1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhi1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhI1I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHI1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhi1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhI1I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHJ0I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhj0i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhJ0I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHJ0I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhj0i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhJ0I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHJ1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhj1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhJ1I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHJ1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhj1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhJ1I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHY0I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhy0i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhY0I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHY0I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhy0i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhY0I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHY1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhy1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhY1I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 inc6r));

_MKL_VML_API(void, VMHY1I_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhy1i_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhY1I_64, (const MKL_INT64 n,  const MKL_F16 a[], const MKL_INT64 inca, MKL_F16 r[],  const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHJNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhjni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhJnI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16, MKL_F16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMHJNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhjni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhJnI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

_MKL_VML_API(void, VHYNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr));
_mkl_vml_api(void, vhyni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr));
_Mkl_Vml_Api(void, vhYnI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16, MKL_F16 r[], const MKL_INT64 incr));

_MKL_VML_API(void, VMHYNI_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_mkl_vml_api(void, vmhyni_64, (const MKL_INT64 *n, const MKL_F16 a[], const MKL_INT64 *inca, const MKL_F16 *, MKL_F16 r[], const MKL_INT64 *incr, MKL_INT64 *mode));
_Mkl_Vml_Api(void, vmhYnI_64, (const MKL_INT64 n, const MKL_F16 a[], const MKL_INT64 inca, const MKL_F16, MKL_F16 r[], const MKL_INT64 incr, MKL_INT64 mode));

/*
//++
// VML PACK FUNCTION DECLARATIONS.
//--
*/
/* Positive Increment Indexing */
_MKL_VML_API(void,VSPACKI,(const MKL_INT *n, const float a[], const MKL_INT * incra, float y[]));
_MKL_VML_API(void,VDPACKI,(const MKL_INT *n, const double a[], const MKL_INT * incra, double y[]));
_mkl_vml_api(void,vspacki,(const MKL_INT *n, const float a[], const MKL_INT * incra, float y[]));
_mkl_vml_api(void,vdpacki,(const MKL_INT *n, const double a[], const MKL_INT * incra, double y[]));
_Mkl_Vml_Api(void,vsPackI,(const MKL_INT n, const float a[], const MKL_INT incra, float y[]));
_Mkl_Vml_Api(void,vdPackI,(const MKL_INT n, const double a[], const MKL_INT incra, double y[]));

_MKL_VML_API(void,VCPACKI,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT * incra, MKL_Complex8 y[]));
_MKL_VML_API(void,VZPACKI,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT * incra, MKL_Complex16 y[]));
_mkl_vml_api(void,vcpacki,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT * incra, MKL_Complex8 y[]));
_mkl_vml_api(void,vzpacki,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT * incra, MKL_Complex16 y[]));
_Mkl_Vml_Api(void,vcPackI,(const MKL_INT n, const MKL_Complex8 a[], const MKL_INT incra, MKL_Complex8 y[]));
_Mkl_Vml_Api(void,vzPackI,(const MKL_INT n, const MKL_Complex16 a[], const MKL_INT incra, MKL_Complex16 y[]));

/* Index Vector Indexing */
_MKL_VML_API(void,VSPACKV,(const MKL_INT *n, const float a[], const MKL_INT ia[], float y[]));
_MKL_VML_API(void,VDPACKV,(const MKL_INT *n, const double a[], const MKL_INT ia[], double y[]));
_mkl_vml_api(void,vspackv,(const MKL_INT *n, const float a[], const MKL_INT ia[], float y[]));
_mkl_vml_api(void,vdpackv,(const MKL_INT *n, const double a[], const MKL_INT ia[], double y[]));
_Mkl_Vml_Api(void,vsPackV,(const MKL_INT n, const float a[], const MKL_INT ia[], float y[]));
_Mkl_Vml_Api(void,vdPackV,(const MKL_INT n, const double a[], const MKL_INT ia[], double y[]));

_MKL_VML_API(void,VCPACKV,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT ia[], MKL_Complex8 y[]));
_MKL_VML_API(void,VZPACKV,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT ia[], MKL_Complex16 y[]));
_mkl_vml_api(void,vcpackv,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT ia[], MKL_Complex8 y[]));
_mkl_vml_api(void,vzpackv,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT ia[], MKL_Complex16 y[]));
_Mkl_Vml_Api(void,vcPackV,(const MKL_INT n, const MKL_Complex8 a[], const MKL_INT ia[], MKL_Complex8 y[]));
_Mkl_Vml_Api(void,vzPackV,(const MKL_INT n, const MKL_Complex16 a[], const MKL_INT ia[], MKL_Complex16 y[]));

/* Mask Vector Indexing */
_MKL_VML_API(void,VSPACKM,(const MKL_INT *n, const float a[], const MKL_INT ma[], float y[]));
_MKL_VML_API(void,VDPACKM,(const MKL_INT *n, const double a[], const MKL_INT ma[], double y[]));
_mkl_vml_api(void,vspackm,(const MKL_INT *n, const float a[], const MKL_INT ma[], float y[]));
_mkl_vml_api(void,vdpackm,(const MKL_INT *n, const double a[], const MKL_INT ma[], double y[]));
_Mkl_Vml_Api(void,vsPackM,(const MKL_INT n, const float a[], const MKL_INT ma[], float y[]));
_Mkl_Vml_Api(void,vdPackM,(const MKL_INT n, const double a[], const MKL_INT ma[], double y[]));

_MKL_VML_API(void,VCPACKM,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT ma[], MKL_Complex8 y[]));
_MKL_VML_API(void,VZPACKM,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT ma[], MKL_Complex16 y[]));
_mkl_vml_api(void,vcpackm,(const MKL_INT *n, const MKL_Complex8 a[], const MKL_INT ma[], MKL_Complex8 y[]));
_mkl_vml_api(void,vzpackm,(const MKL_INT *n, const MKL_Complex16 a[], const MKL_INT ma[], MKL_Complex16 y[]));
_Mkl_Vml_Api(void,vcPackM,(const MKL_INT n, const MKL_Complex8 a[], const MKL_INT ma[], MKL_Complex8 y[]));
_Mkl_Vml_Api(void,vzPackM,(const MKL_INT n, const MKL_Complex16 a[], const MKL_INT ma[], MKL_Complex16 y[]));

/*
//++
// VML UNPACK FUNCTION DECLARATIONS.
//--
*/
/* Positive Increment Indexing */
_MKL_VML_API(void,VSUNPACKI,(const MKL_INT *n, const float a[], float y[], const MKL_INT * incry));
_MKL_VML_API(void,VDUNPACKI,(const MKL_INT *n, const double a[], double y[], const MKL_INT * incry));
_mkl_vml_api(void,vsunpacki,(const MKL_INT *n, const float a[], float y[], const MKL_INT * incry));
_mkl_vml_api(void,vdunpacki,(const MKL_INT *n, const double a[], double y[], const MKL_INT * incry));
_Mkl_Vml_Api(void,vsUnpackI,(const MKL_INT n, const float a[], float y[], const MKL_INT incry ));
_Mkl_Vml_Api(void,vdUnpackI,(const MKL_INT n, const double a[], double y[], const MKL_INT incry ));

_MKL_VML_API(void,VCUNPACKI,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT * incry));
_MKL_VML_API(void,VZUNPACKI,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT * incry));
_mkl_vml_api(void,vcunpacki,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT * incry));
_mkl_vml_api(void,vzunpacki,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT * incry));
_Mkl_Vml_Api(void,vcUnpackI,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT incry ));
_Mkl_Vml_Api(void,vzUnpackI,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT incry ));

/* Index Vector Indexing */
_MKL_VML_API(void,VSUNPACKV,(const MKL_INT *n, const float a[], float y[], const MKL_INT iy[] ));
_MKL_VML_API(void,VDUNPACKV,(const MKL_INT *n, const double a[], double y[], const MKL_INT iy[] ));
_mkl_vml_api(void,vsunpackv,(const MKL_INT *n, const float a[], float y[], const MKL_INT iy[] ));
_mkl_vml_api(void,vdunpackv,(const MKL_INT *n, const double a[], double y[], const MKL_INT iy[] ));
_Mkl_Vml_Api(void,vsUnpackV,(const MKL_INT n, const float a[], float y[], const MKL_INT iy[] ));
_Mkl_Vml_Api(void,vdUnpackV,(const MKL_INT n, const double a[], double y[], const MKL_INT iy[] ));

_MKL_VML_API(void,VCUNPACKV,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT iy[]));
_MKL_VML_API(void,VZUNPACKV,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT iy[]));
_mkl_vml_api(void,vcunpackv,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT iy[]));
_mkl_vml_api(void,vzunpackv,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT iy[]));
_Mkl_Vml_Api(void,vcUnpackV,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT iy[]));
_Mkl_Vml_Api(void,vzUnpackV,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT iy[]));

/* Mask Vector Indexing */
_MKL_VML_API(void,VSUNPACKM,(const MKL_INT *n, const float a[], float y[], const MKL_INT my[] ));
_MKL_VML_API(void,VDUNPACKM,(const MKL_INT *n, const double a[], double y[], const MKL_INT my[] ));
_mkl_vml_api(void,vsunpackm,(const MKL_INT *n, const float a[], float y[], const MKL_INT my[] ));
_mkl_vml_api(void,vdunpackm,(const MKL_INT *n, const double a[], double y[], const MKL_INT my[] ));
_Mkl_Vml_Api(void,vsUnpackM,(const MKL_INT n, const float a[], float y[], const MKL_INT my[] ));
_Mkl_Vml_Api(void,vdUnpackM,(const MKL_INT n, const double a[], double y[], const MKL_INT my[] ));

_MKL_VML_API(void,VCUNPACKM,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT my[]));
_MKL_VML_API(void,VZUNPACKM,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT my[]));
_mkl_vml_api(void,vcunpackm,(const MKL_INT *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT my[]));
_mkl_vml_api(void,vzunpackm,(const MKL_INT *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT my[]));
_Mkl_Vml_Api(void,vcUnpackM,(const MKL_INT n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT my[]));
_Mkl_Vml_Api(void,vzUnpackM,(const MKL_INT n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT my[]));

/* Positive Increment Indexing */
_MKL_VML_API(void,VSPACKI_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 * incra, float y[]));
_MKL_VML_API(void,VDPACKI_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 * incra, double y[]));
_mkl_vml_api(void,vspacki_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 * incra, float y[]));
_mkl_vml_api(void,vdpacki_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 * incra, double y[]));
_Mkl_Vml_Api(void,vsPackI_64, (const MKL_INT64 n, const float a[], const MKL_INT64 incra, float y[]));
_Mkl_Vml_Api(void,vdPackI_64, (const MKL_INT64 n, const double a[], const MKL_INT64 incra, double y[]));

_MKL_VML_API(void,VCPACKI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 * incra, MKL_Complex8 y[]));
_MKL_VML_API(void,VZPACKI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 * incra, MKL_Complex16 y[]));
_mkl_vml_api(void,vcpacki_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 * incra, MKL_Complex8 y[]));
_mkl_vml_api(void,vzpacki_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 * incra, MKL_Complex16 y[]));
_Mkl_Vml_Api(void,vcPackI_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 incra, MKL_Complex8 y[]));
_Mkl_Vml_Api(void,vzPackI_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 incra, MKL_Complex16 y[]));

/* Index Vector Indexing */
_MKL_VML_API(void,VSPACKV_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 ia[], float y[]));
_MKL_VML_API(void,VDPACKV_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 ia[], double y[]));
_mkl_vml_api(void,vspackv_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 ia[], float y[]));
_mkl_vml_api(void,vdpackv_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 ia[], double y[]));
_Mkl_Vml_Api(void,vsPackV_64, (const MKL_INT64 n, const float a[], const MKL_INT64 ia[], float y[]));
_Mkl_Vml_Api(void,vdPackV_64, (const MKL_INT64 n, const double a[], const MKL_INT64 ia[], double y[]));

_MKL_VML_API(void,VCPACKV_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 ia[], MKL_Complex8 y[]));
_MKL_VML_API(void,VZPACKV_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 ia[], MKL_Complex16 y[]));
_mkl_vml_api(void,vcpackv_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 ia[], MKL_Complex8 y[]));
_mkl_vml_api(void,vzpackv_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 ia[], MKL_Complex16 y[]));
_Mkl_Vml_Api(void,vcPackV_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 ia[], MKL_Complex8 y[]));
_Mkl_Vml_Api(void,vzPackV_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 ia[], MKL_Complex16 y[]));

/* Mask Vector Indexing */
_MKL_VML_API(void,VSPACKM_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 ma[], float y[]));
_MKL_VML_API(void,VDPACKM_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 ma[], double y[]));
_mkl_vml_api(void,vspackm_64, (const MKL_INT64 *n, const float a[], const MKL_INT64 ma[], float y[]));
_mkl_vml_api(void,vdpackm_64, (const MKL_INT64 *n, const double a[], const MKL_INT64 ma[], double y[]));
_Mkl_Vml_Api(void,vsPackM_64, (const MKL_INT64 n, const float a[], const MKL_INT64 ma[], float y[]));
_Mkl_Vml_Api(void,vdPackM_64, (const MKL_INT64 n, const double a[], const MKL_INT64 ma[], double y[]));

_MKL_VML_API(void,VCPACKM_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 ma[], MKL_Complex8 y[]));
_MKL_VML_API(void,VZPACKM_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 ma[], MKL_Complex16 y[]));
_mkl_vml_api(void,vcpackm_64, (const MKL_INT64 *n, const MKL_Complex8 a[], const MKL_INT64 ma[], MKL_Complex8 y[]));
_mkl_vml_api(void,vzpackm_64, (const MKL_INT64 *n, const MKL_Complex16 a[], const MKL_INT64 ma[], MKL_Complex16 y[]));
_Mkl_Vml_Api(void,vcPackM_64, (const MKL_INT64 n, const MKL_Complex8 a[], const MKL_INT64 ma[], MKL_Complex8 y[]));
_Mkl_Vml_Api(void,vzPackM_64, (const MKL_INT64 n, const MKL_Complex16 a[], const MKL_INT64 ma[], MKL_Complex16 y[]));

/*
//++
// VML UNPACK FUNCTION DECLARATIONS.
//--
*/
/* Positive Increment Indexing */
_MKL_VML_API(void,VSUNPACKI_64, (const MKL_INT64 *n, const float a[], float y[], const MKL_INT64 * incry));
_MKL_VML_API(void,VDUNPACKI_64, (const MKL_INT64 *n, const double a[], double y[], const MKL_INT64 * incry));
_mkl_vml_api(void,vsunpacki_64, (const MKL_INT64 *n, const float a[], float y[], const MKL_INT64 * incry));
_mkl_vml_api(void,vdunpacki_64, (const MKL_INT64 *n, const double a[], double y[], const MKL_INT64 * incry));
_Mkl_Vml_Api(void,vsUnpackI_64, (const MKL_INT64 n, const float a[], float y[], const MKL_INT64 incry ));
_Mkl_Vml_Api(void,vdUnpackI_64, (const MKL_INT64 n, const double a[], double y[], const MKL_INT64 incry ));

_MKL_VML_API(void,VCUNPACKI_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 * incry));
_MKL_VML_API(void,VZUNPACKI_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 * incry));
_mkl_vml_api(void,vcunpacki_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 * incry));
_mkl_vml_api(void,vzunpacki_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 * incry));
_Mkl_Vml_Api(void,vcUnpackI_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 incry ));
_Mkl_Vml_Api(void,vzUnpackI_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 incry ));

/* Index Vector Indexing */
_MKL_VML_API(void,VSUNPACKV_64, (const MKL_INT64 *n, const float a[], float y[], const MKL_INT64 iy[] ));
_MKL_VML_API(void,VDUNPACKV_64, (const MKL_INT64 *n, const double a[], double y[], const MKL_INT64 iy[] ));
_mkl_vml_api(void,vsunpackv_64, (const MKL_INT64 *n, const float a[], float y[], const MKL_INT64 iy[] ));
_mkl_vml_api(void,vdunpackv_64, (const MKL_INT64 *n, const double a[], double y[], const MKL_INT64 iy[] ));
_Mkl_Vml_Api(void,vsUnpackV_64, (const MKL_INT64 n, const float a[], float y[], const MKL_INT64 iy[] ));
_Mkl_Vml_Api(void,vdUnpackV_64, (const MKL_INT64 n, const double a[], double y[], const MKL_INT64 iy[] ));

_MKL_VML_API(void,VCUNPACKV_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 iy[]));
_MKL_VML_API(void,VZUNPACKV_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 iy[]));
_mkl_vml_api(void,vcunpackv_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 iy[]));
_mkl_vml_api(void,vzunpackv_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 iy[]));
_Mkl_Vml_Api(void,vcUnpackV_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 iy[]));
_Mkl_Vml_Api(void,vzUnpackV_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 iy[]));

/* Mask Vector Indexing */
_MKL_VML_API(void,VSUNPACKM_64, (const MKL_INT64 *n, const float a[], float y[], const MKL_INT64 my[] ));
_MKL_VML_API(void,VDUNPACKM_64, (const MKL_INT64 *n, const double a[], double y[], const MKL_INT64 my[] ));
_mkl_vml_api(void,vsunpackm_64, (const MKL_INT64 *n, const float a[], float y[], const MKL_INT64 my[] ));
_mkl_vml_api(void,vdunpackm_64, (const MKL_INT64 *n, const double a[], double y[], const MKL_INT64 my[] ));
_Mkl_Vml_Api(void,vsUnpackM_64, (const MKL_INT64 n, const float a[], float y[], const MKL_INT64 my[] ));
_Mkl_Vml_Api(void,vdUnpackM_64, (const MKL_INT64 n, const double a[], double y[], const MKL_INT64 my[] ));

_MKL_VML_API(void,VCUNPACKM_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 my[]));
_MKL_VML_API(void,VZUNPACKM_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 my[]));
_mkl_vml_api(void,vcunpackm_64, (const MKL_INT64 *n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 my[]));
_mkl_vml_api(void,vzunpackm_64, (const MKL_INT64 *n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 my[]));
_Mkl_Vml_Api(void,vcUnpackM_64, (const MKL_INT64 n, const MKL_Complex8 a[], MKL_Complex8 y[], const MKL_INT64 my[]));
_Mkl_Vml_Api(void,vzUnpackM_64, (const MKL_INT64 n, const MKL_Complex16 a[], MKL_Complex16 y[], const MKL_INT64 my[]));

/*
//++
// VML ERROR HANDLING FUNCTION DECLARATIONS.
//--
*/
/* Set VML Error Status */
_MKL_VML_API(int,VMLSETERRSTATUS,(const MKL_INT * status));
_mkl_vml_api(int,vmlseterrstatus,(const MKL_INT * status));
_Mkl_Vml_Api(int,vmlSetErrStatus,(const MKL_INT status));

/* Get VML Error Status */
_MKL_VML_API(int,VMLGETERRSTATUS,(void));
_mkl_vml_api(int,vmlgeterrstatus,(void));
_Mkl_Vml_Api(int,vmlGetErrStatus,(void));

/* Clear VML Error Status */
_MKL_VML_API(int,VMLCLEARERRSTATUS,(void));
_mkl_vml_api(int,vmlclearerrstatus,(void));
_Mkl_Vml_Api(int,vmlClearErrStatus,(void));

/* Set VML Error Callback Function */
_MKL_VML_API(VMLErrorCallBack,VMLSETERRORCALLBACK,(const VMLErrorCallBack func));
_mkl_vml_api(VMLErrorCallBack,vmlseterrorcallback,(const VMLErrorCallBack func));
_Mkl_Vml_Api(VMLErrorCallBack,vmlSetErrorCallBack,(const VMLErrorCallBack func));

/* Get VML Error Callback Function */
_MKL_VML_API(VMLErrorCallBack,VMLGETERRORCALLBACK,(void));
_mkl_vml_api(VMLErrorCallBack,vmlgeterrorcallback,(void));
_Mkl_Vml_Api(VMLErrorCallBack,vmlGetErrorCallBack,(void));

/* Reset VML Error Callback Function */
_MKL_VML_API(VMLErrorCallBack,VMLCLEARERRORCALLBACK,(void));
_mkl_vml_api(VMLErrorCallBack,vmlclearerrorcallback,(void));
_Mkl_Vml_Api(VMLErrorCallBack,vmlClearErrorCallBack,(void));

/* Set VML Error Status */
_MKL_VML_API(MKL_INT64,VMLSETERRSTATUS_64, (const MKL_INT64 * status));
_mkl_vml_api(MKL_INT64,vmlseterrstatus_64, (const MKL_INT64 * status));
_Mkl_Vml_Api(MKL_INT64,vmlSetErrStatus_64, (const MKL_INT64 status));

/* Get VML Error Status */
_MKL_VML_API(MKL_INT64,VMLGETERRSTATUS_64, (void));
_mkl_vml_api(MKL_INT64,vmlgeterrstatus_64, (void));
_Mkl_Vml_Api(MKL_INT64,vmlGetErrStatus_64, (void));

/* Clear VML Error Status */
_MKL_VML_API(MKL_INT64,VMLCLEARERRSTATUS_64, (void));
_mkl_vml_api(MKL_INT64,vmlclearerrstatus_64, (void));
_Mkl_Vml_Api(MKL_INT64,vmlClearErrStatus_64, (void));

/* Set VML Error Callback Function */
_MKL_VML_API(VMLErrorCallBack,VMLSETERRORCALLBACK_64, (const VMLErrorCallBack func));
_mkl_vml_api(VMLErrorCallBack,vmlseterrorcallback_64, (const VMLErrorCallBack func));
_Mkl_Vml_Api(VMLErrorCallBack,vmlSetErrorCallBack_64, (const VMLErrorCallBack func));

/* Get VML Error Callback Function */
_MKL_VML_API(VMLErrorCallBack,VMLGETERRORCALLBACK_64, (void));
_mkl_vml_api(VMLErrorCallBack,vmlgeterrorcallback_64, (void));
_Mkl_Vml_Api(VMLErrorCallBack,vmlGetErrorCallBack_64, (void));

/* Reset VML Error Callback Function */
_MKL_VML_API(VMLErrorCallBack,VMLCLEARERRORCALLBACK_64, (void));
_mkl_vml_api(VMLErrorCallBack,vmlclearerrorcallback_64, (void));
_Mkl_Vml_Api(VMLErrorCallBack,vmlClearErrorCallBack_64, (void));

/*
//++
// VML MODE FUNCTION DECLARATIONS.
//--
*/
/* Set VML Mode */
_MKL_VML_API(unsigned int,VMLSETMODE,(const MKL_UINT *newmode));
_mkl_vml_api(unsigned int,vmlsetmode,(const MKL_UINT *newmode));
_Mkl_Vml_Api(unsigned int,vmlSetMode,(const MKL_UINT newmode));

/* Get VML Mode */
_MKL_VML_API(unsigned int,VMLGETMODE,(void));
_mkl_vml_api(unsigned int,vmlgetmode,(void));
_Mkl_Vml_Api(unsigned int,vmlGetMode,(void));

_MKL_VML_API(void,MKLFREETLS,(const MKL_UINT *fdwReason));
_mkl_vml_api(void,mklfreetls,(const MKL_UINT *fdwReason));
_Mkl_Vml_Api(void,MKLFreeTls,(const MKL_UINT  fdwReason));

/* Set VML Mode */
_MKL_VML_API(MKL_UINT64,VMLSETMODE_64, (const MKL_UINT64 *newmode));
_mkl_vml_api(MKL_UINT64,vmlsetmode_64, (const MKL_UINT64 *newmode));
_Mkl_Vml_Api(MKL_UINT64,vmlSetMode_64, (const MKL_UINT64 newmode));

/* Get VML Mode */
_MKL_VML_API(MKL_UINT64,VMLGETMODE_64, (void));
_mkl_vml_api(MKL_UINT64,vmlgetmode_64, (void));
_Mkl_Vml_Api(MKL_UINT64,vmlGetMode_64, (void));

_MKL_VML_API(void,MKLFREETLS_64, (const MKL_UINT64 *fdwReason));
_mkl_vml_api(void,mklfreetls_64, (const MKL_UINT64 *fdwReason));
_Mkl_Vml_Api(void,MKLFreeTls_64, (const MKL_UINT64  fdwReason));

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* __MKL_VML_FUNCTIONS_H__ */
