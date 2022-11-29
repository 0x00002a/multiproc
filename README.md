# multiproc
_run multiple processes at once_

Yeah thats it. It runs n versions of the same process in parallel and
interleaves their output.

e.g.

```bash
> multiproc 10 echo "hello world"
[child 005]: hello world
[child 001]: hello world
[child 008]: hello world
[child 006]: hello world
[child 009]: hello world
[child 002]: hello world
[child 004]: hello world
[child 003]: hello world
[child 010]: hello world
[child 007]: hello world
```
