/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 */

#ifndef KML_MG_API_STATUS_H
#define KML_MG_API_STATUS_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @ingroup KML_MG_LIB
 * @brief Status codes
 */
typedef enum {
    KML_MG_OK = 0, /**< Everything is good */
    /* Failure codes */
    KML_MG_ERR_INVALID_PARAM = -1, /**< The parameters is invalid */
    KML_MG_ERR_UNSUPPORTED = -2, /**< The function invoked is not yet supported */
    KML_MG_ERR_NO_MEMORY = -3, /** Failed to allocate memory */
    KML_MG_ERR_EXCEEDS_LIMIT = -4, /**< Exceeded the set limit, such as the memory quota */
    KML_MG_ERR_INCOMPATIBLE = -5, /**< Incompatible version */
    KML_MG_ERR_NO_REQUIRED_FIELD = -6, /**< There is no required field */
    KML_MG_ERR_UNKNOWN_TYPE = -7, /**< Unknown types, such as undefined matrix storage types */
    KML_MG_ERR_THIRD_PART = -8, /**< Internally invoked third-party error. */
    KML_MG_ERR_INCONSISTENT = -9, /**< Inconsistent information transferred in different phases during solution */
    KML_MG_ERR_UNEXPECTED_CALL_ORDER = -10, /**< Unexpected call order in the solution process */
    KML_MG_ERR_MATRIX_VALUE_IS_NULL = -11, /**< Matrix value is null */
    KML_MG_ERR_NO_RESOURCE = -12, /**< Resource is not ready */
} KmlMgStatus;

/**
 * @ingroup KML_MG_LIB
 * @brief Status string
 *
 * @return Status string
 */
const char *KmlMgStatusStr(KmlMgStatus status);

#ifdef __cplusplus
}
#endif

#endif
