/*******************************************************************************
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2023. All rights reserved.
 * Description: Part of KML library
 * Author: KML
 * Create: 2023
 ******************************************************************************/

#ifndef KML_ILUT_H
#define KML_ILUT_H

#define KML_EXPORT __attribute__((visibility("default")))

#if defined(__cplusplus)
extern "C" {
#endif

#ifndef KmlIssPsilutHandle
struct KmlIssPcilut;
typedef struct KmlIssPcilut *KmlIssPsilutHandle ;
#endif

KML_EXPORT int KmlIssPcilutSetup(KmlIssPsilutHandle  *gSmoother, float *v,
    const int *xl, const int *xr, const int *yl, const int *yr,
    const int *zl, const int *zr, int *dropFill, float *dropTol);

KML_EXPORT int KmlIssPcilutApply(KmlIssPsilutHandle  gSmoother, float *bb, float *xx);

KML_EXPORT int KmlIssPcilutClean(KmlIssPsilutHandle  gSmoother);

#if defined(__cplusplus)
}
#endif

#endif