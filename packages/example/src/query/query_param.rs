pub enum QueryParam<'a> {
    Scalar(Box<dyn QueryScalar>),
}

pub trait QueryScalar {
    fn getSqlStr<'a>(self) -> &'a str;
}

pub struct NamedParam<'a> {
    name: &'a str,
    param: QueryParam<'a>,
}

impl QueryScalar for u64 {
    fn getSqlStr<'a>(self) -> &'a str {
        self.to_string()
    }
}
