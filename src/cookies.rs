use cookie::{Cookie, CookieJar, ParseError};

use crate::Context;
use std::sync::Arc;
use http::HeaderMap;

/// A representation of cookies which wraps `CookieJar` from `cookie` crate
///
/// Currently this only exposes getting cookie by name but future enhancements might allow more
/// operations
pub(crate) struct CookieData {
    pub(crate) content: Arc<CookieJar>,
}

/// An extension to `Context` that provides cached access to cookies
pub trait ExtractCookies {
    /// returns a `Cookie` by name of the cookie
    fn cookie(&mut self, name: &str) -> Option<Cookie<'static>>;
}

impl<AppData> ExtractCookies for Context<AppData> {
    fn cookie(&mut self, name: &str) -> Option<Cookie<'static>> {
        let cookie_data = self
            .extensions_mut()
            .remove()
            .unwrap_or_else(|| CookieData {
                content: Arc::new(self
                    .headers()
                    .get("Cookie")
                    .and_then(|raw| parse_from_header(raw.to_str().unwrap()).ok())
                    .unwrap_or_default()),
            });
        let cookie = cookie_data.content.get(name).cloned();
        self.extensions_mut().insert(cookie_data);

        cookie
    }
}

fn parse_from_header(s: &str) -> Result<CookieJar, ParseError> {
    let mut jar = CookieJar::new();

    s.split(';').try_for_each(|s| -> Result<_, ParseError> {
        jar.add(Cookie::parse(s.trim().to_owned())?);

        Ok(())
    })?;

    Ok(jar)
}

impl CookieData{
    pub fn from_headers(headers: &HeaderMap)->Self{
        CookieData {
            content: Arc::new(headers
                .get("Cookie")
                .and_then(|raw| parse_from_header(raw.to_str().unwrap()).ok())
                .unwrap_or_default()),
        }
    }
}