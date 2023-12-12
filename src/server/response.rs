use crate::server::header::Header;

use super::header::HeaderError;

const HTTP_SERVER: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Response {
    status_line: StatusLine,
    headers: Option<Vec<Header>>,
    body: Option<String>,
}

impl Response {
    pub fn build_response(
        status_line: StatusLine,
        headers: Option<Vec<Header>>,
        body: Option<String>,
    ) -> Response {
        Response {
            status_line,
            headers,
            body,
        }
    }

    pub fn try_build_response_fields(
        status_code: u16,
        headers_strings: Option<&[&str]>,
        body_string: Option<&str>,
    ) -> Result<(StatusLine, Option<Vec<Header>>, Option<String>), HeaderError> {
        let status_line: StatusLine = StatusLine::build(status_code);
        let headers: Option<Vec<Header>> = match Header::try_build_headers(headers_strings) {
            Ok(h) => Some(h),
            Err(e) => return Err(e),
        };
        let body: Option<String> = match body_string {
            Some(b) => Some(b.to_string()),
            None => None,
        };
        Ok((status_line, headers, body))
    }

    pub fn build_response_as_string(&self) -> String {
        let status_line: String = StatusLine::status_line_as_string(&self.status_line);
        let headers: String = Header::build_headers_to_string(self.headers.as_ref());
        let body: String = match &self.body {
            Some(b) => b.to_owned(),
            None => String::new(),
        };
        format!("{}\r\n{}\r\n{}", status_line, headers, body)
    }
}

#[derive(Debug)]
pub struct StatusLine {
    http_version: String,
    status_code: u16,
    status_text: StatusText,
}

impl StatusLine {
    pub fn build(status_code: u16) -> StatusLine {
        StatusLine {
            http_version: HTTP_SERVER.to_string(),
            status_code,
            status_text: StatusText::identify_status_line(status_code),
        }
    }

    pub fn status_line_as_string(status_line: &StatusLine) -> String {
        format!(
            "{} {} {}",
            status_line.http_version,
            status_line.status_code,
            status_line.status_text.status_to_string()
        )
    }
}

#[derive(Debug)]
pub enum StatusText {
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

    pub fn status_to_string(&self) -> String {
        let result: &str = match self {
            StatusText::Ok => "OK",
            StatusText::NotFound => "Not Found",
            StatusText::InternalServerError => "Internal Server Error",
            StatusText::BadRequest => "Bad Request",
            _ => "",
        };
        result.to_string()
    }
}
