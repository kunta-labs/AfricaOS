/*
Copyright 2018-Present The AfricaOS Authors
This file is part of the AfricaOS library.
The AfricaOS Platform is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
The AfricaOS Platform is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with the AfricaOS Platform. If not, see <http://www.gnu.org/licenses/>.
*/

extern crate chrono;
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use chrono::format::ParseError;

#[derive(Clone,Debug,PartialEq)]
pub struct Timestamp {
    pub timestamp: String
}

/*
    @name NewTimestamp
    @desc create a new timestamp right now
*/
pub trait NewTimestamp {
    fn new() -> Option<Timestamp>;
}

impl NewTimestamp for Timestamp {
    fn new() -> Option<Timestamp> {
        let now: DateTime<Utc> = Utc::now();
        let timestamp_string: String = format!("{:?}", now.timestamp()).to_string();
        let new_timestamp: Timestamp = Timestamp {
            timestamp: timestamp_string
        };
        Some(new_timestamp)
    }
}

/*
@name BlockFromString
@desc convert a string to a block, optionally
*/
pub trait StringToTimestamp {
    fn string_to_timestamp(timestamp_string: String) -> Option<Timestamp>;
}

impl StringToTimestamp for Timestamp {
    fn string_to_timestamp(timestamp_string: String) -> Option<Timestamp> {
        let int_timestamp: Result<i64, std::num::ParseIntError> =  timestamp_string.parse::<i64>();
        if int_timestamp.is_ok() {
            let timestamp_from_i64: DateTime::<Utc> = Utc.timestamp(int_timestamp.unwrap(), 0);
            let timestamp_string: String = format!("{:?}", timestamp_from_i64.timestamp()).to_string();
            let new_timestamp: Timestamp = Timestamp {
                timestamp: timestamp_string
            };
            Some(new_timestamp)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Timestamp, NewTimestamp, StringToTimestamp};
    use std::time::{Duration, SystemTime};
    use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

    #[test]
    fn test_to_string() {
        let now = SystemTime::now();
        let expected_timestamp: &str = "test timestamp";
        let timestamp: Timestamp = Timestamp {
            timestamp: String::from(expected_timestamp)
        };
        assert_eq!(timestamp.timestamp,
                   String::from(expected_timestamp));
    }

    #[test]
    fn test_create_new_timestamp_issome() {
        let new_timestamp: Option<Timestamp> = Timestamp::new();
        assert!(new_timestamp.is_some())
    }

    #[test]
    fn test_from_string() {
        let timestamp = String::from("1441497364");
        let expected_timestamp: Option<Timestamp> = Timestamp::string_to_timestamp(timestamp.clone());
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        let static_timestamp: DateTime::<Utc> = Utc.timestamp(62, 0);
        println!("dt: {:?}", dt);
        println!("static_timestamp {:?}", static_timestamp);
        println!("1441497364: {}", Utc.timestamp(1441497364, 0).timestamp());
        assert_eq!(expected_timestamp.unwrap().timestamp, timestamp);
    }

}
