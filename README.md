## gcommit
[![ci](https://github.com/veritem/gcommit/actions/workflows/ci.yml/badge.svg)](https://github.com/veritem/gcommit/actions/workflows/ci.yml)
![release](https://shields.io/github/v/release/veritem/gcommit)

Conventional git commit messages done right
### Get started 
1. download to your platform binary from [releases](https://github.com/veritem/gcommit/releases)
2. basic commands 
```bash
$ gcommit   // starts the commit dialog, using data in .gcommit.yml or creates one if not available 🌟
```
```bash
$ gcommit --help  // Prints the help and how to use the cli.
```
3. Commit by opening the dialog or using single line
```bash
$ gcommit -c <type> -s <scope> -m <commit_message>
```
4. Enjoy!

### Features
- Configurable: Devs can config how their commits should look like in `.gcommit.yml` file
- Validation:  types or scopes that can be added in commits are the ones in the config file only
- Commit dialog: use the commit dialog ui to make pro commit message
- Simplicity:  So simple to make a pro commit message

### Preview
|   | |
|--------|--------|
| <h2><pre>&gt;_ gcommit</pre></h2> | ![Screenshot 2023-10-09 205709](https://github.com/regisrex/gcommit/assets/94565752/1b980c0f-1c76-4fae-a051-3ecb2c16ca0f)|
| ![Screenshot 2023-10-09 205613](https://github.com/regisrex/gcommit/assets/94565752/930fb9cf-7e56-4b02-a2c6-6d3f61f752dc)|![Screenshot 2023-10-09 205834](https://github.com/regisrex/gcommit/assets/94565752/1328a76e-4d33-49a0-87b1-4a7e7929da32) |




[MIT](./LICENSE) Licence &copy; 2022 
