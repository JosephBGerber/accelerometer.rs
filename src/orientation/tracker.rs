//! Orientation tracker

use crate::{
    orientation::Orientation,
    vector::{Component, Vector, VectorExt},
};
use micromath::generic_array::typenum::U3;

// Used for intra-doc-link resolution only
#[allow(unused_imports)]
use crate::accelerometer::Accelerometer;

// Spuriously triggers unused import warnings in cases std is linked
#[allow(unused_imports)]
use micromath::F32Ext;

/// Orientation tracker: computes a device's [`Orientation`] from accelerometer
/// readings.
pub struct Tracker {
    /// Threshold at which acceleration due to gravity is registered
    threshold: f32,

    /// Last orientation type read from the accelerometer
    last_orientation: Orientation,
}

impl Tracker {
    /// Create a new orientation tracker.
    ///
    /// The `threshold` value should be slightly less than the absolute value
    /// of the reading you get from the accelerometer when the device is lying
    /// in a position where two of the axes are reading 0 (i.e. getting a
    /// strong reading from one axis alone). It may require some
    /// experimentation to properly tune this threshold.
    ///
    /// For best results, set the accelerometer's sensitivity higher than ±2G,
    /// e.g. ±4G or ±8G. This will help reduce noise in the accelerometer data.
    pub fn new(threshold: impl Into<f32>) -> Self {
        Self {
            threshold: threshold.into(),
            last_orientation: Orientation::Unknown,
        }
    }

    /// Update the tracker's internal state from the given acceleration vector
    /// (i.e. obtained from [`Accelerometer::acceleration`]), returning a new
    /// computed orientation value.
    pub fn update<V, C>(&mut self, acceleration: V) -> Orientation
    where
        V: Vector<Axes = U3, Component = C> + VectorExt,
        C: Component + Into<f32>,
    {
        let components = acceleration.to_array();
        let x: f32 = components[0].into();
        let y: f32 = components[1].into();
        let z: f32 = components[2].into();

        let result = if x.abs() > self.threshold {
            // Landscape
            if x >= 0.0 {
                Orientation::LandscapeUp
            } else {
                Orientation::LandscapeDown
            }
        } else if y.abs() > self.threshold {
            // Portrait
            if y >= 0.0 {
                Orientation::PortraitUp
            } else {
                Orientation::PortraitDown
            }
        } else if z.abs() > self.threshold {
            // Flat
            if z >= 0.0 {
                Orientation::FaceUp
            } else {
                Orientation::FaceDown
            }
        } else {
            Orientation::Unknown
        };

        if result != Orientation::Unknown {
            self.last_orientation = result;
        }

        result
    }

    /// Get the last known orientation reading for the device.
    ///
    /// Use [`Tracker::update`] to obtain a new reading.
    pub fn orientation(&self) -> Orientation {
        self.last_orientation
    }
}
