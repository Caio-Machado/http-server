use crate::server::header::Header;

struct Response {
    status_line: StatusLine,
    headers: Option<Vec<Header>>,
    body: Option<String>,
}

impl Response {
    pub fn build_response(status_line: StatusLine, headers: Option<Vec<Header>>, body: Option<String>) -> Response {
        Response {
            status_line,
            headers,
            body,
        }
    }

    pub fn build_message(&self) {
        // "status_line\r\nheaders\r\nbody"
    }
}

struct StatusLine {
    http_version: String,
    status_code: u16,
    status_text: StatusText,
}

impl StatusLine {
    pub fn build(status_code: u16) -> StatusLine {
        StatusLine {
            http_version: "HTTP/1.1".to_string(),
            status_code,
            status_text: StatusText::identify_status_line(status_code),
        }
    }
}

enum StatusText {
    Ok,
    NotFound,
    InternalServerError,
    BadRequest,
    Invalid,
}

impl StatusText {
    pub fn identify_status_line(status_code: u16) -> StatusText {
        match status_code {
            200 => StatusText::Ok,
            404 => StatusText::NotFound,
            500 => StatusText::InternalServerError,
            400 => StatusText::BadRequest,
            _ => StatusText::Invalid,
        }
    }

    pub fn status_to_str<'a>(&self) -> &'a str {
        match self {
            StatusText::Ok => "OK",
            StatusText::NotFound => "Not Found",
            StatusText::InternalServerError => "Internal Server Error",
            StatusText::BadRequest => "Bad Request",
            _ => "",
        }
    }
}