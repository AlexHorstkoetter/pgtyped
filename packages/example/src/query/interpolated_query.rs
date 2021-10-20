struct InterpolatedQuery {
    query: &'static str,
}

enum QueryParam {
    ScalarParam {
        name: &str,
        required: bool,
        assignedIndex: usize,
    },
}
