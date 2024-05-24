use rocket::Response;

#[allow(unused)]
pub enum HxHeader {
    Redirect(String),
    Retarget(String),
    Reswap(String),
}

impl HxHeader {
    pub fn _redirect(to: &str) -> Self {
        HxHeader::Redirect(to.to_string())
    }
}

impl Into<(String, String)> for HxHeader {
    fn into(self) -> (String, String) {
        match self {
            HxHeader::Redirect(to) => ("HX-Redirect".to_string(), to),
            HxHeader::Retarget(target) => ("HX-Retarget".to_string(), target),
            HxHeader::Reswap(swap) => ("HX-Reswap".to_string(), swap),
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for HxHeader {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut builder = Response::build();

        builder.ok()
    }
}

impl<'a> Into<rocket::http::Header<'a>> for HxHeader {
    fn into(self) -> rocket::http::Header<'a> {
        let h: (String, String) = self.into();
        rocket::http::Header::new(h.0, h.1)
    }
}
