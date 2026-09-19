//! Axum extractors for htmx request headers.

use std::marker::PhantomData;

use axum_core::{
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use http::{StatusCode, header::LOCATION, request::Parts};

use crate::{
    HX_BOOSTED, HX_CURRENT_URL, HX_HISTORY_RESTORE_REQUEST, HX_PROMPT, HX_REQUEST, HX_TARGET,
    HX_TRIGGER, HX_TRIGGER_NAME,
};

/// The `HX-Boosted` header.
///
/// This is set when a request is made from an element where its parent has the
/// `hx-boost` attribute set to `true`.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `false`.
///
/// See <https://htmx.org/attributes/hx-boost/> for more information.
#[derive(Debug, Clone, Copy)]
pub struct HxBoosted(pub bool);

impl<S> FromRequestParts<S> for HxBoosted
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts.headers.contains_key(HX_BOOSTED) {
            Ok(HxBoosted(true))
        } else {
            Ok(HxBoosted(false))
        }
    }
}

/// The `HX-Current-Url` header.
///
/// This is set on every request made by htmx itself. As its name implies, it
/// just contains the current url.
///
/// This extractor will always return a value. If the header is not present, or
/// extractor fails to parse the url it will return `None`.
#[derive(Debug, Clone)]
pub struct HxCurrentUrl(pub Option<http::Uri>);

impl<S> FromRequestParts<S> for HxCurrentUrl
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(url) = parts.headers.get(HX_CURRENT_URL) {
            let url = url
                .to_str()
                .ok()
                .and_then(|url| url.parse::<http::Uri>().ok());

            return Ok(HxCurrentUrl(url));
        }

        Ok(HxCurrentUrl(None))
    }
}

/// The `HX-History-Restore-Request` header.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `false`.
#[derive(Debug, Clone, Copy)]
pub struct HxHistoryRestoreRequest(pub bool);

impl<S> FromRequestParts<S> for HxHistoryRestoreRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts.headers.contains_key(HX_HISTORY_RESTORE_REQUEST) {
            Ok(HxHistoryRestoreRequest(true))
        } else {
            Ok(HxHistoryRestoreRequest(false))
        }
    }
}

/// The `HX-Prompt` header.
///
/// This is set when a request is made from an element that has the `hx-prompt`
/// attribute set. The value will contain the string input by the user.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxPrompt(pub Option<String>);

impl<S> FromRequestParts<S> for HxPrompt
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(prompt) = parts.headers.get(HX_PROMPT) {
            if let Ok(prompt) = prompt.to_str() {
                return Ok(HxPrompt(Some(prompt.to_string())));
            }
        }

        Ok(HxPrompt(None))
    }
}

/// The `HX-Request` header.
///
/// This is set on every request made by htmx itself. It won't be present on
/// requests made manually, or by other libraries.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `false`.
#[derive(Debug, Clone, Copy)]
pub struct HxRequest(pub bool);

impl<S> FromRequestParts<S> for HxRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxRequestExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if parts.headers.contains_key(HX_REQUEST) {
            Ok(HxRequest(true))
        } else {
            Ok(HxRequest(false))
        }
    }
}

/// The `HX-Target` header.
///
/// This is set when a request is made from an element that has the `hx-target`
/// attribute set. The value will contain the target element's id. If the id
/// does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxTarget(pub Option<String>);

impl<S> FromRequestParts<S> for HxTarget
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxTargetExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(target) = parts.headers.get(HX_TARGET) {
            if let Ok(target) = target.to_str() {
                return Ok(HxTarget(Some(target.to_string())));
            }
        }

        Ok(HxTarget(None))
    }
}

/// The `HX-Trigger-Name` header.
///
/// This is set when a request is made from an element that has the `hx-trigger`
/// attribute set. The value will contain the trigger element's name. If the
/// name does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxTriggerName(pub Option<String>);

impl<S> FromRequestParts<S> for HxTriggerName
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxTriggerNameExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(trigger_name) = parts.headers.get(HX_TRIGGER_NAME) {
            if let Ok(trigger_name) = trigger_name.to_str() {
                return Ok(HxTriggerName(Some(trigger_name.to_string())));
            }
        }

        Ok(HxTriggerName(None))
    }
}

/// The `HX-Trigger` header.
///
/// This is set when a request is made from an element that has the `hx-trigger`
/// attribute set. The value will contain the trigger element's id. If the id
/// does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxTrigger(pub Option<String>);

