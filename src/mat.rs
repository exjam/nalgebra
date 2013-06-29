use std::uint::iterate;
use std::num::{One, Zero};
use std::vec::swap;
use std::cmp::ApproxEq;
use std::rand::{Rand, Rng, RngUtil};
use std::iterator::IteratorUtil;
use vec::{Vec1, Vec2, Vec3, Vec4, Vec5, Vec6};
use traits::dim::Dim;
use traits::ring::Ring;
use traits::inv::Inv;
use traits::division_ring::DivisionRing;
use traits::transpose::Transpose;
use traits::rlmul::{RMul, LMul};
use traits::transformation::Transform;

mod mat_impl;

#[deriving(ToStr)]
pub struct Mat1<N>
{ mij: [N, ..1 * 1] }

mat_impl!(Mat1, 1)
one_impl!(Mat1, [ _1 ])
zero_impl!(Mat1, [ _0  ])
dim_impl!(Mat1, 1)
mat_indexing_impl!(Mat1, 1)
mul_impl!(Mat1, 1)
rmul_impl!(Mat1, Vec1, 1)
lmul_impl!(Mat1, Vec1, 1)
transform_impl!(Mat1, Vec1)
// inv_impl!(Mat1, 1)
transpose_impl!(Mat1, 1)
approx_eq_impl!(Mat1)
rand_impl!(Mat1, rng, [ rng ])

#[deriving(ToStr)]
pub struct Mat2<N>
{ mij: [N, ..2 * 2] }

mat_impl!(Mat2, 2)
one_impl!(Mat2, [ _1 | _0 |
                      _0 | _1 ])
zero_impl!(Mat2, [ _0 | _0 |
                       _0 | _0 ])
dim_impl!(Mat2, 2)
mat_indexing_impl!(Mat2, 2)
mul_impl!(Mat2, 2)
rmul_impl!(Mat2, Vec2, 2)
lmul_impl!(Mat2, Vec2, 2)
transform_impl!(Mat2, Vec2)
// inv_impl!(Mat2, 2)
transpose_impl!(Mat2, 2)
approx_eq_impl!(Mat2)
rand_impl!(Mat2, rng, [ rng | rng |
                            rng | rng ])

#[deriving(ToStr)]
pub struct Mat3<N>
{ mij: [N, ..3 * 3] }

mat_impl!(Mat3, 3)
one_impl!(Mat3, [ _1 | _0 | _0 |
                      _0 | _1 | _0 |
                      _0 | _0 | _1 ])
zero_impl!(Mat3, [ _0 | _0 | _0 |
                       _0 | _0 | _0 |
                       _0 | _0 | _0 ])
dim_impl!(Mat3, 3)
mat_indexing_impl!(Mat3, 3)
mul_impl!(Mat3, 3)
rmul_impl!(Mat3, Vec3, 3)
lmul_impl!(Mat3, Vec3, 3)
transform_impl!(Mat3, Vec3)
// inv_impl!(Mat3, 3)
transpose_impl!(Mat3, 3)
approx_eq_impl!(Mat3)
rand_impl!(Mat3, rng, [ rng | rng | rng |
                            rng | rng | rng |
                            rng | rng | rng])

#[deriving(ToStr)]
pub struct Mat4<N>
{ mij: [N, ..4 * 4] }

mat_impl!(Mat4, 4)
one_impl!(Mat4, [
          _1 | _0 | _0 | _0 |
          _0 | _1 | _0 | _0 |
          _0 | _0 | _1 | _0 |
          _0 | _0 | _0 | _1
          ])
zero_impl!(Mat4, [
          _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0
          ])
dim_impl!(Mat4, 4)
mat_indexing_impl!(Mat4, 4)
mul_impl!(Mat4, 4)
rmul_impl!(Mat4, Vec4, 4)
lmul_impl!(Mat4, Vec4, 4)
transform_impl!(Mat4, Vec4)
inv_impl!(Mat4, 4)
transpose_impl!(Mat4, 4)
approx_eq_impl!(Mat4)
rand_impl!(Mat4, rng, [
           rng | rng | rng | rng |
           rng | rng | rng | rng |
           rng | rng | rng | rng |
           rng | rng | rng | rng
           ])

#[deriving(ToStr)]
pub struct Mat5<N>
{ mij: [N, ..5 * 5] }

mat_impl!(Mat5, 5)
one_impl!(Mat5, [
          _1 | _0 | _0 | _0 | _0 |
          _0 | _1 | _0 | _0 | _0 |
          _0 | _0 | _1 | _0 | _0 |
          _0 | _0 | _0 | _1 | _0 |
          _0 | _0 | _0 | _0 | _1
          ])
zero_impl!(Mat5, [
          _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0
          ])
dim_impl!(Mat5, 5)
mat_indexing_impl!(Mat5, 5)
mul_impl!(Mat5, 5)
rmul_impl!(Mat5, Vec5, 5)
lmul_impl!(Mat5, Vec5, 5)
transform_impl!(Mat5, Vec5)
inv_impl!(Mat5, 5)
transpose_impl!(Mat5, 5)
approx_eq_impl!(Mat5)
rand_impl!(Mat5, rng, [
           rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng
           ])

#[deriving(ToStr)]
pub struct Mat6<N>
{ mij: [N, ..6 * 6] }

mat_impl!(Mat6, 6)
one_impl!(Mat6, [
          _1 | _0 | _0 | _0 | _0 | _0 |
          _0 | _1 | _0 | _0 | _0 | _0 |
          _0 | _0 | _1 | _0 | _0 | _0 |
          _0 | _0 | _0 | _1 | _0 | _0 |
          _0 | _0 | _0 | _0 | _1 | _0 |
          _0 | _0 | _0 | _0 | _0 | _1
          ])
zero_impl!(Mat6, [
          _0 | _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 | _0 |
          _0 | _0 | _0 | _0 | _0 | _0
          ])
dim_impl!(Mat6, 6)
mat_indexing_impl!(Mat6, 6)
mul_impl!(Mat6, 6)
rmul_impl!(Mat6, Vec6, 6)
lmul_impl!(Mat6, Vec6, 6)
transform_impl!(Mat6, Vec6)
inv_impl!(Mat6, 6)
transpose_impl!(Mat6, 6)
approx_eq_impl!(Mat6)
rand_impl!(Mat6, rng, [
           rng | rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng | rng |
           rng | rng | rng | rng | rng | rng
           ])
