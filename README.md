# ⛏️gcm
Commit like a pro

## Installation

download the latest release from [here]() and add it to your path

## Usage
1. In your project directory run
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