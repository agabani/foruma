use maxminddb::{geoip2::City, MaxMindDBError, Reader};
use std::{net::IpAddr, string::ToString};

pub struct GeoIp(Reader<Vec<u8>>);

#[derive(Debug, PartialEq)]
pub struct GeoIpLookup {
    // city
    city: Option<String>,
    // continent
    continent: Option<String>,
    continent_code: Option<String>,
    // country
    country: Option<String>,
    country_iso_code: Option<String>,
    // location
    longitude: Option<f64>,
    latitude: Option<f64>,
    time_zone: Option<String>,
    // postal
    postal: Option<String>,
    // subdivisions
    subdivisions: Option<Vec<String>>,
    subdivisions_iso_code: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum LookupErrors {
    AddressNotFound,
}

impl GeoIp {
    pub fn new(path: &str) -> Result<Self, ()> {
        let reader = match Reader::open_readfile(path) {
            Ok(reader) => reader,
            Err(error) => todo!("{}", error),
        };

        Ok(Self(reader))
    }

    pub fn lookup(
        &self,
        ip_address: &IpAddr,
        language_code: &str,
    ) -> Result<GeoIpLookup, LookupErrors> {
        let result = &self.0.lookup::<City>(*ip_address).map_err(|x| match x {
            MaxMindDBError::AddressNotFoundError(_) => LookupErrors::AddressNotFound,
            x => todo!("{}", x),
        })?;

        // city
        let city = result
            .city
            .as_ref()
            .and_then(|city| city.names.as_ref())
            .and_then(|names| names.get(language_code))
            .map(|&value| value.to_string());

        // continent
        let continent = result
            .continent
            .as_ref()
            .and_then(|continent| continent.names.as_ref())
            .and_then(|names| names.get(language_code))
            .map(|&value| value.to_string());

        let continent_code = result
            .continent
            .as_ref()
            .and_then(|continent| continent.code.as_ref())
            .map(|&value| value.to_string());

        // country
        let country = result
            .country
            .as_ref()
            .and_then(|country| country.names.as_ref())
            .and_then(|names| names.get(language_code))
            .map(|&value| value.to_string());

        let country_iso_code = result
            .country
            .as_ref()
            .and_then(|country| country.iso_code.as_ref())
            .map(|&value| value.to_string());

        // location
        let longitude = result
            .location
            .as_ref()
            .and_then(|location| location.longitude);

        let latitude = result
            .location
            .as_ref()
            .and_then(|location| location.latitude);

        let time_zone = result
            .location
            .as_ref()
            .and_then(|location| location.time_zone)
            .map(ToString::to_string);

        // postal
        let postal = result
            .postal
            .as_ref()
            .and_then(|postal| postal.code)
            .map(ToString::to_string);

        // subdivisions
        let subdivisions = result.subdivisions.as_ref().map(|v| {
            v.iter()
                .filter_map(|s| s.names.as_ref())
                .filter_map(|names| names.get(language_code).map(ToString::to_string))
                .collect::<Vec<_>>()
        });

        let subdivisions_iso_code = result.subdivisions.as_ref().map(|v| {
            v.iter()
                .filter_map(|s| s.iso_code)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        });

        Ok(GeoIpLookup {
            city,
            continent,
            continent_code,
            country,
            country_iso_code,
            longitude,
            latitude,
            time_zone,
            postal,
            subdivisions,
            subdivisions_iso_code,
        })
    }
}

impl GeoIpLookup {
    pub fn to_human_readable(&self) -> String {
        let mut parts: Vec<&str> = vec![];

        if let Some(city) = &self.city {
            parts.push(city);
        }

        if let Some(country) = &self.country {
            parts.push(country);
        }

        parts.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn ipv4() {
        let geo_ip = new();

        let result = geo_ip
            .lookup(&IpAddr::V4(Ipv4Addr::new(31, 52, 52, 128)), "en")
            .unwrap();

        assert_eq!(
            result,
            GeoIpLookup {
                city: Some("City of Westminster".to_string()),
                continent: Some("Europe".to_string()),
                continent_code: Some("EU".to_string()),
                country: Some("United Kingdom".to_string()),
                country_iso_code: Some("GB".to_string()),
                longitude: Some(-0.1196),
                latitude: Some(51.5074),
                time_zone: Some("Europe/London".to_string()),
                postal: Some("W1W".to_string()),
                subdivisions: Some(vec![
                    "England".to_string(),
                    "City of Westminster".to_string()
                ]),
                subdivisions_iso_code: Some(vec!["ENG".to_string(), "WSM".to_string()])
            }
        );

        assert_eq!(
            "City of Westminster, United Kingdom",
            result.to_human_readable()
        );
    }

    #[test]
    fn ipv6() {
        let geo_ip = new();

        let result = geo_ip
            .lookup(
                &IpAddr::V6(Ipv6Addr::new(
                    0x2a00, 0x23c4, 0x5000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                )),
                "en",
            )
            .unwrap();

        assert_eq!(
            result,
            GeoIpLookup {
                city: Some("City of Westminster".to_string()),
                continent: Some("Europe".to_string()),
                continent_code: Some("EU".to_string()),
                country: Some("United Kingdom".to_string()),
                country_iso_code: Some("GB".to_string()),
                longitude: Some(-0.1196),
                latitude: Some(51.5074),
                time_zone: Some("Europe/London".to_string()),
                postal: Some("W1W".to_string()),
                subdivisions: Some(vec![
                    "England".to_string(),
                    "City of Westminster".to_string()
                ]),
                subdivisions_iso_code: Some(vec!["ENG".to_string(), "WSM".to_string()])
            }
        );

        assert_eq!(
            "City of Westminster, United Kingdom",
            result.to_human_readable()
        );
    }

    fn new() -> GeoIp {
        GeoIp::new(&"../geoip").unwrap()
    }
}
