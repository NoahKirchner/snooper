use std::env;


mod parser;
use parser::*;



/* ===================== */



fn main() {

    /*
    let instagram = PageInfo {
        site_name: "Instagram".to_string(),
        url: "https://www.instagram.com/{}/".to_string(),
        bio_url: "https://www.instagram.com/{}/".to_string(),
        title_match: vec!["Page not found".to_string()],
        title_trim: " • Instagram photos and videos".to_string(),
        xpath: "//div[contains(@class, '_aa_c')]".to_string()

    };
    let facebook = PageInfo {
        site_name: "facebook".to_string(),
        url: "https://www.facebook.com/{}".to_string(),
        bio_url: "https://www.facebook.com/{}".to_string(),
        title_match: vec!["Page Not Found".to_string()],
        title_trim: " | Facebook".to_string(),
        xpath: "/html/body/div[1]/div/div[1]/div/div[3]/div/div/div/div[1]/div[1]/div/div/div[4]/div[2]/div/div[1]/div/div[1]/div/div/div[2]".to_string()

    };
    let youtube = PageInfo {
        site_name: "youtube".to_string(),
        url: "https://www.youtube.com/user/{}/".to_string(),
        bio_url: "https://www.youtube.com/user/{}/about".to_string(),
        title_match: vec!["404 Not Found".to_string()],
        title_trim: " - YouTube".to_string(),
        xpath: "//*[@id='description-container']".to_string()

    };
    let deviantart = PageInfo {
        site_name: "deviantart".to_string(),
        url: "https://www.deviantart.com/{}".to_string(),
        bio_url: "https://www.deviantart.com/{}".to_string(),
        title_match: vec!["DeviantArt: 404".to_string()],
        title_trim: " User Profile | DeviantArt".to_string(),
        xpath: "/html/body/div[1]/div[2]/div/div[2]/div[3]/div/div[2]/div[2]/div[1]/section".to_string()

    };

    let twitter = PageInfo {
        site_name: "twitter".to_string(),
        url: "https://www.twitter.com/{}".to_string(),
        bio_url: "https://www.twitter.com/{}".to_string(),
        title_match: vec!["Profile / Twitter".to_string(), "Page not found".to_string()],
        title_trim: " / Twitter".to_string(),
        xpath: "/html/body/div[1]/div/div/div[2]/main/div/div/div/div[1]/div/div[2]/div/div/div".to_string()

    };

    let args = env::args().nth(1)
        .expect("Input should be snooper <username>");

    let browser = load_browser().ok()
        .expect("Error assigning browser object.");

    let mut output:Vec<WebReturn> = Vec::new();

    output.push(parse_page(&browser, &args, instagram).ok().unwrap());
    output.push(parse_page(&browser, &args, twitter).ok().unwrap());
    output.push(parse_page(&browser, &args, deviantart).ok().unwrap());
    output.push(parse_page(&browser, &args, facebook).ok().unwrap());
    output.push(parse_page(&browser, &args, youtube).ok().unwrap());



    println!("{:?}", output); */ */
    targets_from_yaml("targets.yaml".to_string());
}
