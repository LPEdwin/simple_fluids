use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::cell::UnsafeCell;

thread_local! {
    static RNG: UnsafeCell<Pcg64Mcg> = UnsafeCell::new(Pcg64Mcg::seed_from_u64(12345));
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

macro_rules! impl_op_assign {
    ($trait:ident, $func:ident, $op:tt) => {
        impl std::ops::$trait for Vector2 {
            #[inline]
            fn $func(&mut self, other: Self) {
                self.x $op other.x;
                self.y $op other.y;
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);
impl_op_assign!(DivAssign, div_assign, /=);

macro_rules! impl_op {
    ($trait:ident, $func:ident, $op:tt) => {
        impl std::ops::$trait for Vector2 {
            type Output = Self;
            #[inline]
            fn $func(self, other: Self) -> Self {
                Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                }
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

macro_rules! impl_op_scalar {
    ($trait:ident, $func:ident, $op:tt) => {
        impl std::ops::$trait<f64> for Vector2 {
            type Output = Vector2;
            #[inline]
            fn $func(self, rhs: f64) -> Vector2 {
                Vector2 { x: self.x $op rhs, y: self.y $op rhs }
            }
        }

        impl std::ops::$trait<Vector2> for f64 {
            type Output = Vector2;
            #[inline]
            fn $func(self, rhs: Vector2) -> Vector2 {
                Vector2 { x: self $op rhs.x, y: self $op rhs.y }
            }
        }
    };
}

impl_op_scalar!(Add, add, +);
impl_op_scalar!(Sub, sub, -);
impl_op_scalar!(Mul, mul, *);

impl std::ops::Div<f64> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: f64) -> Vector2 {
        (1.0 / rhs)
            * Vector2 {
                x: self.x,
                y: self.y,
            }
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    pub const ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        dot(self, self)
    }

    pub fn normalized(self) -> Vector2 {
        self / self.length()
    }

    pub fn random_in_square(range: std::ops::Range<f64>) -> Vector2 {
        RNG.with(|cell| {
            let rng = unsafe { &mut *cell.get() };
            Vector2 {
                x: rng.random_range(range.clone()),
                y: rng.random_range(range),
            }
        })
    }

    pub fn random_in_rectangle(
        x_range: std::ops::Range<f64>,
        y_range: std::ops::Range<f64>,
    ) -> Vector2 {
        RNG.with(|cell| {
            let rng = unsafe { &mut *cell.get() };
            Vector2 {
                x: rng.random_range(x_range),
                y: rng.random_range(y_range),
            }
        })
    }

    pub fn random_min_max(min: Vector2, max: Vector2) -> Vector2 {
        RNG.with(|cell| {
            let rng = unsafe { &mut *cell.get() };
            Vector2 {
                x: rng.random_range(min.x..max.x),
                y: rng.random_range(min.y..max.y),
            }
        })
    }

    pub fn random_in_disk() -> Vector2 {
        RNG.with(|cell| {
            loop {
                let rng = unsafe { &mut *cell.get() };
                let v = Vector2 {
                    x: rng.random_range(-1.0..1.0),
                    y: rng.random_range(-1.0..1.0),
                };
                if v.length_squared() < 1.0 {
                    return v;
                }
            }
        })
    }

    pub fn random_gaussian(mean: f64, std_dev: f64) -> Vector2 {
        use rand_distr::Normal;

        RNG.with(|cell| {
            let rng = unsafe { &mut *cell.get() };
            let normal = Normal::new(mean, std_dev).unwrap();
            let x: f64 = normal.sample(rng);
            let y: f64 = normal.sample(rng);

            Vector2 { x, y }
        })
    }

    pub fn reflect(self, n: Vector2) -> Vector2 {
        return self - 2.0 * dot(self, n) * n;
    }

    pub fn lerp(origin: Vector2, target: Vector2, t: f64) -> Vector2 {
        Vector2 {
            x: origin.x + (target.x - origin.x) * t,
            y: origin.y + (target.y - origin.y) * t,
        }
    }
}

#[inline]
pub fn dot(v1: Vector2, v2: Vector2) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}
