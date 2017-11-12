use rocket::request::FromParam;
use rocket::http::RawStr;

pub struct TurboFish(String);

impl TurboFish {
    pub fn gut(self) -> String {
        self.0
    }
}

impl<'a> FromParam<'a> for TurboFish {
    //TODO: fix bug that causes extra <> to appear
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let param_cow = param.percent_decode().map_err(|_| param)?;
        let mut param_content = String::new();
        if param_cow.starts_with(':') && param_cow.ends_with('>') {
            for (i, letter) in param_cow.char_indices() {
                if letter == ':' && param_cow.chars().nth(i+1) == Some('<') {
                    param_content = param_cow.split_at(i+1).1.to_string();
                    break
                
                } else if letter != ':' {
                    return Err(param);
                }
            }
            Ok(TurboFish(param_content.replace("<", "<​")))
        } else {
            Err(param)
        }
    }
}
