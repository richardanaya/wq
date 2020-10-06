use watson::*;

pub fn wasm_to_json(buffer:&[u8]) -> std::result::Result<String,String>{
    match parse(&buffer) {
        Ok(p) => {
            let json_string = match serde_json::to_string(&p.to_owned()) {
                Ok(s) => s,
                Err(e) => {
                    return Err(e.to_string())
                }
            };
            Ok(json_string)
        }
        Err(e) => {
            Err(e.to_owned())
        }
    }
}

pub fn json_to_wasm(json: &str) -> std::result::Result<Vec<u8>,String>{
    let mut p: Program = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            return Err(e.to_string())
        }
    };
    Ok(p.compile())
}