impl<S> FromRequestParts<S> for HxTrigger
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxTriggerExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(trigger) = parts.headers.get(HX_TRIGGER) {
            if let Ok(trigger) = trigger.to_str() {
                return Ok(HxTrigger(Some(trigger.to_string())));
            }
        }

        Ok(HxTrigger(None))
    }
}

/// Requires the `HX-Request` header, redirecting requests that omit it to a
/// user defined location.
///
/// Put this first in a handler's arguments to reject requests before later
/// extractors and the handler run. `T` implements [`HxRequiredRedirect`]
/// to choose the redirect location.
///
/// # Example
///
/// ```rust
/// use axum::{http::request::Parts, response::Html};
/// use axum_htmx::{HxRequired, HxRequiredRedirect};
///
/// struct Home;
///
/// impl HxRequiredRedirect for Home {
///     fn location(_: &Parts) -> impl AsRef<str> {
///         "/"
///     }
/// }
///
/// async fn fragment(_: HxRequired<Home>) -> Html<&'static str> {
///     Html("<p>A fragment</p>")
/// }
/// ```
#[derive(Debug)]
pub struct HxRequired<T>(PhantomData<T>);

/// Supplies the redirect location for requests rejected by [`HxRequired`].
pub trait HxRequiredRedirect {
    /// Returns the full-page location, borrowed from request parts or owned.
    fn location(parts: &Parts) -> impl AsRef<str>;
}

impl<S, T> FromRequestParts<S> for HxRequired<T>
where
    S: Send + Sync,
    T: HxRequiredRedirect,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let HxRequest(is_htmx) = match HxRequest::from_request_parts(parts, state).await {
            Ok(hx_request) => hx_request,
            Err(never) => match never {},
        };
        if is_htmx {
            Ok(Self(PhantomData))
        } else {
            Err((
                StatusCode::SEE_OTHER,
                [(LOCATION, T::location(parts).as_ref())],
                (),
            )
                .into_response())
        }
    }
}

#[cfg(test)]
mod hx_required_tests {
    use axum::{Router, extract::Path, routing::get};
    use http::StatusCode;

    use super::*;

    struct ParentPage;

    impl HxRequiredRedirect for ParentPage {
        fn location(parts: &Parts) -> impl AsRef<str> {
            parts.uri.path().strip_suffix("/partial").unwrap()
        }
    }

    struct OwnedLocation;

    impl HxRequiredRedirect for OwnedLocation {
        fn location(parts: &Parts) -> impl AsRef<str> {
            format!("/full{}", parts.uri.path())
        }
    }

    struct InvalidLocation;

    impl HxRequiredRedirect for InvalidLocation {
        fn location(_: &Parts) -> impl AsRef<str> {
            "invalid\nlocation"
        }
    }

    #[tokio::test]
    async fn required_redirects_before_later_extractors_and_allows_htmx() {
        let app = Router::new().route(
            "/items/{id}/partial",
            get(|_: HxRequired<ParentPage>, _: Path<u32>| async { StatusCode::NO_CONTENT }),
        );
        #[cfg(feature = "auto-vary")]
        let app = app.layer(crate::AutoVaryLayer);
        let server = axum_test::TestServer::new(app).unwrap();

        // Reject before Path<u32> tries to parse the invalid ID.
        let rejected = server.get("/items/invalid/partial").await;
        rejected.assert_status(StatusCode::SEE_OTHER);
        rejected.assert_header("location", "/items/invalid");

        let accepted = server
            .get("/items/1/partial")
            .add_header(HX_REQUEST, "true")
            .await;
        accepted.assert_status(StatusCode::NO_CONTENT);

        #[cfg(feature = "auto-vary")]
        for response in [rejected, accepted] {
            response.assert_header("vary", "hx-request");
        }
    }

    #[tokio::test]
    async fn redirects_to_owned_location() {
        let app = Router::new().route("/fragment", get(|_: HxRequired<OwnedLocation>| async {}));
        let server = axum_test::TestServer::new(app).unwrap();

        server
            .get("/fragment")
            .await
            .assert_header("location", "/full/fragment");
    }

    #[tokio::test]
    async fn invalid_redirect_location_returns_server_error() {
        let app = Router::new().route("/", get(|_: HxRequired<InvalidLocation>| async {}));
        let server = axum_test::TestServer::new(app).unwrap();

        server
            .get("/")
            .await
            .assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
