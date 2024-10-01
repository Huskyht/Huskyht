use reqwest::{Url, ClientBuilder, Client, RequestBuilder, Response, cookie::Jar, header::{self, HeaderMap, HeaderName, HeaderValue, USER_AGENT}};
use std::{collections::HashMap, sync::Arc, time::Duration};

use std::fs::File;
use std::io::{Write, BufWriter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let url = Url::parse("https://www.danielloboutique.it/jp").unwrap();
    //let url = Url::parse("https://stackoverflow.com/questions/77862683/rust-reqwest-cant-make-a-request").unwrap();

    // create default headers
    let mut default_headers: HeaderMap = HeaderMap::new();
    let default_user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36";
    //default_headers.insert(header::ACCEPT_ENCODING, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"));
    default_headers.insert(header::ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    default_headers.insert(USER_AGENT, default_user_agent.parse().unwrap());

    // create cookies
    let cookie_str = "cookie1=foo; cookie2=bar";
    let cookies = Arc::new(Jar::default());
    cookies.add_cookie_str(cookie_str, &url);

    // get client builder and gen client
    let client_builder: ClientBuilder = Client::builder(); // also can use ClientBuilder::new()
    let client: Client = client_builder
        .default_headers(default_headers)
        .cookie_provider(cookies)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    // create onetime headers, and else
    let mut onetime_headers: HeaderMap = HeaderMap::new();
    onetime_headers.insert(HeaderName::try_from("Custom-Header").unwrap(), HeaderValue::from_static("header value"));
    let body = r#"{ "body_value": 1 }"#;
    let queries = HashMap::from([("q1".to_string(),"1".to_string()),("q2".to_string(),"2".to_string())]);

    // get request builder and send request
    let request_builder: RequestBuilder = client.get(url); // also can use get, or else
    let mut response: Response = request_builder
        .headers(onetime_headers)
        .body(body)
        .query(&queries)
        .send()
        .await
        .unwrap();

    // get response as string
    let res_str =  match response.headers().get(header::TRANSFER_ENCODING) {
        Some(v) if v == "chunked" => {
            let mut raw_res = Vec::new();
            while let Some(chunk) = response.chunk().await.unwrap() {
                chunk
                    .to_vec()
                    .into_iter()
                    .for_each(|x| raw_res.push(x));
            }
            String::from_utf8(raw_res).unwrap()
        },
        _ => response.text().await.unwrap()
    };

    //println!("Response is : \n{}", res_str);
    let file_name_str = "response9.html";
    output_file(file_name_str, &res_str);
}

fn output_file(file_name: &str, insert_data: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let file_full_path = String::from("~/huskyProjects/rust/scrape_net_code/") + file_name;
    //println!("File name is :\n{}", file_full_path);
    let  file = File::create(file_name)?;
    let mut buff_file = BufWriter::new(file);
    //write!(buff_file, "{}", insert_data.as_bytes())?;
    buff_file.write_all(insert_data.as_bytes()).expect("Unable to write data");
    //file.write_all(insert_data.as_bytes()).expect("Unable to write data");
    buff_file.flush().unwrap();
    Ok(())


}









