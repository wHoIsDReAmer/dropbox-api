pub mod dropbox {
    use reqwest::header;

    #[derive(Default)]
    pub struct Dropbox {
        access_token: String
    }
    impl Dropbox {
        pub fn new() -> Self {
            Dropbox::default()
        }

        pub fn set_access_token(&mut self, token: String) {
            self.access_token = token;
        }

        pub fn upload(self, path: &str, file: std::fs::File) -> Result<(), Box<dyn std::error::Error>> {
            let mut client = reqwest::blocking::Client::new();
            let result = client
                .post("https://content.dropboxapi.com/2/files/upload")
                .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header("Dropbox-API-Arg", format!("{{\"path\":\"{}\", \"mode\": {{\".tag\": \"overwrite\"}}}}", path))
                .body(file)
                .send()?;
            Ok(())
        }

        pub fn delete(self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let mut client = reqwest::blocking::Client::new();
            let result = client
                .post("https://api.dropboxapi.com/2/files/delete_v2")
                .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
                .header(header::CONTENT_TYPE, "application/json")
                .body(format!("{{\"path\":\"{}\"}}", path))
                .send()?;
            println!("{}", result.text()?);
            Ok(())
        }

        pub fn download(self, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            let mut client = reqwest::blocking::Client::new();
            let result = client
                .post("https://content.dropboxapi.com/2/files/download")
                .header(header::AUTHORIZATION, format!("Bearer {}", self.access_token))
                .header("Dropbox-API-Arg", format!("{{\"path\":\"{}\"}}", path))
                .send()?;

            let bytes = result.bytes().unwrap();
            Ok(Vec::from(bytes.as_ref()))
        }
    }
}