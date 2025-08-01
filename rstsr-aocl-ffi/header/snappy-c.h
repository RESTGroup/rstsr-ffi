/*
 * Copyright 2011 Martin Gieseking <martin.gieseking@uos.de>.
 * Modifications Copyright (C) 2023-2024, Advanced Micro Devices. All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Plain C interface (a wrapper around the C++ implementation).
 */

#ifndef THIRD_PARTY_SNAPPY_OPENSOURCE_SNAPPY_C_H_
#define THIRD_PARTY_SNAPPY_OPENSOURCE_SNAPPY_C_H_

#ifndef SNAPPYLIB_VISIBILITY
#  if defined(__GNUC__) && (__GNUC__ >= 4)
#    define SNAPPYLIB_VISIBILITY __attribute__ ((visibility ("default")))
#  else
#    define SNAPPYLIB_VISIBILITY
#  endif
#endif
#if defined(SNAPPY_DLL_EXPORT) && (SNAPPY_DLL_EXPORT==1)
#  define SNAPPYLIB_API __declspec(dllexport) SNAPPYLIB_VISIBILITY
#elif defined(SNAPPY_DLL_IMPORT) && (SNAPPY_DLL_IMPORT==1)
#  define SNAPPYLIB_API __declspec(dllimport) SNAPPYLIB_VISIBILITY /* It isn't required but allows to generate better code, saving a function pointer load from the IAT and an indirect jump.*/
#else
#  define SNAPPYLIB_API SNAPPYLIB_VISIBILITY
#endif

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

/**
 * @defgroup SNAPPY_C_API SNAPPY C API
 * @ingroup SNAPPY_API
 * 
 * @brief
 * Plain C interface (a wrapper around the C++ implementation).
 * 
 * @{
 */

/**
 * @brief
 * Return values; see the documentation for each function to know
 * what each can return.
 */
typedef enum {
  SNAPPY_OK = 0,
  SNAPPY_INVALID_INPUT = 1,
  SNAPPY_BUFFER_TOO_SMALL = 2
} snappy_status;

/**
 * @brief
 * Takes the data stored in "input[0..input_length-1]" and stores
 * it in the array pointed to by "compressed".
 *
 *  |Parameters            |Direction|Description                                                          |
 *  |:---------------------|:-------:|:--------------------------------------------------------------------|
 *  | \b input             |  in     | This is the buffer where the data we want to compress is accessible.|
 *  | \b input_length      |  in     | Length of the input buffer.                                         |
 *  | \b compressed        |  out    | This is a buffer in which compressed data is stored.                |
 *  | \b compressed_length | in,out  | \b compressed_length signals the space available in "compressed" buffer. After successful compression \b compressed_length contains the true length of the compressed output.|
 * 
 * 
 *
 * @note - Example:\n\n
 * \code{.c}
 *   size_t output_length = snappy_max_compressed_length(input_length);
 *   char* output = (char*)malloc(output_length);
 *   if (snappy_compress(input, input_length, output, &output_length)
 *       == SNAPPY_OK) {
 *     ... Process(output, output_length) ...
 *   }
 *   free(output);
 * \endcode
 * 
 *  @return
 *  |Result | Description                                                            |
 *  |:------|:-----------------------------------------------------------------------|
 *  |Success| Returns \b SNAPPY_OK if successful.                                    |
 *  |Failure| While calling this function if \b compressed_length is not at least equal to "snappy_max_compressed_length(input_length)", SNAPPY_BUFFER_TOO_SMALL is returned.|
 */
SNAPPYLIB_API snappy_status snappy_compress(const char* input,
                              size_t input_length,
                              char* compressed,
                              size_t* compressed_length);

