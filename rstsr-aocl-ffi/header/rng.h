/*
 * Copyright AMD,NAG 2003-2023
 */


/*
 * This is the RNG header file.
 *
 * It contains function prototypes to allow a C programmer to call RNG routines
 * via their C or Fortran interfaces.
 *
 * C interfaces to RNG routines differ from FORTRAN interfaces in the following
 * major respects:
 *
 * 1. If WIN_IFORT, The FORTRAN interface names are in upper case.
 *
 * 2. If GFORTRAN,  The FORTRAN interface names are appended by an underscore.
 *
 * 3. The C interfaces contain no workspace arguments.
 *    All workspace memory is allocated internally.
 *
 * 4. Scalar input arguments are passed by value in C interfaces.
 *    FORTRAN interfaces pass all arguments (except for character
 *    string "length" arguments that are normally hidden from
 *    FORTRAN programmers) by reference.
 *
 * 5. Most arguments that are passed as character string pointers
 *    to FORTRAN interfaces are passed by value as single
 *    characters to C interfaces. The character string "length"
 *    arguments of FORTRAN interfaces are not required in the C interfaces.
 *
 * 6. It is important to note that in both the FORTRAN and C interfaces,
 *    2-dimensional arrays are assumed to be stored in column-major order.
 *    e.g. the matrix A = [ 1.0 2.0 ]
 *                        [ 3.0 4.0 ]
 *    would be stored in memory as 1.0, 3.0, 2.0, 4.0. This storage order
 *    corresponds to a FORTRAN-style 2-D array declaration A(2,2), but not
 *    to an array declared as a[2][2] in C which would be stored in
 *    row-major order as 1.0, 2.0, 3.0, 4.0.
 */


/** @file rng.h
 *
 *  @brief AOCL-RNG library header file
 */


#ifndef __RNG_H__
#define __RNG_H__


/* Under Windows math.h
 * defines "complex" to mean "_complex".
 */
#include <math.h>


#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */


/**
 * @defgroup rng RNG
 *
 * @brief AOCL-RNG Library APIs and Data Structure
 *
 */


#ifdef INTEGER64             /* ILP64, integer size: 64-bit */
  #ifdef WIN64               /* ILP64 on Windows */
    typedef __int64          rng_int_t;
    typedef unsigned __int64 rng_uint_t;
    typedef int              rng_strlen_t;
  #else                      /* ILP64 on Linux */
    typedef long             rng_int_t;
    typedef unsigned long    rng_uint_t;
    typedef int              rng_strlen_t;
  #endif /* WIN64 */
#else                        /* LP64,  integer size: 32-bit */
    typedef int              rng_int_t;
    typedef unsigned int     rng_uint_t;
    typedef int              rng_strlen_t;
#endif /* INTEGER64 */


/**
 * @addtogroup rng RNG
 *
 * @brief AOCL-RNG provides Pseudo-Random Number Generators.
 *
 * @{
 */


/**
 * @addtogroup base Base Generators
 *
 * @brief AOCL-RNG provides 7 - base generators. They consist of
 *        6 pseudo random number generators, 1 quasi random number generator.
 *        Initialization API is used to select one of the available base generators.
 *
 * @{
 */

/**
 * @brief Initialize RNG supplied Base Random Number Generators (BRNG)
 *
 * @param [in] genid  Generator ID \n
 *                    (1 <= genid <= 7) \n
 *                    1 = NAG Basic generator \n
 *                    2 = Wichmann-Hill generator \n
 *                    3 = Mersenne Twister generator \n
 *                    4 = L'Ecuyers Combined Recursive generator \n
 *                    5 = Blum-Blum Shub generator \n
 *                    6 = SIMD based Fast Mersenne Twister generator \n
 *                    7 = Sobol generator \n
 *
 * @param [in] subid  Sub-generator ID \n
 *                    (if genid=2, 1 <= subid <= 273) \n
 *                    If genid = 2, then subid indicates which of the 273
 *                    independent generators to use. \n
 *                    Otherwise subid is not referenced.
 *
 * @param [in] seed  Seed vector \n
 *                   Vector holding the seeds/initial state of the
 *                   generator, seed(lseed)
                    If GENID ≠ 5, then SEED is a vector of initial values for the base generator. \n
                    These values must be positive integers. The number of values required depends on
                    the base generator being used. The NAG basic generator requires one initial value,
                    the Wichmann-Hill generator requires four initial values,
                    the L’Ecuyer Combined Recursive Generator requires six initial values,
                    the Mersenne Twister and SFMT requires 624 initial values. \n
                    If the number of seeds required by the chosen generator is > LSEED,
                    then SEED(1) is used to initialize the NAG basic generator.
                    This is then used to generate all of the remaining seed values required. \n
                    In general, it is best not to set all the elements of SEED to anything too obvious,
                    such as a single repeated value or a simple sequence.
                    Using such a seed array may lead to several similar values being created in a row
                    when the generator is subsequently called. This is particularly true for
                    the Mersenne Twister and SFMT generators. \n
                    If GENID=7, SEED is used as single element array which is used to
                    provide dimension value. Sobol generator (QRNG) is using dimension value
                    to generate multidimensional sequences. \n
                    In order to initialize the Blum-Blum-Shub generator (i.e. if GENID = 5),
                    two large prime values, p and q are required, as well as an initial value s.
                    As p, q and s can be of an arbitrary size, these values are expressed as a polynomial in B,
                    where B = 224. For example, p can be factored into a polynomial of order lp,
                    with p = p1 + p2B + p3B2 + · · · + pl Blp−1. \n
                    The elements of SEED should then be set to the following: \n
                    SEED(1) = lp \n
                    SEED(2) to SEED(lp + 1) = p1 to plp \n
                    SEED(lp + 2) = lq \n
                    SEED(lp + 3) to SEED(lp + lq + 2) = q1 to qlq \n
                    SEED(lp + lq + 3) = ls \n
                    SEED(lp + lq + 4) to SEED(lp + lq + ls + 3) = s1 to sls \n
                    Constraint: If GENID ≠ 5, then SEED(i) > 0, i = 1, 2, ... \n
                    If GENID = 5 then, SEED must take the values described above. \n
 *
 * @param [in,out] lseed  Length of the seed vector \n
 *                        Value of lseed is dependent on the generator being used. \n
 *                        If ***lseed = -1***, then lseed is set to the value required
 *                        by currently selected generator and the function returns.
 *
 * @param [out] state  State vector \n
 *                     Vector that will define the generator and hold
 *                     its current state, state(lstate).
 *
 * @param [in,out] lstate  Length of the state vector \n
 *                         Value of lstate is dependent on the generator being used. \n
 *                         If ***lstate = -1***, then lstate is set to the value required
 *                         by the currently selected generator and the function returns.
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but generator is not initialized (as lstate or lseed were -1) \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 *
 * @note
 * - If **lstate = -1**, or **lseed = -1** then function returns without initialising
 *   the generator.
 * - If both **lstate = -1** and **lseed = -1** then both are set to their
 *   ***expected value*** before returning.
 */
void drandinitialize (  rng_int_t   genid,
                        rng_int_t   subid,
                        rng_int_t  *seed,
                        rng_int_t  *lseed,
                        rng_int_t  *state,
                        rng_int_t  *lstate,
                        rng_int_t  *info     );

/**
 * @brief Fortran version of @ref drandinitialize
 *
 */
void drandinitialize_ (  rng_int_t  *genid,
                         rng_int_t  *subid,
                         rng_int_t  *seed,
                         rng_int_t  *lseed,
                         rng_int_t  *state,
                         rng_int_t  *lstate,
                         rng_int_t  *info     );

/**
 * @brief Fortran version of @ref drandinitialize
 *
 */
void DRANDINITIALIZE (  rng_int_t  *genid,
                        rng_int_t  *subid,
                        rng_int_t  *seed,
                        rng_int_t  *lseed,
                        rng_int_t  *state,
                        rng_int_t  *lstate,
                        rng_int_t  *info     );

/**
 * @brief Single precision version of @ref drandinitialize
 *
 * @note
 * @ref srandinitialize is the single precision version of @ref drandinitialize. \n
 * The argument lists of both APIs are identical ***except*** that any
 * **double precision** arguments of @ref drandinitialize are replaced by
 * **single precision** arguments in @ref srandinitialize.
 */
