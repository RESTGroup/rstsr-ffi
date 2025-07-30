/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 */
#ifndef KES_MACROS_H
#define KES_MACROS_H

#define KES_EXPORT __attribute__ ((visibility ("default")))

#ifdef DEBUG
#include <assert.h>
#define KES_ASSERT assert
#define KES_CHECK(ierr) KES_ASSERT((ierr) == KES_GOOD)
#else
#define KES_CHECK(ierr) ierr
#endif

#endif