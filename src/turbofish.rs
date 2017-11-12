use rocket::request::FromParam;
use rocket::http::RawStr;

pub struct TurboFish(String);

impl TurboFish {
    pub fn gut(self) -> String {
        self.0
    }
}

impl<'a> FromParam<'a> for TurboFish {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let param_cow = param.percent_decode().map_err(|_| param)?;
        
        if param_cow.starts_with(':') && param_cow.contains(":<") && param_cow.ends_with('>') {
            Ok(TurboFish(param_cow.replacen("<", "<​", 1)))
        } else {
            Err(param)
        }
    }
}
