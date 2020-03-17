# kayrx-auth

web authentication schemes for kayrx framework.

Provides:

- typed [Authorization] and [WWW-Authenticate] headers
- [extractors] for an [Authorization] header
- [middleware] for easier authorization checking

All supported schemas are actix Extractors, and can be used both in the middlewares and request handlers.

Supported schemes

- [Basic](https://tools.ietf.org/html/rfc7617)
- [Bearer](https://tools.ietf.org/html/rfc6750)
