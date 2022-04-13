/*! \file */
/*******************************************
 *                                         *
 *  File auto-generated by `::safer_ffi`.  *
 *                                         *
 *  Do not manually edit this file.        *
 *                                         *
 *******************************************/

#ifndef __RUST_GRANNE_C__
#define __RUST_GRANNE_C__

#ifdef __cplusplus
extern "C" {
#endif


#include <stddef.h>
#include <stdint.h>

void granne_new_index (
    int8_t const * name);

size_t granne_add (
    int8_t const * name,
    float const * features,
    size_t dimension);

void granne_build (
    int8_t const * name);

/** \brief
 *  Same as [`Vec<T>`][`rust::Vec`], but with guaranteed `#[repr(C)]` layout
 */
typedef struct Vec_size {

    size_t * ptr;

    size_t len;

    size_t cap;

} Vec_size_t;

Vec_size_t granne_search (
    int8_t const * name,
    size_t k,
    float const * features,
    size_t dimension);

void granne_save (
    int8_t const * name,
    int8_t const * _index_filename,
    int8_t const * _elements_filename);

void granne_load (
    int8_t const * name,
    int8_t const * _index_filename,
    int8_t const * _elements_filename);


#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __RUST_GRANNE_C__ */
