use headless_chrome::Browser;
use headless_chrome::Tab;
use std::sync::Arc;
use std::format;
use std::collections::HashMap;
use std::fs::read_to_string;
use yaml_rust::{YamlLoader, YamlEmitter};

use std::error::Error;

/*== Web Handling == */
pub fn load_browser() -> Result<Browser, Box<dyn Error>>{

    // Create a browser instance to pass to other functions.
    let browser = Browser::default()
        .expect("Error finding chrome executable.");

    Ok(browser)
}

pub fn visit_page(browser:&Browser, url:&str) -> Result<Arc<Tab>, Box<dyn Error>>{
    let tab = browser.wait_for_initial_tab()
        .expect("Error creating new tab.");
    tab.navigate_to(&url)?;
    // This variable exists to make sure that all of the scripts on the page run before the tab
    // is passed on.
    let mut _waitfor = tab.wait_for_elements("script");
    let mut _waitfor = tab.wait_for_elements("link");

    Ok(tab)
}

/*== Data Deserialization ==*/
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

 
pub fn targets_from_yaml(filename:String){
    let raw = read_to_string(&filename.as_str()).ok()
        .expect(format!("Error opening yaml file. Filename: {}", &filename).as_str());

    let yaml = YamlLoader::load_from_str(&raw).ok()
        .expect("Error loading yaml from string.");

    let doc = &yaml[0];
    println!("{:?}", doc.into_string());

    for item in doc.as_vec(){
        println!("{:?}", item[0]);
    }
    /* 
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        println!("{:?}",emitter.dump(&yaml[0]).unwrap());
    }
    */

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
pub fn parse_page(browser: &Browser, username:&String, pageinfo:PageInfo) -> Result<WebReturn,Box<dyn Error>>{
    let mut fail_switch:bool = false;
    
    let url = pageinfo.url.replace("{}", username);
    let bio_url = pageinfo.bio_url.replace("{}", username);
    let site_name = pageinfo.site_name;
    let title_match = pageinfo.title_match; // FACEBOOK HAS A PAGE THAT NEEDS DIRECT MATCH
    let title_trim = pageinfo.title_trim;
    let xpath = pageinfo.xpath;


    let tab = visit_page(browser, &url).ok()
        .expect(format!("Error connecting to {} ({}).", site_name, url).as_str());

    let title = tab.get_title().ok()
        .expect(format!("Error grabbing tab title from {} ({})", site_name, url).as_str());
    
    for _item in &title_match{
        if title.contains(&_item.as_str()){
            fail_switch = true;
        }
    }
    if fail_switch {
        Ok(WebReturn::Failure(format!("No {} account found.", site_name)))
    } else {
        let bio_page = visit_page(browser, &bio_url);

        let bio = bio_page.as_ref().ok()
            .expect("Error scraping bio of {} ({})")
            .wait_for_xpath(xpath.as_str()).ok()
            .expect(format!("Error unwrapping bio of {} ({})", site_name, url).as_str());

        Ok(
            WebReturn::Success {
                username: (title.replace(title_trim.as_str(), "")),
                url: (url.to_string()),
                other: (bio.get_inner_text().ok()
                .expect(format!("Error parsing inner bio text of {} ({})", site_name, url).as_str()))
            }
        )

    }
    
}
