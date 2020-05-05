<link href="https://fonts.googleapis.com/css2?family=Work+Sans:ital,wght@0,200;0,300;1,200&display=swap" rel="stylesheet">
<style>
.header {
    font-family: 'Work Sans', sans-serif;
    font-style: italic;
    font-size: 48pt;
    border-style: solid;
    border-color: #b0c4de;
    opacity: 0.5;
    border-radius: 5px;
    padding: 15px;
}
.sub-head{
    font-family: 'Work Sans', sans-serif;
    font-weight: bold;
    font-weight: 300;
    font-size: 16pt;
}
.line{
    font-family: 'Work Sans', sans-serif;
    color: black;
    border-style: solid;
    border-width: 0.75px;
    opacity: 0.7;
    margin-top: -5px;
}
.left-head{
    font-size: 24pt;
    font-weight: bold;
}
</style>
<center>
<font class="header">$> clifi</font><br><br>
<font class="sub-head">play your favorite streams straight from the command line</font>
<br><br>

</center>

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