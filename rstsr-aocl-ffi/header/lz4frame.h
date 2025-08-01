/*
   LZ4F LZ4-Frame library
   Header File
   Copyright (C) 2011-2020, Yann Collet.
   Modifications Copyright (C) 2024, Advanced Micro Devices. All rights reserved.
   BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions are
   met:

       * Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
       * Redistributions in binary form must reproduce the above
   copyright notice, this list of conditions and the following disclaimer
   in the documentation and/or other materials provided with the
   distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

   You can contact the author at :
   - LZ4 source repository : https://github.com/lz4/lz4
   - LZ4 public forum : https://groups.google.com/forum/#!forum/lz4c
*/

/* LZ4F is a stand-alone API able to create and decode LZ4 frames
 * conformant with specification v1.6.1 in doc/lz4_Frame_format.md .
 * Generated frames are compatible with `lz4` CLI.
 *
 * LZ4F also offers streaming capabilities.
 *
 * lz4.h is not required when using lz4frame.h,
 * except to extract common constants such as LZ4_VERSION_NUMBER.
 * */

#ifndef LZ4F_H_09782039843
#define LZ4F_H_09782039843

#if defined (__cplusplus)
extern "C" {
#endif

/* ---   Dependency   --- */
#include <stddef.h>   /* size_t */


/**
 * Introduction
 *
 * lz4frame.h implements LZ4 frame specification: see doc/lz4_Frame_format.md .
 * LZ4 Frames are compatible with `lz4` CLI,
 * and designed to be interoperable with any system.
**/

/*-***************************************************************
 *  Compiler specifics
 *****************************************************************/
/*  LZ4_DLL_EXPORT :
 *  Enable exporting of functions when building a Windows DLL
 *  LZ4FLIB_VISIBILITY :
 *  Control library symbols visibility.
 */
#ifndef LZ4FLIB_VISIBILITY
#  if defined(__GNUC__) && (__GNUC__ >= 4)
#    define LZ4FLIB_VISIBILITY __attribute__ ((visibility ("default")))
#  else
#    define LZ4FLIB_VISIBILITY
#  endif
#endif
#if defined(LZ4_DLL_EXPORT) && (LZ4_DLL_EXPORT==1)
#  define LZ4FLIB_API __declspec(dllexport) LZ4FLIB_VISIBILITY
#elif defined(LZ4_DLL_IMPORT) && (LZ4_DLL_IMPORT==1)
#  define LZ4FLIB_API __declspec(dllimport) LZ4FLIB_VISIBILITY
#else
#  define LZ4FLIB_API LZ4FLIB_VISIBILITY
#endif

#ifdef LZ4F_DISABLE_DEPRECATE_WARNINGS
#  define LZ4F_DEPRECATE(x) x
#else
#  if defined(_MSC_VER)
#    define LZ4F_DEPRECATE(x) x   /* __declspec(deprecated) x - only works with C++ */
#  elif defined(__clang__) || (defined(__GNUC__) && (__GNUC__ >= 6))
#    define LZ4F_DEPRECATE(x) x __attribute__((deprecated))
#  else
#    define LZ4F_DEPRECATE(x) x   /* no deprecation warning for this compiler */
#  endif
#endif

/**
 * \defgroup LZ4FRAME_API LZ4 FRAME FORMAT API
 * @ingroup LZ4_API
 * 
 * @brief
 * LZ4F is a stand-alone API able to create and decode LZ4 frames
 * conformant with specification v1.6.1 in https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md .
 * Generated frames are compatible with `lz4` CLI, and designed to be interoperable with any system.
 *
 * lz4frame.h implements LZ4 frame specification: see doc/lz4_Frame_format.md . * 
 * LZ4F also offers streaming capabilities.
 *
 * lz4.h is not required when using lz4frame.h,
 * except to extract common constants such as LZ4_VERSION_NUMBER.
 * 
 * @{
 */


/*-************************************
 *  Error management
 **************************************/
typedef size_t LZ4F_errorCode_t;

/*!
 * @brief 
 * tells when a function result is an error code
 * 
 * @return
 * |Result | Description                                 |
 * |:------|:--------------------------------------------|
 * |Success| Return 1 if the code represents error code. |
 * |Failure| Return 0 if non-error code is passed.       |
 */
LZ4FLIB_API unsigned    LZ4F_isError(LZ4F_errorCode_t code);

/*!
 * @brief 
 * return error code string; for debugging
 * 
 * @return
 * |Result | Description                                |
 * |:------|:-------------------------------------------|
 * |Success| Returns a proper error string.             |
 * |Failure| Returns a string "Unspecified error code". |
 */
LZ4FLIB_API const char* LZ4F_getErrorName(LZ4F_errorCode_t code);


/*-************************************
 *  Frame compression types
 ************************************* */
/// @cond DOXYGEN_SHOULD_SKIP_THIS

/* #define LZ4F_ENABLE_OBSOLETE_ENUMS   // uncomment to enable obsolete enums */
#ifdef LZ4F_ENABLE_OBSOLETE_ENUMS
#  define LZ4F_OBSOLETE_ENUM(x) , LZ4F_DEPRECATE(x) = LZ4F_##x
#else
#  define LZ4F_OBSOLETE_ENUM(x)
#endif

/// @endcond /* DOXYGEN_SHOULD_SKIP_THIS */

/*!
 * @brief
 * The larger the block size, the (slightly) better the compression ratio,
 * though there are diminishing returns.
 * Larger blocks also increase memory usage on both compression and decompression sides.
 */
typedef enum {
    LZ4F_default=0,
    LZ4F_max64KB=4,
    LZ4F_max256KB=5,
    LZ4F_max1MB=6,
    LZ4F_max4MB=7
    LZ4F_OBSOLETE_ENUM(max64KB)
    LZ4F_OBSOLETE_ENUM(max256KB)
    LZ4F_OBSOLETE_ENUM(max1MB)
    LZ4F_OBSOLETE_ENUM(max4MB)
} LZ4F_blockSizeID_t;

/* Linked blocks sharply reduce inefficiencies when using small blocks,
 * they compress better.
 * However, some LZ4 decoders are only compatible with independent blocks */
typedef enum {
    LZ4F_blockLinked=0,
    LZ4F_blockIndependent
    LZ4F_OBSOLETE_ENUM(blockLinked)
    LZ4F_OBSOLETE_ENUM(blockIndependent)
} LZ4F_blockMode_t;

typedef enum {
    LZ4F_noContentChecksum=0,
    LZ4F_contentChecksumEnabled
    LZ4F_OBSOLETE_ENUM(noContentChecksum)
    LZ4F_OBSOLETE_ENUM(contentChecksumEnabled)
} LZ4F_contentChecksum_t;

typedef enum {
    LZ4F_noBlockChecksum=0,
    LZ4F_blockChecksumEnabled
} LZ4F_blockChecksum_t;

typedef enum {
    LZ4F_frame=0,
    LZ4F_skippableFrame
    LZ4F_OBSOLETE_ENUM(skippableFrame)
} LZ4F_frameType_t;

#ifdef LZ4F_ENABLE_OBSOLETE_ENUMS
typedef LZ4F_blockSizeID_t blockSizeID_t;
typedef LZ4F_blockMode_t blockMode_t;
typedef LZ4F_frameType_t frameType_t;
typedef LZ4F_contentChecksum_t contentChecksum_t;
#endif

/*! 
 * @brief
 *  makes it possible to set or read frame parameters.
 * 
 *  Structure must be first init to 0, using memset() or LZ4F_INIT_FRAMEINFO,
 *  setting all parameters to default.
 *  It's then possible to update selectively some parameters */
typedef struct {
  LZ4F_blockSizeID_t     blockSizeID;         /* max64KB, max256KB, max1MB, max4MB; 0 == default */
  LZ4F_blockMode_t       blockMode;           /* LZ4F_blockLinked, LZ4F_blockIndependent; 0 == default */
  LZ4F_contentChecksum_t contentChecksumFlag; /* 1: frame terminated with 32-bit checksum of decompressed data; 0: disabled (default) */
  LZ4F_frameType_t       frameType;           /* read-only field : LZ4F_frame or LZ4F_skippableFrame */
  unsigned long long     contentSize;         /* Size of uncompressed content ; 0 == unknown */
  unsigned               dictID;              /* Dictionary ID, sent by compressor to help decoder select correct dictionary; 0 == no dictID provided */
  LZ4F_blockChecksum_t   blockChecksumFlag;   /* 1: each block followed by a checksum of block's compressed data; 0: disabled (default) */
} LZ4F_frameInfo_t;

/// @cond DOXYGEN_SHOULD_SKIP_THIS
#define LZ4F_INIT_FRAMEINFO   { LZ4F_default, LZ4F_blockLinked, LZ4F_noContentChecksum, LZ4F_frame, 0ULL, 0U, LZ4F_noBlockChecksum }    /* v1.8.3+ */
/// @endcond /* DOXYGEN_SHOULD_SKIP_THIS */

/*! @brief
 *  makes it possible to supply advanced compression instructions to streaming interface.
 *
 *  Structure must be first init to 0, using memset() or LZ4F_INIT_PREFERENCES,
 *  setting all parameters to default.
 *  All reserved fields must be set to zero. */
typedef struct {
  LZ4F_frameInfo_t frameInfo;
  int      compressionLevel;    /* 0: default (fast mode); values > LZ4HC_CLEVEL_MAX count as LZ4HC_CLEVEL_MAX; values < 0 trigger "fast acceleration" */
  unsigned autoFlush;           /* 1: always flush; reduces usage of internal buffers */
  unsigned favorDecSpeed;       /* 1: parser favors decompression speed vs compression ratio. Only works for high compression modes (>= LZ4HC_CLEVEL_OPT_MIN) */  /* v1.8.2+ */
  unsigned reserved[3];         /* must be zero for forward compatibility */
} LZ4F_preferences_t;

/// @cond DOXYGEN_SHOULD_SKIP_THIS
#define LZ4F_INIT_PREFERENCES   { LZ4F_INIT_FRAMEINFO, 0, 0u, 0u, { 0u, 0u, 0u } }    /* v1.8.3+ */
/// @endcond /* DOXYGEN_SHOULD_SKIP_THIS */

/*-*********************************
*  Simple compression function
***********************************/
/*!
 *  @brief
 *  Returns maximum compression level, the higher the compression level, the better the compression ratio.
 * 
 *  @return Max compression level is returned.
 */
LZ4FLIB_API int LZ4F_compressionLevel_max(void);   /* v1.8.0+ */

/*! @brief
 *  This function outputs the maximum possible compressed size with LZ4F_compressFrame() given srcSize and preferences.
 *  
 * | Parameters        | Direction  | Description |
 * |:------------------|:----------:|:------------|
 * | \b srcSize        | in         | Input buffer size. |
 * | \b preferencesPtr | in         | An optional pointer which makes it possible to supply advanced compression instructions to streaming interface. It can be replaced by NULL, in which case, the function will assume default preferences. |
 * 
 *  @note this result is only usable with LZ4F_compressFrame(). It may also be relevant to LZ4F_compressUpdate() _only if_ no flush() operation is ever performed.
 * 
 *  @return Returns the maximum possible compressed size with LZ4F_compressFrame() given srcSize and preferences. 
 */
LZ4FLIB_API size_t LZ4F_compressFrameBound(size_t srcSize, const LZ4F_preferences_t* preferencesPtr);

/*! @brief
 *  Compress an entire srcBuffer into a valid LZ4 frame.
 *  
 *  The LZ4F_preferences_t structure is optional : you can provide NULL as argument. All preferences will be set to default.
 * 
 *  |Parameters         |Direction  |Description                                                                           |
 *  |:------------------|:---------:|:-------------------------------------------------------------------------------------|
 *  | \b dstBuffer      |   out     | Destination buffer, compressed data is kept here, memory should be allocated already.|
 *  | \b dstCapacity    |   in      | Size of pre-allocated 'dst' buffer.                                                  |
 *  | \b srcBuffer      |   in      | Source buffer, the data you want to compress is copied/or pointed here.              |
 *  | \b srcSize        |   in      | Size of srcBuffer.                                                                   |
 *  | \b preferencesPtr |   in      | An optional pointer which makes it possible to supply advanced compression instructions to streaming interface. It can be replaced by NULL, in which case, the function will assume default preferences. |
 * 
 * 
 * @note dstCapacity MUST be >= LZ4F_compressFrameBound(srcSize, preferencesPtr).
 * 
 * @return
 * |Result | Description                                                      |
 * |:------|:-----------------------------------------------------------------|
 * |Success| Number of bytes written into dstBuffer.                          |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()).  |
 */
LZ4FLIB_API size_t LZ4F_compressFrame(void* dstBuffer, size_t dstCapacity,
                                const void* srcBuffer, size_t srcSize,
                                const LZ4F_preferences_t* preferencesPtr);


/*-***********************************
*  Advanced compression functions
*************************************/
typedef struct LZ4F_cctx_s LZ4F_cctx;   /* incomplete type */
typedef LZ4F_cctx* LZ4F_compressionContext_t;   /* for compatibility with older APIs, prefer using LZ4F_cctx */

typedef struct {
  unsigned stableSrc;    /* 1 == src content will remain present on future calls to LZ4F_compress(); skip copying src content within tmp buffer */
  unsigned reserved[3];
} LZ4F_compressOptions_t;

/*---   Resource Management   ---*/

/** @brief This number can be used to check for an incompatible API breaking change */
#define LZ4F_VERSION 100    
LZ4FLIB_API unsigned LZ4F_getVersion(void);

/*! @brief
 *  Used to create compression context that can be employed multiple times for consecutive streaming operations.
 *  
 *  The first thing to do before compression is to create a compressionContext object,
 *  which will keep track of operation state during streaming compression, this task is achieved by this function.
 * 
 *  |Parameters   | Direction | Description                                                                          |
 *  |:------------|:---------:|:-------------------------------------------------------------------------------------|
 *  | \b cctxPtr  |  in,out   | a pointer of pointer to fully allocated LZ4F_cctx object, to write the resulting pointer into. |
 *  | \b version  |    in     | provided MUST be LZ4F_VERSION. It is intended to track potential version mismatch, notably when using DLL. |
 * 
 *  @note cctxPtr MUST be != NULL.
 * 
 * @return
 * |Result | Description                                                      |
 * |:------|:-----------------------------------------------------------------|
 * |Success| Returns \b LZ4F_OK_NoError on success.                         |
 * |Failure| Returns \b LZ4F_ERROR_parameter_null  if \b cctxPtr is NULL. |
 * |       | Returns \b LZ4F_ERROR_allocation_failed if there is problem with allocation of memory. |
**/
LZ4FLIB_API LZ4F_errorCode_t LZ4F_createCompressionContext(LZ4F_cctx** cctxPtr, unsigned version);

/*! @brief
 *  Once all streaming compression jobs are completed, the state object can be released using this function.
 * 
 *  |Parameters | Direction | Description                                                                                     |
 *  |:----------|:---------:|:------------------------------------------------------------------------------------------------|
 *  | \b cctx   |   in      | A compression context that can be employed multiple times for consecutive streaming operations. |
 * 
 * @return Is always successful even if input is NULL pointer (do nothing), so return value can be ignored.
 */
LZ4FLIB_API LZ4F_errorCode_t LZ4F_freeCompressionContext(LZ4F_cctx* cctx);


/*----    Compression    ----*/
/// @cond DOXYGEN_SHOULD_SKIP_THIS
#define LZ4F_HEADER_SIZE_MIN  7   /* LZ4 Frame header size can vary, depending on selected parameters */
#define LZ4F_HEADER_SIZE_MAX 19

/* Size in bytes of a block header in little-endian format. Highest bit indicates if block data is uncompressed */
#define LZ4F_BLOCK_HEADER_SIZE 4

/* Size in bytes of a block checksum footer in little-endian format. */
#define LZ4F_BLOCK_CHECKSUM_SIZE 4

/* Size in bytes of the content checksum. */
#define LZ4F_CONTENT_CHECKSUM_SIZE 4
/// @endcond /* DOXYGEN_SHOULD_SKIP_THIS */
/*! @brief
 *  will write the frame header into dstBuffer.

 *  |Parameters         |Direction  |Description                                                                           |
 *  |:------------------|:---------:|:-------------------------------------------------------------------------------------|
 *  | \b cctx           |   in      | A compression context that can be employed multiple times for consecutive streaming operations. |
 *  | \b dstBuffer      |   out     | Destination buffer, compressed data is kept here, memory should be allocated already.|
 *  | \b dstCapacity    |   in      | Size of pre-allocated 'dst' buffer.                                                  |
 *  | \b prefsPtr       |   in      | An optional pointer which makes it possible to supply advanced compression instructions to streaming interface. It can be replaced by NULL, in which case, the function will assume default preferences. |
 * 
 * @note dstCapacity must be >= LZ4F_HEADER_SIZE_MAX bytes.
 * 
 * @return
 * |Result | Description                                                      |
 * |:------|:-----------------------------------------------------------------|
 * |Success| Number of bytes written into dstBuffer.                          |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()).  |
 */
LZ4FLIB_API size_t LZ4F_compressBegin(LZ4F_cctx* cctx,
                                      void* dstBuffer, size_t dstCapacity,
                                      const LZ4F_preferences_t* prefsPtr);

/*! @brief
 *  Provides minimum dstCapacity required to guarantee success of
 *  LZ4F_compressUpdate(), given a srcSize and preferences, for a worst case scenario.
 * 
 *  When srcSize==0, LZ4F_compressBound() provides an upper bound for LZ4F_flush() and LZ4F_compressEnd() instead.
 * 
 *  @note The result is only valid for a single invocation of LZ4F_compressUpdate().
 *  When invoking LZ4F_compressUpdate() multiple times,
 *  if the output buffer is gradually filled up instead of emptied and re-used from its start,
 *  one must check if there is enough remaining capacity before each invocation, using LZ4F_compressBound().
 *  
 * | Parameters        | Direction  | Description |
 * |:------------------|:----------:|:------------|
 * | \b srcSize        | in         | Input buffer size. |
 * | \b preferencesPtr | in         | An optional pointer which makes it possible to supply advanced compression instructions to streaming interface. It can be replaced by NULL, in which case, the function will assume default preferences. |
 * 
 * @return is always the same for a srcSize and prefsPtr.
 *  prefsPtr is optional : when NULL is provided, preferences will be set to cover worst case scenario.
 *  tech details :
 * @return if automatic flushing is not enabled, includes the possibility that internal buffer might already be filled by up to (blockSize-1) bytes.
 *  It also includes frame footer (ending + checksum), since it might be generated by LZ4F_compressEnd().
 * @return doesn't include frame header, as it was already generated by LZ4F_compressBegin().
 */
LZ4FLIB_API size_t LZ4F_compressBound(size_t srcSize, const LZ4F_preferences_t* prefsPtr);

/*! @brief
 *  LZ4F_compressUpdate() can be called repetitively to compress as much data as necessary.
 *
 *  @important
 *  \b dstCapacity MUST be large enough to ensure operation success even in worst case situations.
 *  This value is provided by LZ4F_compressBound().
 *  If this condition is not respected, LZ4F_compress() will fail (result is an errorCode).
 *  After an error, the state is left in a UB state, and must be re-initialized or freed.
 *  If previously an uncompressed block was written, buffered data is flushed
 *  before appending compressed data is continued.
 * 
 *  |Parameters         | Direction | Description                                                                                     |
 *  |:------------------|:---------:|:------------------------------------------------------------------------------------------------|
 *  | \b cctx           |   in      | A compression context that can be employed multiple times for consecutive streaming operations. |
 *  | \b dstBuffer      |   out     | Destination buffer, compressed data is kept here, memory should be allocated already.           |
 *  | \b dstCapacity    |   in      | Size of pre-allocated 'dst' buffer. Must be large enough to work properly, value should be >= LZ4F_compressBound()  |
 *  | \b srcBuffer      |   in      | Source buffer, the data you want to compress is copied/or pointed here.                         |
 *  | \b srcSize        |   in      | Size of srcBuffer.                                                                              |
 *  | \b cOptPtr        |   in      | is optional : NULL can be provided, in which case all options are set to default.               |
 * 
 * @return
 * |Result | Description                                                                                        |
 * |:------|:---------------------------------------------------------------------------------------------------|
 * |Success| number of bytes written into `dstBuffer` (it can be zero, meaning input data was just buffered).   |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()).                                    |
 */
LZ4FLIB_API size_t LZ4F_compressUpdate(LZ4F_cctx* cctx,
                                       void* dstBuffer, size_t dstCapacity,
                                 const void* srcBuffer, size_t srcSize,
                                 const LZ4F_compressOptions_t* cOptPtr);

/*! @brief
 *  When data must be generated and sent immediately, without waiting for a block to be completely filled,
 *  it's possible to call LZ4_flush(). It will immediately compress any data buffered within cctx.
 * 
 * 
 *  @note LZ4F_flush() is guaranteed to be successful when dstCapacity >= LZ4F_compressBound(0, prefsPtr).
 * 
 *  |Parameters         | Direction | Description                                                                                     |
 *  |:------------------|:---------:|:------------------------------------------------------------------------------------------------|
 *  | \b cctx           |   in      | A compression context that can be employed multiple times for consecutive streaming operations. |
 *  | \b dstBuffer      |   out     | Destination buffer, compressed data is kept here, memory should be allocated already.           |
 *  | \b dstCapacity    |   in      | Size of pre-allocated 'dst' buffer. Must be large enough to work properly, value should be >= LZ4F_compressBound()  |
 *  | \b cOptPtr        |   in      | is optional : NULL can be provided, in which case all options are set to default.               |
 * 
 * @return
 * |Result | Description                                                                                  |
 * |:------|:---------------------------------------------------------------------------------------------|
 * |Success| nb of bytes written into dstBuffer (can be zero, when there is no data stored within cctx)   |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()).                              |
 */
LZ4FLIB_API size_t LZ4F_flush(LZ4F_cctx* cctx,
                              void* dstBuffer, size_t dstCapacity,
                        const LZ4F_compressOptions_t* cOptPtr);

/*! @brief
 *  To properly finish an LZ4 frame, invoke LZ4F_compressEnd().
 *
 *  It will flush whatever data remained within `cctx` (like LZ4_flush())
 *  and properly finalize the frame, with an endMark and a checksum.
 * 
 *  @note LZ4F_compressEnd() is guaranteed to be successful when dstCapacity >= LZ4F_compressBound(0, prefsPtr).
 *  A successful call to LZ4F_compressEnd() makes `cctx` available again for another compression task.
 * 
 *  |Parameters         | Direction | Description                                                                                     |
 *  |:------------------|:---------:|:------------------------------------------------------------------------------------------------|
 *  | \b cctx           |   in      | A compression context that can be employed multiple times for consecutive streaming operations. |
 *  | \b dstBuffer      |   out     | Destination buffer, compressed data is kept here, memory should be allocated already.           |
 *  | \b dstCapacity    |   in      | Size of pre-allocated 'dst' buffer. Must be large enough to work properly, value should be >= LZ4F_compressBound(0, prefsPtr)  |
 *  | \b cOptPtr        |   in      | is optional : NULL can be provided, in which case all options are set to default.               |
 * 
 * 
 * @return
 * |Result | Description                                                      |
 * |:------|:-----------------------------------------------------------------|
 * |Success| nb of bytes written into dstBuffer, necessarily >= 4 (endMark)   |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()).  |
 */
LZ4FLIB_API size_t LZ4F_compressEnd(LZ4F_cctx* cctx,
                                    void* dstBuffer, size_t dstCapacity,
                              const LZ4F_compressOptions_t* cOptPtr);


/*-*********************************
*  Decompression functions
***********************************/
typedef struct LZ4F_dctx_s LZ4F_dctx;   /* incomplete type */
typedef LZ4F_dctx* LZ4F_decompressionContext_t;   /* compatibility with previous API versions */

/*!
 * @brief
 * Advanced decompression options.
*/
typedef struct {
  unsigned stableDst;     /**< pledges that last 64KB decompressed data will remain available unmodified between invocations.
                           * This optimization skips storage operations in tmp buffers. */
  unsigned skipChecksums; /**< disable checksum calculation and verification, even when one is present in frame, to save CPU time.
                           * Setting this option to 1 once disables all checksums for the rest of the frame. */
  unsigned reserved1;     /**< must be set to zero for forward compatibility. */
  unsigned reserved0;     /**< idem. */
} LZ4F_decompressOptions_t;


/* Resource management */

/*! @brief
 *  Create an LZ4F_dctx object, to track all decompression operations.
 *
 *  The function fills @dctxPtr with the value of a pointer to an allocated and initialized LZ4F_dctx object.
 *  dctx memory can be released using LZ4F_freeDecompressionContext();
 * 
 *  |Parameters  | Direction | Description                                              |
 *  |:-----------|:---------:|:---------------------------------------------------------|
 *  | \b dctxPtr |   in      | Pointer to pointer of a LZ4F_dctx object, to track all decompression operations. |
 *  | \b version |   in      | provided MUST be LZ4F_VERSION. It is intended to track potential version mismatch, notably when using DLL. |
 *
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| 0 is returned if decompression has been completed fully and correctly.  |
 * |Failure| An errorCode is returned on failure, which can be tested using LZ4F_isError(). |
 */
LZ4FLIB_API LZ4F_errorCode_t LZ4F_createDecompressionContext(LZ4F_dctx** dctxPtr, unsigned version);

/*! @brief
 *  dctx memory can be released using LZ4F_freeDecompressionContext();
 *
 *  Result of LZ4F_freeDecompressionContext() indicates current state of decompressionContext when being released.
 *  That is, it should be == 0 if decompression has been completed fully and correctly.
 * 
 *  |Parameters  | Direction | Description                                              |
 *  |:-----------|:---------:|:---------------------------------------------------------|
 *  | \b dctx    |   in      | LZ4F_dctx object, to track all decompression operations. |
 * 
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| 0 is returned on success.  |
 * |Failure| Any value not equal to 0.  |
 */
LZ4FLIB_API LZ4F_errorCode_t LZ4F_freeDecompressionContext(LZ4F_dctx* dctx);


/*-***********************************
*  Streaming decompression functions
*************************************/
/// @cond DOXYGEN_SHOULD_SKIP_THIS

#define LZ4F_MAGICNUMBER 0x184D2204U
#define LZ4F_MAGIC_SKIPPABLE_START 0x184D2A50U
#define LZ4F_MIN_SIZE_TO_KNOW_HEADER_LENGTH 5

/// @endcond /* DOXYGEN_SHOULD_SKIP_THIS */

/*! @brief v1.9.0+
 *  Provide the header size of a frame starting at `src`.
 *
 * `srcSize` must be >= LZ4F_MIN_SIZE_TO_KNOW_HEADER_LENGTH,
 *  which is enough to decode the header length.
 *
 * @note Frame header size is variable, but is guaranteed to be
 *         >= LZ4F_HEADER_SIZE_MIN bytes, and <= LZ4F_HEADER_SIZE_MAX bytes.
 *  
 *  |Parameters  | Direction | Description                                              |
 *  |:-----------|:---------:|:---------------------------------------------------------|
 *  | \b src     |   in      | LZ4 Frame format compatible compressed stream . |
 *  | \b srcSize |   in      | Size of src buffer. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| Size of frame header.  |
 * |Failure| or an error code, which can be tested using LZ4F_isError().  |
 */
LZ4FLIB_API size_t LZ4F_headerSize(const void* src, size_t srcSize);

/*! @brief
 *  This function extracts frame parameters (max blockSize, dictID, etc.).
 *  Its usage is optional: user can also invoke LZ4F_decompress() directly.
 *
 *  Extracted information will fill an existing LZ4F_frameInfo_t structure.
 *  This can be useful for allocation and dictionary identification purposes.
 *
 *  LZ4F_getFrameInfo() can work in the following situations :
 *
 *  1) At the beginning of a new frame, before any invocation of LZ4F_decompress().
 *     It will decode header from `srcBuffer`,
 *     consuming the header and starting the decoding process.
 *
 *     Input size must be large enough to contain the full frame header.
 *     Frame header size can be known beforehand by LZ4F_headerSize().
 *     Frame header size is variable, but is guaranteed to be >= LZ4F_HEADER_SIZE_MIN bytes,
 *     and not more than <= LZ4F_HEADER_SIZE_MAX bytes.
 *     Hence, blindly providing LZ4F_HEADER_SIZE_MAX bytes or more will always work.
 *     It's allowed to provide more input data than the header size,
 *     LZ4F_getFrameInfo() will only consume the header.
 *
 *     If input size is not large enough,
 *     aka if it's smaller than header size,
 *     function will fail and return an error code.
 *
 *  2) After decoding has been started,
 *     it's possible to invoke LZ4F_getFrameInfo() anytime
 *     to extract already decoded frame parameters stored within dctx.
 *
 *     Note that, if decoding has barely started,
 *     and not yet read enough information to decode the header,
 *     LZ4F_getFrameInfo() will fail.
 *
 *  The number of bytes consumed from srcBuffer will be updated in *srcSizePtr (necessarily <= original value).
 *  LZ4F_getFrameInfo() only consumes bytes when decoding has not yet started,
 *  and when decoding the header has been successful.
 *  Decompression must then resume from (srcBuffer + *srcSizePtr).
 *
 *  @note in case of error, dctx is not modified. Decoding operation can resume from beginning safely.
 *  @note frame parameters are *copied into* an already allocated LZ4F_frameInfo_t structure.
 * 
 *  | Parameters      | Direction | Description                                             |
 *  |:----------------|:---------:|:---------------------------------------------------------|
 *  | \b dctx         |  in,out   | LZ4F_dctx object, to track all decompression operations. |
 *  | \b frameInfoPtr |    out    | This is a pointer to an already allocated LZ4F_frameInfo_t structure where frame parameters are *copied into*. |
 *  | \b srcBuffer    |    in     | LZ4 Frame format compatible compressed input stream. |
 *  | \b srcSizePtr   |  in,out   | Pointer to size of src buffer. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| A hint about how many srcSize bytes LZ4F_decompress() expects for next call  |
 * |Failure| An error code, which can be tested using LZ4F_isError().  |
 */
LZ4FLIB_API size_t
LZ4F_getFrameInfo(LZ4F_dctx* dctx,
                  LZ4F_frameInfo_t* frameInfoPtr,
            const void* srcBuffer, size_t* srcSizePtr);

/*! @brief
 *  Call this function repetitively to regenerate data compressed in `srcBuffer`.
 *
 *  The function requires a valid dctx state.
 *  It will read up to *srcSizePtr bytes from srcBuffer,
 *  and decompress data into dstBuffer, of capacity *dstSizePtr.
 *
 *  The nb of bytes consumed from srcBuffer will be written into *srcSizePtr (necessarily <= original value).
 *  The nb of bytes decompressed into dstBuffer will be written into *dstSizePtr (necessarily <= original value).
 *
 *  The function does not necessarily read all input bytes, so always check value in *srcSizePtr.
 *  Unconsumed source data must be presented again in subsequent invocations.
 *
 * `dstBuffer` can freely change between each consecutive function invocation.
 * `dstBuffer` content will be overwritten.
 *
 *  After decompression a hint of how many `srcSize` bytes LZ4F_decompress() expects for next call.
 *  Schematically, it's the size of the current (or remaining) compressed block + header of next block.
 *  Respecting the hint provides some small speed benefit, because it skips intermediate buffers.
 *  This is just a hint though, it's always possible to provide any srcSize.
 *
 *  When provided with more bytes than necessary to decode a frame,
 *  LZ4F_decompress() will stop reading exactly at end of current frame, and return 0.
 *
 *  After a decompression error, the `dctx` context is not resumable.
 *  Use LZ4F_resetDecompressionContext() to return to clean state.
 *
 *  After a frame is fully decoded, dctx can be used again to decompress another frame.
 * 
 *  | Parameters      | Direction | Description                                              |
 *  |:----------------|:---------:|:---------------------------------------------------------|
 *  | \b dctx         |  in,out   | LZ4F_dctx object, to track all decompression operations. |
 *  | \b dstBuffer    |    out    | Decompressed data is written in `dstBuffer`, `dstBuffer` can freely change between each consecutive function invocation & content inside it will be overwritten. |
 *  | \b dstSizePtr   |  in,out   | While calling this function "*dstSizePtr" provides the length of dstBuffer, after decompression is done nb of bytes decompressed into dstBuffer will be written into *dstSizePtr (necessarily <= original value). |
 *  | \b srcBuffer    |    in     | LZ4 Frame format compatible compressed input stream. |
 *  | \b srcSizePtr   |  in,out   | It will read up to *srcSizePtr bytes from srcBuffer and write the nb of bytes consumed from srcBuffer into *srcSizePtr (necessarily <= original value).|
 *  | \b dOptPtr      |  in,out   | Advanced decompression options, could be NULL in which case default options are assumed. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| When a frame is fully decoded, 0 will be returned (no more data expected), if not a hint of how many `srcSize` bytes LZ4F_decompress() expects for next call. |
 * |Failure| If decompression failed, an error code is returned, which can be tested using LZ4F_isError().|
 */
LZ4FLIB_API size_t
LZ4F_decompress(LZ4F_dctx* dctx,
                void* dstBuffer, size_t* dstSizePtr,
          const void* srcBuffer, size_t* srcSizePtr,
          const LZ4F_decompressOptions_t* dOptPtr);


/*! @brief added in v1.8.0
 *  In case of an error, the context is left in "undefined" state.
 *  In which case, it's necessary to reset it, before re-using it.
 * 
 *  This method can also be used to abruptly stop any unfinished decompression,
 *  and start a new one using same context resources.
 * 
 *  | Parameters      | Direction | Description                                              |
 *  |:----------------|:---------:|:---------------------------------------------------------|
 *  | \b dctx         |    out    | LZ4F_dctx object, to track all decompression operations. |
 * 
 * @warning NULL pointer cannot be passed.
 * 
 * @return void
 */
LZ4FLIB_API void LZ4F_resetDecompressionContext(LZ4F_dctx* dctx);   /* always successful */

/**
 * 
 * @}
 */

#if defined (__cplusplus)
}
#endif