/**
 * @brief Given data in "compressed[0..compressed_length-1]" generated by
 * calling the snappy_compress routine, this routine stores
 * the uncompressed data to
 *   uncompressed[0..uncompressed_length-1].
 *
 *
 *|Parameters              |Direction| Description                                      |
 *|:-----------------------|:-------:|:-------------------------------------------------|
 *| \b compressed          |  in     | This is a buffer which contains compressed data. |
 *| \b compressed_length   |  in     | This is the length of the compressed buffer.     |
 *| \b uncompressed        |  out    | Uncompressed data is stored in this buffer.      |
 *| \b uncompressed_length |  in,out | \b uncompressed_length signals the space available in "uncompressed". After successful decompression, \b uncompressed_length contains the true length of the decompressed output. |
 * 
 * @note - Example:\n\n
 *  \code{.c}
 *   size_t output_length;
 *   if (snappy_uncompressed_length(input, input_length, &output_length)
 *       != SNAPPY_OK) {
 *     ... fail ...
 *   }
 *   char* output = (char*)malloc(output_length);
 *   if (snappy_uncompress(input, input_length, output, &output_length)
 *       == SNAPPY_OK) {
 *     ... Process(output, output_length) ...
 *   }
 *   free(output);
 *  \endcode
 * 
 *  @return
 *  |Result | Description                                                            |
 *  |:------|:-----------------------------------------------------------------------|
 *  |Success| Returns \b SNAPPY_OK if successful.                                    |
 *  |Failure| While calling this function if \b uncompressed_length is not at least equal to the value returned by snappy_uncompressed_length for this stream, SNAPPY_BUFFER_TOO_SMALL is returned Returns (a value \b not \b equal to SNAPPY_OK) if the message is corrupted and could not be decrypted. |
 */
SNAPPYLIB_API snappy_status snappy_uncompress(const char* compressed,
                                size_t compressed_length,
                                char* uncompressed,
                                size_t* uncompressed_length);

/**
 * @brief This function determines the maximal size of the compressed representation of
 * input data that is "source_length" bytes in length.
 * 
 *  | Parameters       |Direction| Description                 |
 *  |:-----------------|:-------:|:----------------------------|
 *  | \b source_length |   in    | The size of source in bytes.|
 *
 *  @return
 *  |Result | Description                                                                                                   |
 *  |:------|:--------------------------------------------------------------------------------------------------------------|
 *  |Success|Returns the maximal size of the compressed representation of input data that is "source_bytes" bytes in length.|
 */
SNAPPYLIB_API size_t snappy_max_compressed_length(size_t source_length);

/**
 * @brief Get the Uncompressed Length object.
 * 
 * This operation takes O(1) time.
 * 
 * @attention REQUIRES: "compressed[]" was produced by RawCompress() or Compress().
 *
 *  | Parameters           |Direction| Description                                                                 |
 *  |:---------------------|:-------:|:----------------------------------------------------------------------------|
 *  | \b compressed        |  in     | This is a buffer which contains compressed data.                            |
 *  | \b compressed_length |  in     | This is the length of the compressed buffer.                                |
 *  | \b result            |  out    |  This is the pointer to type size_t where the uncompressed length is stored.|
 *
 *  @return
 *  |Result | Description                            |
 *  |:------|:---------------------------------------|
 *  |Success| Returns \b true on successful parsing. |
 *  |Failure| Returns \b false on parsing error.     |
 */
SNAPPYLIB_API snappy_status snappy_uncompressed_length(const char* compressed,
                                         size_t compressed_length,
                                         size_t* result);

/**
 * @brief 
 * Returns \b true iff the contents of "compressed[]" can be uncompressed
 * successfully.  Does not return the uncompressed data.  Takes
 * time proportional to compressed_length, but is usually at least
 * a factor of four faster than actual decompression.
 *
 *  |Parameters            |Direction|Description                                      |
 *  |:---------------------|:-------:|:------------------------------------------------|
 *  | \b compressed        |  in     | This is a buffer which contains compressed data.|
 *  | \b compressed_length |  in     | This is the length of the compressed buffer.    |
 *
 *  @return
 *  |Result | Description                                                                          |
 *  |:------|:-------------------------------------------------------------------------------------|
 *  |Success| Returns \b true iff the contents of "compressed[]" can be uncompressed successfully. |
 *  |Failure| Returns \b false if error.                                                           |
 */
SNAPPYLIB_API snappy_status snappy_validate_compressed_buffer(const char* compressed,
                                                size_t compressed_length);

/**
 * 
 * @}
 */

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  /* THIRD_PARTY_SNAPPY_OPENSOURCE_SNAPPY_C_H_ */
