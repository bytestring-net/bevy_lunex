<h1 align="left">Bevy Lunex</h1>
<p align="left">A novel path-based approach to UI built on top of existing Bevy features.</p>

![image](https://github.com/bytestring-net/bevy_lunex/assets/49441831/73d96dd1-d851-4a9f-9d58-11aba63e579d)


## Table of Contents

- [Description](#description)
- [Usage](#usage)
- [License](#license)

## Description

Bevy_Lunex is a ECS sync UI framework built around the concepts of simplicity while still retaining absolute creative freedom. It adds layout capibilities to Bevy ECS.
It uses a path-based hierarchy to manage UI widgets. Pointers to these widgets are used as components to an entity you want synchronize.
```
#HIERARCHY
  |-> Main menu
  |    |-> Wallpaper
  |    |    |-> Background
  |    |-> Board widget
  |    |    |-> Logo
  |    |    |    |-> Logo Shadow
  |    |    |-> Button List
  |    |    |    |-> Continue
  |    |    |    |-> New Game
  |    |    |    |-> Load_Game
  |    |    |    |-> Settings
  |    |    |    |-> Credits
  |    |    |    |-> Additional Content
  |    |    |    |-> Quit Game
 ```

## Usage

Basic usage instructions to help people to just get started using your project.

![image](https://github.com/bytestring-net/bevy_lunex/assets/49441831/180b773d-cbd3-4b3e-8d97-fbedde011e10)


For more examples and usage, please refer to the [Wiki](https://github.com/yourname/yourproject/wiki).


## License

LICENSE NAME - see the [LICENSE](https://github.com/satwikkansal/readme_styles/blob/master/LICENSE) file for more details.
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
