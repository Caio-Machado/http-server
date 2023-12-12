#[derive(PartialEq)]
#[derive(Debug)]
pub struct Header {
    section: String,
    content: String,
}

impl Header {
    pub fn try_build_headers(possible_headers: Option<&[&str]>) -> Result<Vec<Header>, HeaderError> {
        match possible_headers {
            Some(headers) => {
                let builded_headers: Vec<Header> = 
                    headers
                        .iter()
                        .filter_map(|header_string| Header::try_build_header(header_string))
                        .collect::<Vec<Header>>();
                Ok(builded_headers)
            },
            None => return Err(HeaderError::InvalidHeader),
        }
    }

    fn try_build_header(header_string: &str) -> Option<Header> {
        match header_string.split_once(": ") {
            Some((section, content)) => Some(Header {
                section: String::from(section),
                content: String::from(content),
            }),
            None => None,
        }
    }

    pub fn header_as_string(&self) -> String {
        format!("{}: {}", self.section, self.content)
    }
}

pub enum HeaderError {
    InvalidHeader,
}