use crate::request::Request;
use crate::{Client, Result};
use async_stream::try_stream;
use futures::stream::Stream;
use std::pin::Pin;

pub(crate) trait Paginate<'a>
where
    Self: Request + Sized + 'a,
    Self::Response: Unpin,
{
    fn paginate(
        self,
        client: &'a Client,
    ) -> Pin<Box<dyn Stream<Item = Result<Self::Response>> + 'a>> {
        let mut next = Some(self);

        Box::pin(try_stream! {
            while let Some(request) = next {
                let response = client.invoke(&request).await?;
                next = Self::next_request(&response);
                yield response;
            }
        })
    }

    fn next_request(response: &Self::Response) -> Option<Self>;
}
