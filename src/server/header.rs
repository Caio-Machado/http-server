use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Header {
    section: String,
    content: String,
}

impl Header {
    pub fn try_build_headers_from_slice(
        possible_headers: Option<&[&str]>,
    ) -> Result<Vec<Header>, HeaderError> {
        match possible_headers {
            Some(headers) => {
                let builded_headers: Vec<Header> = headers
                    .iter()
                    .filter_map(|header_string| Header::try_build_header(header_string))
                    .collect::<Vec<Header>>();
                Ok(builded_headers)
            }
            None => return Err(HeaderError::InvalidHeader),
        }
    }

    pub fn try_build_headers_from_hashmap(possible_headers: HashMap<&str, String>) -> Vec<Header> {
        possible_headers
            .iter()
            .map(|(section, content)| Header {
                section: section.to_string(),
                content: content.to_string(),
            })
            .collect::<Vec<Header>>()
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
        format!("{}: {}\r\n", self.section, self.content)
    }

    pub fn build_headers_to_string(headers: Option<&Vec<Header>>) -> String {
        match headers {
            Some(headers_vec) => headers_vec
                .iter()
                .map(|h| h.header_as_string())
                .reduce(|acc, h| format!("{}{}", acc, h))
                .unwrap(),
            None => String::new(),
        }
    }
}

pub enum HeaderError {
    InvalidHeader,
}
