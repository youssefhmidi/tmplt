
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
- [CLI command](#cli-command)
  - [new/init command](#newinit-command)
  - [generate command](#generate-command)
- [tmplt internals](#tmplt-internals)
- [a prefered way of using .tmplt](#a-prefered-way-of-using-tmplt)
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
a lot of time I would've wasted thinking should I use gorm or sqlx ( I did chose gorm and it was a mistake).

so after that I decided to make a cli tool that would take a file containing all the information about where files should be and execute some
commands like go mod init and others

and basically that is the purpose of this templating language


# Getting started

> for the moment, the tmplt ready-for-use binary will only work for windows.
> if you would like to use you may want to self compile it, check [self compiling section](#self-compiling--feedbacks)

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
the `__SCRIPTS` section is responsible for defining scripts that should be run to fully setup a workspace,
for example you may want to make a svelte and golang app
well take the following example:

```
__VAR :
    ...
    package = pakage/name
    dep1 = github.com/labstack/echo/v4
    dep2 = github.com/jmoiron/sqlx
...

__SCRIPTS :
    ...
    // 1
    go mod init #package 
    // 2
    go get #dep1
    // 3
    go get #dep2
    ...
```
with the previews example we told tmplt to run this commands in order -- 1 then 2 then 3 --
and for the last step, creating svelte kit, unfortunatly, I didn't find a way to programmatically make a sveltkit project,
if you have a solution you can create an Issue / PR and I will merge it after reviewing it.

> Note: this is not a ready to use tool it is still very immature and needs proper testing and more optimizations

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

# CLI command
after reading about the syntax you may want to use tmplt for the rest of your life 'hopefully'. well, it is pretty easy to use.

if you want some help you can use the next command and it will show you all commands/flags and additional information
```bash
tmplt help
```
## new/init command
usage: initialize a new tmplt file with default example and sections
example :
```bash
tmplt new
    # or
tmplt init
``` 
usecases: file name, specifiy the file name along with the .tmplt extension. otherwise it just makes a default `new.tmplt` file
e.g.: `tmplt new filename.tmplt`

## generate command
aliases: gen or g
usage: interpret the file and generate te directory the file describes also runs the scrips/make files&folders and copy into files
flags: 
--save-logs(alias --sl or --logged): default: false
    save the commands output into logs. can be found in the directory if not found the logs folder get created
example:
```bash
tmplt generate template.tmplt --save-logs
        # or 
tmplt gen template.tmplt --logged
```     
--batch-size(alias --task-num): default: 10
    the tmplt execute commands (files/folders creation, scripts and coping demo files) asynchronously. a specified number of tasks 
    (also refered to as batches) get executed per thread.
example:
```bash
tmplt gen big-template.tmplt --batch-size=20
```

> Future Idea: more flags / more features, e.g 'tmplt new --template-url=git-url'

# tmplt internals
tmplt executing cycle is as follow:
![tmplt executing cycle](https://github.com/youssefhmidi/tmplt/blob/main/.assets/2.png)
<i align="center">

Image made using [Excalidraw](https://excalidraw.com/) 

</i>

And we can visualize the syntax tree using the following image;
![tmplt syntax tree](https://github.com/youssefhmidi/tmplt/blob/main/.assets/3.png)
<i align="center">

Image also made using ![Excalidraw](https://excalidraw.com/), thanks for creating this amazing website

</i>

In the first image you can see that there are 8 general steps:
  - 1: Reading the file the passed in the args nad constructing a syntax tree.
  - 2: Storing variables into a buffer. -- a hashmap of structure {var_name:var_value} --
  - 3: Translating the commands in the tmplt file into terminal/fs actions --a terminal command, fs create file or folder or a fs copy--.
  - 4: Storing variables and commands into a struct.
  - 5: Making a task buffers and serializing the commands and storing them to the buffer.
  - 6: Executing batches of tasks asynchronously -the size of a batch is default to 10 and can be changed through a flag, see [this section](#generate-command)-
  - 7 and 8: The actual execution and writing to stdout.

The second image represent how the tmplt file is loaded into memory.  
Syntax Tree terminology -- not to be confused with AST --  
root level: the entire file.  
branches: sections, and there are only 4.  
node: a node is the entire line.  
token: token represent a word
# a prefered way of using .tmplt

# self compiling / feedbacks