/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2024. All rights reserved.
 * Description: image processing headers for users
 * Author: KPL
 * Create: 2023-12-18
 * Notes: NA
 */

#ifndef CV_H
#define CV_H

#include <stdbool.h>
#ifndef OPENCV_IMGPROC_HAL_REPLACEMENT_HPP
typedef struct cvhalFilter2D {
} cvhalFilter2D;
#else
#include <opencv2/core/base.hpp>

#undef cv_hal_filterInit
#define cv_hal_filterInit kml_init_filter
#undef cv_hal_filter
#define cv_hal_filter kml_execute_filter2d
#undef cv_hal_filterFree
#define cv_hal_filterFree kml_free_filter

struct cvhalFilter2D;
#endif

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

int kml_init_filter(cvhalFilter2D **context, unsigned char *kernel_data, size_t kernel_step, int kernel_type,
    int kernel_width, int kernel_height, int max_width, int max_height, int src_type, int dst_type, int borderType,
    double delta, int anchor_x, int anchor_y, bool allowSubmatrix, bool allowInplace);

int kml_execute_filter2d(cvhalFilter2D *context, unsigned char *src_data, size_t src_step, unsigned char *dst_data,
    size_t dst_step, int width, int height, int full_width, int full_height, int offset_x, int offset_y);

int kml_free_filter(cvhalFilter2D *context);

#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif

