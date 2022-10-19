# rexpect-examples

```
[me@ipa01 rexpect-examples]$ make build
<snipped>
[me@ipa01 rexpect-examples]$ target/debug/forumsys_ldap_check -h
ldapcheck 0.1
T.J. Yang <tjyang2001@gmail.com>
LDAP programming using Rust Language.
Ex1: cargo run --bin forumsys_ldap_check  -- -c -l ldap://ldap.forumsys.com:389 -a read-only-admin -p password.
Ex2: forumsys_ldap_check --check -l ldap://ldap.forumsys.com:389 -a read-only-admin -p password

USAGE:
    forumsys_ldap_check [FLAGS] [OPTIONS] [INPUT] [SUBCOMMAND]

FLAGS:
    -c, --check      check ldap password
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --account <Account name for LDAP server>                         ldap account name
    -l, --ldapurl <ldap server URL, Ex: ldap://ldap.forumsys.com:389>    ldap://ldap.forumsys.com:389
    -p, --password <Passord for LDAP Server>                             ldap password

ARGS:
    <INPUT>    Sets the input file to use

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    controls testing features
[me@ipa01 rexpect-examples]$
```