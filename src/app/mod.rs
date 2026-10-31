pub mod ui;
use std::{fs, path::{Path, PathBuf}, sync::Arc};
use reqwest::{Url, blocking::{Client, Response}, cookie::{CookieStore, Jar}};

pub struct App {
    pub domain: Option<String>,
    pub domain_url: Option<Url>,

    pub storage: PathBuf,

    pub arcjar: Arc<Jar>,
    pub client: Client,

    pub singleline_input: String,
}

impl Default for App {
    fn default() -> Self {
        let storage = dirs::data_local_dir()
            .expect("No storage directory detected! Aborting...")
            .join("canvascached");

        if !storage.exists() {
            fs::create_dir(storage.clone()).expect("Failed to create a storage directory! Aborting...");
        }

        let arcjar = Arc::new(Jar::default());
        let client: Client = Client::builder().cookie_provider(arcjar.clone()).user_agent("CanvasCached/1.0.0").build().expect("Failed to create a reqwest client! Aborting...");

        App {
            domain: None,
            domain_url: None,
            storage,

            arcjar,
            client,

            singleline_input: String::new(),
        }
    }
}

impl App {
    fn request_get(&mut self, endpoint: &'static str) -> Result<Response, String> {
        let domain = self.domain.clone();
        if let Some(real_domain) = domain {
            let url = format!("{}{}", real_domain, endpoint);
            let response = self.client.get(url).send();

            if response.is_err() {
                return Err(format!("request_get failed: self.client response is an Err!"));
            }

            return Ok(response.unwrap());
        }
        Err(format!("request_get failed: self.domain is None!"))
    }

    pub fn request_get_store(&mut self, endpoint: &'static str) -> Result<(), String> {
        if endpoint.is_empty() {
            return Err(format!("request_get_store failed: endpoint parameter is an empty string!"));
        }

        let preresults = self.request_get(endpoint);
        if preresults.is_err() {
            return Err(format!("request_get_store failed: '{}'", preresults.err().unwrap()));
        }

        let results = preresults.unwrap();

        if !results.status().is_success() {
            return Err(format!("request_get_store failed: '{}'", results.status()));
        }

        let results_path = self.storage.join(format!("{}.json", endpoint));
        if !results_path.exists() {
            if let Some(newpath) = Path::new(results_path.as_os_str()).parent() {
                fs::create_dir_all(newpath.as_os_str()).expect("Failed to create a directory in storage! Aborting...");
            }
        }

        println!("Writing to '{}'", results_path.clone().into_string().expect("Invalid unicode data in PathBuf! Aborting..."));

        let result_bytes = results.bytes().expect("Failed to copy data to bytes! Aborting...");
        fs::write(results_path, result_bytes).expect("Failed to write file to results_path! Aborting...");

        Ok(())
    }

    pub fn store_cookies(&mut self, cookies: &str) {
        fs::write(self.storage.join("cookies.txt"), cookies.as_bytes()).expect("Failed to write cookies to storage directory! Aborting...");
    }

    pub fn store_domain(&mut self, cookies: &str) {
        fs::write(self.storage.join("domain.txt"), cookies.as_bytes()).expect("Failed to write domain to storage directory! Aborting...");
    }

    pub fn load_cookies(&mut self) -> Result<(), String> {
        let cookies = fs::read_to_string(self.storage.join("cookies.txt"));
        if let Ok(real_cookies) = cookies {
            return self.set_cookies(&real_cookies);
        }

        Err(format!("load_cookies failed: cookies.txt could not be read to string!"))
    }

    pub fn load_domain(&mut self) -> Result<(), String> {
        let domain = fs::read_to_string(self.storage.join("domain.txt"));
        if let Ok(real_domain) = domain {
            return self.set_domain(&real_domain);
        }

        Err(format!("load_domain failed: domain.txt could not be read to string!"))
    }


    pub fn get_cookies(&mut self) -> Result<String, String> {
        if let Some(real_domain_url) = &self.domain_url {
            let cookies = self.arcjar.cookies(&real_domain_url);
            if let Some(real_cookies) = cookies {
                return Ok(String::from(real_cookies.to_str().expect("String did not contain visible ASCII characters! Aborting...")));
            }
            return Err(format!("get_cookies failed: self.arcjar.cookies is None!"))
        }

        Err(format!("get_cookies failed: self.domain_url is None!"))
    }

    pub fn set_cookies(&mut self, cookies: &str) -> Result<(), String> {
        if let Some(real_domain_url) = &self.domain_url {
            for cookie in cookies.split("; ") {
                self.arcjar.add_cookie_str(cookie, real_domain_url);
            }
            return Ok(())
        }

        Err(format!("set_cookies failed: self.domain_url is None!"))
    }

    pub fn set_domain(&mut self, domain: &str) -> Result<(), String> {
        let precookies = self.get_cookies();

        let domain_url = domain.parse::<Url>();
        if domain_url.is_err() {
            return Err(format!("set_domain failed: could not parse the domain string: '{}'", domain_url.err().unwrap()));
        }

        self.domain = Some(format!("{domain}"));
        self.domain_url = Some(domain_url.unwrap());

        if let Ok(cookies) = precookies {
            let cookies_result = self.set_cookies(&cookies);
            if cookies_result.is_err() {
                return cookies_result;
            }
        }

        Ok(())
    }

    pub fn save_cookies(&mut self) -> Result<(), String> {
        let precookies = self.get_cookies();
        if let Ok(cookies) = precookies {
            self.store_cookies(&cookies);
            return Ok(());
        }

        Err(format!("save_cookies failed: '{}'", precookies.err().unwrap()))
    }
}
