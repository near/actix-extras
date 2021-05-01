use std::cell::Ref;

use actix_web::{
    cookie::{Cookie, ParseError as CookieParseError},
    dev::ServiceRequest,
    http::header,
    HttpMessage,
};

struct Cookies(Vec<Cookie<'static>>);

pub trait Cookied: HttpMessage {
    fn cookies(&self) -> Result<Ref<'_, Vec<Cookie<'static>>>, CookieParseError> {
        if self.extensions().get::<Cookies>().is_none() {
            let mut cookies = Vec::new();
            for hdr in self.headers().get_all(header::COOKIE) {
                let s = std::str::from_utf8(hdr.as_bytes())
                    .map_err(CookieParseError::from)?;
                for cookie_str in s.split(';').map(|s| s.trim()) {
                    if !cookie_str.is_empty() {
                        cookies.push(Cookie::parse_encoded(cookie_str)?.into_owned());
                    }
                }
            }
            self.extensions_mut().insert(Cookies(cookies));
        }

        Ok(Ref::map(self.extensions(), |ext| {
            &ext.get::<Cookies>().unwrap().0
        }))
    }

    /// Return request cookie.
    fn cookie(&self, name: &str) -> Option<Cookie<'static>> {
        if let Ok(cookies) = self.cookies() {
            for cookie in cookies.iter() {
                if cookie.name() == name {
                    return Some(cookie.to_owned());
                }
            }
        }
        None
    }
}

impl Cookied for ServiceRequest {}
