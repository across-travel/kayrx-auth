use std::fmt;

use kayrx::web::error::ParseError;
use kayrx::http::header::{
    Header, HeaderName, HeaderValue, IntoHeaderValue, AUTHORIZATION,
};
use kayrx::web::HttpMessage;

use crate::headers::authorization::scheme::Scheme;

/// `Authorization` header, defined in [RFC 7235](https://tools.ietf.org/html/rfc7235#section-4.2)
///
/// The "Authorization" header field allows a user agent to authenticate
/// itself with an origin server -- usually, but not necessarily, after
/// receiving a 401 (Unauthorized) response.  Its value consists of
/// credentials containing the authentication information of the user
/// agent for the realm of the resource being requested.
///
/// `Authorization` header is generic over [authentication
/// scheme](./trait.Scheme.html).
///
/// # Example
///
/// ```
/// # use kayrx::http::header::Header;
/// # use kayrx::web::{HttpRequest, Result};
/// # use keclc_httpauth::headers::authorization::{Authorization, Basic};
/// fn handler(req: HttpRequest) -> Result<String> {
///     let auth = Authorization::<Basic>::parse(&req)?;
///
///     Ok(format!("Hello, {}!", auth.as_ref().user_id()))
/// }
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Clone)]
pub struct Authorization<S: Scheme>(S);

impl<S> Authorization<S>
where
    S: Scheme,
{
    /// Consumes `Authorization` header and returns inner [`Scheme`]
    /// implementation.
    ///
    /// [`Scheme`]: ./trait.Scheme.html
    pub fn into_scheme(self) -> S {
        self.0
    }
}

impl<S> From<S> for Authorization<S>
where
    S: Scheme,
{
    fn from(scheme: S) -> Authorization<S> {
        Authorization(scheme)
    }
}

impl<S> AsRef<S> for Authorization<S>
where
    S: Scheme,
{
    fn as_ref(&self) -> &S {
        &self.0
    }
}

impl<S> AsMut<S> for Authorization<S>
where
    S: Scheme,
{
    fn as_mut(&mut self) -> &mut S {
        &mut self.0
    }
}

impl<S: Scheme> Header for Authorization<S> {
    #[inline]
    fn name() -> HeaderName {
        AUTHORIZATION
    }

    fn parse<T: HttpMessage>(msg: &T) -> Result<Self, ParseError> {
        let header =
            msg.headers().get(AUTHORIZATION).ok_or(ParseError::Header)?;
        let scheme = S::parse(header).map_err(|_| ParseError::Header)?;

        Ok(Authorization(scheme))
    }
}

impl<S: Scheme> IntoHeaderValue for Authorization<S> {
    type Error = <S as IntoHeaderValue>::Error;

    fn try_into(self) -> Result<HeaderValue, <Self as IntoHeaderValue>::Error> {
        self.0.try_into()
    }
}

impl<S: Scheme> fmt::Display for Authorization<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}