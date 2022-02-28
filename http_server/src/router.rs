use std::io::prelude::*;

use http::{httprequest::{HttpRequest, self}, httpresponse::HttpResopnse};

use crate::handler::{WebServiceHandler, PageNotFoundHandler, StaticPageHandler, Handler};


pub struct Router;

impl Router {
	pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
		match req.method {
			httprequest::Method::Get => match &req.resource {
				httprequest::Resource::Path(s) => {
					let route: Vec<&str> = s.split("/").collect();
					match route[1] {
						"api" => {
							let resp: HttpResopnse = WebServiceHandler::handle(&req);
							let _ = resp.send_response(stream);
						},
						_ => {
							let resp: HttpResopnse = StaticPageHandler::handle(&req);
							let _ = resp.send_response(stream);
						}
					}
				}
			},
			_ => {
				let resp: HttpResopnse = PageNotFoundHandler::handle(&req);
				let _ = resp.send_response(stream);
			}
		}
	}
}