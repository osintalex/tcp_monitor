# Overview

This is just a super basic application for checking tcp
on ipv4 and ipv6 addresses on any port.

You can use it to send yourself email alerts if you have set up a sendgrid account.

Mostly though it is just a very basic project I wrote to try and get more familiar
with Rust.

## Usage

Settings in repo root should look like this:

```toml
ipv4 = "8.8.8.8:443"
ipv6 = "[2001:4860:4860::8888]:443"
ipv6_enabled = true
sender_email = "example@hello.com"
sender_name = "example"
recipient_email ="example@hello.com"
recipient_name = "example"
sendgrid_apikey = "blahdeblah"
email_enabled = false
how_many_failures_before_send_email = 3
seconds_between_checks = 3600
```

Then:

```shell
cargo clippy
cargo fmt
cargo run
```
