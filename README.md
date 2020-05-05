<p align="center">

<img src=clifi.png>
<br>

<img src="https://img.shields.io/github/license/kevinshome/clifi">
<img src="https://img.shields.io/maintenance/yes/2020">
<img src="https://img.shields.io/github/last-commit/kevinshome/clifi">

</p>

## Index
- [Installing and Using clifi](#Installation)
    - [Installation](#Installation)
    - [How to use clifi](#Usage)
        - [Adding a stream](#Managing-clifi-streams)
        - [Removing a stream](#rm-stream)
        - [Switching streams](#switch-stream)

## Installation
Installing clifi is fast and easy, just enter the following command on your machine's terminal:

```
$ pip3 install git+https://github.com/kevinshome/clifi
```

## Usage
To use clifi, simply run:

```
$ clifi
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