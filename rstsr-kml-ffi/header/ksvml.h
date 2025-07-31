/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: DesignWare kunpeng libmath interface definition.
 * Author:
 * Create: 2020-08-28
 */

#ifndef KML_SVML_H
#define KML_SVML_H

#include <arm_neon.h>
#ifdef __ARM_FEATURE_SVE
#include <arm_sve.h>
#endif
#ifdef __cplusplus
extern "C" {
#endif
#define KSVML_VERSION_STRUCT_LEN 100
typedef struct {
    char productName[KSVML_VERSION_STRUCT_LEN];
    char productVersion[KSVML_VERSION_STRUCT_LEN];
    char componentName[KSVML_VERSION_STRUCT_LEN];
    char componentVersion[KSVML_VERSION_STRUCT_LEN];
    char componentAppendInfo[KSVML_VERSION_STRUCT_LEN];
    char softwareName[KSVML_VERSION_STRUCT_LEN];
    char softwareVersion[KSVML_VERSION_STRUCT_LEN];
}KSVMLVersion;
int KSVMLGetVersion(KSVMLVersion* ver);
#ifdef AARCH64_VECTOR
#define AARCH64_VECTOR_ATTR __attribute__((aarch64_vector_pcs))
#else
#define AARCH64_VECTOR_ATTR
#endif
/**
 * @Computes log of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_log_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_log_f64(float64x2_t src);

/**
 * @Computes log base 10 of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_log10_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_log10_f64(float64x2_t src);

/**
 * @Computes log of vector elements plus one.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_log1p_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_log1p_f64(float64x2_t src);

/**
 * @Computes tangent of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_tan_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_tan_f64(float64x2_t src);

/**
 * @Computes exponential of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_exp_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_exp_f64(float64x2_t src);

/**
 * @Computes power of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_pow_f32(float32x4_t src1, float32x4_t src2);
AARCH64_VECTOR_ATTR float64x2_t svml128_pow_f64(float64x2_t src1, float64x2_t src2);

/**
 * @Computes inverse tangent of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_atan_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_atan_f64(float64x2_t src);

/**
 * @Computes inverse tangent of vector elements' quotient.
 * @param[in]  		src1       	source vectors.
 * @param[in]  		src2       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_atan2_f32(float32x4_t src1, float32x4_t src2);
AARCH64_VECTOR_ATTR float64x2_t svml128_atan2_f64(float64x2_t src1, float64x2_t src2);

/**
 * @Computes cos of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_cos_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_cos_f64(float64x2_t src);

/**
 * @Computes sin of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_sin_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_sin_f64(float64x2_t src);

/**
 * @Computes tanh of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_tanh_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_tanh_f64(float64x2_t src);

/**
 * @Computes expm1 of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_expm1_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_expm1_f64(float64x2_t src);

/**
 * @Computes sinh of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_sinh_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_sinh_f64(float64x2_t src);

/**
 * @Computes exp2 of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_exp2_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_exp2_f64(float64x2_t src);


/**
 * @Computes cosh of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out] 		dst        	destination vector.
 * */
AARCH64_VECTOR_ATTR float32x4_t svml128_cosh_f32(float32x4_t src);
AARCH64_VECTOR_ATTR float64x2_t svml128_cosh_f64(float64x2_t src);

/**
 * @Computes sin and cos of vector elements.
 * @param[in]  		src       	source vectors.
 * @param[out]  		sindst       	sin destination vector.
 * @param[out]  		cosdst       	cos destination vector.
 * */
void svml128_sincos_f32(float32x4_t src, float32x4_t *sindst, float32x4_t *cosdst);
void svml128_sincos_f64(float64x2_t src, float64x2_t *sindst, float64x2_t *cosdst);

#ifdef __ARM_FEATURE_SVE
svfloat32_t svml_log_f32(svfloat32_t src);
svfloat64_t svml_log_f64(svfloat64_t src);
svfloat32_t svml_log10_f32(svfloat32_t src);
svfloat64_t svml_log10_f64(svfloat64_t src);
svfloat32_t svml_log1p_f32(svfloat32_t src);
svfloat64_t svml_log1p_f64(svfloat64_t src);
svfloat32_t svml_tan_f32(svfloat32_t src);
svfloat64_t svml_tan_f64(svfloat64_t src);
svfloat32_t svml_exp_f32(svfloat32_t src);
svfloat64_t svml_exp_f64(svfloat64_t src);
svfloat32_t svml_pow_f32(svfloat32_t src1, svfloat32_t src2);
svfloat64_t svml_pow_f64(svfloat64_t src1, svfloat64_t src2);
svfloat32_t svml_atan_f32(svfloat32_t src);
svfloat64_t svml_atan_f64(svfloat64_t src);
svfloat32_t svml_atan2_f32(svfloat32_t src1, svfloat32_t src2);
svfloat64_t svml_atan2_f64(svfloat64_t src1, svfloat64_t src2);
svfloat32_t svml_cos_f32(svfloat32_t src);
svfloat64_t svml_cos_f64(svfloat64_t src);
svfloat32_t svml_sin_f32(svfloat32_t src);
svfloat64_t svml_sin_f64(svfloat64_t src);
svfloat32_t svml_tanh_f32(svfloat32_t src);
svfloat64_t svml_tanh_f64(svfloat64_t src);
svfloat32_t svml_expm1_f32(svfloat32_t src);
svfloat64_t svml_expm1_f64(svfloat64_t src);
svfloat32_t svml_sinh_f32(svfloat32_t src);
svfloat64_t svml_sinh_f64(svfloat64_t src);
svfloat32_t svml_exp2_f32(svfloat32_t src);
svfloat64_t svml_exp2_f64(svfloat64_t src);
svfloat32_t svml_cosh_f32(svfloat32_t src);
svfloat64_t svml_cosh_f64(svfloat64_t src);
void svml_sincos_f32(svfloat32_t src, svfloat32_t *sindst, svfloat32_t *cosdst);
void svml_sincos_f64(svfloat64_t src, svfloat64_t *sindst, svfloat64_t *cosdst);
#endif

#ifdef __cplusplus
}
#endif

#endif