use headless_chrome::Browser;
use headless_chrome::Tab;
use std::sync::Arc;
use std::format;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Duration;
use yaml_rust::{YamlLoader};

use std::error::Error;

/*== Web Handling == */
pub fn load_browser() -> Result<Browser, Box<dyn Error>>{

    // Create a browser instance to pass to other functions.
    let browser;
    
    // Sometimes the headless chrome crate will error out here if chrome or chromium is not
    // installed on the system. This is dumb, but I really, truly cannot be assed to implement the
    // same testing for chromium that the underlying crate does. The error message is ugly, and a
    // panic, but it tells you exactly what to fix so I am going to leave it.
    match Browser::default() {
        Err(e) => {println!("A chrome executable was not autodetected on the system, please make sure
                       you have chrome/chromium installed. (Specific error message: {}", e); std::process::exit(1);},
        Ok(value) => {browser = value}
    }
    Ok(browser)
}

pub fn visit_page(browser:&Browser, url:&str) -> Result<Arc<Tab>, Box<dyn Error>>{
    let tab = browser.wait_for_initial_tab()
        .expect("Error creating new tab.");
    tab.set_default_timeout(Duration::from_secs(60));
    tab.navigate_to(&url)?;
    
    // This variable exists to make sure that all of the scripts on the page run before the tab
    // is passed on.
    let mut _waitfor = tab.wait_for_elements("script");
    let mut _waitfor = tab.wait_for_elements("link");

    Ok(tab)
}

/*== Data Deserialization ==*/
#[derive(Debug)]
pub struct PageInfo{
    pub site_name:String, // Name of the site to be checked.
    pub url:String, // Format like "https://www.youtube.com/user/{}/"
    pub bio_url:String, // Same format as above, but for the page with the bio on it.
    pub title_match:Vec<String>, // Format for matching tab header, IE "404 Not Found"    
    pub title_trim:String, // The string to trim off of a tab header, IE " / Twitter"
    pub xpath:String, // The XPATH string to the element you want to scrape. "//div"
}

impl PageInfo {
    pub fn new(
    _site_name:String,
    _url:String,
    _bio_url:String,
    _title_match:Vec<String>,
    _title_trim:String,
    _xpath:String)
    -> Self{
        Self {
            site_name:      _site_name,
            url:            _url,
            bio_url:        _bio_url,
            title_match:    _title_match,
            title_trim:     _title_trim,
            xpath:          _xpath
        }
    }
}

pub fn targets_from_yaml(filename:String) -> Result<HashMap<String,PageInfo>, Box<dyn Error>>{
    // Converts yaml file to a string.
    let raw = read_to_string(&filename.as_str()).ok()
        .expect(format!("Error opening yaml file. Filename: {}", &filename).as_str());

    // Creates a vector of Yaml objects where the size of the vector is equal to
    // the number of yaml documents (denoted in yaml with ----)
    let yaml = YamlLoader::load_from_str(&raw).ok()
        .expect("Error loading yaml from string.");

    // Takes the yaml file's 0th document and converts it to a hashmap.
    let doc = &yaml[0].as_hash().unwrap();

    let mut targets:HashMap<String, PageInfo> = HashMap::new();

    for (k,v) in doc.iter(){
        let target_name = k.as_str().unwrap().to_string();
        
        let mut title_match:Vec<String> = Vec::new();

        if let Some(title_vector) = v["title_match"].as_vec(){
            for title in title_vector {
                title_match.push(title.as_str().unwrap().to_string())
            }
        }

        let target_values = PageInfo::new(
            v["site_name"].as_str().unwrap().to_string(),
            v["url"].as_str().unwrap().to_string(),
            v["bio_url"].as_str().unwrap().to_string(),
            title_match,
            v["title_trim"].as_str().unwrap().to_string(),
            v["xpath"].as_str().unwrap().to_string(),
        );

        targets.insert(target_name,target_values);
    }
    Ok(targets)

}


#[derive(Debug)]
pub enum WebReturn {
    Success {
        username:String,
        url:String,
        other:String},
    Failure(String)
}

/*
@TODO Implement a case for Facebook's sign in tab title that does not try
to parse the page. Perhaps try using a regex for page parsing or changing the
logic?
*/
pub fn parse_page(browser: &Browser, username:&String, pageinfo:&PageInfo) -> Result<WebReturn,Box<dyn Error>>{
    
    let url = &pageinfo.url.replace("{}", username);
    let bio_url = &pageinfo.bio_url.replace("{}", username);
    let site_name = &pageinfo.site_name;
    let title_match = &pageinfo.title_match; // FACEBOOK HAS A PAGE THAT NEEDS DIRECT MATCH
    let title_trim = &pageinfo.title_trim;
    let xpath = &pageinfo.xpath;

    let tab;

    match visit_page(browser, &url){
        Ok(value) => {
            tab = value;        
        },
        Err(error) => {
            println!("Failure to connect to {} (Error: {:?} )", url, error);
            return Ok(WebReturn::Failure(format!("Unable to connect to {}", url)))
        }
    };

    let title; 

    match tab.get_title() {
        Ok(value) => {
            for item in title_match {
                if value.contains(&item.as_str()){
                    return Ok(WebReturn::Failure(format!("No {} account found.", site_name)))
                }
            }
            title = value;
        }
        Err(error) => {
            println!("Failure to get tab name from {} (Error: {:?})", url, error);
            return Ok(WebReturn::Failure(format!("Unable to access {} title", url)))
        }
    }
    
    let bio_page = visit_page(browser, &bio_url);
    
    match bio_page?.wait_for_xpath(xpath.as_str())?.get_inner_text() {
        Ok(value) => {
            return Ok(WebReturn::Success {
                username:(title.replace(title_trim.as_str(), "")),
                url: (url.to_string()),
                other: (value)
            })
        }
        Err(error) => {
            return Ok(WebReturn::Success {
                username:(title.replace(title_trim.as_str(), "")),
                url: (url.to_string()),
                other: ("Error reading page bio".to_string())
            })
        }
    };

    
}
