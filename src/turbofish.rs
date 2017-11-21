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
        let mut end_tail: usize = 0;
        let mut is_valid = false;
        if param_cow.starts_with(':') && param_cow.ends_with('>') {
            for (i, letter) in param_cow.char_indices() {
                if letter == '<' {
                    // Ugly flag for valid URL because of my code structure here
                    is_valid = true;
                    // Check if every character *before* the < is a colon
                    if param_cow[..i].chars().all(|character| character == ':') {
                        end_tail = i+1;
                        break;
                    } else {
                        return Err(param);
                    }
                }
            }
            if is_valid {
                // modify any <> within user input to contain a zero-width-space to prevent breakage
                return Ok(TurboFish(param_cow[end_tail..param_cow.len()-1].replace("<", "<​")));
            }
        }
        Err(param)
    }
}
