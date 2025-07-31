/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: DesignWare kunpeng libmath interface definition.
 * Author:
 * Create: 2020-08-28
 */

#ifndef KML_VML_H
#define KML_VML_H
#ifdef __cplusplus
extern "C" {
#endif

#ifndef _Complex_I
#define _Complex_I (__extension__ 1.0iF)
#endif
#undef I
#define I _Complex_I

typedef struct complex_fp16 {
    __fp16 real;
    __fp16 imag;
} complex_fp16;

#define KVML_VERSION_STRUCT_LEN 100
typedef struct {
    char productName[KVML_VERSION_STRUCT_LEN];
    char productVersion[KVML_VERSION_STRUCT_LEN];
    char componentName[KVML_VERSION_STRUCT_LEN];
    char componentVersion[KVML_VERSION_STRUCT_LEN];
    char componentAppendInfo[KVML_VERSION_STRUCT_LEN];
    char softwareName[KVML_VERSION_STRUCT_LEN];
    char softwareVersion[KVML_VERSION_STRUCT_LEN];
}KVMLVersion;
int KVMLGetVersion(KVMLVersion* ver);

/**
 *  KML_VML FUNCTION ACCURACY CONTROL
 *  KML_HA - when KML_HA is set, high accuracy VML functions are called
 *  KML_LA - when KML_LA is set, low accuracy VML functions are called
 *  KML_EP - when KML_EP is set, enhanced performance VML functions are called
 * */
#define KML_LA 0x1
#define KML_HA 0x2
#define KML_EP 0x3

/**
 * @Brief computes abs of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * @param[in]       mode        Function precision mode[La:1,Ha:2,Ep:3].
 * */
void vsabs(const int len, const float *src, float *dst);
void vdabs(const int len, const double *src, double *dst);
void vcabs(const int len, const float _Complex *src, float *dst);
void vzabs(const int len, const double _Complex *src, double *dst);
void vmsabs(const int len, const float *src, float *dst, long long int mode);

/**
 * @Brief computes inverse cos of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointer to the source vector.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vsacos(const int len, const float *src, float *dst);
void vdacos(const int len, const double *src, double *dst);
void vcacos(const int len, const float _Complex *src, float _Complex *dst);
void vzacos(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes acospi of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsacospi(const int len, const float *src, float *dst);
void vdacospi(const int len, const double *src, double *dst);

/**
 * @Brief Adds the elements of two vectors.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src1, src2 	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vhadd(const int len, const __fp16 *src1, const __fp16 *src2, __fp16 *dst);
void vsadd(const int len, const float *src1, const float *src2, float *dst);
void vdadd(const int len, const double *src1, const double *src2, double *dst);
void vchadd(const int len, const complex_fp16 *src1, const complex_fp16 *src2, complex_fp16 *dst);
void vcadd(const int len, const float _Complex *src1, const float _Complex *src2, float _Complex *dst);
void vzadd(const int len, const double _Complex *src1, const double _Complex *src2, double _Complex *dst);

/**
 * @Brief computes the argument of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vcarg(const int len, const float _Complex *src, float *dst);
void vzarg(const int len, const double _Complex *src, double *dst);

/**
 * @Brief computes inverse sin of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsasin(const int len, const float *src, float *dst);
void vdasin(const int len, const double *src, double *dst);
void vcasin(const int len, const float _Complex *src, float _Complex *dst);
void vzasin(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes asinpi of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsasinpi(const int len, const float *src, float *dst);
void vdasinpi(const int len, const double *src, double *dst);

/**
 * @Brief computes cdfnorm of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vscdfnorm(const int len, const float *src, float *dst);
void vdcdfnorm(const int len, const double *src, double *dst);

/**
 * @Brief computes cdfnorminv of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vscdfnorminv(const int len, const float *src, float *dst);
void vdcdfnorminv(const int len, const double *src, double *dst);

/**
 * @Brief computes an integer value rounded towards plus infinity of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsceil(const int len, const float *src, float *dst);
void vdceil(const int len, const double *src, double *dst);

/**
 * @Brief computes cis of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vccis(const int len, const float *src, float _Complex *dst);
void vzcis(const int len, const double *src, double _Complex *dst);

/**
 * @Brief performs element by element division of vector.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointer to the source vector.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vcconj(const int len, const float _Complex *src, float _Complex *dst);
void vzconj(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief returns the first argument with the sign changed to match that of the second argument
 * @param[in]		len			Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vscopysign(const int len, const float *src1, const float *src2, float *dst);
void vdcopysign(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes cospi of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vscospi(const int len, const float *src, float *dst);
void vdcospi(const int len, const double *src, double *dst);

/**
 * @Brief computes an erf function of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vserf(const int len, const float *src, float *dst);
void vderf(const int len, const double *src, double *dst);

/**
 * @Brief computes an erfc function of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vserfc(const int len, const float *src, float *dst);
void vderfc(const int len, const double *src, double *dst);

/**
 * @Brief computes erfcinv of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vserfcinv(const int len, const float *src, float *dst);
void vderfcinv(const int len, const double *src, double *dst);

/**
 * @Brief computes the exponential integral of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsexpint1(const int len, const float *src, float *dst);
void vdexpint1(const int len, const double *src, double *dst);

/**
 * @Brief computes the difference between the first argument and the second argument if the first
 * argument is larger than the second one
 * @param[in]		len			Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsfdim(const int len, const float *src1, const float *src2, float *dst);
void vdfdim(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes erfinv of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vserfinv(const int len, const float *src, float *dst);
void vderfinv(const int len, const double *src, double *dst);

/**
 * @Brief computes an integer value rounded towards minus infinity of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsfloor(const int len, const float *src, float *dst);
void vdfloor(const int len, const double *src, double *dst);

/**
 * @Brief performs element by element conjugation of vector.
 * @Brief performs maximum elements of vector.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vhfmax(const int len, const __fp16 *src1, const __fp16 *src2, __fp16 *dst);
void vsfmax(const int len, const float *src1, const float *src2, float *dst);
void vdfmax(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief performs minimum elements of vector.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vhfmin(const int len, const __fp16 *src1, const __fp16 *src2, __fp16 *dst);
void vsfmin(const int len, const float *src1, const float *src2, float *dst);
void vdfmin(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes modulus of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointer to the source vector.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vsfmod(const int len, const float *src1, const float *src2, float *dst);
void vdfmod(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes the fractional part of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsfrac(const int len, const float *src, float *dst);
void vdfrac(const int len, const double *src, double *dst);

/**
 * @Brief computes the natural logarithm of the absolute value of gamma function for vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vslgamma(const int len, const float *src, float *dst);
void vdlgamma(const int len, const double *src, double *dst);

/**
 * @Brief Performs linear fraction transformation of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src1       	Pointer to the source vector a.
 * @param[in]  		src2       	Pointer to the source vector b.
 * @param[in]  		scalea     	Const scalea for linearfrac.
 * @param[in]  		shifta     	Const shifta for linearfrac.
 * @param[in]  		scaleb     	Const scaleb for linearfrac.
 * @param[in]  		shiftb   	Const shiftb for linearfrac.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vslinearfrac(const int len, const float *src1, const float *src2, const float scalea, const float shifta,
    const float scaleb, const float shiftb, float *dst);
void vdlinearfrac(const int len, const double *src1, const double *src2, const double scalea, const double shifta,
    const double scaleb, const double shiftb, double *dst);

/**
 * @Returns the element with the larger magnitude between each pair of elements of the two vector arguments.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vsmaxmag(const int len, const float *src1, const float *src2, float *dst);
void vdmaxmag(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Returns the element with the smaller magnitude between each pair of elements of the two vector arguments.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vsminmag(const int len, const float *src1, const float *src2, float *dst);
void vdminmag(const int len, const double *src1, const double *src2, double *dst);


/**
 * @Brief computes a truncated integer value a and the remaining fraction part b of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst1   	    Pointer to the destination vector a.
 * @param[out]  	dst2   	    Pointer to the destination vector b.
 * */
void vsmodf(const int len, const float *src, float *dst1, float *dst2);
void vdmodf(const int len, const double *src, double *dst1, double *dst2);

/**
 * @Brief computes  the next representable floating-point values of the first element in the direction of the
 * second element
 * @param[in]		len			Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsnextafter(const int len, const float *src1, const float *src2, float *dst);
void vdnextafter(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes remainder of vector elements of a and b.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the source vector a.
 * @param[in]       src2        Pointer to the source vector b.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vsremainder(const int len, const float *src1, const float *src2, float *dst);
void vdremainder(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief Subs the elements of two vectors.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src1, src2 	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vhsub(const int len, const __fp16 *src1, const __fp16 *src2, __fp16 *dst);
void vssub(const int len, const float *src1, const float *src2, float *dst);
void vdsub(const int len, const double *src1, const double *src2, double *dst);
void vchsub(const int len, const complex_fp16 *src1, const complex_fp16 *src2, complex_fp16 *dst);
void vcsub(const int len, const float _Complex *src1, const float _Complex *src2, float _Complex *dst);
void vzsub(const int len, const double _Complex *src1, const double _Complex *src2, double _Complex *dst);

/**
 * @Brief performs element by element squaring of the vector.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * @param[in]       mode        Function precision mode[La:1,Ha:2,Ep:3].
 * */
void vssqr(const int len, const float *src, float *dst);
void vdsqr(const int len, const double *src, double *dst);
void vmssqr(const int len, const float *src, float *dst, long long int mode);

/**
 * @BriefComputes a inverse square root of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vsinvsqrt(const int len, const float *src, float *dst);
void vdinvsqrt(const int len, const double *src, double *dst);

/**
 * @BriefComputes a square root of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 * @param[in]       mode        Function precision mode[La:1,Ha:2,Ep:3].
 **/
void vssqrt(const int len, const float *src, float *dst);
void vdsqrt(const int len, const double *src, double *dst);
void vcsqrt(const int len, const float _Complex *src, float _Complex *dst);
void vzsqrt(const int len, const double _Complex *src, double _Complex *dst);
void vmssqrt(const int len, const float *src, float *dst, long long int mode);

/**
 * @BriefComputes a cube root of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vscbrt(const int len, const float *src, float *dst);
void vdcbrt(const int len, const double *src, double *dst);

/**
 * @BriefComputes a inverse cube root of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vsinvcbrt(const int len, const float *src, float *dst);
void vdinvcbrt(const int len, const double *src, double *dst);

/**
 * @Brief Performs element by element multiplication of vector.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1, src2  Pointers to the source vectors.
 * @param[out]      p _dst      Pointer to the destination vector.
 **/
void vhmul(const int len, const __fp16 *src1, const __fp16 *src2, __fp16 *dst);
void vsmul(const int len, const float *src1, const float *src2, float *dst);
void vdmul(const int len, const double *src1, const double *src2, double *dst);
void vchmul(const int len, const complex_fp16 *src1, const complex_fp16 *src2, complex_fp16 *dst);
void vcmul(const int len, const float _Complex *src1, const float _Complex *src2, float _Complex *dst);
void vzmul(const int len, const double _Complex *src1, const double _Complex *src2, double _Complex *dst);

/**
 * @Brief performs element by element multiplication of src1 vector and conjugated src2 vector.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the first source vector.
 * @param[in]       src2        Pointer to the second source vector.
 * @param[out]      dst         Pointer to the destination vector.
 **/
void vcmulbyconj(const int len, const float _Complex *src1, const float _Complex *src2, float _Complex *dst);
void vzmulbyconj(const int len, const double _Complex *src1, const double _Complex *src2, double _Complex *dst);

/**
 * @Brief computes an integer value rounded towards of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsnearbyint(const int len, const float *src, float *dst);
void vdnearbyint(const int len, const double *src, double *dst);

/**
 * @Brief performs element by element division of vector.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1, src2  Pointers to the source vectors.
 * @param[out]      p _dst      Pointer to the destination vector.
 **/
void vhdiv(const int len, const __fp16 *src1, const __fp16 *src2, __fp16 *dst);
void vsdiv(const int len, const float *src1, const float *src2, float *dst);
void vddiv(const int len, const double *src1, const double *src2, double *dst);
void vchdiv(const int len, const complex_fp16 *src1, const complex_fp16 *src2, complex_fp16 *dst);
void vcdiv(const int len, const float _Complex *src1, const float _Complex *src2, float _Complex *dst);
void vzdiv(const int len, const double _Complex *src1, const double _Complex *src2, double _Complex *dst);

/**
 * @Brief computes an exponential of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * @param[in]       mode        Accuary mode.
 * */
void vsexp(const int len, const float *src, float *dst);
void vdexp(const int len, const double *src, double *dst);
void vcexp(const int len, const float _Complex *src, float _Complex *dst);
void vzexp(const int len, const double _Complex *src, double _Complex *dst);
void vmsexp(const int len, const float *src, float *dst, long long int mode);

/**
 * @Brief computes the base 2 exponential of vector elements
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsexp2(const int len, const float *src, float *dst);
void vdexp2(const int len, const double *src, double *dst);

/**
 * @Brief computes the base 10 exponential of vector elements
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsexp10(const int len, const float *src, float *dst);
void vdexp10(const int len, const double *src, double *dst);

/**
 * @Brief computes the base e exponential of vector elements decreased by 1
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsexpm1(const int len, const float *src, float *dst);
void vdexpm1(const int len, const double *src, double *dst);

/**
 * @Brief computes ln of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * @param[in]       mode        Function precision mode[La:1,Ha:2,Ep:3].
 * */
void vsln(const int len, const float *src, float *dst);
void vdln(const int len, const double *src, double *dst);
void vcln(const int len, const float _Complex *src, float _Complex *dst);
void vzln(const int len, const double _Complex *src, double _Complex *dst);
void vmsln(const int len, const float *src, float *dst, long long int mode);

/**
 * @Brief computes log2 of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vslog2(const int len, const float *src, float *dst);
void vdlog2(const int len, const double *src, double *dst);

/**
 * @Brief computes log10 of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vslog10(const int len, const float *src, float *dst);
void vdlog10(const int len, const double *src, double *dst);
void vclog10(const int len, const float _Complex *src, float _Complex *dst);
void vzlog10(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes the natural logarithm of vector elements that are increased by 1
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vslog1p(const int len, const float *src, float *dst);
void vdlog1p(const int len, const double *src, double *dst);

/**
 * @Brief computes the exponents of the elements of input vector
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vslogb(const int len, const float *src, float *dst);
void vdlogb(const int len, const double *src, double *dst);

/**
 * @Computes tangent of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vstan(const int len, const float *src, float *dst);
void vdtan(const int len, const double *src, double *dst);
void vctan(const int len, const float _Complex *src, float _Complex *dst);
void vztan(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes tand of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vstand(const int len, const float *src, float *dst);
void vdtand(const int len, const double *src, double *dst);

/**
 * @Computes inverse tangent of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsatan(const int len, const float *src, float *dst);
void vdatan(const int len, const double *src, double *dst);
void vcatan(const int len, const float _Complex *src, float _Complex *dst);
void vzatan(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes atanpi of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsatanpi(const int len, const float *src, float *dst);
void vdatanpi(const int len, const double *src, double *dst);

/**
 * @Computes inverse tangent of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointers to the source vectors.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsatan2(const int len, const float *src1, const float *src2, float *dst);
void vdatan2(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes inverse tangent of vector elements divided by pi.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src1       	Pointer to the source vector y.
 * @param[in]  		src2       	Pointer to the source vector x.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vsatan2pi(const int len, const float *src1, const float *src2, float *dst);
void vdatan2pi(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes pow of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vspow(const int len, const float *src1, const float *src2, float *dst);
void vdpow(const int len, const double *src1, const double *src2, double *dst);
void vcpow(const int len, const float _Complex *src1, const float _Complex *src2, float _Complex *dst);
void vzpow(const int len, const double _Complex *src1, const double _Complex *src2, double _Complex *dst);

/**
 * @Brief computes pow of vector elements a and b, where b is scalar.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the source vectors.
 * @param[in]       src2        Scalar value for power b.
 * @param[out]      dst         Pointer to the destination vector.
 * @param[in]       mode        Function precision mode[La:1,Ha:2,Ep:3].
 * */
void vspowx(const int len, const float *src1, const float src2, float *dst);
void vdpowx(const int len, const double *src1, const double src2, double *dst);
void vcpowx(const int len, const float _Complex *src1, const float _Complex src2, float _Complex *dst);
void vzpowx(const int len, const double _Complex *src1, const double _Complex src2, double _Complex *dst);
void vmspowx(const int len, const float *src1, const float src2, float *dst, long long int mode);
/**
 * @Brief computes pow of vector elements a and b, where a >= 0.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src1        Pointer to the source vector a.
 * @param[in]       src2        Pointer to the source vector b.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vspowr(const int len, const float *src1, const float *src2, float *dst);
void vdpowr(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes (src1^2 + src2^2)^0.5 of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src1       	Pointer to the first source vector.
 * @param[in]  		src2       	Pointer to the second source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vshypot(const int len, const float *src1, const float *src2, float *dst);
void vdhypot(const int len, const double *src1, const double *src2, double *dst);

/**
 * @Brief computes pow2o3 of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vspow2o3(const int len, const float *src, float *dst);
void vdpow2o3(const int len, const double *src, double *dst);

/**
 * @Brief computes pow3o2 of vector elements.
 * @param[in]       len         Number of elements in the vector
 * @param[in]       src         Pointers to the source vectors.
 * @param[out]      dst         Pointer to the destination vector.
 * */
void vspow3o2(const int len, const float *src, float *dst);
void vdpow3o2(const int len, const double *src, double *dst);

/**
 * @Brief computes an integer value rounded towards of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsrint(const int len, const float *src, float *dst);
void vdrint(const int len, const double *src, double *dst);

/**
 * @Brief computes an integer value rounded towards nearest of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsround(const int len, const float *src, float *dst);
void vdround(const int len, const double *src, double *dst);

/**
 * @Brief computes sine of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vssin(const int len, const float *src, float *dst);
void vdsin(const int len, const double *src, double *dst);
void vcsin(const int len, const float _Complex *src, float _Complex *dst);
void vzsin(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes sinpi of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vssinpi(const int len, const float *src, float *dst);
void vdsinpi(const int len, const double *src, double *dst);

/**
 * @Brief computes cos of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vscos(const int len, const float *src, float *dst);
void vdcos(const int len, const double *src, double *dst);
void vccos(const int len, const float _Complex *src, float _Complex *dst);
void vzcos(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes cos degree of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vscosd(const int len, const float *src, float *dst);
void vdcosd(const int len, const double *src, double *dst);

/**
 * @Brief computes inv of vector elements.
 * @param[in]   	len         Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsinv(const int len, const float *src, float *dst);
void vdinv(const int len, const double *src, double *dst);

/**
 * @Brief computes cos sin of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src     	Pointer to the source vector.
 * @param[out]  	sindst   	Pointer to the destination vector.
 * @param[out]  	cosdst   	Pointer to the destination vector.
 * */
void vssincos(const int len, const float *src, float *sindst, float *cosdst);
void vdsincos(const int len, const double *src, double *sindst, double *cosdst);

/**
 * @Brief computes sine degree of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vssind(const int len, const float *src, float *dst);
void vdsind(const int len, const double *src, double *dst);

/**
 * @Brief computes sinh of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vssinh(const int len, const float *src, float *dst);
void vdsinh(const int len, const double *src, double *dst);
void vcsinh(const int len, const float _Complex *src, float _Complex *dst);
void vzsinh(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes cos of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vscosh(const int len, const float *src, float *dst);
void vdcosh(const int len, const double *src, double *dst);
void vccosh(const int len, const float _Complex *src, float _Complex *dst);
void vzcosh(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes sinh of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsasinh(const int len, const float *src, float *dst);
void vdasinh(const int len, const double *src, double *dst);
void vcasinh(const int len, const float _Complex *src, float _Complex *dst);
void vzasinh(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes cos of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsacosh(const int len, const float *src, float *dst);
void vdacosh(const int len, const double *src, double *dst);
void vcacosh(const int len, const float _Complex *src, float _Complex *dst);
void vzacosh(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes tanh of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * @param[in]       mode        Function precision mode[La:1,Ha:2,Ep:3].
 * */
void vstanh(const int len, const float *src, float *dst);
void vdtanh(const int len, const double *src, double *dst);
void vctanh(const int len, const float _Complex *src, float _Complex *dst);
void vztanh(const int len, const double _Complex *src, double _Complex *dst);
void vmstanh(const int len, const float *src, float *dst, long long int mode);

/**
 * @Brief computes atanh of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vsatanh(const int len, const float *src, float *dst);
void vdatanh(const int len, const double *src, double *dst);
void vcatanh(const int len, const float _Complex *src, float _Complex *dst);
void vzatanh(const int len, const double _Complex *src, double _Complex *dst);

/**
 * @Brief computes tanpi of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vstanpi(const int len, const float *src, float *dst);
void vdtanpi(const int len, const double *src, double *dst);

/**
 * @Brief computes the gamma function of vector elements.
 * @param[in]		len			Number of elements in the vector
 * @param[in]  		src       	Pointer to the source vector.
 * @param[out] 		dst        	Pointer to the destination vector.
 * */
void vstgamma(const int len, const float *src, float *dst);
void vdtgamma(const int len, const double *src, double *dst);

/**
 * @Brief computes an integer value rounded towards zero of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointers to the source vectors.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vstrunc(const int len, const float *src, float *dst);
void vdtrunc(const int len, const double *src, double *dst);

/**
 * @Brief convert the argument of vector elements.
 * @param[in]   	len    		Number of elements in the vector
 * @param[in]   	src   	    Pointer to the source vector.
 * @param[out]  	dst   	    Pointer to the destination vector.
 * */
void vfloatcvtf16(const int len, const float *src, __fp16 *dst);
void vf16cvtfloat(const int len, const __fp16 *src, float *dst);

/* thread control functions */
 
/**
 * @Brief Specifies the number of OpenMP threads for all vml functions.
 * @param[in]       nThreads    The Number of threads suggested by the user
 * */
void KmlVmlSetNumThreads(int nThreads);
 
/**
 * @Brief Specifies the number of OpenMP threads for all vml functions on the current execution thread.
 * Caution: If your application is threaded with OpenMp and parallelization of VML is based on nested OpenMp
 * parallelism, you should call omp_set_nested(1) at advance.
 * @param[in]       nThreads    The Number of threads suggested by the user
 * @return          save_nt     Thread-local number of threads that was used before
 * */
int KmlVmlSetNumThreadsLocal(int nThreads);
 
/**
 * @Brief Get the number of OpenMp threads targeted for parallelism.
 * */
int KmlVmlGetMaxThreads(void);

#ifdef __cplusplus
}
#endif

#endif