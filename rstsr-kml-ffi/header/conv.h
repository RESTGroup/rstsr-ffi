/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * Description: External interface for users
 * Author: KPL
 * Create: 2023-09-06
 * Notes: NA
 */

#ifndef CONV_H
#define CONV_H

#ifdef __cplusplus
extern "C" {
/* Assume C declarations for C++ */
#endif /* __cplusplus */

// 1D Convolution
void conv1d_fp32(const float *input, const int batch, const int inputChannels, const int inputLength,
    const float *kernel, const int kernelLength, const int stride, const int padLength, const int dilation,
    const float *bias, float *output, const int outputChannels);

void conv1d_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputLength,
    const __fp16 *kernel, const int kernelLength, const int stride, const int padLength, const int dilation,
    const __fp16 *bias, __fp16 *output, const int outputChannels);

void conv1d_direct_fp32(const float *input, const int batch, const int inputChannels, const int inputLength,
    const float *kernel, const int kernelLength, const int stride, const int padLength, const int dilation,
    const float *bias, float *output, const int outputChannels);

void conv1d_direct_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputLength,
    const __fp16 *kernel, const int kernelLength, const int stride, const int padLength, const int dilation,
    const __fp16 *bias, __fp16 *output, const int outputChannels);

void conv1d_fft_fp32(const float *input, const int batch, const int inputChannels, const int inputLength,
    const float *kernel, const int kernelLength, const int stride, const int padLength, const float *bias,
    float *output, const int outputChannels);

void conv1d_fft_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputLength,
    const __fp16 *kernel, const int kernelLength, const int stride, const int padLength, const __fp16 *bias,
    __fp16 *output, const int outputChannels);

void conv1d_gemm_fp32(const float *input, const int batch, const int inputChannels, const int inputLength,
    const float *kernel, const int kernelLength, const int stride, const int padLength, const int dilation,
    const float *bias, float *output, const int outputChannels);

void conv1d_gemm_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputLength,
    const __fp16 *kernel, const int kernelLength, const int stride, const int padLength, const int dilation,
    const __fp16 *bias, __fp16 *output, const int outputChannels);

void conv1d_winograd_fp32(const float *input, const int batch, const int inputChannels, const int inputLength,
    const float *kernel, const int kernelLength, const int padLength, const float *bias, float *output,
    const int outputChannels);

void conv1d_winograd_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputLength,
    const __fp16 *kernel, const int kernelLength, const int padLength, const __fp16 *bias, __fp16 *output,
    const int outputChannels);

// 2D Convolution
void conv2d_fp32(const float *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const float *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const int dilationY, const int dilationX,
    const float *bias, float *output, const int outputChannels);

void conv2d_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const __fp16 *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const int dilationY, const int dilationX,
    const __fp16 *bias, __fp16 *output, const int outputChannels);

void conv2d_direct_fp32(const float *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const float *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const int dilationY, const int dilationX,
    const float *bias, float *output, const int outputChannels);

void conv2d_direct_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const __fp16 *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const int dilationY, const int dilationX,
    const __fp16 *bias, __fp16 *output, const int outputChannels);

void conv2d_fft_fp32(const float *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const float *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const float *bias, float *output,
    const int outputChannels);

void conv2d_fft_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const __fp16 *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth,  const __fp16 *bias, __fp16 *output,
    const int outputChannels);

void conv2d_gemm_fp32(const float *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const float *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const int dilationY, const int dilationX,
    const float *bias, float *output, const int outputChannels);

void conv2d_gemm_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const __fp16 *kernel, const int kernelHeight, const int kernelWidth, const int strideY,
    const int strideX, const int padHeight, const int padWidth, const int dilationY, const int dilationX,
    const __fp16 *bias, __fp16 *output, const int outputChannels);

void conv2d_winograd_fp32(const float *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const float *kernel, const int kernelHeight, const int kernelWidth, const int padHeight,
    const int padWidth, const float *bias, float *output, const int outputChannels);

void conv2d_winograd_fp16(const __fp16 *input, const int batch, const int inputChannels, const int inputHeight,
    const int inputWidth, const __fp16 *kernel, const int kernelHeight, const int kernelWidth, const int padHeight,
    const int padWidth, const __fp16 *bias, __fp16 *output, const int outputChannels);

// upsampling / downsampling
void usImage2X_cpu(float* outbuf, const float* inimg, const float* filter, int filter_width, int n, int m);
void dsImage_cpu(float* out, const float* in, float *tmp, const float* filter, int filter_width, int n, int m);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif