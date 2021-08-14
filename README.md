<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
![Build Status](https://img.shields.io/badge/status-active-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Stability](https://img.shields.io/badge/stability-unstable-orange)


<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/Cyclip/pyproj/">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h2 align="center">PyProj</h2>

  <p align="center">
    A Python project creator/manager built in Rust.
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#commands">Commands</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

pyproj is a Rust-based project manager designed to automate and speed up your Python projects. With this tool, you can set up a git-ready project directory within seconds to get straight to programming. Its use extends to automatically generating `requirements.txt` files, cleaning cache, easily run tests and more. pyproj was designed to be easy, simple and quick to use, and in the future allow you to tailor project generation to uses such as Machine Learning, Discord bots, etc.

A list of commonly used resources that I find helpful are listed in the acknowledgements.

### Built With

* [Rust](https://www.rust-lang.org/)
* [lazy_static](https://docs.rs/lazy_static)

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

These steps assume you already have [Rust](https://www.rust-lang.org/) installed along with Cargo to build and test the source code.

### Installation

1. Download the [latest release](https://github.com/Cyclip/pyproj/releases) from the sidebar
2. Extract the `.zip`
3. Add `pyproj.exe` to [PATH](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)
4. Reload all command prompts/shells and use `pyproj` as a command.

### Building
This is if you would like to clone the source code. The above section covers installation

1. Clone the repository
   ```
   git clone https://github.com/Cyclip/pyproj/
   ```
2. cd into the repo
   ```
   cd pyproj
   ```
3. Build and run the project
   ```
   cargo run
   ```


<!-- USAGE EXAMPLES -->
## Commands

### help
Displays the help message

### create
Creates a new project at the empty/non-existant target directory.
Example: `pyproj create scraper`

### clean
Cleans all unused files and folders, including `__pycache__`

### build
Builds certain special files including `requirements.txt`
Example: `pyproj build`

### test
Run unit tests for the project, either a single test or all tests in the ./tests/ folder.
Examples
- `pyproj test`
- `pyproj test test_file`
- `pyproj test test_file.py`

## Usage

Let's make a project to print the user's IP address and exit.

To create a project, you should use the command below. Note that the target directory should **not exist**, although if it does exist pyproj will accept an empty directory. It will then give 3 prompts; a short description of your project, the author (you), and the name of the license you're using (i.e MIT):
```
C:/projects/
λ pyproj create example_project
Project description: An example project for pyproj
Author: Cyclip
License name: MIT
Successfully created project at ./example_project
```

This will create a project structure as seen below:
```
│   
├─── .gitignore				
├─── LICENSE				
├─── MANIFEST.in			
├─── requirements.txt		
├─── setup.cfg				
├─── setup.py				
│
├─── docs					Documentation detailing how to use {proj}
├─── examples				        Example usage
├─── src					Source code for use in this project
│    └───example_project				
│        └─── main.py		                Main python file
│
└─── tests					All tests (unit, etc.)
     └─── __init__.py		
```

The main file is `./src/example_project/main.py`. After writing the code to get and print the current IP address, it should look like this:
```Python
#!/usr/bin/python3

import sys
import requests
import json

# ---------- CONSTANTS ----------
# URL to send a request to
URL = "http://ip.jsontest.com/"
# -------------------------------


def main():
	"""Main function."""
	try:
		# Send a GET request to URL and load it in a json object
		data = json.loads(
			requests.get(URL).content
		)
		ip = data["ip"]
		print(f"IP: {ip}")

		# Success
		return 0
	except requests.exceptions.ConnectionError:
		print(f"Error while connecting to {URL}")
	except Exception as e:
		print(f"Unexpected error while connecting to {URL}: {str(e)}")
	
	# Error
	return 1


if __name__ == "__main__":
	sys.exit(main())
```

We're using [requests](https://pypi.org/project/requests/), an external library which you need to install via `pip`. Now, we're able to generate an appropriate `requirements.txt` file automatically:

```
C:/projects/example_projects
λ pyproj build
Successfully updated requirements.txt
```

And our `requirements.txt` file should look something similar to this (of course the version may differ):
```
requests==2.25.1
```

<!-- ROADMAP -->
## Roadmap

- Create a unit test command
- Parse and provide code suggestions in Python files
  - Unused imports, variables, etc.
- Tailor to certain project types
  - ML, web development
- Implement pyinstaller

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.


<!-- CONTACT -->
## Contact

Ifaz Ahmed ([Cyclip](https://github.com/Cyclip/) - ifazahmed375@gmail.com - [LinkedIn](https://www.linkedin.com/in/ifaz-ahmed/))

Project Link: [https://github.com/Cyclip/pyproj/](https://github.com/Cyclip/pyproj/)


<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Img Shields](https://shields.io)
* [Choose an Open Source License](https://choosealicense.com)



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: images/screenshot.png
