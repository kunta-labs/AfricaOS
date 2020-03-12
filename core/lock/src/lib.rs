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

extern crate fs2;

use fs2::FileExt;
use std::io::Result;
use std::env::args;
use std::fs::File;
use std::io::{Write, Error};
use std::time::Duration;
use std::thread::sleep;

pub struct Locker {}

pub trait FileLockWrite {
    fn write(content: String, location: String) -> Result<()>;
}

impl FileLockWrite for Locker {
    fn write(content: String, location: String) -> Result<()> {
        let sleep_seconds = 0;
        let sleep_duration = Duration::from_secs(sleep_seconds);
        let file_open: Result<File> = File::open(location.to_string());
        match file_open {
            Ok(_) => {
                let mut file_o: File = File::open(location.to_string())?;
                //let mut file: File = File::open(location.to_string())?;
                println!("{}: Preparing to lock file.", sleep_seconds);
                file_o.lock_exclusive()?; // block until this process can lock the file
                let mut file = File::create(location.to_string())?;
                println!("{}: Obtained lock.", sleep_seconds);
                //sleep(sleep_duration);
                file.write( content.as_bytes() )?;
                println!("{}: Sleep completed", sleep_seconds);
                file_o.unlock()?;
                println!("{}: Released lock, returning", sleep_seconds);
                Ok(())
            },
            Err(_) => {
                //let file_open: File = File::open(location.to_string())?;
                let mut file = File::create(location.to_string())?;
                println!("{}: Preparing to lock file.", sleep_seconds);
                file.lock_exclusive()?; // block until this process can lock the file
                //let mut file = File::create(location.to_string())?;
                println!("{}: Obtained lock.", sleep_seconds);
                //sleep(sleep_duration);
                file.write( content.as_bytes() )?;
                println!("{}: Sleep completed", sleep_seconds);
                file.unlock()?;
                println!("{}: Released lock, returning", sleep_seconds);
                Ok(())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Locker, FileLockWrite};

    #[test]
    fn test_file_write(){
        let test_location: String = String::from("./LOCKTEST");
        let test_content: String = String::from("TEST CONTENT");
        let file_location: String = format!("{}", test_location);
        //let mut file = fs::File::create(file_location.to_string())?;
        //Locker::write(test_content, file_location), Ok(())
        //use std::io::Error;
        let file_lock_write_result: Result<(), std::io::Error> = Locker::write(test_content,file_location);
        assert!( file_lock_write_result.is_ok() );
    }
}
