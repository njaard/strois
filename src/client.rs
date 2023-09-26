use std::time::Duration;

use rusty_s3::{Credentials, S3Action};
use ureq::Response;
use url::Url;

use crate::{Bucket, Result};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) addr: Url,
    pub(crate) cred: Credentials,
}

impl Client {
    pub fn new(addr: impl AsRef<str>, cred: Credentials) -> Result<Self> {
        Ok(Self {
            addr: addr.as_ref().parse()?,
            cred,
        })
    }

    /// /!\ Do not create the bucket on the S3.
    pub fn bucket(&self, name: impl Into<String>) -> Result<Bucket> {
        Bucket::new(self.clone(), name)
    }

    pub(crate) fn put<'a>(&self, action: impl S3Action<'a>) -> Result<Response> {
        Ok(ureq::put(action.sign(Duration::from_secs(60)).as_str()).call()?)
    }

    pub(crate) fn put_with_body<'a>(
        &self,
        action: impl S3Action<'a>,
        body: &[u8],
    ) -> Result<Response> {
        Ok(ureq::put(action.sign(Duration::from_secs(60)).as_str()).send_bytes(body)?)
    }

    pub(crate) fn delete<'a>(&self, action: impl S3Action<'a>) -> Result<Response> {
        Ok(ureq::delete(action.sign(Duration::from_secs(60)).as_str()).call()?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use testcontainers::{clients::Cli, images::minio::MinIO};

    #[test]
    fn new_client() {
        let docker = Cli::docker();
        let image = docker.run(MinIO::default());

        let client = Client::new("http://127.0.0.1:9000", Credentials::new("", ""));
    }
}