#endif  /* LZ4F_H_09782039843 */

#if defined(LZ4F_STATIC_LINKING_ONLY) && !defined(LZ4F_H_STATIC_09782039843)
#define LZ4F_H_STATIC_09782039843

#if defined (__cplusplus)
extern "C" {
#endif

/* These declarations are not stable and may change in the future.
 * They are therefore only safe to depend on
 * when the caller is statically linked against the library.
 * To access their declarations, define LZ4F_STATIC_LINKING_ONLY.
 *
 * By default, these symbols aren't published into shared/dynamic libraries.
 * You can override this behavior and force them to be published
 * by defining LZ4F_PUBLISH_STATIC_FUNCTIONS.
 * Use at your own risk.
 */
#ifdef LZ4F_PUBLISH_STATIC_FUNCTIONS
# define LZ4FLIB_STATIC_API LZ4FLIB_API
#else
# define LZ4FLIB_STATIC_API
#endif


/* ---   Error List   --- */
#define LZ4F_LIST_ERRORS(ITEM) \
        ITEM(OK_NoError) \
        ITEM(ERROR_GENERIC) \
        ITEM(ERROR_maxBlockSize_invalid) \
        ITEM(ERROR_blockMode_invalid) \
        ITEM(ERROR_contentChecksumFlag_invalid) \
        ITEM(ERROR_compressionLevel_invalid) \
        ITEM(ERROR_headerVersion_wrong) \
        ITEM(ERROR_blockChecksum_invalid) \
        ITEM(ERROR_reservedFlag_set) \
        ITEM(ERROR_allocation_failed) \
        ITEM(ERROR_srcSize_tooLarge) \
        ITEM(ERROR_dstMaxSize_tooSmall) \
        ITEM(ERROR_frameHeader_incomplete) \
        ITEM(ERROR_frameType_unknown) \
        ITEM(ERROR_frameSize_wrong) \
        ITEM(ERROR_srcPtr_wrong) \
        ITEM(ERROR_decompressionFailed) \
        ITEM(ERROR_headerChecksum_invalid) \
        ITEM(ERROR_contentChecksum_invalid) \
        ITEM(ERROR_frameDecoding_alreadyStarted) \
        ITEM(ERROR_compressionState_uninitialized) \
        ITEM(ERROR_parameter_null) \
        ITEM(ERROR_maxCode)

#define LZ4F_GENERATE_ENUM(ENUM) LZ4F_##ENUM,

/* enum list is exposed, to handle specific errors */
typedef enum { LZ4F_LIST_ERRORS(LZ4F_GENERATE_ENUM)
              _LZ4F_dummy_error_enum_for_c89_never_used } LZ4F_errorCodes;

LZ4FLIB_STATIC_API LZ4F_errorCodes LZ4F_getErrorCode(size_t functionResult);


/*! @brief
 *  Return, in scalar format (size_t),
 *  the maximum block size associated with blockSizeID.
 * 
 * @return
 * |Result | Description                                                                            |
 * |:------|:---------------------------------------------------------------------------------------|
 * |Success| Return, in scalar format (size_t), the maximum block size associated with blockSizeID. |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()).                        |
**/
LZ4FLIB_STATIC_API size_t LZ4F_getBlockSize(LZ4F_blockSizeID_t blockSizeID);

/*! @brief
 *  LZ4F_uncompressedUpdate() can be called repetitively to add as much data uncompressed data as necessary.
 *  Important rule: dstCapacity MUST be large enough to store the entire source buffer as
 *  no compression is done for this operation
 *  If this condition is not respected, LZ4F_uncompressedUpdate() will fail (result is an errorCode).
 *  After an error, the state is left in a UB state, and must be re-initialized or freed.
 *  If previously a compressed block was written, buffered data is flushed
 *  before appending uncompressed data is continued.
 *  This is only supported when LZ4F_blockIndependent is used
 * 
 *  | Parameters      | Direction | Description                                              |
 *  |:----------------|:---------:|:---------------------------------------------------------|
 *  | \b cctx         |    in     | A compression context that can be employed multiple times for consecutive streaming operations. |
 *  | \b dstBuffer    |    out    | Decompressed data is written in `dstBuffer`, `dstBuffer` can freely change between each consecutive function invocation & content inside it will be overwritten. |
 *  | \b dstCapacity  |    in     | Length of dstBuffer. |
 *  | \b srcBuffer    |    in     | LZ4 Frame format compatible compressed input stream. |
 *  | \b srcSize      |    in     | Length of srcBuffer. |
 *  | \b cOptPtr      |    in     | Can provide additional options with this pointer. It is optional pointer : NULL can be provided, in which case all options are set to default. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| Number of bytes written into `dstBuffer` (it can be zero, meaning input data was just buffered). |
 * |Failure| If decompression failed, an error code is returned, which can be tested using LZ4F_isError(). |
 */
LZ4FLIB_STATIC_API size_t
LZ4F_uncompressedUpdate(LZ4F_cctx* cctx,
                        void* dstBuffer, size_t dstCapacity,
                  const void* srcBuffer, size_t srcSize,
                  const LZ4F_compressOptions_t* cOptPtr);

/**********************************
 *  Bulk processing dictionary API
 *********************************/

/* A Dictionary is useful for the compression of small messages (KB range).
 * It dramatically improves compression efficiency.
 *
 * LZ4 can ingest any input as dictionary, though only the last 64 KB are useful.
 * Best results are generally achieved by using Zstandard's Dictionary Builder
 * to generate a high-quality dictionary from a set of samples.
 *
 * Loading a dictionary has a cost, since it involves construction of tables.
 * The Bulk processing dictionary API makes it possible to share this cost
 * over an arbitrary number of compression jobs, even concurrently,
 * markedly improving compression latency for these cases.
 *
 * The same dictionary will have to be used on the decompression side
 * for decoding to be successful.
 * To help identify the correct dictionary at decoding stage,
 * the frame header allows optional embedding of a dictID field.
 */
typedef struct LZ4F_CDict_s LZ4F_CDict;

/*! @brief
 *  When compressing multiple messages / blocks using the same dictionary, it's recommended to load it just once.
 *
 *  LZ4_createCDict() will create a digested dictionary, ready to start future compression operations without startup delay.
 *  LZ4_CDict can be created once and shared by multiple threads concurrently, since its usage is read-only.
 * `dictBuffer` can be released after LZ4_CDict creation, since its content is copied within CDict
 * 
 *  | Parameters      | Direction | Description                                              |
 *  |:----------------|:---------:|:---------------------------------------------------------|
 *  | \b dictBuffer   |    in     | A dictionary used to compress multiple blocks. |
 *  | \b dictSize     |    in     | Length of dictBuffer. |
 *
 *  @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| Returns a dictionary unlike that pointed by dictBuffer where all the necessary varaibles are initialized. |
 * |Failure| If LZ4's default memory allocation fails NULL is returned. |
 */
LZ4FLIB_STATIC_API LZ4F_CDict* LZ4F_createCDict(const void* dictBuffer, size_t dictSize);

/*! @brief
 *  Releases memory occupied by LZ4F_CDict element.
 *
 *  | Parameters | Direction | Description                                              |
 *  |:-----------|:---------:|:---------------------------------------------------------|
 *  | \b CDict   |    in     | A dictionary, ready to start future compression operations without startup delay. |
 *
 *  @return \b void
 */
LZ4FLIB_STATIC_API void        LZ4F_freeCDict(LZ4F_CDict* CDict);


/*! @brief
 *  Compress an entire srcBuffer into a valid LZ4 frame using a digested Dictionary.
 *
 *  @note dstBuffer MUST be >= LZ4F_compressFrameBound(srcSize, preferencesPtr).
 *  If this condition is not respected, function will fail (@return an errorCode).
 * 
 * 
 *  | Parameters        | Direction | Description                                              |
 *  |:------------------|:---------:|:---------------------------------------------------------|
 *  | \b cctx           |    in     | A compression context that can be employed multiple times for consecutive streaming operations. \b cctx must point to a context created by LZ4F_createCompressionContext().|
 *  | \b dst            |    out    | Destination buffer, compressed data is kept here, memory should be allocated already.|
 *  | \b dstCapacity    |    in     | Size of pre-allocated 'dst' buffer.                                                  |
 *  | \b src            |    in     | Source buffer, the data you want to compress is copied/or pointed here.              |
 *  | \b srcSize        |    in     | Size of srcBuffer.                                                                   |
 *  | \b cdict          |    in     | If cdict==NULL, compress without a dictionary. |
 *  | \b preferencesPtr |    in     | Can provide preferences in compression with this pointer. It is optional pointer : NULL can be provided, but it's not recommended, as it's the only way to provide dictID in the frame header. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| Returns number of bytes written into dstBuffer. |
 * |Failure| An error code if it fails (can be tested using LZ4F_isError()). |
 */
LZ4FLIB_STATIC_API size_t
LZ4F_compressFrame_usingCDict(LZ4F_cctx* cctx,
                              void* dst, size_t dstCapacity,
                        const void* src, size_t srcSize,
                        const LZ4F_CDict* cdict,
                        const LZ4F_preferences_t* preferencesPtr);


/*! @brief
 *  Inits streaming dictionary compression, and writes the frame header into dstBuffer.
 *
 *  @note dstCapacity must be >= LZ4F_HEADER_SIZE_MAX bytes.
 * 
 *  | Parameters        | Direction | Description                                              |
 *  |:------------------|:---------:|:---------------------------------------------------------|
 *  | \b cctx           |    in     | A compression context that can be employed multiple times for consecutive streaming operations. \b cctx must point to a context created by LZ4F_createCompressionContext().|
 *  | \b dstBuffer      |    out    | Destination buffer, compressed data is kept here, memory should be allocated already. |
 *  | \b dstCapacity    |    in     | Size of pre-allocated 'dst' buffer.                                                   |
 *  | \b cdict          |    in     | If cdict==NULL, compress without a dictionary. |
 *  | \b prefsPtr       |    in     | Can provide preferences in compression with this pointer. It is optional pointer : NULL can be provided, but it's not recommended, as it's the only way to provide dictID in the frame header. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| Returns number of bytes written into dstBuffer for the header. |
 * |Failure| An error code (which can be tested using LZ4F_isError()). |
 */
LZ4FLIB_STATIC_API size_t
LZ4F_compressBegin_usingCDict(LZ4F_cctx* cctx,
                              void* dstBuffer, size_t dstCapacity,
                        const LZ4F_CDict* cdict,
                        const LZ4F_preferences_t* prefsPtr);


/*! @brief
 *  Same as LZ4F_decompress(), using a predefined dictionary.
 *
 *  Dictionary is used "in place", without any preprocessing.
 *  It must remain accessible throughout the entire frame decoding.
 * 
 *  | Parameters                  | Direction | Description                                              |
 *  |:----------------------------|:---------:|:---------------------------------------------------------|
 *  | \b dctxPtr                  |  in,out   | LZ4F_dctx object, to track all decompression operations. |
 *  | \b dstBuffer                |    out    | Decompressed data is written in `dstBuffer`, `dstBuffer` can freely change between each consecutive function invocation & content inside it will be overwritten. |
 *  | \b dstSizePtr               |  in,out   | While calling this function "*dstSizePtr" provides the length of dstBuffer, after decompression is done nb of bytes decompressed into dstBuffer will be written into *dstSizePtr (necessarily <= original value). |
 *  | \b srcBuffer                |    in     | LZ4 Frame format compatible compressed input stream. |
 *  | \b srcSizePtr               |  in,out   | It will read up to *srcSizePtr bytes from srcBuffer and write the nb of bytes consumed from srcBuffer into *srcSizePtr (necessarily <= original value).|
 *  | \b dict                     |    in     | A dictionary used to compress multiple blocks. |
 *  | \b dictSize                 |    in     | Length of dict buffer. |
 *  | \b decompressionOptionsPtr  |    in     | Advanced decompression options, could be NULL in which case default options are assumed. |
 * 
 * @return
 * |Result | Description                |
 * |:------|:---------------------------|
 * |Success| When a frame is fully decoded, 0 will be returned (no more data expected), if not a hint of how many `srcSize` bytes LZ4F_decompress_usingDict() expects for next call. |
 * |Failure| If decompression failed, an error code is returned, which can be tested using LZ4F_isError().|
 */
LZ4FLIB_STATIC_API size_t
LZ4F_decompress_usingDict(LZ4F_dctx* dctxPtr,
                          void* dstBuffer, size_t* dstSizePtr,
                    const void* srcBuffer, size_t* srcSizePtr,
                    const void* dict, size_t dictSize,
                    const LZ4F_decompressOptions_t* decompressOptionsPtr);


/*! @brief
 *  These prototypes make it possible to pass custom allocation/free functions.
 *  LZ4F_customMem is provided at state creation time, using LZ4F_create*_advanced() listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
typedef void* (*LZ4F_AllocFunction) (void* opaqueState, size_t size);
typedef void* (*LZ4F_CallocFunction) (void* opaqueState, size_t size);
typedef void  (*LZ4F_FreeFunction) (void* opaqueState, void* address);
typedef struct {
    LZ4F_AllocFunction customAlloc;
    LZ4F_CallocFunction customCalloc; /* optional; when not defined, uses customAlloc + memset */
    LZ4F_FreeFunction customFree;
    void* opaqueState;
} LZ4F_CustomMem;
static
#ifdef __GNUC__
__attribute__((__unused__))
#endif
LZ4F_CustomMem const LZ4F_defaultCMem = { NULL, NULL, NULL, NULL };  /**< this constant defers to stdlib's functions */

LZ4FLIB_STATIC_API LZ4F_cctx* LZ4F_createCompressionContext_advanced(LZ4F_CustomMem customMem, unsigned version);
LZ4FLIB_STATIC_API LZ4F_dctx* LZ4F_createDecompressionContext_advanced(LZ4F_CustomMem customMem, unsigned version);
LZ4FLIB_STATIC_API LZ4F_CDict* LZ4F_createCDict_advanced(LZ4F_CustomMem customMem, const void* dictBuffer, size_t dictSize);


#if defined (__cplusplus)
}
#endif

#endif  /* defined(LZ4F_STATIC_LINKING_ONLY) && !defined(LZ4F_H_STATIC_09782039843) */
