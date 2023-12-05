use serde::{Deserialize, Serialize};

use super::{abs, copysign, macc};
use core::iter::Sum;
use num_traits::{clamp, Float, One, Zero};

pub type Vec3<T> = [T; 3];

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct PID<T> {
    pub p: T,
    pub i: T,
    pub d: T,
    pub x_offset: T,
    pub y_offset: T,
    pub y_min: T,
    pub y_max: T,
    pub i_limit: T,
}

impl<T: Float + Zero + One + Sum<T>> PID<T> {
    pub fn new(y_min: T, y_max: T, i_limit: T) -> Self {
        Self {
            p: T::one(),
            i: T::zero(),
            d: T::zero(),
            x_offset: T::zero(),
            y_offset: T::zero(),
            y_min,
            y_max,
            i_limit,
        }
    }

    /// # Arguments
    /// * `state` - Current filter state. [last x, last y, integral sum]
    /// * `x_raw` - New input.
    pub fn update(&self, state: &mut Vec3<T>, x_raw: T, hold: bool) -> T {
        let x = x_raw - self.x_offset;
        let diff = x - state[0];
        state[0] = x;
        let y = if hold {
            state[1]
        } else {
            let y = self.p * x + state[2] + self.d * diff;
            state[2] = clamp(state[2] + self.i * x, -self.i_limit, self.i_limit);
            y
        };
        let y = clamp(y, self.y_min - self.y_offset, self.y_max - self.y_offset);
        state[1] = y;
        y + self.y_offset
    }
}
