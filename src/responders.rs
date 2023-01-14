pub mod responses {
    use actix_files::NamedFile;
    use actix_web::{
        body::{BoxBody, EitherBody},
        HttpResponse, Responder,
    };
    use actix_web_lab::web::Redirect;

    pub enum AppResponse<R, U>
    where
        R: Responder,
        U: Responder,
    {
        File(Option<NamedFile>),
        Redirect(R),
        Status(U),
    }

    impl Responder for AppResponse<HttpResponse, HttpResponse> {
        type Body = EitherBody<BoxBody>;

        fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
            match self {
                AppResponse::File(file) => file.respond_to(req),
                AppResponse::Redirect(redirect) => redirect.respond_to(req).map_into_left_body(),
                AppResponse::Status(status) => status.respond_to(req).map_into_left_body(),
            }
        }
    }

    impl Responder for AppResponse<Redirect, HttpResponse> {
        type Body = EitherBody<BoxBody>;

        fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
            match self {
                AppResponse::File(file) => file.respond_to(req),
                AppResponse::Redirect(redirect) => {
                    redirect.respond_to(req).map_body(|_, _| EitherBody::Right {
                        body: BoxBody::new("Redirecting..."),
                    })
                }
                AppResponse::Status(status) => status.respond_to(req).map_into_left_body(),
            }
        }
    }
}
