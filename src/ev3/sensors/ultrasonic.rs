use std::str::FromStr;

use crate::device::{ReadOnlyAttributeFile, ReadWriteAttributeFile};

#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "lego-sensor", driver = "lego-ev3-us")]
pub struct UltrasonicSensor {
    mode: ReadWriteAttributeFile,
    #[device(attr_name = "value0")]
    value: ReadOnlyAttributeFile,
}

impl UltrasonicSensor {
    pub fn measure_cm(&mut self) -> anyhow::Result<CmMeter> {
        self.mode.set_value("US-DIST-CM")?;
        Ok(CmMeter { ultrasonic_sensor: self })
    }

    pub fn measure_inches(&mut self) -> anyhow::Result<InchMeter> {
        self.mode.set_value("US-DIST-IN")?;
        Ok(InchMeter { ultrasonic_sensor: self })
    }

    pub fn listen(&mut self) -> anyhow::Result<UltrasoundListener> {
        self.mode.set_value("US-LISTEN")?;
        Ok(UltrasoundListener { ultrasonic_sensor: self })
    }

    fn value<T: FromStr>(&mut self) -> anyhow::Result<T>
    where
        T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self.value.value().map_err(Into::into)
    }
}

macro_rules! ultrasonic_sensor_mode {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            ultrasonic_sensor: &'a mut UltrasonicSensor,
        }
    };
}

ultrasonic_sensor_mode!(CmMeter);
ultrasonic_sensor_mode!(InchMeter);
ultrasonic_sensor_mode!(UltrasoundListener);

impl CmMeter<'_> {
    pub fn cm(&mut self) -> anyhow::Result<f32> {
        self.ultrasonic_sensor.value().map(div_by_10)
    }
}

impl InchMeter<'_> {
    pub fn inches(&mut self) -> anyhow::Result<f32> {
        self.ultrasonic_sensor.value().map(div_by_10)
    }
}

fn div_by_10(n: u32) -> f32 {
    n as f32 / 10_f32
}

impl UltrasoundListener<'_> {
    pub fn is_ultrasound_present(&mut self) -> anyhow::Result<bool> {
        self.ultrasonic_sensor.value().map(|present: char| present == '1')
    }
}
