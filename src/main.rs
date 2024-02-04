use std::{any, fs};
use serde_json::from_str;
use urlencoding::encode;
use std::io::{stdin,stdout,Write};

const HANDOUTS_PATH : &str = "./handouts.json";
const BASE_DOWNLOAD_URL :&str = "https://github.com/Divyateja04/handoutsforyou/raw/main/public/handouts";

// fn format_url(){

// }

#[tokio::main]
async fn main() {
    
    let handouts = fs::read_to_string(HANDOUTS_PATH).expect("Unable to read handouts json file");

    let handouts_json: serde_json::Value =
    from_str(&handouts).expect("The handouts JSON file was incorrectly read");

    println!("Please enter semester");

    
    let mut semester_input = String::new();
         stdin()
        .read_line(&mut semester_input)
        .expect("Failed to read line");

    let length = semester_input.len();
    semester_input.truncate(length-1);

    let handout = &handouts_json["handoutsMap"][&semester_input][0];
    print!("{}",handout);
    let uri_encoded_handout_name = encode(handout.as_str().unwrap());
    let file_download_url = format!("{}/{}/{}",BASE_DOWNLOAD_URL,&semester_input,uri_encoded_handout_name);
    println!("{}", file_download_url);
    make_request(&file_download_url).await;
}

async fn make_request(url: &str){
        let resp = reqwest::get(url).await.expect("request failed");
        let body = resp.bytes().await.expect("body invalid");
        std::fs::write("handout.docx", &body);
}


