use url::Url;

pub(crate) trait JoinSegment: Sized {
    fn join_segment(self, segment: &str) -> crate::Result<Self>;
}

impl JoinSegment for Url {
    fn join_segment(mut self, segment: &str) -> crate::Result<Self> {
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

        Ok(self.join(segment)?)
    }
}
