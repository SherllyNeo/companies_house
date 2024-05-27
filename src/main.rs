use anyhow::{Result,Error,Context};
use reqwest::blocking::Client;
use std::io::stdin;
use serde_json::Value;
use base64::prelude::*;

fn get_user_input() -> Result<String> {
    // helper function to take user input and pop it into the buffer
    let mut buffer = String::new();
    println!("Please enter a name: ");
    stdin().read_line(&mut buffer).context("unable to read user input")?;

    // this returns either a string, or an error
    Ok(buffer.trim().to_owned())
}

fn get_reqest(url: &str, api_key: &str) -> Result<Value> {
    // helper function to get stuff
    
    // companies house is weird for this one boys
    let username = api_key;
    let password = "";

    // most APIs don't use this anymore in favour of more secure methods. 
    // You usually do a get request with a .header("authorization",api_key) to a special url to get
    // a temporary token, then use that token for future requests until it times out

    // for this weird one we base64 encode and use that to make an api key ish thing
    let credentials = format!("{username}:{password}");
    let encoded_credentials = format!("basic {}",BASE64_STANDARD.encode(credentials));

    // renaming so it looks more like 'normal' 
    let api_key_proper = encoded_credentials;

    // make a new client, with a get request, add a header and send the request. usually you'd want
    // to reuse a client over and over for speed
    let response = Client::new()
        .get(url)
        .header("authorization",api_key_proper)
        .send()?;

    // store the response
    let status = response.status();

    // check if it worked or not
    if status.is_success() {
        // get response as json
        let response_json: Value = response
            .json()
            .context("Failed to deserialize response JSON")?;

        // Get the top hit
        let top_hit = response_json
            .get("top_hit")
            .context("Failed to find 'top_hit' in response")?;
        
        // return top hit
        Ok(top_hit.to_owned())
    }
    else {
        // if failed, return an error
        let error_text = format!("unable to get data, status {}",status);
        Err(Error::msg(error_text))
    }
}


fn main() -> Result<()> {
    // We use the anyhow crate to return generic errors from this with Context
    // This means we can ignore complex error handling 
    // The ? operator checks if something failed and returns an error from the function is so
    // You can also be verbose and use match statements as shown by User input

    // get api key from envrioment, source .env if you need to
    let api_key = std::env::var("COMPANIES_HOUSE_API_KEY").context("No api key")?;

    // long way, this is like using get_user_input()?;
    let input = match get_user_input() {
        Ok(user_input) => user_input,
        Err(err) =>  return Err(Error::msg(format!("There was an error getting user input: {err:?}")))
    };

    // define a base url
    let base_search_url = "https://api.company-information.service.gov.uk/advanced-search/companies";


    // combine the base search url and user input to make the url proper
    let search_url = format!("{base_search_url}?company_name_includes={input}");

    // send the request, we use & to pass a reference, like the addresses in C. As this takes  &str
    // rather than String. This is for performance reasons, take &str if you are only reading values
    let company_data = get_reqest(&search_url, &api_key)?;
    
    // turn the company data into a pretty format, not required
    let pretty_data = serde_json::to_string_pretty(&company_data)?;


    // print out stuff
    println!("you typed \"{input}\" ");
    println!("It returned \n\"{pretty_data}\" ");
    Ok(())
}
