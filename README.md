<p align="center">

<img src=clifi.png>
<br>

<img src="https://img.shields.io/github/license/kevinshome/clifi">
<img src="https://img.shields.io/maintenance/no/2020">
<img src="https://img.shields.io/github/last-commit/kevinshome/clifi">

</p>

# NOTICE

## THIS VERSION OF CLIFI IS DEPRECATED AS OF JUNE 2020. A RUST REWRITE IS CURRENTLY IN THE WORKS IN THE RUSTRW BRANCH, AND SHOULD BE READY AS A REPLACEMENT SOON. THIS BRANCH EXISTS SOLELY TO FREEZE THIS VERSION IN PLACE FOR HISTORICAL PURPOSES.

<br> <hr>

## Index
- [Installing and Using clifi](#Installation)
    - [Installation](#Installation)
    - [Configuration](#Configuration)
    - [How to use clifi](#Usage)
        - [Adding a stream](#Managing-clifi-streams)
        - [Removing a stream](#rm-stream)
        - [Switching streams](#switch-stream)
        - [Starting a local stream](#Starting-a-local-stream)
- [Issues and Bugs](#Issues-and-bugs)

## Installation
Installing clifi is fast and easy, just enter the following command on your machine's terminal:

```
$ pip3 install git+https://github.com/kevinshome/clifi
```

## Configuration
Clifi has two main configuration files:

- *~/.clifi/clifi.cfg*

    - holds the following configuration settings for clifi:

        - streamfile location
        - default stream
        - local stream options (see [Starting a local stream](#Starting-a-local-stream)) 

- *~/.clifi/streams.json*

    - holds all the stream information used by clifi, which includes:

        - **Stream name**: name used to start stream with clifi (i.e. "lofi")
        - **Full stream name (optional)**: full name of stream (i.e. "lofi hip hop radio - beats to relax/study to")
        - **Stream URL**

The default versions of these configuration files are usually installed to **/etc/default/clifi**. However, if they are unavailable there, they can also be found in the **/default** directory of this repository.

## Usage
To use clifi, simply run:

```
$ clifi
```

For a full list of commands, run:

```
$ clifi -h
```

And when you're ready to stop the tunes, run:

```
$ clifi -k
```

#### Managing clifi streams
In order to add a new stream to clifi, simply run the command:

```
$ clifi -n 
```

When this command is run, it will launch an interactive program that asks you for the following:

- \[name\]: the name that you want to use to call the stream from clifi
- \[full-name\] (optional): the full title of the stream
- \[url\]: the stream's URL 

<div id='rm-stream'></div> <!-- anchor #rm-stream -->

Removing a stream is even simpler! If need be, you can remove a stream from clifi with the command:

```
$ clifi -rm [name]
```

<div id='switch-stream'></div> <!-- anchor #switch-stream -->

If you want to switch to a different stream, you can run:

```
$ clifi -s [name]
```

#### Starting a local stream

Clifi also has the power to start a stream itself. In order to start your own local stream, simply run:

```
$ clifi -SS [file/directory to stream]
```

The default stream settings are as follows:

- **Hostname**: 0.0.0.0 (localhost)
- **Port**: 8080
- **Protocol**: HTTP
- **Format**: mp3

These settings can be changed in the clifi configuration file.

## Issues and Bugs

If you come across any issues/bugs using clifi, please report them on the **Issues Page** of this GitHub repository.