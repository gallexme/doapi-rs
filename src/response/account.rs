
use std::fmt;

// TODO: Implement response headers:
// content-type: application/json; charset=utf-8
// status: 200 OK
// ratelimit-limit: 1200
// ratelimit-remaining: 1137
// ratelimit-reset: 1415984218

#[derive(Deserialize, Debug)]
pub struct Account {
    /// droplet_limit is a "number" in json, which could be a float, even thought that's not a
    /// reasonable value for a droplet limit, neither is a negative number
    pub droplet_limit: f64,
    pub email: String,
    pub uuid: String,
    pub email_verified: bool
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DigitalOcean Account:\n\t\
                        Email: {}\n\t\
                        Droplet Limit: {:.0}\n\t\
                        UUID: {}\n\t\
                        E-Mail Verified: {}",
                self.email,
                self.droplet_limit,
                self.uuid,
                self.email_verified)
    }
}