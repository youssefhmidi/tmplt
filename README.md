
<div align="center">

![tmplt logo](https://github.com/youssefhmidi/tmplt/blob/main/.assets/1.png)

![Static Badge](https://img.shields.io/badge/version-1.0v-858522)
![Static Badge](https://img.shields.io/badge/lang-rust-orange)
![Static Badge](https://img.shields.io/badge/license-MIT-red)

</div>

# table of index
  **before reading if you  want to install it go check releases ;)**

- [table of index](#table-of-index)
- [templating language for creating folders structures](#templating-language-for-creating-folders-structures)
  - [Why?](#why)
- [Getting started](#getting-started)
  - [Installation (windows)](#installation-windows)
  - [welcome to .tmplt](#welcome-to-tmplt)
- [Syntax](#syntax)
  - [CWD. current working directory section](#cwd-current-working-directory-section)
    - [DEFER keyword](#defer-keyword)
  - [DEMO.](#demo)
  - [SCRIPTS.](#scripts)
  - [VAR. well, here your variables are declared](#var-well-here-your-variables-are-declared)
- [CLI commands](#cli-commands)
- [tmplt internals](#tmplt-internals)
- [self compiling / feedbacks](#self-compiling--feedbacks)

> Side note: this is a side-project so I can learn rust even more
> I just want to share it for others who can use it 
> yes I'm a beginner so I would appreciate any feedbacks about the code 

# templating language for creating folders structures

tmplt, a templating language (also a scripting language) for creating a reusable files that can be used to create a folders structures along side with
some initial scripts (for js developper it would be "npm init -y"). this project is mainly focused on creating a file that can be used to setup
workspaces

## Why?

One day I was making a golang backend that used a similar folder/file structure to another [project](https://github.com/youssefhmidi/Backend_in_go) 
I had done, and the only issue I had was making the "folder structure", copy-pasting wasn't a great idea because I found myself tweaking a lot of 
the original code so it would match the way I want it to be. what did I do after that? I did nothing, I rebuild the folders from scratch and it wasted
a lot of time I would've wasted thinking should I use gorm or sqlx ( I did chose gorm and I was a mistake).

so after that I decided to make a cli tool that would take a file containing all the information about where files should be and execute some
commands like go mod init and others

and basically that is the purpose of this templating language


# Getting started

> for the moment, the tmplt ready-for-use binary will only work for windows.
> if you would like to use you may want to self compile it 

## Installation (windows)

for installation go to the releases section of this repository, download the .zip file and then extract it.
you can use it like this but if you would like to have the command available anywhere, you can add it to your PATH envirement variables.

## welcome to .tmplt

start by this command

```bash

tmplt help

```

it should show you a help text telling you about the command.
if you would want to initialize a new .tmplt file, you can either create it your self or use the following command

```bash

tmplt new 'filename'.tmplt

```
it will contain a demonstration of the tmplt sections and keywords.

# Syntax
there are 4 sections: **__CWD** ,**__VAR**, **__DEMO** and **__SCRIPTS**.
and there are 6 keywords: Assign( the "=" symbol),the "#" symbol, DEFER, COPY_INTO, FILE and FOLDER

## CWD. current working directory section
example code of a CWD section
```
__CWD:
    FOLDER out\
    FOLDER include\
    FILE main.c
```
Basicaly a section where you declare folder and files names.

the files/folders you declare here are going to be created in the same directory as the `tmplt` command where executed,
for example:

the previouse example will produce the following directory:

```bash
├─out\
├─include\
├─main.c
├─example.tmplt # this is the file that contains the code
```

### DEFER keyword
> Pre-Note: the DEFER keyword can be used everywhere and anywhere

well, for ordering purposes the DEFER keyword objectif is to order which folders/files should be created last or first.
consider the following example:
```
__CWD:
    FOLDER out\
    FOLDER include\
    DEFER src\
    DEFER src\.test\
    FILE main.c
```
the order of the execusion will be different, hence we will have the next order
```
create folder out->create folder include->create file main.c
->create folder src\->create folder src\.test
```
## DEMO.
your good looking wrokdir is ready and you want to start coding. But, unfortunalty your files are empty and you wish there are some demostrations or
some sort of starting point that you would love to start from.

**Enter __DEMO**
the `__DEMO` section's whole purpose is to do just what we discussed. make the files not empty

for example:
```
__DEMO:
    DEFER path/to/demo_code.c COPY_INTO ./useful_utils/utils.rs
    path/to/config.yaml COPY_INTO ./copied_config.yaml
``` 

the example above will produce the next directory

```bash
    .
    ├──copied_config.yaml
    ├──useful_utils
    │  ├───utils.rs
    .
    .
```

> It is also preferable to make the COPY_INTO action defered if you're unsure --assuming the folder is yet to be created--
> about in wich order the folder/file will be created. (file then folder or the opposite)

## SCRIPTS.
--insert content here--
## VAR. well, here your variables are declared
> NOTE: before you continue, these are not variables, they are constants, sorry to break it for ya.

here, you can define variables that you can use multiple times.
example :
```
__VAR:
    libs_path = /path/to/libs/
    build_scripts = /path/to/build/

...

// and later use it, for example
__DEMO:
    // defering it so we make sure the ./include file does exist
    DEFER #libs_path/mylib.h COPY_INTO ./include

...

__SCRIPT:
    // imagine a build tool with the possibility of configuring it using a config file
    // or a build file
    yourbuildtool compile --import=#build_scripts
```
as you can see variables are prifexed with '#' so the interpreter can replace them with the variable value

# CLI commands
# tmplt internals
# self compiling / feedbacks