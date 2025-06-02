mod webserver;

use webserver::server_initiasise;
use database::conn::start_database;

// Import all the files and directories
pub mod pages {
    pub mod index;
    pub mod login;
    pub mod dashboard;
}
pub mod database {
    pub mod conn;
}
pub mod jwt {
    pub mod validate_token;
    pub mod manage_req;
}
pub mod posts;
pub mod add_img;
pub mod req_manager;
pub mod fetch_imgs;

fn main() {
    // The port the website runs on
    const PORT: &str = "8080";

    // Starting the database
    start_database().unwrap();
    // Starting the webserver
    server_initiasise(&PORT)
}

