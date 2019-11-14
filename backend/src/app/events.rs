use actix_web::{Error, HttpRequest, HttpResponse};
use futures::{Future, future::result};

#[inline]
pub fn get_all_events(
    _req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {

    result(Ok(HttpResponse::Ok().json("Not implemented api.")))
}
