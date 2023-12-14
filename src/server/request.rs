use crate::server::header::Header;

#[derive(Debug)]
pub struct Request {
    pub start_line: StartLine,
    pub headers: Vec<Header>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(start_line: StartLine, headers: Vec<Header>, body: Option<String>) -> Request {
        Request {
            start_line,
            headers,
            body,
        }
    }

    pub fn extract_request_fields(
        request_string: String,
    ) -> Result<(StartLine, Vec<Header>, Option<String>), RequestError> {
        let splited_request: Vec<&str> = request_string.split("\r\n").collect();

        let start_line: StartLine = match StartLine::build(splited_request.get(0)) {
            Ok(stl) => stl,
            Err(e) => return Err(e),
        };
        let header: Vec<Header> =
            match Header::try_build_headers_from_slice(splited_request.get(1..splited_request.len() - 1)) {
                Ok(headers) => headers,
                Err(_) => return Err(RequestError::InvalidHeader),
            };
        let body: Option<String> = match splited_request.last() {
            Some(body_str) => match body_str.to_owned() {
                "" => None,
                body_string => Some(body_string.to_string()),
            },
            None => None,
        };
        Ok((start_line, header, body))
    }
}

#[derive(Debug)]
pub struct StartLine {
    method: Method,
    pub request_target: RequestTarget,
    http_version: HttpVersion,
}

impl StartLine {
    pub fn build(start_line_string: Option<&&str>) -> Result<StartLine, RequestError> {
        match start_line_string {
            Some(stl_string) => {
                let fields: Vec<&str> = stl_string.split_whitespace().collect();
                if fields.len() != 3 {
                    return Err(RequestError::InvalidStarLine);
                }
                let method: Method = match Method::identify_method(fields.get(0)) {
                    Ok(method) => method,
                    Err(e) => return Err(e),
                };
                let request_target: RequestTarget = match RequestTarget::build(fields.get(1)) {
                    Ok(req_target) => req_target,
                    Err(e) => return Err(e),
                };
                let http_version: HttpVersion =
                    match HttpVersion::identify_http_version(fields.get(2)) {
                        Ok(http_v) => http_v,
                        Err(e) => return Err(e),
                    };

                Ok(StartLine {
                    method,
                    request_target,
                    http_version,
                })
            }
            None => return Err(RequestError::InvalidStarLine),
        }
    }
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    pub fn identify_method(method_string: Option<&&str>) -> Result<Method, RequestError> {
        match method_string {
            Some(method) => match method.to_owned() {
                "GET" => Ok(Method::GET),
                "POST" => Ok(Method::POST),
                "PUT" => Ok(Method::PUT),
                "PATCH" => Ok(Method::PATCH),
                "DELETE" => Ok(Method::DELETE),
                _ => Err(RequestError::InvalidStarLine),
            },
            None => Err(RequestError::InvalidStarLine),
        }
    }
}

#[derive(Debug)]
pub struct RequestTarget {
    pub full_path: String,
    pub path: String,
    pub random_string: Option<String>,
}

impl RequestTarget {
    pub fn build(possible_full_path: Option<&&str>) -> Result<RequestTarget, RequestError> {
        match possible_full_path {
            Some(full_path) => {
                let splited_path: Vec<&str> =
                    full_path.splitn(3, "/").filter(|s| s != &"").collect();
                if splited_path.is_empty() {
                    return Ok(RequestTarget {
                        full_path: full_path.to_string(),
                        path: String::from("/"),
                        random_string: None,
                    });
                }
                let path: String = match splited_path.get(0) {
                    Some(path) => path.to_string(),
                    None => String::from("/"),
                };
                let random_string: Option<String> = if splited_path.len() > 1 {
                    Some(splited_path.last().unwrap().to_string())
                } else {
                    None
                };
                Ok(RequestTarget {
                    full_path: full_path.to_string(),
                    path,
                    random_string,
                })
            }
            None => Err(RequestError::InvalidStarLine),
        }
    }
}

#[derive(Debug)]
enum HttpVersion {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30,
}

impl HttpVersion {
    fn identify_http_version(
        possible_http_version: Option<&&str>,
    ) -> Result<HttpVersion, RequestError> {
        match possible_http_version {
            Some(http_version) => match http_version.to_owned() {
                "HTTP/0.9" => Ok(HttpVersion::Http09),
                "HTTP/1.0" => Ok(HttpVersion::Http10),
                "HTTP/1.1" => Ok(HttpVersion::Http11),
                "HTTP/2.0" => Ok(HttpVersion::Http20),
                "HTTP/3.0" => Ok(HttpVersion::Http30),
                _ => return Err(RequestError::InvalidStarLine),
            },
            None => Err(RequestError::InvalidStarLine),
        }
    }
}

pub enum RequestError {
    InvalidStarLine,
    InvalidHeader,
    InvalidBody,
}
