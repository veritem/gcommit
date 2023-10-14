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


[MIT](./LICENSE) License &copy; 2022 - PRESENT
