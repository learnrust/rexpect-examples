// WHAT:
//   ldap:389 connection check using public ldap.forumsys.com
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

use clap::{App, Arg, SubCommand};
use ldap3::LdapConn;
use std::error::Error;

pub static LDAP_SERVER: &str = "ldap://ldap.forumsys.com:389";
pub static LDAP_SERVICE_USER_DN: &str = "cn=read-only-admin,dc=example,dc=com";
pub static LDAP_SERVICE_USER_PW: &str = "passwordbad";

fn do_tls_conn(ldapserver: &str, account: &str, password: &str) -> Result<(), Box<dyn Error>> {
    //println!("Will connect to {Server}:{DN} using password:{PW}",Server=ldapserver,DN=ldapserver_dn,PW=ldappassword);
    let local_ldapserver = &ldapserver;
    let local_account: &str = account;
    let cn_prefix = "cn=";
    let ldapserver_ou = ",dc=example,dc=com";
    let dn = [cn_prefix, ldapserver_ou].join(local_account);
    let local_password: &str = password;
    let mut ldap = LdapConn::new(local_ldapserver)?;
    ldap.simple_bind(&dn, local_password)?.success()?;
    Ok(())
}

fn main() {
    let matches = App::new("ldapcheck")
        .version("0.1")
        .author("T.J. Yang <tjyang2001@gmail.com>")
        .about("LDAP programming using Rust Language.\nEx1: cargo run --bin forumsys_ldap_check  -- -c -l ldap://ldap.forumsys.com:389 -a read-only-admin -p password.\nEx2: forumsys_ldap_check --check -l ldap://ldap.forumsys.com:389 -a read-only-admin -p password")
        .arg(
            Arg::with_name("account")
                .short("a")
                .long("account")
                .value_name("Account name for LDAP server")
                .takes_value(true)
                .help("ldap account name"),
        ).arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("Passord for LDAP Server")
                .takes_value(true)
                .help("ldap password"),
        ).arg(
            Arg::with_name("ldapurl")
                .short("l")
                .long("ldapurl")
                .value_name("ldap server URL, Ex: ldap://ldap.forumsys.com:389")
                .takes_value(true)
                .help("ldap://ldap.forumsys.com:389"),
        ).arg(
            Arg::with_name("check")
                .short("c")
                .long("check")
                .help("check ldap password"),
        ).arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        ).subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("T.J. Yang<tjyang2001@gmail.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        ).get_matches();

    let ldapserver = matches.value_of("ldapurl").unwrap();
    let account = matches.value_of("account").unwrap();
    let cn_prefix = "cn=";
    let ldapserver_ou = ",dc=example,dc=com";
    let ldapserver_dn = [cn_prefix, ldapserver_ou].join(account);
    let ldappassword = matches.value_of("password").unwrap();

    // Main

   if matches.is_present("check") {
        match do_tls_conn(ldapserver, account, ldappassword) {
            //case workflow.
            Ok(_) => println!(
                "OK: Connected to {Server}:{DN} using password:{PW}",
                Server = ldapserver,
                DN = ldapserver_dn,
                PW = ldappassword
            ),
            Err(e) => println!("{:?}", e),
        }
    } else {
        // TBC.
    }
}
