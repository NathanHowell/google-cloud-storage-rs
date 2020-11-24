use crate::request::Request;
use crate::{Client, Result};
use async_stream::try_stream;
use futures::stream::Stream;
use std::pin::Pin;

impl Client {
    pub(crate) fn paginate<'a, T>(
        self: &'a Client,
        request: T,
    ) -> Pin<Box<dyn Stream<Item = Result<T::Item>> + 'a>>
    where
        T: Clone + Paginate<'a> + Unpin,
        T::Response: Unpin,
    {
        let initial = request.clone();

        let mut next = Some(request);

        Box::pin(try_stream! {
            while let Some(request) = next {
                let response = self.invoke(&request).await?;
                next = T::into_request(initial.clone(), &response);
                for item in T::extract_items(response) {
                    yield item
                }
            }
        })
    }
}

pub(crate) trait Paginate<'a>
where
    Self: Request + Sized + 'a,
    Self::Response: Unpin,
{
    type Item: Unpin;

    fn extract_items(response: Self::Response) -> Vec<Self::Item>;

    fn into_request(self, response: &Self::Response) -> Option<Self>;
}
