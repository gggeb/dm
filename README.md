dm - dotfile manager
====================
a tool for generating and linking .files.

first, the program deletes its previous output and deletes all broken symlinks recursively in the parent directory.
next the directory tree is duplicated, excluding first folder, such that `nvim/.config/nvim` becomes `.config/nvim`.
all the files in the duplicated tree are then rendered using `ser`.
the rendered files are then all symlinked into the parent directory.

ser
---
takes an input and replaces `~#environment_variables#~` with their values.
```
~#HOME#~ -> /home/username
~~#HOME#~~ -> ~#HOME#~
~~~#HOME#~~~ -> ~/home/username~
~#~#HOME#~#~ -> ~#/home/username#~
```

installing
----------
run `./install` after setting `INSTALL_PREFIX` accordingly.
if installing as root, make sure sudo captures your environment so cargo can be found.

use `./deinstall` to deinstall and make sure `INSTALL_PREFIX` is the same as it was when you installed.
