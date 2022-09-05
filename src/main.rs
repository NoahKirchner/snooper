use headless_chrome::Browser;
use headless_chrome::Tab;
use std::sync::Arc;
use std::env;
use std::format;

use std::error::Error;

#[derive(Debug)]
enum WebReturn {
    Success {
        username:String,
        url:String,
        other:String},
    Failure(String)

}



/* == WEB HANDLER == */
fn load_browser() -> Result<Browser, Box<dyn Error>>{

    // Create a browser instance to pass to other functions.
    let browser = Browser::default()
        .expect("Error finding chrome executable.");

    Ok(browser)
}

fn visit_page(browser:&Browser, url:&str) -> Result<Arc<Tab>, Box<dyn Error>>{
    let tab = browser.wait_for_initial_tab()
        .expect("Error creating new tab.");
    tab.navigate_to(&url)?;
    // This variable exists to make sure that all of the scripts on the page run before the tab
    // is passed on.
    let mut _waitfor = tab.wait_for_elements("script");
    let mut _waitfor = tab.wait_for_elements("link");

    Ok(tab)
}
/* ===================== */


/* == PARSERS == */
fn parse_instagram(browser:&Browser, username:&String) -> Result<WebReturn, Box<dyn Error>>{
    let url = format!("https://www.instagram.com/{}/", username);
    let tab = visit_page(browser, &url).ok()
        .expect("Error assigning visited instagram page to parser tab.");
 
    let title = tab.get_title().ok()
        .expect("Error grabbing title from instagram page.");


    if title.contains("Page not found"){
        Ok(WebReturn::Failure("Account not Found.".to_string()))
    } else {

        let bio = tab.find_element_by_xpath(
            "//div[contains(@class, '_aa_c')]").ok()
            .expect("Error scraping bio.");

        Ok(
            WebReturn::Success { 
            username: (title.replace(" • Instagram photos and videos", "")),
            url: (url),
            other: (bio.get_inner_text().ok().expect("Error unwrapping bio text.")) })
            
    }
}

fn parse_youtube(browser:&Browser, username:&String) -> Result<WebReturn, Box<dyn Error>>{
    let url = format!("https://www.youtube.com/user/{}/", username);
    let tab = visit_page(browser, &url).ok()
        .expect("Error assigning visited youtube page to parser tab.");
    let title = tab.get_title().ok()
        .expect("Error grabbing title from youtube page.");

    


    if title.contains("404 Not Found") {
        Ok(WebReturn::Failure("Account not Found.".to_string()))
    } else {
        let about = visit_page(browser, format!("{}about",&url).as_str());
        
        let bio = about.as_ref().unwrap().wait_for_xpath(
            "//*[@id='description-container']").unwrap();


        Ok(
            WebReturn::Success { 
            username: (title.replace(" - YouTube", "")),
            url: (url),
            other: (bio.get_inner_text().ok().expect("Error unwrapping bio.")) })
    }
}

fn parse_twitter(browser:&Browser, username:&String) -> Result<WebReturn, Box<dyn Error>>{
    let url = format!("https://www.twitter.com/{}", username);
    let tab = visit_page(browser, &url).ok()
        .expect("Error assigning visited twitter page to parser tab.");
    let title = tab.get_title().ok()
        .expect("Error grabbing title from twitter page.");
    

    if title.contains("Profile / Twitter") || title.contains("Page not found"){
        Ok(WebReturn::Failure("Account not Found.".to_string()))

    } else {

        let bio = tab.find_element_by_xpath(
            "/html/body/div[1]/div/div/div[2]/main/div/div/div/div[1]/div/div[2]/div/div/div").ok()
            .expect("Error scraping bio.");


        Ok(
            WebReturn::Success {
            username: (title.replace(" / Twitter", "")),
            url: (url),
            other: (bio.get_inner_text().ok().expect("Error unwrapping bio text.")) })
    }
}

fn parse_facebook(browser:&Browser, username:&String) -> Result<WebReturn, Box<dyn Error>>{
    let url = format!("https://www.facebook.com/{}", username);
    let tab = visit_page(browser, &url).ok()
        .expect("Error assigning visited Facebook page to parser tab.");
    let title = tab.get_title().ok()
        .expect("Error grabbing title from Facebook page.");
    

    if title.contains("Page Not Found") || title == "Facebook"{
        Ok(WebReturn::Failure("Account not Found.".to_string()))

    } else {

        let bio = tab.find_element_by_xpath(
            "/html/body/div[1]/div/div[1]/div/div[3]/div/div/div/div[1]/div[1]/div/div/div[4]/div[2]/div/div[1]/div/div[1]/div/div/div[2]").ok()
            .expect("Error scraping bio.");


        Ok(
            WebReturn::Success {
            username: (title.replace(" | Facebook", "")),
            url: (url),
            other: (bio.get_inner_text().ok().expect("Error unwrapping bio text.")) })
    }
}

fn parse_deviantart(browser:&Browser, username:&String) -> Result<WebReturn, Box<dyn Error>>{
    let url = format!("https://www.deviantart.com/{}", username);
    let tab = visit_page(browser, &url).ok()
        .expect("Error assigning visited DeviantArt page to parser tab.");
    let title = tab.get_title().ok()
        .expect("Error grabbing title from DeviantArt page.");
    

    if title.contains("DeviantArt: 404"){
        Ok(WebReturn::Failure("Account not Found.".to_string()))

    } else {

        let bio = tab.wait_for_xpath(
            "/html/body/div[1]/div[2]/div/div[2]/div[3]/div/div[2]/div[2]/div[1]/section").ok()
            .expect("Error scraping bio.");

        Ok(
            WebReturn::Success {
            username: (title.replace(" User Profile | DeviantArt", "")),
            url: (url),
            other: (bio.get_inner_text().ok().expect("Error unwrapping bio text.")) })
    }
}

fn main() {

    let args = env::args().nth(1)
        .expect("Input should be snooper <username>");

    let browser = load_browser().ok()
        .expect("Error assigning browser object.");

    let mut output:Vec<WebReturn> = Vec::new();

    output.push(parse_instagram(&browser, &args).ok().unwrap());
    output.push(parse_youtube(&browser, &args).ok().unwrap());
    output.push(parse_twitter(&browser, &args).ok().unwrap());
    output.push(parse_facebook(&browser, &args).ok().unwrap());
    output.push(parse_deviantart(&browser, &args).ok().unwrap());




    println!("{:?}", output);
}