void srandinitialize (  rng_int_t   genid,
                        rng_int_t   subid,
                        rng_int_t  *seed,
                        rng_int_t  *lseed,
                        rng_int_t  *state,
                        rng_int_t  *lstate,
                        rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandinitialize
 *
 */
void srandinitialize_ (  rng_int_t  *genid,
                         rng_int_t  *subid,
                         rng_int_t  *seed,
                         rng_int_t  *lseed,
                         rng_int_t  *state,
                         rng_int_t  *lstate,
                         rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandinitialize
 *
 */
void SRANDINITIALIZE (  rng_int_t  *genid,
                        rng_int_t  *subid,
                        rng_int_t  *seed,
                        rng_int_t  *lseed,
                        rng_int_t  *state,
                        rng_int_t  *lstate,
                        rng_int_t  *info     );

/**
 * @brief Initialize RNG supplied BRNG using hardware generated random seed values
 *
 * @param [in] genid  Generator ID \n
 *                    (genid = 3) \n
 *                    3 = Mersenne Twister generator \n
 *
 * @param [in] subid  Sub-generator ID, Not referenced \n
 *
 * @param [out] state  State vector \n
 *                     Vector that will define the generator and hold
 *                     its current state, state(lstate).
 *
 * @param [in,out] lstate  Length of the state vector \n
 *                         Value of lstate is dependent on the generator being used. \n
 *                         If lstate = -1, then lstate is set to the value required
 *                         by the currently selected generator and the function returns.
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but generator not initialized (as lstate were -1) \n
 *                    2 = Non supported OS \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 *
 */
void drandinitialize_secrng (  rng_int_t   genid,
                               rng_int_t   subid,
                               rng_int_t  *state,
                               rng_int_t  *lstate,
                               rng_int_t  *info     );

/**
 * @}
*/


/**
 * @addtogroup user User Supplied Generators
 *
 * @brief AOCL-RNG supports User-supplied generator's registry as base generator.
 *
 * @{
 */

typedef void (* RNG_DRANDINITIALIZEUSER_UINI) (rng_int_t *, rng_int_t *, rng_int_t *, \
                                               rng_int_t *, rng_int_t *, rng_int_t *, rng_int_t *);

typedef void (* RNG_DRANDINITIALIZEUSER_UGEN) (rng_int_t *, \
                                               rng_int_t *, double *, rng_int_t *);

typedef void (* RNG_SRANDINITIALIZEUSER_UINI) (rng_int_t *, rng_int_t *, rng_int_t *, \
                                               rng_int_t *, rng_int_t *, rng_int_t *, rng_int_t *);

typedef void (* RNG_SRANDINITIALIZEUSER_UGEN) (rng_int_t *, \
                                               rng_int_t *, float *, rng_int_t *);

/**
 * @brief Initialise a User supplied Base Generator
 *
 * @param [in] uini   Subroutine used to initialize the User supplied Generator
 *
 * @param [in] ugen   Subroutine for User defined Base Generator
 *
 * @param [in] genid  Generator ID
 *
 * @param [in] subid  Sub-generator ID
 *
 * @param [in] seed   Seed, seed(lseed)
 *
 * @param [in,out] lseed  Length of the seed vector \n
 *                        Value of lseed is dependent on the generator being used. \n
 *                        If ***lseed < 1***, then lseed is set to the value required
 *                        by currently selected generator and the function returns.
 *
 * @param [out] state  State vector \n
 *                     Vector that will define the generator and hold
 *                     its current state, state(lstate).
 *
 * @param [in,out] lstate  Length of the state vector \n
 *                         Value of lstate is dependent on the generator being used. \n
 *                         If ***lstate < 1***, then lstate is set to the value required
 *                         by the currently selected generator and the function returns.
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info)
 *
 * @note
 * genid, subid, lseed, seed and lstate are passed directly to the subroutine,
 * supplied as **uini**.
 */
void drandinitializeuser (  RNG_DRANDINITIALIZEUSER_UINI   uini,
                            RNG_DRANDINITIALIZEUSER_UGEN   ugen,
                            rng_int_t                      genid,
                            rng_int_t                      subid,
                            rng_int_t                     *seed,
                            rng_int_t                     *lseed,
                            rng_int_t                     *state,
                            rng_int_t                     *lstate,
                            rng_int_t                     *info     );

/**
 * @brief Fortran version of @ref drandinitializeuser
 *
 */
void drandinitializeuser_ (  RNG_DRANDINITIALIZEUSER_UINI   uini,
                             RNG_DRANDINITIALIZEUSER_UGEN   ugen,
                             rng_int_t                      genid,
                             rng_int_t                      subid,
                             rng_int_t                     *seed,
                             rng_int_t                     *lseed,
                             rng_int_t                     *state,
                             rng_int_t                     *lstate,
                             rng_int_t                     *info     );

/**
 * @brief Fortran version of @ref drandinitializeuser
 *
 */
void DRANDINITIALIZEUSER (  RNG_DRANDINITIALIZEUSER_UINI   uini,
                            RNG_DRANDINITIALIZEUSER_UGEN   ugen,
                            rng_int_t                      genid,
                            rng_int_t                      subid,
                            rng_int_t                     *seed,
                            rng_int_t                     *lseed,
                            rng_int_t                     *state,
                            rng_int_t                     *lstate,
                            rng_int_t                     *info     );

/**
 * @brief Single precision of @ref drandinitializeuser
 *
 * @note
 * @ref srandinitializeuser is the single precision version of @ref drandinitializeuser. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandinitializeuser are replaced by
 * **single precision** in @ref srandinitializeuser.
 */
void srandinitializeuser (  RNG_SRANDINITIALIZEUSER_UINI   uini,
                            RNG_SRANDINITIALIZEUSER_UGEN   ugen,
                            rng_int_t                      genid,
                            rng_int_t                      subid,
                            rng_int_t                     *seed,
                            rng_int_t                     *lseed,
                            rng_int_t                     *state,
                            rng_int_t                     *lstate,
                            rng_int_t                     *info     );

/**
 * @brief Fortran version of @ref srandinitializeuser
 *
 */
void srandinitializeuser_ (  RNG_SRANDINITIALIZEUSER_UINI   uini,
                             RNG_SRANDINITIALIZEUSER_UGEN   ugen,
                             rng_int_t                      genid,
                             rng_int_t                      subid,
                             rng_int_t                     *seed,
                             rng_int_t                     *lseed,
                             rng_int_t                     *state,
                             rng_int_t                     *lstate,
                             rng_int_t                     *info     );

/**
 * @brief Fortran version of @ref srandinitializeuser
 *
 */
void SRANDINITIALIZEUSER (  RNG_SRANDINITIALIZEUSER_UINI   uini,
                            RNG_SRANDINITIALIZEUSER_UGEN   ugen,
                            rng_int_t                      genid,
                            rng_int_t                      subid,
                            rng_int_t                     *seed,
                            rng_int_t                     *lseed,
                            rng_int_t                     *state,
                            rng_int_t                     *lstate,
                            rng_int_t                     *info     );

/**
 * @}
 */


/**
 * @addtogroup blum Blum-Blum-Shub Generator
 *
 * @brief Blum-Blum-Shub base generator is cryptograhically secure under
 *        the assumption that the quadratic residuosity problem is intractable.
 *
 * @{
*/

/**
 * @brief Alternative initialisation routine for the Blum-Blum-Shub generator
 *
 * @param [in] nbits  The number of bits to use from each iteration \n
 *                    if **nbits < 1**,  then **nbits = 1**, \n
 *                    if **nbits > 15**, then **nbits = 15**
 *                    (i.e. between 1 and 15 bits can be used)
 *
 * @param [in] lp  The number of digits in the first prime **p, lp <= 25**
 *
 * @param [in] p  The first prime value \n
 *                This integer is stored in an array. \n
 *                Each element of the array is a digit, and the integer is stored \n
 *                base B2, where B2 = 2**30, i.e. \n
 *                p = p(1) + p(2)*B2 + p(3)*B2**2 + ... + p(lp)*B2**(lp-1), p(lp). \n
 *                p mod 4 = 3 \n
 *
 * @param [in] lq  The number of digits in the second prime **q, lq <= 25**
 *
 * @param [in] q  The first prime value \n
 *                This integer is stored in an array. \n
 *                Each element of the array is a digit, and the integer is stored \n
 *                base B2, where B2 = 2**30, i.e. \n
 *                q = p(1) + q(2)*B2 + q(3)*B2**2 + ... + q(lq)*B2**(lq-1), q(lq). \n
 *                q mod 4 = 3 \n
 *
 * @param [in] ls  The number of digits in the initial state **s, ls <= 25**
 *
 * @param [in] s  The first prime value \n
 *                This integer is stored in an array. \n
 *                Each element of the array is a digit, and the integer is stored \n
 *                base B2, where B2 = 2**30, i.e. \n
 *                s = s(1) + s(2)*B2 + s(3)*B2**2 + ... + s(lp)*B2**(ls-1), s(ls). \n
 *
 * @param [out] state  Array holding information about the generator: \n
 *                       1 = Amount of state used = 310 \n
 *                       2 = 'Magic' number = 5010 \n
 *                       3 = GENID \n
 *                       4 = SUBID (the number of bits to use) \n
 *                       5 = PN, the location in state of N \n
 *                       6 = PX, the location in state of X \n
 *                      PX = LX, the number of digits of X \n
 *                      PX+1 to PX+LX = The digits of X \n
 *                      PN+1 to PN+LN = The digits of N \n
 *                      310 = 'Magic' number = 5010 \n
 *
 * @param [in,out] lstate  Length of array state, lstate >=
 *                         If **lstate < 1**, then lstate is set to
 *                         310 and the function returns.
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but generator not initialized
 *                        (returned when values of LSEED or lstate given) \n
 *                    0 = Everything OK \n
 *             -1 to -8 = Error in parameter, abs(info).
 */
void drandinitializebbs (  rng_int_t   nbits,
                           rng_int_t   lp,
                           rng_int_t  *p,
                           rng_int_t   lq,
                           rng_int_t  *q,
                           rng_int_t   ls,
                           rng_int_t  *s,
                           rng_int_t  *state,
                           rng_int_t  *lstate,
                           rng_int_t  *info     );

/**
 *  @brief Fortran version of @ref drandinitializebbs
 *
 */
void drandinitializebbs_ (  rng_int_t  *nbits,
                            rng_int_t  *lp,
                            rng_int_t  *p,
                            rng_int_t  *lq,
                            rng_int_t  *q,
                            rng_int_t  *ls,
                            rng_int_t  *s,
                            rng_int_t  *state,
                            rng_int_t  *lstate,
                            rng_int_t  *info     );

/**
 *  @brief Fortran version of @ref drandinitializebbs
 *
 */
void DRANDINITIALIZEBBS (  rng_int_t  *nbits,
                           rng_int_t  *lp,
                           rng_int_t  *p,
                           rng_int_t  *lq,
                           rng_int_t  *q,
                           rng_int_t  *ls,
                           rng_int_t  *s,
                           rng_int_t  *state,
                           rng_int_t  *lstate,
                           rng_int_t  *info     );

/**
 * @brief Single precision of @ref drandinitializebbs
 *
 * @note
 * @ref srandinitializebbs is the single precision version of @ref drandinitializebbs. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandinitializebbs are replaced by
 * **single precision** arguments in @ref srandinitializebbs.
 */
void srandinitializebbs (  rng_int_t   nbits,
                           rng_int_t   lp,
                           rng_int_t  *p,
                           rng_int_t   lq,
                           rng_int_t  *q,
                           rng_int_t   ls,
                           rng_int_t  *s,
                           rng_int_t  *state,
                           rng_int_t  *lstate,
                           rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandinitializebbs
 *
 */
void srandinitializebbs_ (  rng_int_t  *nbits,
                            rng_int_t  *lp,
                            rng_int_t  *p,
                            rng_int_t  *lq,
                            rng_int_t  *q,
                            rng_int_t  *ls,
                            rng_int_t  *s,
                            rng_int_t  *state,
                            rng_int_t  *lstate,
                            rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandinitializebbs
 *
 */
void SRANDINITIALIZEBBS (  rng_int_t  *nbits,
                           rng_int_t  *lp,
                           rng_int_t  *p,
                           rng_int_t  *lq,
                           rng_int_t  *q,
                           rng_int_t  *ls,
                           rng_int_t  *s,
                           rng_int_t  *state,
                           rng_int_t  *lstate,
                           rng_int_t  *info     );

/**
 * @brief Alternative interface to Blum-Blum-Shub generator
 *
 * @param [in] n  Number of values to generate, n >= 0
 *
 * @param [in,out] state  On Entry: State of generator before values are generated \n
 *                        On Exit : State of generator after values have been generated \n
 *
 * @param [out] x  Vector of values, x(n) \n
 *                 The least-significant 30-bits of **x** are generated via
 *                 BBS generator.
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -2 = Error in parameter, abs(info).
 */
void drandblumblumshub (  rng_int_t   n,
                          rng_int_t  *state,
                          double     *x,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandblumblumshub
 *
 */
void drandblumblumshub_ (  rng_int_t  *n,
                           rng_int_t  *state,
                           double     *x,
                           rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandblumblumshub
 *
 */
void DRANDBLUMBLUMSHUB (  rng_int_t  *n,
                          rng_int_t  *state,
                          double     *x,
                          rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandblumblumshub
 *
 * @note
 * @ref srandblumblumshub is the single precision version of @ref drandblumblumshub. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandblumblumshub are replaced by
 * **single precision** in @ref srandblumblumshub.
 */
void srandblumblumshub (  rng_int_t  n,
                          rng_int_t  *state,
                          float      *x,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandblumblumshub
 *
 */
void srandblumblumshub_ (  rng_int_t  *n,
                           rng_int_t  *state,
                           float      *x,
                           rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandblumblumshub
 *
 */
void SRANDBLUMBLUMSHUB (  rng_int_t  *n,
                          rng_int_t  *state,
                          float      *x,
                          rng_int_t  *info    );

/**
 * @}
 */


/**
 * @addtogroup dist Distribution Generator
 *
 * @brief Distribution generators takes variates generated from base generator
 *        and transform them into variates from specific distribution.
 *
 * @{
 */

/**
 * @addtogroup contuni Continuous Univariate Distributions
 *
 * @brief These distribution functions involve single variable which is
 *        continuous in nature. These are used to generate real numbers.
 *
 * @{
 */

/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from beta distribution
 *
 * @param [in] n  Number of values to return, **n >= 0**
 *
 * @param [in] a  First parameter of the distribution, **a > 0**
 *
 * @param [in] b  Second parameter of the distribution, **b > 0**
 *
 * @param [out] state  On Entry: Current state of the base generator being used.
 *                               state must have first been initialized with a
 *                               call to @ref drandinitialize \n
 *                      On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n values from the beta distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandbeta (  rng_int_t   n,
                  double      a,
                  double      b,
                  rng_int_t  *state,
                  double     *x,
                  rng_int_t  *info    );

/**
 * @brief Fortran version (linux) of @ref drandbeta
 *
 */
void drandbeta_ (  rng_int_t  *n,
                   double     *a,
                   double     *b,
                   rng_int_t  *state,
                   double     *x,
                   rng_int_t  *info    );

/**
 * @brief Fortran version (windows) of @ref drandbeta
 *
 */
void DRANDBETA (  rng_int_t  *n,
                  double     *a,
                  double     *b,
                  rng_int_t  *state,
                  double     *x,
                  rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandbeta
 *
 * @note
 * @ref srandbeta is the single precision version of @ref drandbeta. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of DRANDBETA are replaced by
 * **single precision** arguments in @ref srandbeta.
 */
void srandbeta (  rng_int_t   n,
                  float       a,
                  float       b,
                  rng_int_t  *state,
                  float      *x,
                  rng_int_t  *info    );

/**
 * @brief Fortran version (linux) of @ref srandbeta
 *
 */
void srandbeta_ (  rng_int_t  *n,
                   float      *a,
                   float      *b,
                   rng_int_t  *state,
                   float      *x,
                   rng_int_t  *info    );

/**
 * @brief Fortran version (windows) of @ref srandbeta
 *
 */
void SRANDBETA (  rng_int_t  *n,
                  float      *a,
                  float      *b,
                  rng_int_t  *state,
                  float      *x,
                  rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from cauchy distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] a  Median
 *
 * @param [in] b  Semi-quartile range, **b >= 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the cauchy distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandcauchy (  rng_int_t   n,
                    double      a,
                    double      b,
                    rng_int_t  *state,
                    double     *x,
                    rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandcauchy
 *
 */
void drandcauchy_ (  rng_int_t  *n,
                     double     *a,
                     double     *b,
                     rng_int_t  *state,
                     double     *x,
                     rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandcauchy
 *
 */
void DRANDCAUCHY (  rng_int_t  *n,
                    double     *a,
                    double     *b,
                    rng_int_t  *state,
                    double     *x,
                    rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandcauchy
 *
 * @note
 * @ref srandcauchy is the single precision version of @ref drandcauchy. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandcauchy are replaced by
 * **single precision** arguments in @ref srandcauchy.
 */
void srandcauchy (  rng_int_t   n,
                    float       a,
                    float       b,
                    rng_int_t  *state,
                    float      *x,
                    rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandcauchy
 *
 */
void srandcauchy_ (  rng_int_t  *n,
                     float      *a,
                     float      *b,
                     rng_int_t  *state,
                     float      *x,
                     rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandcauchy
 *
 */
void SRANDCAUCHY (  rng_int_t  *n,
                    float      *a,
                    float      *b,
                    rng_int_t  *state,
                    float      *x,
                    rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from chi-squared distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] df  Degrees of freedom, **df > 0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the chi-squared distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandchisquared (  rng_int_t   n,
                        rng_int_t   df,
                        rng_int_t  *state,
                        double     *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandchisquared
 *
 */
void drandchisquared_ (  rng_int_t  *n,
                         rng_int_t  *df,
                         rng_int_t  *state,
                         double     *x,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandchisquared
 *
 */
void DRANDCHISQUARED (  rng_int_t  *n,
                        rng_int_t  *df,
                        rng_int_t  *state,
                        double     *x,
                        rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandchisquared
 *
 * @note
 * @ref srandchisquared is the single precision version of @ref drandchisquared. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandchisquared are replaced by
 * **single precision** arguments in @ref srandchisquared.
 */
void srandchisquared (  rng_int_t   n,
                        rng_int_t   df,
                        rng_int_t  *state,
                        float      *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandchisquared
 *
 */
void srandchisquared_ (  rng_int_t  *n,
                         rng_int_t  *df,
                         rng_int_t  *state,
                         float      *x,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandchisquared
 *
 */
void SRANDCHISQUARED (  rng_int_t  *n,
                        rng_int_t  *df,
                        rng_int_t  *state,
                        float      *x,
                        rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from exponential distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] a  Expontential parameter, **a >= 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the exponential distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandexponential (  rng_int_t   n,
                         double      a,
                         rng_int_t  *state,
                         double     *x,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandexponential
 *
 */
void drandexponential_ (  rng_int_t  *n,
                          double     *a,
                          rng_int_t  *state,
                          double     *x,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandexponential
 *
 */
void DRANDEXPONENTIAL (  rng_int_t  *n,
                         double     *a,
                         rng_int_t  *state,
                         double     *x,
                         rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandexponential
 *
 * @note
 * @ref srandexponential is the single precision version of @ref drandexponential. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandexponential are replaced by
 * **single precision** arguments in @ref srandexponential.
 */
void srandexponential (  rng_int_t   n,
                         float       a,
                         rng_int_t  *state,
                         float      *x,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandexponential
 *
 */
void srandexponential_ (  rng_int_t  *n,
                          float      *a,
                          rng_int_t  *state,
                          float      *x,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandexponential
 *
 */
void SRANDEXPONENTIAL (  rng_int_t  *n,
                         float      *a,
                         rng_int_t  *state,
                         float      *x,
                         rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from F- distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] df1  First degrees of freedom, **df1 > 0**
 *
 * @param [in] df2  Second degrees of freedom, **df2 > 0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the F- distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandf (  rng_int_t   n,
               rng_int_t   df1,
               rng_int_t   df2,
               rng_int_t  *state,
               double     *x,
               rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandf
 *
 */
void drandf_ (  rng_int_t  *n,
                rng_int_t  *df1,
                rng_int_t  *df2,
                rng_int_t  *state,
                double     *x,
                rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandf
 *
 */
void DRANDF (  rng_int_t  *n,
               rng_int_t  *df1,
               rng_int_t  *df2,
               rng_int_t  *state,
               double     *x,
               rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandf
 *
 * @note
 * @ref srandf is the single precision version of @ref drandf. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandf are replaced by
 * **single precision** arguments in @ref srandf.
 */
void srandf (  rng_int_t   n,
               rng_int_t   df1,
               rng_int_t   df2,
               rng_int_t  *state,
               float      *x,
               rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandf
 *
 */
void srandf_ (  rng_int_t  *n,
                rng_int_t  *df1,
                rng_int_t  *df2,
                rng_int_t  *state,
                float      *x,
                rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandf
 *
 */
void SRANDF (  rng_int_t  *n,
               rng_int_t  *df1,
               rng_int_t  *df2,
               rng_int_t  *state,
               float      *x,
               rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from exponential distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] a  First parameter of the distribution, **a > 0**
 *
 * @param [in] b  Second parameter of the distribution, **b > 0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the gamma distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *            -1 to - 4 = Error in parameter, abs(info) \n
 */
void drandgamma (  rng_int_t   n,
                   double      a,
                   double      b,
                   rng_int_t  *state,
                   double     *x,
                   rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgamma
 *
 */
void drandgamma_ (  rng_int_t  *n,
                    double     *a,
                    double     *b,
                    rng_int_t  *state,
                    double     *x,
                    rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgamma
 *
 */
void DRANDGAMMA (  rng_int_t  *n,
                   double     *a,
                   double     *b,
                   rng_int_t  *state,
                   double     *x,
                   rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandgamma
 *
 * @note
 * @ref srandgamma is the single precision version of @ref drandgamma. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandgamma are replaced by
 * **single precision** arguments in @ref srandgamma.
 */
void srandgamma (  rng_int_t   n,
                   float       a,
                   float       b,
                   rng_int_t  *state,
                   float      *x,
                   rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgamma
 *
 */
void srandgamma_ (  rng_int_t  *n,
                    float      *a,
                    float      *b,
                    rng_int_t  *state,
                    float      *x,
                    rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgamma
 *
 */
void SRANDGAMMA (  rng_int_t  *n,
                   float      *a,
                   float      *b,
                   rng_int_t  *state,
                   float      *x,
                   rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from gaussian (or normal) distribution
 *
 * @param [in] n  Number of values return, **n > 0**
 *
 * @param [in] xmu  Mean of distribution
 *
 * @param [in] var  Variance of distribution, **var >= 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the gaussian distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandgaussian (  rng_int_t   n,
                      double      xmu,
                      double      var,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgaussian
 *
 */
void drandgaussian_ (  rng_int_t  *n,
                       double     *xmu,
                       double     *var,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgaussian
 *
 */
void DRANDGAUSSIAN (  rng_int_t  *n,
                      double     *xmu,
                      double     *var,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandgaussian
 *
 * @note
 * @ref srandgaussian is the single precision version of @ref drandgaussian. \n
 * The argument lists of both routines are identical ***except*** that any
 * double precision arguments of @ref drandgaussian are replaced by
 * single precision arguments in @ref srandgaussian.
 */
void srandgaussian (  rng_int_t   n,
                      float       xmu,
                      float       var,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgaussian
 *
 */
void srandgaussian_ (  rng_int_t  *n,
                       float      *xmu,
                       float      *var,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgaussian
 *
 */
void SRANDGAUSSIAN (  rng_int_t  *n,
                      float      *xmu,
                      float      *var,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from logistic distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] a  Mean of distribution \n
 *
 * @param [in] b  Spread of distribution, \n
 *                **b = sqrt(3)/PI * standard deviation, b > 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the logistic distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandlogistic (  rng_int_t   n,
                      double      a,
                      double      b,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandlogistic
 *
 */
void drandlogistic_ (  rng_int_t  *n,
                       double     *a,
                       double     *b,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandlogistic
 *
 */
void DRANDLOGISTIC (  rng_int_t  *n,
                      double     *a,
                      double     *b,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandlogistic
 *
 * @note
 * @ref srandlogistic is the single precision version of @ref drandlogistic. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandlogistic are replaced by
 * **single precision** arguments in @ref srandlogistic.
 */
void srandlogistic (  rng_int_t   n,
                      float       a,
                      float       b,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandlogistic
 *
 */
void srandlogistic_ (  rng_int_t  *n,
                       float      *a,
                       float      *b,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandlogistic
 *
 */
void SRANDLOGISTIC (  rng_int_t  *n,
                      float      *a,
                      float      *b,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from log-normal distribution based on
 *        underlying normal distribution of **n(xmu,var)**
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] xmu  Mean of the underlying normal distribution
 *
 * @param [in] var  Variance of the underlying normal distribution, **var >= 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the log-normal distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandlognormal (  rng_int_t   n,
                       double      xmu,
                       double      var,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandlognormal
 *
 */
void drandlognormal_ (  rng_int_t  *n,
                        double     *xmu,
                        double     *var,
                        rng_int_t  *state,
                        double     *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandlognormal
 *
 */
void DRANDLOGNORMAL (  rng_int_t  *n,
                       double     *xmu,
                       double     *var,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandlognormal
 *
 * @note
 * @ref srandlognormal is the single precision version of @ref drandlognormal. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandlognormal are replaced by
 * **single precision** arguments in @ref srandlognormal.
 */
void srandlognormal (  rng_int_t   n,
                       float       xmu,
                       float       var,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandlognormal
 *
 */
void srandlognormal_ (  rng_int_t  *n,
                        float      *xmu,
                        float      *var,
                        rng_int_t  *state,
                        float      *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandlognormal
 *
 */
void SRANDLOGNORMAL (  rng_int_t  *n,
                       float      *xmu,
                       float      *var,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from students t distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] df  Degrees of freedom, **df > 0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the von mises distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandstudentst (  rng_int_t   n,
                       rng_int_t   df,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandstudentst
 *
 */
void drandstudentst_ (  rng_int_t  *n,
                        rng_int_t  *df,
                        rng_int_t  *state,
                        double     *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandstudentst
 *
 */
void DRANDSTUDENTST (  rng_int_t  *n,
                       rng_int_t  *df,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandstudentst
 *
 * @note
 * @ref srandstudentst is the single precision version of @ref drandstudentst. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandstudentst are replaced by
 * **single precision** arguments in @ref srandstudentst.
 */
void srandstudentst (  rng_int_t   n,
                       rng_int_t   df,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandstudentst
 *
 */
void srandstudentst_ (  rng_int_t  *n,
                        rng_int_t  *df,
                        rng_int_t  *state,
                        float      *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandstudentst
 *
 */
void SRANDSTUDENTST (  rng_int_t  *n,
                       rng_int_t  *df,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from triangular distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] xmin  Minimum of distribution
 *
 * @param [in] xmed  Median of distribution, **xmin <= xmed <= xmax**
 *
 * @param [in] xmax  Maximum of distribution
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the triangular distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 */
void drandtriangular (  rng_int_t   n,
                        double      xmin,
                        double      xmed,
                        double      xmax,
                        rng_int_t  *state,
                        double     *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandtriangular
 *
 */
void drandtriangular_ (  rng_int_t  *n,
                         double     *xmin,
                         double     *xmed,
                         double     *xmax,
                         rng_int_t  *state,
                         double     *x,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandtriangular
 *
 */
void DRANDTRIANGULAR (  rng_int_t  *n,
                        double     *xmin,
                        double     *xmed,
                        double     *xmax,
                        rng_int_t  *state,
                        double     *x,
                        rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandtriangular
 *
 * @note
 * @ref srandtriangular is the single precision version of @ref drandtriangular. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandtriangular are replaced by
 * **single precision** arguments in @ref srandtriangular.
 */
void srandtriangular (  rng_int_t   n,
                        float       xmin,
                        float       xmed,
                        float       xmax,
                        rng_int_t  *state,
                        float      *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandtriangular
 *
 */
void srandtriangular_ (  rng_int_t  *n,
                         float      *xmin,
                         float      *xmed,
                         float      *xmax,
                         rng_int_t  *state,
                         float      *x,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandtriangular
 *
 */
void SRANDTRIANGULAR (  rng_int_t  *n,
                        float      *xmin,
                        float      *xmed,
                        float      *xmax,
                        rng_int_t  *state,
                        float      *x,
                        rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        uniformly distributed between **a** and **b**
 *
 * @param [in] n  Number of values to generate, **n >= 0**
 *
 * @param [in] a  Lower limit for distribution
 *
 * @param [in] b  Upper limit for distribution
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of values, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void dranduniform (  rng_int_t   n,
                     double      a,
                     double      b,
                     rng_int_t  *state,
                     double     *x,
                     rng_int_t  *info    );

/**
 * @brief Fortran version of @ref dranduniform
 *
 */
void dranduniform_ (  rng_int_t  *n,
                      double     *a,
                      double     *b,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref dranduniform
 *
 */
void DRANDUNIFORM (  rng_int_t  *n,
                     double     *a,
                     double     *b,
                     rng_int_t  *state,
                     double     *x,
                     rng_int_t  *info    );

/**
 * @brief Single precision of @ref dranduniform
 *
 * @note
 * @ref sranduniform is the single precision version of @ref dranduniform. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref dranduniform are replaced by
 * **single precision** arguments in @ref sranduniform.
 */
void sranduniform (  rng_int_t   n,
                     float       a,
                     float       b,
                     rng_int_t  *state,
                     float      *x,
                     rng_int_t  *info    );

/**
 * @brief Fortran version of @ref sranduniform
 *
 */
void sranduniform_ (  rng_int_t  *n,
                      float      *a,
                      float      *b,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref sranduniform
 *
 */
void SRANDUNIFORM (  rng_int_t  *n,
                     float      *a,
                     float      *b,
                     rng_int_t  *state,
                     float      *x,
                     rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from von-mises distribution
 *
 * @param [in] n  Number of values to return, **n > 0**
 *
 * @param [in] vk  Concentration parameter, **vk > 0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the Von-Mises distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandvonmises (  rng_int_t   n,
                      double      vk,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandvonmises
 *
 */
void drandvonmises_ (  rng_int_t  *n,
                       double     *vk,
                       rng_int_t  *state,
                       double     *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandvonmises
 *
 */
void DRANDVONMISES (  rng_int_t  *n,
                      double     *vk,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandvonmises
 *
 * @note
 * @ref srandvonmises is the single precision version of @ref drandvonmises. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandvonmises are replaced by
 * **single precision** arguments in @ref srandvonmises.
 */
void srandvonmises (  rng_int_t   n,
                      float       vk,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandvonmises
 *
 */
void srandvonmises_ (  rng_int_t  *n,
                       float      *vk,
                       rng_int_t  *state,
                       float      *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandvonmises
 *
 */
void SRANDVONMISES (  rng_int_t  *n,
                      float      *vk,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from weibull distribution
 *
 * @param [in] n  Number of values to generate, **n > 0**
 *
 * @param [in] a  Shape parameter, **a > 0.0**
 *
 * @param [in] b  Scale parameter, **b > 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- values from the weibull distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandweibull (  rng_int_t   n,
                     double      a,
                     double      b,
                     rng_int_t  *state,
                     double     *x,
                     rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandweibull
 *
 */
void drandweibull_ (  rng_int_t  *n,
                      double     *a,
                      double     *b,
                      rng_int_t  *state,
                      double     *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandweibull
 *
 */
void DRANDWEIBULL (  rng_int_t  *n,
                     double     *a,
                     double     *b,
                     rng_int_t  *state,
                     double     *x,
                     rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandweibull
 *
 * @note
 * @ref srandweibull is the single precision version of @ref drandweibull. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandweibull are replaced by
 * **single precision** arguments in @ref srandweibull.
 */
void srandweibull (  rng_int_t   n,
                     float       a,
                     float       b,
                     rng_int_t  *state,
                     float      *x,
                     rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandweibull
 *
 */
void srandweibull_ (  rng_int_t  *n,
                      float      *a,
                      float      *b,
                      rng_int_t  *state,
                      float      *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandweibull
 *
 */
void SRANDWEIBULL (  rng_int_t  *n,
                     float      *a,
                     float      *b,
                     rng_int_t  *state,
                     float      *x,
                     rng_int_t  *info    );

/**
 * @}
 */


/**
 * @addtogroup disuni Discrete Univariate Distributions
 *
 * @brief These distribution functions involve single variable which is
 *        discrete in nature. These are used to generate integers.
 *
 * @{
 */

/**
 * @brief Returns a vector of pseudo-random integers
 *        from binomial distribution
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] m  Number of trials, **m >= 0**
 *
 * @param [in] p  Probability of success, **0 <= p <= 1**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the binomial distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandbinomial (  rng_int_t   n,
                      rng_int_t   m,
                      double      p,
                      rng_int_t  *state,
                      rng_int_t  *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandbinomial
 *
 */
void drandbinomial_ (  rng_int_t  *n,
                       rng_int_t  *m,
                       double     *p,
                       rng_int_t  *state,
                       rng_int_t  *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandbinomial
 *
 */
void DRANDBINOMIAL (  rng_int_t  *n,
                      rng_int_t  *m,
                      double     *p,
                      rng_int_t  *state,
                      rng_int_t  *x,
                      rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandbinomial
 *
 * @note
 * @ref srandbinomial is the single precision version of @ref drandbinomial. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandbinomial are replaced by
 * **single precision** arguments in @ref srandbinomial.
 */
void srandbinomial (  rng_int_t   n,
                      rng_int_t   m,
                      float       p,
                      rng_int_t  *state,
                      rng_int_t  *x,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandbinomial
 *
 */
void srandbinomial_ (  rng_int_t  *n,
                       rng_int_t  *m,
                       float      *p,
                       rng_int_t  *state,
                       rng_int_t  *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandbinomial
 *
 */
void SRANDBINOMIAL (  rng_int_t  *n,
                      rng_int_t  *m,
                      float      *p,
                      rng_int_t  *state,
                      rng_int_t  *x,
                      rng_int_t  *info    );


/**
 * @brief Returns a vector of pseudo-random integers
 *        from geometric distribution
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] p  Distribution parameter, **EPS < p <= 1.0**,
 *                where **EPS** is the machine precision \n
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the geometric distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandgeometric (  rng_int_t   n,
                       double      p,
                       rng_int_t  *state,
                       rng_int_t  *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgeometric
 *
 */
void drandgeometric_ (  rng_int_t  *n,
                        double     *p,
                        rng_int_t  *state,
                        rng_int_t  *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgeometric
 *
 */
void DRANDGEOMETRIC (  rng_int_t  *n,
                       double     *p,
                       rng_int_t  *state,
                       rng_int_t  *x,
                       rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandgeometric
 *
 * @note
 * @ref srandgeometric is the single precision version of @ref drandgeometric. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandgeometric are replaced by
 * **single precision** arguments in @ref srandgeometric.
 */
void srandgeometric (  rng_int_t   n,
                       float       p,
                       rng_int_t  *state,
                       rng_int_t  *x,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgeometric
 *
 */
void srandgeometric_ (  rng_int_t  *n,
                        float      *p,
                        rng_int_t  *state,
                        rng_int_t  *x,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgeometric
 *
 */
void SRANDGEOMETRIC (  rng_int_t  *n,
                       float      *p,
                       rng_int_t  *state,
                       rng_int_t  *x,
                       rng_int_t  *info    );


/**
 * @brief Returns a vector of pseudo-random integers
 *        from hypergeometric distribution
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] np  Size of the population, **np >= 0**
 *
 * @param [in] ns  Size of the sample, **0 <= ns <= np**
 *
 * @param [in] m  Number of 'specified' items in the population, **0 <= m <= np**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the hypergeometric distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 */
void drandhypergeometric (  rng_int_t   n,
                            rng_int_t   np,
                            rng_int_t   ns,
                            rng_int_t   m,
                            rng_int_t  *state,
                            rng_int_t  *x,
                            rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandhypergeometric
 *
 */
void drandhypergeometric_ (  rng_int_t  *n,
                             rng_int_t  *np,
                             rng_int_t  *ns,
                             rng_int_t  *m,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandhypergeometric
 *
 */
void DRANDHYPERGEOMETRIC (  rng_int_t  *n,
                            rng_int_t  *np,
                            rng_int_t  *ns,
                            rng_int_t  *m,
                            rng_int_t  *state,
                            rng_int_t  *x,
                            rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandhypergeometric
 *
 * @note
 * @ref srandhypergeometric is the single precision version of @ref drandhypergeometric. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandhypergeometric are replaced by
 * **single precision** arguments in @ref srandhypergeometric.
 */
void srandhypergeometric (  rng_int_t   n,
                            rng_int_t   np,
                            rng_int_t   ns,
                            rng_int_t   m,
                            rng_int_t  *state,
                            rng_int_t  *x,
                            rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandhypergeometric
 *
 */
void srandhypergeometric_ (  rng_int_t  *n,
                             rng_int_t  *np,
                             rng_int_t  *ns,
                             rng_int_t  *m,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandhypergeometric
 *
 */
void SRANDHYPERGEOMETRIC (  rng_int_t  *n,
                            rng_int_t  *np,
                            rng_int_t  *ns,
                            rng_int_t  *m,
                            rng_int_t  *state,
                            rng_int_t  *x,
                            rng_int_t  *info    );

/**
 * @brief Returns a vector of pseudo-random integers
 *        from negtive binomial distribution
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] m  Number of failures, **m >= 0**
 *
 * @param [in] p  Probability of success, **0 <= p < 1**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the negtive binomial distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandnegativebinomial (  rng_int_t   n,
                              rng_int_t   m,
                              double      p,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandnegativebinomial
 *
 */
void drandnegativebinomial_ (  rng_int_t  *n,
                               rng_int_t  *m,
                               double     *p,
                               rng_int_t  *state,
                               rng_int_t  *x,
                               rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandnegativebinomial
 *
 */
void DRANDNEGATIVEBINOMIAL (  rng_int_t  *n,
                              rng_int_t  *m,
                              double     *p,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandnegativebinomial
 *
 * @note
 * @ref srandnegativebinomial is the single precision version of @ref drandnegativebinomial. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandnegativebinomial are replaced by
 * **single precision** arguments in @ref srandnegativebinomial.
 */
void srandnegativebinomial (  rng_int_t   n,
                              rng_int_t   m,
                              float       p,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandnegativebinomial
 *
 */
void srandnegativebinomial_ (  rng_int_t  *n,
                               rng_int_t  *m,
                               float      *p,
                               rng_int_t  *state,
                               rng_int_t  *x,
                               rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandnegativebinomial
 *
 */
void SRANDNEGATIVEBINOMIAL (  rng_int_t  *n,
                              rng_int_t  *m,
                              float      *p,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );


/**
 * @brief Returns a vector of pseudo-random integers
 *        from poisson distribution
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] lambda  Distribution parameter, **lambda >= 0.0**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the poisson distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandpoisson (  rng_int_t   n,
                     double      lambda,
                     rng_int_t  *state,
                     rng_int_t  *x,
                     rng_int_t  *info     );

/**
 * @brief Fortran version of @ref drandpoisson
 *
 */
void drandpoisson_ (  rng_int_t  *n,
                      double     *lambda,
                      rng_int_t  *state,
                      rng_int_t  *x,
                      rng_int_t  *info     );

/**
 * @brief Fortran version of @ref drandpoisson
 *
 */
void DRANDPOISSON (  rng_int_t  *n,
                     double     *lambda,
                     rng_int_t  *state,
                     rng_int_t  *x,
                     rng_int_t  *info     );

/**
 * @brief Single precision of @ref drandpoisson
 *
 * @note
 * @ref srandpoisson is the single precision version of @ref drandpoisson. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandpoisson are replaced by
 * **single precision** arguments in @ref srandpoisson.
 */
void srandpoisson (  rng_int_t   n,
                     float       lambda,
                     rng_int_t  *state,
                     rng_int_t  *x,
                     rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandpoisson
 *
 */
void srandpoisson_ (  rng_int_t  *n,
                      float      *lambda,
                      rng_int_t  *state,
                      rng_int_t  *x,
                      rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandpoisson
 *
 */
void SRANDPOISSON (  rng_int_t  *n,
                     float      *lambda,
                     rng_int_t  *state,
                     rng_int_t  *x,
                     rng_int_t  *info     );


/**
 * @brief Returns a vector of pseudo-random integers
 *        uniformly distributed between **a** and **b**
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] a  Lower limit for distribution
 *
 * @param [in] b  Upper limit for distribution
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the uniform distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void dranddiscreteuniform (  rng_int_t   n,
                             rng_int_t   a,
                             rng_int_t   b,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref dranddiscreteuniform
 *
 */
void dranddiscreteuniform_ (  rng_int_t  *n,
                              rng_int_t  *a,
                              rng_int_t  *b,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref dranddiscreteuniform
 *
 */
void DRANDDISCRETEUNIFORM (  rng_int_t  *n,
                             rng_int_t  *a,
                             rng_int_t  *b,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );


/**
 * @brief Single precision of @ref dranddiscreteuniform
 *
 * @note
 * @ref sranddiscreteuniform is the single precision version of @ref dranddiscreteuniform. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref dranddiscreteuniform are replaced by
 * **single precision** arguments in @ref sranddiscreteuniform.
 */
void sranddiscreteuniform (  rng_int_t   n,
                             rng_int_t   a,
                             rng_int_t   b,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref sranddiscreteuniform
 *
 */
void sranddiscreteuniform_ (  rng_int_t  *n,
                              rng_int_t  *a,
                              rng_int_t  *b,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref sranddiscreteuniform
 *
 */
void SRANDDISCRETEUNIFORM (  rng_int_t  *n,
                             rng_int_t  *a,
                             rng_int_t  *b,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );


/**
 * @brief Returns a vector of pseudo-random integers
 *        from a general discrete distribution \n
 *        where the distribution is defined by the reference vector
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] ref  Reference vector as set up by one of the following APIs, \n
 *                  @ref drandbinomialreference,  \n
 *                  @ref drandgeometricreference, \n
 *                  @ref drandhypergeometricreference, \n
 *                  @ref drandnegativebinomialreference, \n
 *                  @ref drandpoissonreference. \n
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]    x  Vector of n- integers from the binomial distribution, **x(n)**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandgeneraldiscrete (  rng_int_t   n,
                             double     *ref,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgeneraldiscrete
 *
 */
void drandgeneraldiscrete_ (  rng_int_t  *n,
                              double     *ref,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandgeneraldiscrete
 *
 */
void DRANDGENERALDISCRETE (  rng_int_t  *n,
                             double     *ref,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandgeneraldiscrete
 *
 * @note
 * @ref srandgeneraldiscrete is the single precision version of @ref drandgeneraldiscrete. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandgeneraldiscrete are replaced by
 * **single precision** arguments in @ref srandgeneraldiscrete.
 */
void srandgeneraldiscrete (  rng_int_t   n,
                             float      *ref,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgeneraldiscrete
 *
 */
void srandgeneraldiscrete_ (  rng_int_t  *n,
                              float      *ref,
                              rng_int_t  *state,
                              rng_int_t  *x,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandgeneraldiscrete
 *
 */
void SRANDGENERALDISCRETE (  rng_int_t  *n,
                             float      *ref,
                             rng_int_t  *state,
                             rng_int_t  *x,
                             rng_int_t  *info    );


/**
 * @brief Setsup a double precision reference vector for generating
 *        a vector of random variates from a binomial distribution
 *        via calls to @ref drandgeneraldiscrete
 *
 * @param [in] m  Number of trials, **m >= 0**
 *
 * @param [in] p  Probability of success, **0 <= p < 1**
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref.
 *                                 **lref >= 30+20*SQRT(m*p*(1-p))** \n
 *                        On Exit: If **lref = -1** then lref is set to a recommended value
 *                                 and the function returns; otherwise lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandbinomialreference (  rng_int_t   m,
                               double      p,
                               double     *ref,
                               rng_int_t  *lref,
                               rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandbinomialreference
 *
 */
void drandbinomialreference_ (  rng_int_t  *m,
                                double     *p,
                                double     *ref,
                                rng_int_t  *lref,
                                rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandbinomialreference
 *
 */
void DRANDBINOMIALREFERENCE (  rng_int_t  *m,
                               double     *p,
                               double     *ref,
                               rng_int_t  *lref,
                               rng_int_t  *info   );

/**
 * @brief Single precision of @ref drandbinomialreference
 *
 * @note
 * @ref srandbinomialreference is the single precision version of @ref drandbinomialreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandbinomialreference are replaced by
 * **single precision** arguments in @ref srandbinomialreference.
 */
void srandbinomialreference (  rng_int_t   m,
                               float       p,
                               float      *ref,
                               rng_int_t  *lref,
                               rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandbinomialreference
 *
 */
void srandbinomialreference_ (  rng_int_t  *m,
                                float      *p,
                                float      *ref,
                                rng_int_t  *lref,
                                rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandbinomialreference
 *
 */
void SRANDBINOMIALREFERENCE (  rng_int_t  *m,
                               float      *p,
                               float      *ref,
                               rng_int_t  *lref,
                               rng_int_t  *info   );


/**
 * @brief Setsup a double precision reference vector for generating
 *        a vector of random variates from a geometric distribution
 *        via calls to @ref drandgeneraldiscrete
 *
 * @param [in] p  Distribution parameter, **EPS < p <= 1.0**
 *                where **EPS** is the machine precision \n
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref.
 *                                 **lref >= RNG_INT_T(30/p)+8** \n
 *                        On Exit: If **lref = -1** then lref is set to a
 *                        recommended value and the function returns;
 *                        otherwise lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandgeometricreference (  double      p,
                                double     *ref,
                                rng_int_t  *lref,
                                rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandgeometricreference
 *
 */
void drandgeometricreference_ (  double     *p,
                                 double     *ref,
                                 rng_int_t  *lref,
                                 rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandgeometricreference
 *
 */
void DRANDGEOMETRICREFERENCE (  double     *p,
                                double     *ref,
                                rng_int_t  *lref,
                                rng_int_t  *info   );

/**
 * @brief Single precision of @ref drandgeometricreference
 *
 * @note
 * @ref srandgeometricreference is the single precision version of @ref drandgeometricreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandgeometricreference are replaced by
 * **single precision** arguments in @ref srandgeometricreference.
 */
void srandgeometricreference (  float       p,
                                float      *ref,
                                rng_int_t  *lref,
                                rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandgeometricreference
 *
 */
void srandgeometricreference_ (  float      *p,
                                 float      *ref,
                                 rng_int_t  *lref,
                                 rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandgeometricreference
 *
 */
void SRANDGEOMETRICREFERENCE (  float      *p,
                                float      *ref,
                                rng_int_t  *lref,
                                rng_int_t  *info   );


/**
 * @brief Setsup a double precision reference vector for generating
 *        a vector of random variates from a hypergeometric distribution
 *        via calls to @ref drandgeneraldiscrete
 *
 * @param [in] np  Size of the population, **np >= 0**
 *
 * @param [in] ns  Size of the sample, **0 <= ns <= np**
 *
 * @param [in] m   Number of 'specified' items in the population,
 *                 **0 <= m <= np**
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref.
 *                                 **lref >= complicated formula** \n
 *                       On Exit: If **lref = -1** then lref is set to a
 *                       recommended value and the function returns;
 *                       otherwise, lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 */
void drandhypergeometricreference (  rng_int_t   np,
                                     rng_int_t   ns,
                                     rng_int_t   m,
                                     double     *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandhypergeometricreference
 *
 */
void drandhypergeometricreference_ (  rng_int_t  *np,
                                      rng_int_t  *ns,
                                      rng_int_t  *m,
                                      double     *ref,
                                      rng_int_t  *lref,
                                      rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandhypergeometricreference
 *
 */
void DRANDHYPERGEOMETRICREFERENCE (  rng_int_t  *np,
                                     rng_int_t  *ns,
                                     rng_int_t  *m,
                                     double     *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );

/**
 * @brief Single precision of @ref drandhypergeometricreference
 *
 * @note
 * @ref srandhypergeometricreference is the single precision version of
 * @ref drandhypergeometricreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandhypergeometricreference are replaced by
 * **single precision** arguments in @ref srandhypergeometricreference.
 */
void srandhypergeometricreference (  rng_int_t   np,
                                     rng_int_t   ns,
                                     rng_int_t   m,
                                     float      *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandhypergeometricreference
 *
 */
void srandhypergeometricreference_ (  rng_int_t  *np,
                                      rng_int_t  *ns,
                                      rng_int_t  *m,
                                      float      *ref,
                                      rng_int_t  *lref,
                                      rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandhypergeometricreference
 *
 */
void SRANDHYPERGEOMETRICREFERENCE (  rng_int_t  *np,
                                     rng_int_t  *ns,
                                     rng_int_t  *m,
                                     float      *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );


/**
 * @brief Setsup a double precision reference vector for generating
 *        a vector of random variates from a negative binomial distribution
 *        via calls to @ref drandgeneraldiscrete
 *
 * @param [in] m  Number of failures, **m >= 0**
 *
 * @param [in] p  Probability of success, **0 <= p < 1**
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref. \n
 *                                 lref >= RNG_INT_T((m*p+7.15*SQRT(m*p)+20.15*p)/(1-p)+8.5) \
 *                                 - max(0,RNG_INT_T((m*p-7.15*SQRT(m*p))/(1-p))) + 8 \n
 *                        On Exit: If **lref = -1** then lref is set to a
 *                                 recommended value and the function returns;
 *                                 otherwise lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -4 = Error in parameter, abs(info) \n
 */
void drandnegativebinomialreference (  rng_int_t   m,
                                       double      p,
                                       double     *ref,
                                       rng_int_t  *lref,
                                       rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandnegativebinomialreference
 *
 */
void drandnegativebinomialreference_ (  rng_int_t  *m,
                                        double      p,
                                        double     *ref,
                                        rng_int_t  *lref,
                                        rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandnegativebinomialreference
 *
 */
void DRANDNEGATIVEBINOMIALREFERENCE (  rng_int_t  *m,
                                       double     *p,
                                       double     *ref,
                                       rng_int_t  *lref,
                                       rng_int_t  *info   );

/**
 * @brief Single precision of @ref drandnegativebinomialreference
 *
 * @note
 * @ref srandnegativebinomialreference is the single precision version of
 * @ref drandnegativebinomialreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandnegativebinomialreference are replaced by
 * **single precision** arguments in @ref srandnegativebinomialreference.
 */
void srandnegativebinomialreference (  rng_int_t   m,
                                       float       p,
                                       float      *ref,
                                       rng_int_t  *lref,
                                       rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandnegativebinomialreference
 *
 */
void srandnegativebinomialreference_ (  rng_int_t  *m,
                                        float      *p,
                                        float      *ref,
                                        rng_int_t  *lref,
                                        rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandnegativebinomialreference
 *
 */
void SRANDNEGATIVEBINOMIALREFERENCE (  rng_int_t  *m,
                                       float      *p,
                                       float      *ref,
                                       rng_int_t  *lref,
                                       rng_int_t  *info   );


/**
 * @brief Setsup a double precision reference vector for generating
 *        a vector of random variates from a poisson distribution
 *        via calls to @ref drandgeneraldiscrete
 *
 * @param [in] lambda  Poisson parameter, **lambda >= 0.0**
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref. \n
 *                                 **lref >= 30 + 20*SQRT(lambda)** \n
 *                        On Exit: If **lref = -1** then lref is set to a
 *                                 recommended value and the function returns;
 *                                 otherwise lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info) \n
 */
void drandpoissonreference (  double      lambda,
                              double     *ref,
                              rng_int_t  *lref,
                              rng_int_t  *info     );

/**
 * @brief Fortran version of @ref drandpoissonreference
 *
 */
void drandpoissonreference_ (  double     *lambda,
                               double     *ref,
                               rng_int_t  *lref,
                               rng_int_t  *info     );

/**
 * @brief Fortran version of @ref drandpoissonreference
 *
 */
void DRANDPOISSONREFERENCE (  double     *lambda,
                              double     *ref,
                              rng_int_t  *lref,
                              rng_int_t  *info     );

/**
 * @brief Single precision of @ref drandpoissonreference
 *
 * @note
 * @ref srandpoissonreference is the single precision version of @ref drandpoissonreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandpoissonreference are replaced by
 * **single precision** arguments in @ref srandpoissonreference.
 */
void srandpoissonreference (  float       lambda,
                              float      *ref,
                              rng_int_t  *lref,
                              rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandpoissonreference
 *
 */
void srandpoissonreference_ (  float      *lambda,
                               float      *ref,
                               rng_int_t  *lref,
                               rng_int_t  *info     );

/**
 * @brief Fortran version of @ref srandpoissonreference
 *
 */
void SRANDPOISSONREFERENCE (  float      *lambda,
                              float      *ref,
                              rng_int_t  *lref,
                              rng_int_t  *info     );

/**
 * @}
 */


/**
 * @addtogroup contmul Continuous Multivariate Distributions
 *
 * @brief These distribution functions involve more than one variables which are
 *        continuous in nature. These are used to generate real numbers.
 *
 * @{
 */

/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from multivariate normal distribution
 *
 * @param [in] n  Number of values to generate, **n > 0**
 *
 * @param [in] m  Number of dimensions to the multivariate distribution, **m >= 1**
 *
 * @param [in] xmu  Means, **xmu(m)**
 *
 * @param [in] c  Covariance matrix, **c(ldc,m)**
 *
 * @param [in] ldc  Lead dimension of c in the calling routine, **ldc >= m**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out] x  Vector of n- values from a multivariate normal distribution, **x(ldx,m)**
 *
 * @param [in] ldx  Lead dimension of x in the calling routine, **ldx >= n**
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -8 = Error in parameter, abs(info) \n
 */
void drandmultinormal (  rng_int_t   n,
                         rng_int_t   m,
                         double     *xmu,
                         double     *c,
                         rng_int_t   ldc,
                         rng_int_t  *state,
                         double     *x,
                         rng_int_t   ldx,
                         rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandmultinormal
 *
 */
void drandmultinormal_ (  rng_int_t  *n,
                          rng_int_t  *m,
                          double     *xmu,
                          double     *c,
                          rng_int_t  *ldc,
                          rng_int_t  *state,
                          double     *x,
                          rng_int_t  *ldx,
                          rng_int_t  *info     );

/**
 * @brief Single precision of @ref drandmultinormal
 *
 */
void DRANDMULTINORMAL (  rng_int_t  *n,
                         rng_int_t  *m,
                         double     *xmu,
                         double     *c,
                         rng_int_t  *ldc,
                         rng_int_t  *state,
                         double     *x,
                         rng_int_t  *ldx,
                         rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandmultinormal
 *
 * @note
 * @ref srandmultinormal is the single precision version of @ref drandmultinormal. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultinormal are replaced by
 * **single precision** arguments in @ref srandmultinormal.
 */
void srandmultinormal (  rng_int_t   n,
                         rng_int_t   m,
                         float      *xmu,
                         float      *c,
                         rng_int_t   ldc,
                         rng_int_t  *state,
                         float      *x,
                         rng_int_t   ldx,
                         rng_int_t  *info    );

/**
 * @brief Single precision of @ref srandmultinormal
 *
 */
void srandmultinormal_ (  rng_int_t  *n,
                          rng_int_t  *m,
                          float      *xmu,
                          float      *c,
                          rng_int_t  *ldc,
                          rng_int_t  *state,
                          float      *x,
                          rng_int_t  *ldx,
                          rng_int_t  *info    );

/**
 * @brief Single precision of @ref srandmultinormal
 *
 */
void SRANDMULTINORMAL (  rng_int_t  *n,
                         rng_int_t  *m,
                         float      *xmu,
                         float      *c,
                         rng_int_t  *ldc,
                         rng_int_t  *state,
                         float      *x,
                         rng_int_t  *ldx,
                         rng_int_t  *info    );


/**
 * @brief Returns a vector of double precision pseudo-random numbers
 *        from multivariate students t distribution
 *
 * @param [in] n  Number of values to generate, **n > 0**
 *
 * @param [in] m  Number of dimensions to the multivariate distribution, **m >= 1**
 *
 * @param [in] df  Degrees of freedom, **df >= 2**
 *
 * @param [in] xmu  Means, **xmu(m)**
 *
 * @param [in] c  Covariance matrix, **c(ldc,m)**
 *
 * @param [in] ldc  Lead dimension of c in the calling routine, **ldc >= m**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out] x  Vector of n- values from a multivariate normal distribution, **x(ldx,m)**
 *
 * @param [in] ldx  Lead dimension of x in the calling routine, **ldx >= n**
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -8 = Error in parameter, abs(info) \n
 */
void drandmultistudentst (  rng_int_t   n,
                            rng_int_t   m,
                            rng_int_t   df,
                            double     *xmu,
                            double     *c,
                            rng_int_t   ldc,
                            rng_int_t  *state,
                            double     *x,
                            rng_int_t   ldx,
                            rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultistudentst
 *
 */
void drandmultistudentst_ (  rng_int_t  *n,
                             rng_int_t  *m,
                             rng_int_t  *df,
                             double     *xmu,
                             double     *c,
                             rng_int_t  *ldc,
                             rng_int_t  *state,
                             double     *x,
                             rng_int_t  *ldx,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultistudentst
 *
 */
void DRANDMULTISTUDENTST (  rng_int_t  *n,
                            rng_int_t  *m,
                            rng_int_t  *df,
                            double     *xmu,
                            double     *c,
                            rng_int_t  *ldc,
                            rng_int_t  *state,
                            double     *x,
                            rng_int_t  *ldx,
                            rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandmultistudentst
 *
 * @note
 * @ref srandmultistudentst is the single precision version of @ref drandmultistudentst. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultistudentst are replaced by
 * **single precision** arguments in @ref srandmultistudentst.
 */
void srandmultistudentst (  rng_int_t   n,
                            rng_int_t   m,
                            rng_int_t   df,
                            float      *xmu,
                            float      *c,
                            rng_int_t   ldc,
                            rng_int_t  *state,
                            float      *x,
                            rng_int_t   ldx,
                            rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultistudentst
 *
 */
void srandmultistudentst_ (  rng_int_t  *n,
                             rng_int_t  *m,
                             rng_int_t  *df,
                             float      *xmu,
                             float      *c,
                             rng_int_t  *ldc,
                             rng_int_t  *state,
                             float      *x,
                             rng_int_t  *ldx,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultistudentst
 *
 */
void SRANDMULTISTUDENTST (  rng_int_t  *n,
                            rng_int_t  *m,
                            rng_int_t  *df,
                            float      *xmu,
                            float      *c,
                            rng_int_t  *ldc,
                            rng_int_t  *state,
                            float      *x,
                            rng_int_t  *ldx,
                            rng_int_t  *info    );


/**
 * @brief Setups a reference vector for generating a vector of random variates
 *        from a multivariate normal distribution via calls @ref drandmultinormalr
 *
 * @param [in] m  Number of dimensions to the multivariate distribution, **m >= 1**
 *
 * @param [in] xmu  Means, **xmu(m)**
 *
 * @param [in] c  Covariance matrix, **c(ldc,m)**
 *
 * @param [in] ldc  Lead dimension of c in the calling routine
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref.
 *                                 **lref >= 6 + m*m+m** \n
 *                        On Exit: If **lref = -1** then lref is set to a
 *                                 recommended value and the function returns;
 *                                 otherwise lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -6 = Error in parameter, abs(info) \n
 */
void drandmultinormalreference (  rng_int_t   m,
                                  double     *xmu,
                                  double     *c,
                                  rng_int_t   ldc,
                                  double     *ref,
                                  rng_int_t  *lref,
                                  rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandmultinormalreference
 *
 */
void drandmultinormalreference_ (  rng_int_t  *m,
                                   double     *xmu,
                                   double     *c,
                                   rng_int_t  *ldc,
                                   double     *ref,
                                   rng_int_t  *lref,
                                   rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandmultinormalreference
 *
 */
void DRANDMULTINORMALREFERENCE (  rng_int_t  *m,
                                  double     *xmu,
                                  double     *c,
                                  rng_int_t  *ldc,
                                  double     *ref,
                                  rng_int_t  *lref,
                                  rng_int_t  *info   );

/**
 * @brief Single precision of @ref drandmultinormalreference
 *
 * @note
 * @ref srandmultinormalreference is the single precision version of
 * @ref drandmultinormalreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultinormalreference are replaced by
 * **single precision** arguments in @ref srandmultinormalreference.
 */
void srandmultinormalreference (  rng_int_t   m,
                                  float      *xmu,
                                  float      *c,
                                  rng_int_t   ldc,
                                  float      *ref,
                                  rng_int_t  *lref,
                                  rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandmultinormalreference
 *
 */
void srandmultinormalreference_ (  rng_int_t  *m,
                                   float      *xmu,
                                   float      *c,
                                   rng_int_t  *ldc,
                                   float      *ref,
                                   rng_int_t  *lref,
                                   rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandmultinormalreference
 *
 */
void SRANDMULTINORMALREFERENCE (  rng_int_t  *m,
                                  float      *xmu,
                                  float      *c,
                                  rng_int_t  *ldc,
                                  float      *ref,
                                  rng_int_t  *lref,
                                  rng_int_t  *info   );


/**
 * @brief Generates double precision values from multivariate normal distribution
 *        using a reference vector previously set-up via a call to
 *        @ref drandmultinormalreference
 *
 * @param [in] n  Number of values to generate, **n > 0**
 *
 * @param [in] ref  Reference vector, generated by a call to @ref drandmultinormalreference
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]  x  Vector of n- values from a multivariate normal distribution, **x(ldx,M)**
 *
 * @param [in] ldx  Lead dimension of x as defined in the calling routine, **ldx >= n**
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 */
void drandmultinormalr (  rng_int_t   n,
                          double     *ref,
                          rng_int_t  *state,
                          double     *x,
                          rng_int_t   ldx,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultinormalr
 *
 */
void drandmultinormalr_ (  rng_int_t  *n,
                           double     *ref,
                           rng_int_t  *state,
                           double     *x,
                           rng_int_t  *ldx,
                           rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultinormalr
 *
 */
void DRANDMULTINORMALR (  rng_int_t  *n,
                          double     *ref,
                          rng_int_t  *state,
                          double     *x,
                          rng_int_t  *ldx,
                          rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandmultinormalr
 *
 * @note
 * @ref srandmultinormalr is the single precision version of @ref drandmultinormalr. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultinormalr are replaced by
 * **single precision** arguments in @ref srandmultinormalr.
 */
void srandmultinormalr (  rng_int_t   n,
                          float      *ref,
                          rng_int_t  *state,
                          float      *x,
                          rng_int_t   ldx,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultinormalr
 *
 */
void srandmultinormalr_ (  rng_int_t  *n,
                           float      *ref,
                           rng_int_t  *state,
                           float      *x,
                           rng_int_t  *ldx,
                           rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultinormalr
 *
 */
void SRANDMULTINORMALR (  rng_int_t  *n,
                          float      *ref,
                          rng_int_t  *state,
                          float      *x,
                          rng_int_t  *ldx,
                          rng_int_t  *info    );


/**
 * @brief Setups a reference vector for generating a vector of random variates
 *        from a multivariate normal distribution via calls to @ref drandmultistudentstr
 *
 * @param [in] m  Number of dimensions to the multivariate distribution, **m >= 1**
 *
 * @param [in] df  Degrees of freedom, **df >= 2**
 *
 * @param [in] xmu  Means, **xmu(m)**
 *
 * @param [in] c  Covariance matrix, **c(ldc,m)**
 *
 * @param [in] ldc  Lead dimension of c in the calling routine
 *
 * @param [out] ref  Reference vector, **ref(lref)**
 *
 * @param [in,out] lref  On Entry: Either -1, or Length of ref.
 *                                 **lref >= 6 + m*m+m** \n
 *                        On Exit: If **lref = -1** then lref is set to a
 *                                 recommended value and the function returns;
 *                                 otherwise lref is not altered \n
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -7 = Error in parameter, abs(info) \n
 */
void drandmultistudentstreference (  rng_int_t   m,
                                     rng_int_t   df,
                                     double     *xmu,
                                     double     *c,
                                     rng_int_t   ldc,
                                     double     *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandmultistudentstreference
 *
 */
void drandmultistudentstreference_ (  rng_int_t  *m,
                                      rng_int_t  *df,
                                      double     *xmu,
                                      double     *c,
                                      rng_int_t  *ldc,
                                      double     *ref,
                                      rng_int_t  *lref,
                                      rng_int_t  *info   );

/**
 * @brief Fortran version of @ref drandmultistudentstreference
 *
 */
void DRANDMULTISTUDENTSTREFERENCE (  rng_int_t  *m,
                                     rng_int_t  *df,
                                     double     *xmu,
                                     double     *c,
                                     rng_int_t  *ldc,
                                     double     *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );

/**
 * @brief Single precision of @ref drandmultistudentstreference
 *
 * @note
 * @ref srandmultistudentstreference is the single precision version of
 * @ref drandmultistudentstreference. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultistudentstreference are replaced by
 * **single precision** arguments in @ref srandmultistudentstreference.
 */
void srandmultistudentstreference (  rng_int_t   m,
                                     rng_int_t   df,
                                     float      *xmu,
                                     float      *c,
                                     rng_int_t   ldc,
                                     float      *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandmultistudentstreference
 *
 */
void srandmultistudentstreference_ (  rng_int_t  *m,
                                      rng_int_t  *df,
                                      float      *xmu,
                                      float      *c,
                                      rng_int_t  *ldc,
                                      float      *ref,
                                      rng_int_t  *lref,
                                      rng_int_t  *info   );

/**
 * @brief Fortran version of @ref srandmultistudentstreference
 *
 */
void SRANDMULTISTUDENTSTREFERENCE (  rng_int_t  *m,
                                     rng_int_t  *df,
                                     float      *xmu,
                                     float      *c,
                                     rng_int_t  *ldc,
                                     float      *ref,
                                     rng_int_t  *lref,
                                     rng_int_t  *info   );


/**
 * @brief Generates double precision values from  multivariate students t distribution
 *        using a reference vector previously set-up via a call to
 *        @ref drandmultistudentstreference
 *
 * @param [in] n  Number of integers to generate, **n > 0**
 *
 * @param [in] ref  Reference vector, generated by a call to @ref drandmultistudentstreference
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out] x Vector of n- integers from a multivariate normal distribution, **x(ldx,M)**
 *
 * @param [in] ldx  Lead dimension of x as defined in the calling routine, **ldx >= n**
 *
 * @param [out] info  Error code \n
 *                    1 = Everything OK,
 *                        but reference vector not set up, due to lref being -1 on entry \n
 *                    0 = Everything OK \n
 *             -1 to -5 = Error in parameter, abs(info) \n
 */
void drandmultistudentstr (  rng_int_t   n,
                             double     *ref,
                             rng_int_t  *state,
                             double     *x,
                             rng_int_t   ldx,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultistudentstr
 *
 */
void drandmultistudentstr_ (  rng_int_t  *n,
                              double     *ref,
                              rng_int_t  *state,
                              double     *x,
                              rng_int_t  *ldx,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultistudentstr
 *
 */
void DRANDMULTISTUDENTSTR (  rng_int_t  *n,
                             double     *ref,
                             rng_int_t  *state,
                             double     *x,
                             rng_int_t  *ldx,
                             rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandmultistudentstr
 *
 * @note
 * @ref srandmultistudentstr is the single precision version of @ref drandmultistudentstr. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultistudentstr are replaced by
 * **single precision** arguments in @ref srandmultistudentstr.
 */
void srandmultistudentstr (  rng_int_t   n,
                             float      *ref,
                             rng_int_t  *state,
                             float      *x,
                             rng_int_t   ldx,
                             rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultistudentstr
 *
 */
void srandmultistudentstr_ (  rng_int_t  *n,
                              float      *ref,
                              rng_int_t  *state,
                              float      *x,
                              rng_int_t  *ldx,
                              rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultistudentstr
 *
 */
void SRANDMULTISTUDENTSTR (  rng_int_t  *n,
                             float      *ref,
                             rng_int_t  *state,
                             float      *x,
                             rng_int_t  *ldx,
                             rng_int_t  *info    );

/**
* @}
*/


/**
 * @addtogroup dismul Discrete Multivariate Distributions
 *
 * @brief These distribution functions involve more than one variables which are
 *        discrete in nature. These are used to generate integers.
 *
 * @{
 */

/**
 * @brief Returns an array of pseudo-random integers from
 *        a multinomial distribution
 *
 * @param [in] n Number of integers generated, **n > 0**
 *
 * @param [in] m  Number of trials, **m >= 0**
 *
 * @param [in] p  Vector of probabilities,
 *                where p(i) is the probability of observing outcome i.
 *                0 <= p(i) <= 1, for all i and p(1)+...+p(k) = 1,  p(k)
 *
 * @param [in] k  Number of possible outcomes, **k >= 2**
 *
 * @param [in,out] state  On Entry: Current state of the base generator being used.
 *                                  state must have first been initialized with a
 *                                  call to @ref drandinitialize \n
 *                         On Exit: State of generator after n- values are generated \n
 *
 * @param [out]  x  Array of n- values from the multinomial distribution, **x(ldx,k)**
 *
 * @param [in] ldx  Lead dimension of x in the calling routine, **ldx >= n**
 *
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -7 = Error in parameter, abs(info) \n
 */
void drandmultinomial (  rng_int_t   n,
                         rng_int_t   m,
                         double     *p,
                         rng_int_t   k,
                         rng_int_t  *state,
                         rng_int_t  *x,
                         rng_int_t   ldx,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultinomial
 *
 */
void drandmultinomial_ (  rng_int_t  *n,
                          rng_int_t  *m,
                          double     *p,
                          rng_int_t  *k,
                          rng_int_t  *state,
                          rng_int_t  *x,
                          rng_int_t  *ldx,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandmultinomial
 *
 */
void DRANDMULTINOMIAL (  rng_int_t  *n,
                         rng_int_t  *m,
                         double     *p,
                         rng_int_t  *k,
                         rng_int_t  *state,
                         rng_int_t  *x,
                         rng_int_t  *ldx,
                         rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandmultinomial
 *
 * @note
 * @ref srandmultinomial is the single precision version of @ref drandmultinomial. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandmultinomial are replaced by
 * **single precision** arguments in @ref srandmultinomial.
 */
void srandmultinomial (  rng_int_t   n,
                         rng_int_t   m,
                         float      *p,
                         rng_int_t   k,
                         rng_int_t  *state,
                         rng_int_t  *x,
                         rng_int_t   ldx,
                         rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultinomial
 *
 */
void srandmultinomial_ (  rng_int_t  *n,
                          rng_int_t  *m,
                          float      *p,
                          rng_int_t  *k,
                          rng_int_t  *state,
                          rng_int_t  *x,
                          rng_int_t  *ldx,
                          rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandmultinomial
 *
 */
void SRANDMULTINOMIAL (  rng_int_t  *n,
                         rng_int_t  *m,
                         float      *p,
                         rng_int_t  *k,
                         rng_int_t  *state,
                         rng_int_t  *x,
                         rng_int_t  *ldx,
                         rng_int_t  *info    );

/**
 * @}
 */

/**
 * @}
 */


/**
 * @addtogroup multi Multiple Streams
 *
 * @brief 'Skip-Ahead' and 'Leap Frog' are multiple stream generator.
 *
 * @{
 */

/**
 * @brief Advance the generator defined by state so that the next call generates
 *        a number n- places after the previous number.
 *        Allows multiple streams using the Skip Ahead method.
 *
 * @param [in] n  Number of places to skip ahead, **n >= 0**
 *
 * @param [in,out] state  On Entry: state array generated by one of the
 *                                  initialisation routines \n
 *                        On Exit : The entry stream, advanced n- places and
 *                                  set to return every Nth value \n
 *
 * @param [out] info Error code: \n
 *                0 = Everything OK \n
 *         -1 to -2 = Error in parameter, abs(info)
 *                    If the generator defined by state can not be multithreaded using
 *                    the skipahead method then routine returns -2 \n
 */
void drandskipahead (  rng_int_t   n,
                       rng_int_t  *state,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandskipahead
 *
 */
void drandskipahead_ (  rng_int_t  *n,
                        rng_int_t  *state,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandskipahead
 *
 */
void DRANDSKIPAHEAD (  rng_int_t  *n,
                       rng_int_t  *state,
                       rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandskipahead
 *
 * @note
 * @ref srandskipahead is the single precision version of @ref drandskipahead. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandskipahead are replaced by
 * **single precision** arguments in @ref srandskipahead.
 */
void srandskipahead (  rng_int_t   n,
                       rng_int_t  *state,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandskipahead
 *
 */
void srandskipahead_ (  rng_int_t  *n,
                        rng_int_t  *state,
                        rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandskipahead
 *
 */
void SRANDSKIPAHEAD (  rng_int_t  *n,
                       rng_int_t  *state,
                       rng_int_t  *info    );


/**
 * @brief  Adjust the generator defined by state so it can be used
 *         to generate the Kth stream out of n- multiple streams,
 *         using the Leap Frog method.
 *
 * @param [in] n  Total number of streams, **n > 0**
 *
 * @param [in] k  Number of current stream, **0 < k <= n**
 *
 * @param [in,out] state  On Entry: state array generated by one of
 *                                  the initialisation routines \n
 *                         On Exit: The entry stream, advanced k-1 places and
 *                                  set to return every Nth value \n
 * @param [out] info  Error code \n
 *                    0 = Everything OK \n
 *             -1 to -3 = Error in parameter, abs(info)
 *                        If the generator defined by state can not be
 *                        multithreaded using the leapfrog method then routine
 *                        returns with info = -3 \n
 */
void drandleapfrog (  rng_int_t   n,
                      rng_int_t   k,
                      rng_int_t  *state,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandleapfrog
 *
 */
void drandleapfrog_ (  rng_int_t  *n,
                       rng_int_t  *k,
                       rng_int_t  *state,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref drandleapfrog
 *
 */
void DRANDLEAPFROG (  rng_int_t  *n,
                      rng_int_t  *k,
                      rng_int_t  *state,
                      rng_int_t  *info    );

/**
 * @brief Single precision of @ref drandleapfrog
 *
 * @note
 * @ref srandleapfrog is the single precision version of @ref drandleapfrog. \n
 * The argument lists of both routines are identical ***except*** that any
 * **double precision** arguments of @ref drandleapfrog are replaced by
 * **single precision** arguments in @ref srandleapfrog.
 */
void srandleapfrog (  rng_int_t   n,
                      rng_int_t   k,
                      rng_int_t  *state,
                      rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandleapfrog
 *
 */
void srandleapfrog_ (  rng_int_t  *n,
                       rng_int_t  *k,
                       rng_int_t  *state,
                       rng_int_t  *info    );

/**
 * @brief Fortran version of @ref srandleapfrog
 *
 */
void SRANDLEAPFROG (  rng_int_t  *n,
                      rng_int_t  *k,
                      rng_int_t  *state,
                      rng_int_t  *info    );

/**
 * @}
 */


/**
 * @addtogroup version RNG version information
 *
 * @brief Retrieval of AOCL-RNG version string.
 *
 * @{
 */

/**
 * @brief Get AOCL-RNG version
 *
 * @return const char* version string
 */
const char* get_rngversion (void);

/**
 * @brief Get AOCL-RNG version
 *
 * @return const char* version string
 */
const char* get_rngversion_ (void);

/**
 * @brief Get AOCL-RNG version
 *
 * @return const char* version string
 */
const char* GET_RNGVERSION (void);

/**
 * @}
 */

/**
 * @}
 */

/* End of addtogroup rng */


#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif  /* __RNG_H__ */
