# notifier
A notification system that allows you to send notifications to your users on multiple channels, written in rust
## Supported channels
- Email
- Telegram

## Configuration
The following fields can be set in the configuration:
```yaml
#connection string to a postgres database to store notification targets for each user in
connection_string: postgresql://postgres:passwd@localhost
services:
  telegram: 
   token: "my-telegram-bot-token"
  email:
    smtp_relay: "my.smtp.relay.com"
    username: "my.email.username"
    password: "my-password"
    sender: "my.email.username@my-domain.com"
```
The config is loaded by the [config](https://docs.rs/config/latest/config/#) crate and the fields can be loaded from json and yaml files named `config.*` located in the working directory or in the root directory.
## Deployment
Using docker:
```bash
docker run -v $(pwd)/config.yaml:/config.yaml -p 3000:3000 olepetersen/notifier:0.2.0
```