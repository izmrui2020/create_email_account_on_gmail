//
use anyhow::Result;
use tokio::net::tcp::ReuniteError;
use crate::Opt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    id: String,
    email: String,
    pass: String,
}

pub struct HttpModule {
    pub opt: Opt,
    pub user_store: HashMap<String, User>,
}

impl HttpModule {
    pub fn new(opt: Opt) -> Self {
        Self { 
            opt,
            user_store: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {        
        let url = "POST https://admin.googleapis.com/admin/directory/v1/users";

        for id in 1..=30 {
            let user_id = format!("User{}", id);

            self.user_store.insert(
                user_id.to_string(),
                User::default()
            );

            
        }
        
        Ok(())
    }

    pub fn print_user(&mut self) -> Result<()> {
        let mut wtr = csv::Writer::from_path(self.opt.file_path.clone()).unwrap();

        wtr.write_record(&["id", "email", "pass"])?;

        for (_id, user) in self.user_store.iter() {
            wtr.serialize(user).unwrap();
        }

        Ok(())
    }
}