# Seeds Authenticator

The authenticator is the tool used as an API to authenticate blockchain users.

## Installation

The first step is to install rust. If you are using a **unix** based system you can use the following bash command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Otherwise you can check the specific documentation for your system in [Rust Up](https://rustup.rs).

## Usage
The first step to run this project is to duplicate the following file: **config/environment-example.toml** and rename the duplicated file to: **environment.toml**.

In this file are presents all the configurable variables used to determine limits, timeout, urls and paths of the project.

The next step is to open the terminal and do the following: 
```bash
cd path-to-project/seeds-authenticator

cargo run
```

## API
In the table below, all the available paths are explained:
(Replace *localhost:8080/* with the path set in the *environment.toml* file)

| PATH | METHOD | PARAMS | BODY | USAGE | RESPONSE |
| ---- | ------ | ------ | ---- | ----- | ----------- |
| /new | POST | None | **account_name** is the user account name | ```curl -X "POST" "http://localhost:8080/api/v1/new" -H 'Content-Type: application/json; charset=utf-8'  -d $'{"account_name": "account-name"}' ``` | Response contains: **id**, **account_name**, **token**, **valid_until**, **policy**, and **signature**. **id**, **policy** and **signature** need to be stored in the blockchain using the `create` action of the `policy.seeds` contract|
| /check | POST | None | **account_name** is the user account name, **token** is obtained in */new* | ```curl -X "POST" "http://localhost:8080/api/v1/check/<id-obtained-in-/new>" \ -H 'Content-Type: application/json; charset=utf-8' \ -d $'{ "account_name": "account-name", "token": "<token-obtained-in-/new>" }'``` | Response contains a string encapsulating the error if that's the case or a string **ok** if the sent data is valid.  |
| /invalidate | POST | None | **account_name** is the user account name, **token** is obtained in */new* | ```curl -X "POST" "http://localhost:8080/api/v1/invalidate/<id-obtained-in-/new>" \ -H 'Content-Type: application/json; charset=utf-8' \ -d $'{ "account_name": "account-name", "token": "<token-obtained-in-/new>" }'``` | Response contains a string encapsulating the error if that's the case or a string **ok** if the sent data is valid.  |


## License
[GPL](https://choosealicense.com/licenses/gpl-3.0/)
