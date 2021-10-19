use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryAst {
    name: &'static str,
    params: Vec<Param>,
    statement: Statement,
}

#[derive(Deserialize)]
pub struct Param {
    name: String,
    transform: ParamTransform,
    required: bool,
    #[serde(alias = "codeRefs")]
    code_refs: CodeRef,
    #[serde(alias = "isUsed")]
    is_used: bool,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ParamTransform {
    #[serde(alias = "scalar")]
    Scalar,
    #[serde(alias = "array_spread")]
    ArraySpread,
    #[serde(alias = "pick_tuple")]
    PickTuple { keys: Vec<ParamKey> },
    #[serde(alias = "pick_array_spread")]
    PickArraySpread { keys: Vec<ParamKey> },
}

#[derive(Deserialize)]
pub struct ParamKey {
    name: String,
    required: bool,
}

#[derive(Deserialize)]
pub struct CodeRef {
    defined: Option<CodeInterval>,
    used: Vec<CodeInterval>,
}

#[derive(Deserialize)]
pub struct Statement {
    loc: CodeInterval,
    body: String,
}

#[derive(Deserialize)]
pub struct CodeInterval {
    a: usize,
    b: usize,
    line: usize,
    col: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_ast() {
        let json = r#"{
            "name":"InsertBooks",
            "params":[{
                "name":"books",
                "codeRefs":{"defined":{"a":106,"b":110,"line":6,"col":9},
                "used":[{"a":202,"b":206,"line":9,"col":8}]},
                "transform":{
                    "type":"pick_array_spread",
                    "keys":[
                        {"name":"rank","required":true},
                        {"name":"name","required":true},
                        {"name":"authorId","required":true}
                    ]
                },
                "isUsed":true,
                "required":false
            }],
            "statement":{
                "body":"INSERT INTO books (rank, name, author_id)\r\nVALUES :books RETURNING id as book_id",
                "loc":{"a":151,"b":230,"line":8,"col":0}
            }
        }"#;

        let parsed: QueryAst = serde_json::from_str(&json).expect("deserialize error");
        // TODO this works but better finish that unit test!
    }
}
