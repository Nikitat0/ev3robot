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
        Ok(CmMeter(self))
    }

    pub fn measure_inches(&mut self) -> anyhow::Result<InchMeter> {
        self.mode.set_value("US-DIST-IN")?;
        Ok(InchMeter(self))
    }

    pub fn listen(&mut self) -> anyhow::Result<UltrasoundListener> {
        self.mode.set_value("US-LISTEN")?;
        Ok(UltrasoundListener(self))
    }

    fn value<T: FromStr>(&mut self) -> anyhow::Result<T>
    where
        T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self.value.value().map_err(Into::into)
    }
}

pub struct CmMeter<'a>(&'a mut UltrasonicSensor);

impl CmMeter<'_> {
    pub fn cm(&mut self) -> anyhow::Result<f32> {
        self.0.value().map(div_by_10)
    }
}

pub struct InchMeter<'a>(&'a mut UltrasonicSensor);

impl InchMeter<'_> {
    pub fn inches(&mut self) -> anyhow::Result<f32> {
        self.0.value().map(div_by_10)
    }
}

fn div_by_10(n: u32) -> f32 {
    n as f32 / 10_f32
}

pub struct UltrasoundListener<'a>(&'a mut UltrasonicSensor);

impl UltrasoundListener<'_> {
    pub fn is_ultrasound_present(&mut self) -> anyhow::Result<bool> {
        self.0.value().map(|present: char| present == '1')
    }
}
