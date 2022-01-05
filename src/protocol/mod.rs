use serde::{ Serialize, Deserialize };


#[derive(Serialize, Deserialize)]
struct ProtoRequest<T: Into<String>> {
    method: String,
    id: u128,
    params: Vec<T>,
}

impl <T: Into<String>> ProtoRequest<T> {
    fn new(method: String, params: Vec<T>, id: u128) -> Self 
    {
        Self {
            method,
            params,
            id,
        }
    }
}
