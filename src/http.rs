use consul;
use reqwest;
use log::{warn, debug, trace};
use rand::{thread_rng};
use rand::seq::SliceRandom;
use consul::health::Health;

pub fn get<S: Into<String>>(service: S, url: S, tag: Option<&str>) -> Result<reqwest::Result<reqwest::blocking::Response>, String>{
    /// method to quickly look up a service, select a random node, and perform a GET request returning
    /// the output

    let service: String = service.into();
    debug!("looking for service '{}'", &service);
    let config = consul::Config::new();
    if config.is_err() {
        return Err("".to_string());
    }
    let mut config = config.unwrap();
    let client = consul::Client::new(config);

    let services = client.service(service.clone().as_str(), tag, true, None);
    if services.is_err() {
        warn!("unable to load {} services. {:?}", &service, services.err().unwrap());
        return Err("".to_string());

    }

    let mut service_list = services.unwrap().0;
    service_list.shuffle(&mut thread_rng());
    debug!("discovered {} instances of {}", service_list.len(), &service);

    let service = &service_list[0].Service;
    let address = format!("http://{}:{}", &service.Address, &service.Port);
    debug!("selected {} as target node", &address);

    let target = format!("{}{}", address, url.into());
    return Ok(reqwest::blocking::get(&target))

}