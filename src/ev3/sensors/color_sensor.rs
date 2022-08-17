use std::str::FromStr;

use crate::device::{ReadOnlyAttributeFile, ReadWriteAttributeFile};
use crate::percentage::Percentage;

#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "lego-sensor", driver = "lego-ev3-color")]
pub struct ColorSensor {
    mode: ReadWriteAttributeFile,
    #[device(attr_name = "value0")]
    value: ReadOnlyAttributeFile,
}

impl ColorSensor {
    pub fn measure_reflected_light(
        &mut self,
    ) -> anyhow::Result<ReflectedLightMeter> {
        self.mode.set_value("COL-REFLECT")?;
        Ok(ReflectedLightMeter { color_sensor: self })
    }

    pub fn measure_ambient_light(
        &mut self,
    ) -> anyhow::Result<AmbientLightMeter> {
        self.mode.set_value("COL-AMBIENT")?;
        Ok(AmbientLightMeter { color_sensor: self })
    }

    fn value<T: FromStr>(&mut self) -> anyhow::Result<T>
    where
        T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self.value.value().map_err(Into::into)
    }
}

macro_rules! color_sensor_mode {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            color_sensor: &'a mut ColorSensor,
        }
    };
}

color_sensor_mode!(ReflectedLightMeter);
color_sensor_mode!(AmbientLightMeter);

impl ReflectedLightMeter<'_> {
    pub fn reflected_light_intensity(&mut self) -> anyhow::Result<Percentage> {
        self.color_sensor.value()
    }
}

impl AmbientLightMeter<'_> {
    pub fn ambient_light_intensity(&mut self) -> anyhow::Result<Percentage> {
        self.color_sensor.value()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    None,
    Black,
    Blue,
    Green,
    Yellow,
    Red,
    White,
    Brown,
}
