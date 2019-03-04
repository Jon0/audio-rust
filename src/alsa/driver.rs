use alsa::mixer::*;
use format::error::*;
use player::player::*;


struct AlsaDriver {
    device: Device
}


impl AlsaDriver {

    pub fn open() -> Result<AlsaDriver, DriverError> {
        match Device::open("hw:0,0") {
            Ok(mut dev) => Ok(AlsaDriver{ device: dev }),
            Err(err) => Err(err),
        }
    }
}
