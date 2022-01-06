use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ProtoParam<T: Into<String>> {
    name: String,
    value: T,
}

#[derive(Serialize, Deserialize)]
struct ProtoRequest<T: Into<String>> {
    method: String,
    id: u128,
    params: Vec<ProtoParam<T>>,
}

impl <T: Into<String>> ProtoParam<T> {
    fn new(name: &str, value: T) -> Self {
        Self { name: name.to_string(), value }
    }
}

impl<T: Into<String>> ProtoRequest<T> {
    fn new(method: String, params: Vec<ProtoParam<T>>, id: u128) -> Self {
        Self { method, params, id }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proto_request_new_test() {
    }
}
