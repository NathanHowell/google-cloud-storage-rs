use crate::encode;
use crate::Result;
use url::Url;

pub(crate) trait Urls: Sized {
    fn join_segment(self, segment: impl AsRef<str>) -> crate::Result<Self>;
    fn bucket(self, bucket: impl AsRef<str>) -> Result<Self>;
    fn object(self, object: impl AsRef<str>) -> Result<Self>;
    fn slash_object(self, object: impl AsRef<str>) -> Result<Self>;
}

impl Urls for Url {
    fn join_segment(mut self, segment: impl AsRef<str>) -> crate::Result<Self> {
        {
            let path_segments = self.path_segments_mut();

            match path_segments {
                Err(_) => {
                    drop(path_segments);
                    return Err(crate::Error::InvalidRequestUrl {
                        url: self.clone(),
                        #[cfg(feature = "backtrace")]
                        backtrace: std::backtrace::Backtrace::capture(),
                    });
                }
                Ok(mut path_segments) => {
                    path_segments.push("/");
                }
            };
        }

        Ok(self.join(segment.as_ref())?)
    }

    fn bucket(self, bucket: impl AsRef<str>) -> Result<Self> {
        Ok(self.join_segment("b/")?.join(&encode::normal(bucket))?)
    }

    fn object(self, object: impl AsRef<str>) -> Result<Self> {
        Ok(self.join_segment("o/")?.join(&encode::normal(object))?)
    }

    fn slash_object(self, object: impl AsRef<str>) -> Result<Self> {
        Ok(self.join_segment("o/")?.join(&encode::slash(object))?)
    }
}
