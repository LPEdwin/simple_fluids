use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::cell::UnsafeCell;

thread_local! {
    static RNG: UnsafeCell<Pcg64Mcg> = UnsafeCell::new(Pcg64Mcg::from_rng(&mut rand::rng()));
}

#[derive(Debug, Clone, Copy)]
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

impl std::ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

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

impl Vector2 {
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        dot(&self, &self)
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

    pub fn lerp(origin: Vector2, target: Vector2, t: f64) -> Vector2 {
        Vector2 {
            x: origin.x + (target.x - origin.x) * t,
            y: origin.y + (target.y - origin.y) * t,
        }
    }
}

#[inline]
fn dot(v1: &Vector2, v2: &Vector2) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}
