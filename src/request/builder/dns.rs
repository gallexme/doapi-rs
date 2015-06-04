use std::fmt;
use std::marker::PhantomData;

use hyper::method::Method;
use serde::json;

use response;
use request::RequestBuilder;
use request::DoRequest;

/// Lists the types of supported DNS records
doapi_enum! {
    #[derive(Debug)]
    pub enum DnsRecType {
        A,
        AAAA,
        CNAME,
        MX,
        NS,
        SRV,
        TXT
    }
}

// type     string  The record type (A, MX, CNAME, etc).    All Records
// name     string  The host name, alias, or service being defined by the record.   A, AAAA, CNAME, TXT, SRV
// data     string  Variable data depending on record type. See the [Domain Records]() section for more detail on each record type. A, AAAA, CNAME, MX, TXT, SRV, NS
// priority    nullable number The priority of the host (for SRV and MX records. null otherwise).  MX, SRV
// port     nullable number The port that the service is accessible on (for SRV records only. null otherwise).  SRV
// weight   nullable number The weight of records with the same priority (for SRV records only. null otherwise).    SRV
/// A struct for creating a DNS Record
#[derive(Serialize)]
pub struct DnsRecord {
    /// The type of record (A, AAAA, MX, NS, etc.)
    ///
    /// **NOTE:** You can use the `DnsRecType`'s implementation of `std::fmt::Display` to get a 
    /// `String`
    #[serde(rename = "type")]
    pub rec_type: String,
    /// The name of the record (Required for: A, AAAA, CNAME, TXT, and SRV)
    pub name: Option<String>,
    /// The priority of the record (Required for: MX and SRV)
    pub priority: Option<u64>,
    /// The port of the record (Required for: SRV)
    pub port: Option<u64>,
    /// Various data used for record (Required for: A, AAAA, CNAME, MX, TXT, SRV, NS)
    pub data: Option<String>,
    /// The weight of the record (Required for: SRV)
    pub weight: Option<u64>,
}

impl fmt::Display for DnsRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Record Type: {}\n\
             Name: {}\n\
             Data: {}\n\
             Priority: {}\n\
             Port: {}\n\
             Weight: {}\n",
             self.rec_type,
             if let Some(n) = self.name.clone() {
                n
             } else {
                "None".to_owned()
             },
             if let Some(d) = self.data.clone() {
                d
             } else {
                "None".to_owned()
             },
             if let Some(p) = self.priority {
                p.to_string()
             } else {
                "None".to_owned()
             },
             if let Some(p) = self.port {
                p.to_string()
             } else {
                "None".to_owned()
             },
             if let Some(w) = self.weight {
                w.to_string()
             } else {
                "None".to_owned()
             }
        )
    }
}

/// A type of `RequestBuilder` which allows you make requests related to a single DNS record
///
/// # Example
///
/// ```no_run
/// # use doapi::DoManager;
/// # use doapi::DoRequest;
/// let domgr = DoManager::with_token("<token>");
/// let dnsrecord_request = domgr.domain("super.com").dns_record("<record id>");
/// ```
pub type DnsRecordRequest<'t> = RequestBuilder<'t, response::DnsRecord>;
impl<'t> DoRequest<response::DnsRecord> for DnsRecordRequest<'t> {}

impl<'t> DnsRecordsRequest<'t> {
    /// Returns a `RequestBuilder` for creating a DNS record. 
    ///
    /// **Parameters:**
    /// `record`: The instance of `DnsRecord` you'd like to create 
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # use doapi::request::DnsRecord;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// # let record = DnsRecord {
    /// #   rec_type: "A".to_owned(),
    /// #   name: None,
    /// #   priority: None,
    /// #   port: None,
    /// #   data: None,
    /// #   weight: None,
    /// # };
    /// // ... domgr set up same as before
    /// // ... assumes "record" is an instance of doapi::request::DnsRecord
    /// match domgr.domain("super.com")
    ///            .dns_records()
    ///            .create(&record)
    ///            .retrieve() {
    ///     Ok(dns_rec) => println!("Record: {}", dns_rec),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn create(self, record: &DnsRecord) -> DnsRecordRequest<'t> {
        // POST: "https://api.digitalocean.com/v2/domains/$DOMAIN/records"
        // body:
        //      "type" : "MX"            All records
        //      "name" : "alias"         A, AAAA, CNAME, TXT, SRV
        //      "data" : "varies"        A, AAAA, CNAME, MX, TXT, SRV, NS
        //      "priority" : 20          MX, SRV
        //      "port" : 80              SRV
        //      "weight" : 200           SRV

        // FIXME: Don't unwrap()
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(record).ok().unwrap())
        }
    }
}

/// A type of `RequestBuilder` which allows you make requests related to multiple DNS records or
/// the concept of "DNS Records" as a whole
///
/// # Example
///
/// ```no_run
/// # use doapi::DoManager;
/// # use doapi::DoRequest;
/// let domgr = DoManager::with_token("<token>");
/// let dnsrecords_request = domgr.domain("super.com").dns_records();
/// ```
pub type DnsRecordsRequest<'t> = RequestBuilder<'t, response::DnsRecords>;

impl<'t> DnsRecordsRequest<'t> {
    /// Returns a `RequestBuilder` for updating an existing DNS record. 
    ///
    /// **Parameters:**
    /// `record`: The new instance of `DnsRecord` you'd like to update to
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # use doapi::request::DnsRecord;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// # let record = DnsRecord {
    /// #   rec_type: "A".to_owned(),
    /// #   name: None,
    /// #   priority: None,
    /// #   port: None,
    /// #   data: None,
    /// #   weight: None,
    /// # };
    /// // ... domgr set up same as before
    /// // ... assumes "record" is an instance of doapi::request::DnsRecord
    /// match domgr.domain("super.com")
    ///            .dns_record("1234")
    ///            .update(&record)
    ///            .retrieve() {
    ///     Ok(dns_rec) => println!("Record: {}", dns_rec),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn update(self, record: &DnsRecord) -> DnsRecordRequest<'t> {
        // PUT: "https://api.digitalocean.com/v2/domains/$DOMAIN/records/$ID"
        // body:
        //      "type" : "MX"           All records
        //      "name" : "alias"        A, AAAA, CNAME, TXT, SRV
        //      "data" : "varies"       A, AAAA, CNAME, MX, TXT, SRV, NS
        //      "priority" : 20         MX, SRV
        //      "port" : 80             SRV
        //      "weight" : 200          SRV
        // FIXME: Don't unwrap()
        RequestBuilder {
            method: Method::Put,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(record).ok().unwrap())
        }
    }

    /// Returns a `RequestBuilder` for deleting an existing DNS record. 
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.domain("super.com")
    ///            .dns_record("1234")
    ///            .delete()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn delete(self) -> RequestBuilder<'t, response::HeaderOnly> {
        // DELETE: "https://api.digitalocean.com/v2/domains/$id"
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None
        }
    }
}
