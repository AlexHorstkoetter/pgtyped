use std::marker::PhantomData;

use tokio_postgres::Client;

pub struct PreparedQuery<P, R> {
    params: PhantomData<P>,
    result: PhantomData<R>,
    query: &'static str,
}
impl<P, R> PreparedQuery<P, R> {
    pub fn new(query: &'static str) -> Self {
        Self {
            query,
            params: PhantomData,
            result: PhantomData,
        }
    }

    pub async fn run(&self, client: &Client, params: &P) -> Vec<R> {
        client
            .query(&self.query)
            .await
            .expect("query error")
            .iter()
            .map()
    }
}
