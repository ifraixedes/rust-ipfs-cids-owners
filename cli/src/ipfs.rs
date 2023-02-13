//! IPFS high level API to fulfill the requirements of the command-line application.

use crate::error::{BoxError, Error, ExternalSystem};
use std::io::ErrorKind as ioErrorKind;

use std::path::Path;

use async_fs::File;
use ipfs_api_backend_hyper::{request, IpfsApi, IpfsClient};

/// IPFS client wrapper to expose higher level operations.
pub struct Client<'a> {
    client: &'a IpfsClient,
}

impl<'a> Client<'a> {
    /// Creates a client that uses the passed IPFS client.
    pub fn with_client(client: &'a IpfsClient) -> Self {
        Client { client }
    }

    /// Uploads a file to IPFS with optional specifying the remote path and returns its
    /// corresponding CID.
    pub async fn uploload_file(
        &self,
        filepath: &Path,
        remote_path: Option<&str>,
    ) -> Result<String, Error> {
        let add_opts = if let Some(p) = remote_path {
            if !p.starts_with("/") {
                return Err(Error::invalid_arguments(
                    "remote_path",
                    "must begin with slack ('/')",
                ));
            }
            request::Add {
                to_files: remote_path,
                ..Default::default()
            }
        } else {
            Default::default()
        };

        let file = File::open(filepath).await.map_err(|err| match err.kind() {
            ioErrorKind::NotFound => Error::invalid_arguments("filepath", "file not found"),
            ioErrorKind::PermissionDenied => {
                Error::invalid_arguments("filepath", "not read permissions to the file")
            }
            _ => Error::internal("system error when reading the file", BoxError::from(err)),
        })?;

        let res = self
            .client
            .add_async_with_options(file, add_opts)
            .await
            .map_err(|err| Error::external(BoxError::from(err), ExternalSystem::IPFS))?;

        Ok(res.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Write;

    #[test_with::http(localhost:5001)]
    #[tokio::test]
    async fn test_client_upload_file() {
        let (filepath, content_expected) = generate_temp_file();
        let ipfs_cli = &ipfs_client();
        let client = Client::with_client(&ipfs_cli);

        // File without remote path.
        let cid = client
            .uploload_file(filepath.as_path(), None)
            .await
            .expect("no error uploading the file");
        assert!(!cid.is_empty(), "CID isn't empty");

        use futures::TryStreamExt;

        let content = ipfs_cli
            .cat(&cid)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await
            .expect("the content of the uploaded file through the cat operation");
        assert_eq!(
            content_expected,
            String::from_utf8(content).unwrap().to_owned()
        );

        // File with remote path.
        let cid_reuploaded = client
            .uploload_file(filepath.as_path(), Some("/hello-ipfs.txt"))
            .await
            .expect("no error uploading the file");
        assert!(!cid_reuploaded.is_empty(), "CID re-uploaded isn't empty");
        assert_eq!(cid, cid_reuploaded, "CID and CID re-uploaded are the same");
    }

    // Test helpers.
    use ipfs_api_backend_hyper::IpfsClient;
    use mktemp::Temp;

    fn ipfs_client() -> IpfsClient {
        use http::uri::Scheme;
        use ipfs_api_backend_hyper::TryFromUri;

        IpfsClient::from_host_and_port(
            Scheme::try_from("http").expect("'http' scheme to be valid"),
            "localhost",
            5001,
        )
        .expect("an IPFS client from HTTP, localhost, and 5001 port")
    }

    fn generate_temp_file() -> (Temp, String) {
        use std::fs::File;

        // TODO: This data should be generated randomly for guaranteeing not stumble with state
        // tests.
        let data = String::from("Hello IPFS!!");
        let temp_file = Temp::new_file().expect("create temp file");
        let mut file =
            File::create(temp_file.as_path()).expect("open temp file for write-only mode");
        file.write_all(data.as_bytes())
            .expect("write test data to the temp file");

        (temp_file, data)
    }
}
