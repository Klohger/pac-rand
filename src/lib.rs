#![feature(const_trait_impl)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]
use std::{
    mem::{self, MaybeUninit},
    ops::{Add, Mul},
    simd::{LaneCount, Simd, SimdElement, SupportedLaneCount},
};

pub struct PacRand<T, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    seed: Simd<T, N>,
}

macro_rules! impl_pac_rand_num {
    ($($x:ident),* $(,)? ) => (
        $(
            impl PacRandNum for $x {
                const FIVE : Self = 5;
                const ONE : Self = 1;
            }
        )*
    )
}
impl_pac_rand_num! {
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
}

impl<T, const N: usize> PacRandNum for Simd<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    T: PacRandNum,
{
    const FIVE: Self = Simd::from_array([T::FIVE; N]);

    const ONE: Self = Simd::from_array([T::ONE; N]);
}

#[const_trait]
pub trait PacRandNum {
    const FIVE: Self;
    const ONE: Self;
}

impl<T, const N: usize> PacRand<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    [(); mem::size_of::<Simd<T, N>>()]:,
    Simd<T, N>: Mul<Output = Simd<T, N>> + Add<Output = Simd<T, N>> + PacRandNum,
{
    pub fn new() -> Self {
        let mut seed: MaybeUninit<Simd<T, N>> = MaybeUninit::uninit();

        let _ = getrandom::getrandom_uninit(unsafe {
            mem::transmute::<
                &mut MaybeUninit<Simd<T, N>>,
                &mut [MaybeUninit<u8>; mem::size_of::<Simd<T, N>>()],
            >(&mut seed)
        });

        let seed = unsafe { seed.assume_init() };
        Self { seed }
    }
    pub const fn from_seed(seed: Simd<T, N>) -> Self {
        Self { seed }
    }
    pub const fn splat(t: T) -> Self {
        Self {
            seed: Simd::from_array([t; N]),
        }
    }
    #[inline]
    pub const fn state(&self) -> &Simd<T, N> {
        &self.seed
    }
    pub fn update(&mut self) -> &Simd<T, N> {
        self.seed = (self.seed * Simd::FIVE) + Simd::ONE;
        &self.seed
    }
}
