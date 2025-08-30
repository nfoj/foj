# Helix Editor

```md
.
###x.        .|
d#####x,   ,v||
 '+#####v||||||
    ,v|||||+'.      _     _           _
 ,v|||||^'>####    | |   | |   ___   | | (_) __  __
|||||^'  .v####    | |___| |  /   \  | |  _  \ \/ /
||||=..v#####P'    |  ___  | /  ^  | | | | |  \  /
''v'>#####P'       | |   | | |  ---  | | | |  /  \
,######/P||x.      |_|   |_|  \___/  |_| |_| /_/\_\
####P' "x|||||,
|/'       'x|||    A post-modern modal text editor.
 '           '|
```

## Introduction

Helix is a modal code editor written in Rust, inspired by Kakoune and Neovim. It stands out for its "selection-first" approach and exceptional speed. A key advantage is that it provides essential IDE features, like LSP (Language Server Protocol) and DAP (Debug Adapter Protocol), built-in, eliminating the need for a vast collection of plugins. This makes it a powerful and efficient alternative for those seeking a modern editing experience without the complexity of managing countless plugins.

### Basic

`mod`

```md
esc = locked
i = insert
v = select
```

`movement`

```md
    ↑
    k       * h is on the left
← h   l →   * l is on the right
    j       * j looks like a down arrow
    ↓
```
   
`exit`

```md
:q = quit
:q! = quit no save
```

`save`

```md
:w = save
:wq! = save and quit
```

`insert text`

```md
i - Insert before the selection.
I - Insert at the start of the line.
A - Insert at the end of the line.
a - Insert after the selection. (a means 'append')
o - Add a newline and insert below the cursor.
O - Add a newline and insert above the cursor.
```

`motions and selections`

```md
w - Move forward to before the beginning of the next word.
e - Move forward to the end of the current word.
b - Move backward to the beginning of the current word.

and

2w - Move 2 words forward.
3e - Move to the end of the third word forward.
2b - Move 2 words backwards
```

`extend select`

```md
v - select mode + movement
x - select line
; - collapse selections

and

v2w - Select move 2 words forward
f + character - moves to the next occurrence of that character
```
