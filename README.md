# FAMILIES LINEAGE FOR MAESTERS

Dear maesters, here is a little project to help you determine families
lineage.

For this purpose, you have to install the project.

## Installation

- First install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Then install Clippy: [https://github.com/rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy)

(If the `cargo` command is not available, please restart your console)

### Clone the project

- `git clone git@github.com:lakwet/got_lineage.git`

## How to use it ?

### Create a database

- *mysql* service must be running
- Create a *database* named **game_of_throne** (of any name you want if you update the `config` file)
- Update the `config` file with the correct values. (Usually the config file is not `pushed`, but without it, it is a bit complicated to know what environment variables are required)

### Config file

The config file contains environment values to be sourced.

Variables for MYSQL are quite obvious.

But the variable named *GAME_OF_THRONE_RESET_CHARACTER* with a value "true" or "false"
tell the server at each start if the database must be erased and then refill with the
csv file data or not.

Hence, you have to start the server with this variable set to "true" at least once.

If you let the value to "true", this will reset SQL tables at each server start.

Otherwise, the server will remember who is dead.

### Run the project

- Don't forget to `source` the `config` file.

We are good, you can run the project.

- `make run` to launch the server.

### Request the server

Use the tool you want to request (GET) the server at this address:

If you want the next character in line:
- `http://127.0.0.1:9898/api/next?name=Tywin Lannister`

This returns: *"Tywin Lannister" next heir is: "Jamie Lannister"*

You can kill any character:
- `http://127.0.0.1:9898/api/kill?name=Jamie Lannister`

This returns: *"Jamie Lannister" has been killed !*

And if you redo:
- `http://127.0.0.1:9898/api/next?name=Tywin Lannister`

Since Jamie Lannister is dead, the server returns: *"Tywin Lannister" next heir is: "Tyrion Lannister"*

### Test

You can run test: `make test`
(Be careful, the test kills *Aerys Targaryen*)

### Nota Bene

- You can kill several time the same character. He or she will be dead anyway.
- If you try to send an unknown character (for example: "Bob Marley"), the server will answer with a  204 http code status (no content).
- For nephews or nieces, I used this definition: [Wikipedia](https://en.wikipedia.org/wiki/Niece_and_nephew). Hence, husband or wife nephews and nieces are not included into nephews or nieces.
- For the last rule (other), I match members of the same house by their surname.
- Alphabetical order is performed on the first name.
- Test database has not been created. I could add one and then add its name in the config file. It
is a possible improvement for this project.

## Version used

- rustc: 1.39.0
- rustup: 1.20.2
- cargo: 1.39.0
- rustfmt: 1.4.8-stable
