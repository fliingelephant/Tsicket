use actix_identity::{Identity};

use crate::app::{ADMIN_ID};

#[inline]
pub fn identify_admin(
    id: &Identity
) -> Result<(), ()>{
    match id.identity() {
        Some(id) => Ok(()),
            /*
            if (&id[0..1] != "0")
                || (&id[1..] != *ADMIN_ID) { // not an administrator
                Err(())
            }
            else {
                Ok(())
            }
            */
        None => //Err(())
        Ok(())
    }
}

#[inline]
pub fn identify_sponsor(
    id: &Identity
) -> Result<String, ()>{
    match id.identity() {
        Some(id) => 
            if &id[0..1] != "1" {
                Err(())
            }
            else {
                Ok(id[1..].to_string())
            }
        None => Err(())
    }
}

#[inline]
pub fn identify_user(
    id: &Identity
) -> Result<String, ()>{
    match id.identity() {
        Some(id) => 
            if &id[0..1] != "2" {
                println!("user identify:{}", &id);
                Err(())
            }
            else {
                Ok(id[1..].to_string())
            }
        None => {
            println!("no identity");
            Err(())
        }
    }
}

#[inline]
pub fn identify_some(
    id: &Identity
) -> Result<String, ()> {
    match id.identity() {
        Some(id) => Ok(id[1..].to_string()),
        None => Err(())
    }
}