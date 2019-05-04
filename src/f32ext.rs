//! `f32` extension providing various arithmetic approximations and polyfills
//! for `std` functionality.

mod abs;
mod atan;
mod atan2;
mod ceil;
mod cos;
mod floor;
mod invsqrt;
mod sin;
mod sqrt;
mod tan;

/// `f32` extension providing various arithmetic approximations and polyfills
/// for `std` functionality.
pub trait F32Ext: Sized {
    /// Compute absolute value with a constant-time, data-independent
    /// implementation.
    fn abs(self) -> f32;

    /// Approximates `atan(x)` in radians with a maximum error of `0.002`.
    fn atan(self) -> f32;

    /// Approximates `atan(x)` normalized to the `[−1,1]` range with a maximum
    /// error of `0.1620` degrees.
    fn atan_norm(self) -> f32;

    /// Approximates the four quadrant arctangent `atan2(x)` in radians, with
    /// a maximum error of `0.002`.
    fn atan2(self, other: f32) -> f32;

    /// Approximates the four quadrant arctangent.
    /// Normalized to the `[0,4)` range with a maximum error of `0.1620` degrees.
    fn atan2_norm(self, other: f32) -> f32;

    /// Approximates floating point ceiling.
    fn ceil(self) -> f32;

    /// Approximates cosine in radians with a maximum error of `0.002`.
    fn cos(self) -> f32;

    /// Approximates floating point floor.
    fn floor(self) -> f32;

    /// Approximates inverse square root.
    fn invsqrt(self) -> f32;

    /// Approximates sine in radians with a maximum error of `0.002`.
    fn sin(self) -> f32;

    /// Approximates square root with an average deviation of ~5%.
    fn sqrt(self) -> f32;

    /// Approximates `tan(x)` in radians with a maximum error of `0.6`.
    fn tan(self) -> f32;
}

impl F32Ext for f32 {
    fn abs(self) -> f32 {
        self::abs::abs(self)
    }

    fn atan(self) -> f32 {
        self::atan::atan_approx(self)
    }

    fn atan_norm(self) -> f32 {
        self::atan::atan_norm_approx(self)
    }

    fn atan2(self, other: f32) -> f32 {
        self::atan2::atan2_approx(self, other)
    }

    fn atan2_norm(self, other: f32) -> f32 {
        self::atan2::atan2_norm_approx(self, other)
    }

    fn ceil(self) -> f32 {
        self::ceil::ceil(self)
    }

    fn cos(self) -> f32 {
        self::cos::cos_approx(self)
    }

    fn floor(self) -> f32 {
        self::floor::floor(self)
    }

    fn invsqrt(self) -> f32 {
        self::invsqrt::invsqrt_approx(self)
    }

    fn sin(self) -> f32 {
        self::sin::sin_approx(self)
    }

    fn sqrt(self) -> f32 {
        self::sqrt::sqrt_approx(self)
    }

    fn tan(self) -> f32 {
        self::tan::tan_approx(self)
    }
}
