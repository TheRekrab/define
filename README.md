# define - A Rust dictionary tool
A basic command line dictionary built in rust

## Usage

`define` is very easy to use:

```
$ define <word>
```
where `<word>` is the search word.

#### Example:
```
$ define programming
verb
  -  To enter a program or other instructions into (a computer or other electronic device) to instruct it to do a particular task.
     example: "He programmed the DVR to record his favorite show."
  -  To develop (software) by writing program code.
     example: "I programmed a small game as a demonstration."
  -  To put together the schedule of an event.
     example: "Mary will program Tuesday’s festivities."
  -  To cause to automatically behave in a particular way.
     example: "The lab rat was programmed to press the lever when the bell rang."
noun
  -  The designing, scheduling or planning of a radio or television program/programme.
     example: "The network changed its programming to mess with DVRs again."
  -  Brain-washing
  -  The act of writing a computer program.
     example: "Management wanted to know how much programming the project would need."
  -  The software that controls a machine, or the logic expressed in such software; operating instructions.
     example: "A robot's programming doesn't allow for love."
```

## Installation

First, clone the GitHub repo:
```
$ git clone https://github.com/TheRekrab/define.git
```
Then, use `cargo` to install the binary to your PATH:
```
$ cargo install --path define/
```
## Colors!

This project utilizes the [colored](https://docs.rs/colored/latest/colored/) crate to color the text output, including:
- Text colors
- Italics
- Bold Text
- Underlined Text
