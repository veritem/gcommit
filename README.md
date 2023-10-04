# ⛏️gcm
Commit like a pro

## Installation

download the latest release from [here]() and add it to your path

## Usage

Here's a simple description of how you can use gcm.

### 1. In your project directory run
```bash
$ gcm setup 
```
This will add a .gcm.json file in you project's root directory. This file contains the commit message template for your project. You can edit this file to add your own template.

The contents will be:
```json
{
    "types" : [
        {
            "abv" : "feat",	
            "meaning" :"Added a new feature in the code"
        },
        {
            "abv" : "fix",
            "meaning" : "Fixed a bug in the code"
        },
        {
            "abv" : "refactor",
            "meaning" : "Refactored the code"
        },
        {
            "abv" : "docs",
            "meaning" : "Updated the documentation"
        },
        {
            "abv" : "test",
            "meaning" : "Added/Updated the test cases"
        },
        {
            "abv" : "chore",
            "meaning" : "Updated the build system"
        }
    ],
    "scopes" : [
        "core",
        "ui",
        "test",
        "docs",
        "build",
        "release"
    ],
}
```


### 2. Start committing like a pro, type `gcm` to make a new commit 
```bash
$ gcm
```
### 3 You can also commit in one line also without passing through the whole wizard, what we do here is to help you validate commit types and scopes.

```bash
$ gcm -t <type> -s <scope> -m <message>
``` 


## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Maintainers
- [Regis NDIZIHIWE](https://github.com/regisrex)

## License
[MIT](./LICENSE)
