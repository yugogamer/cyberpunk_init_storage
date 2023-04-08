use rand::{distributions::Alphanumeric, Rng};
use s3::{creds::Credentials, Bucket, Region};
use uuid::Uuid;

pub struct BucketHandler {
    storage: Bucket,
}

impl BucketHandler {
    pub async fn new() -> BucketHandler {
        let acces = "SCWPAPF1X1VVK7109MP0";
        let secret = "7605596e-2438-4e97-948d-a14e9d39eebf";
        let region = Region::Custom {
            region: "fr-par".into(),
            endpoint: "https://s3.fr-par.scw.cloud".into(),
        };
        let credential = Credentials::new(Some(acces), Some(secret), None, None, None).unwrap();
        let bucket = Bucket::new("raina-test-dev", region, credential).unwrap();
        BucketHandler { storage: bucket }
    }

    pub async fn signe_upload(&self, filename: &str) -> String {
        let expiry_secs = 3600;
        let filename = self.generate_filename(filename);
        self.storage
            .presign_put(format!("image/{filename}"), expiry_secs, None)
            .unwrap()
    }

    pub async fn signe_download(&self, filename: &str) -> String {
        let expiry_secs = 3600;
        self.storage
            .presign_get(format!("image/{filename}"), expiry_secs, None)
            .unwrap()
    }

    pub async fn list(&self) -> Vec<String> {
        let mut list = Vec::new();
        if let Ok(object) = self.storage.list("".into(), None).await {
            for object in object {
                for object in object.contents {
                    list.push(object.key);
                }
            }
        }
        list
    }

    fn generate_filename(&self, filename: &str) -> String {
        let random_string = generate_random_string();
        let extension = filename.split('.').last().unwrap();
        let uuid = Uuid::new_v4();
        format!("{random_string}-{uuid}.{extension}")
    }
}

fn generate_random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    s
}
