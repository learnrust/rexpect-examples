// WHAT:
//  Using rexpect and ldap3 crates to check ldap:389 connection.
//
// HOW:
//  1. To run: cargo run --bin forumsys_ldap_check  -- -c -l ldap://ldap.forumsys.com:389 -a read-only-admin -p password
//
// Reference:
//  1. https://www.forumsys.com/tutorials/integration-how-to/ldap/online-ldap-test-server/
//  2. https://github.com/rust-cli/rexpect
//  TODOs
//  1. using latest clap crate.

extern crate clap;
extern crate ldap3;

use clap::{Parser, Subcommand};
use ldap3::LdapConn;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
    /// Testing ldap:// connection
    Test { name: Option<String> },
}

pub static LDAP_SERVER: &str = "ldap://ldap.forumsys.com:389";
pub static LDAP_SERVICE_USER_DN: &str = "cn=read-only-admin,dc=example,dc=com";
pub static LDAP_SERVICE_USER_PW: &str = "password";

fn do_tls_conn(ldapserver: &str, account: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let local_ldapserver = &ldapserver;
    let local_account: &str = account;
    let cn_prefix = "cn=";
    let ldapserver_ou = ",dc=example,dc=com";
    let dn = [cn_prefix, ldapserver_ou].join(local_account);
    let local_password: &str = password;
    let mut ldap = LdapConn::new(local_ldapserver)?;
    println!("Will connect to {Server}:{DN} using password:{PW}",Server=ldapserver,DN=&dn,PW=local_password);
    ldap.simple_bind(&dn, local_password)?.success()?;
    Ok(())
}


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd

    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
        Commands::Test { name } => {
            println!("Connecting  to {Server}:{DN} using password:{PW}",
                    Server = LDAP_SERVER,
                    DN = LDAP_SERVICE_USER_DN,
                    PW = LDAP_SERVICE_USER_PW
                );
            match do_tls_conn(LDAP_SERVER,LDAP_SERVICE_USER_DN,LDAP_SERVICE_USER_PW){
                //case workflow.
                Ok(_) => println!(
                    "OK: Connected to {Server}:{DN} using password:{PW}",
                    Server = LDAP_SERVER,
                    DN = LDAP_SERVICE_USER_DN,
                    PW = LDAP_SERVICE_USER_PW
                ),
            Err(e) => println!("{:?}", e),
            } 
            println!("'myapp Test' was used, name is: {:?}", name)
        }
    }
}
