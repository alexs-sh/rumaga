![badge](https://github.com/alexs-sh/rumaga/actions/workflows/build.yml/badge.svg)

# About
A simple math game in Rust and Tokio. It supports interactive and network modes. It's just for fun and testing Tokio/Async.

# Quick start
Connect to the server and try it :)

```
nc ovz1.shirokovalexs.pl50n.vps.myjino.ru 49264
```

```
putty ovz1.shirokovalexs.pl50n.vps.myjino.ru 49264
```


# Run

Run the game in interactive mode

```
cargo run
```


Run server

```
cargo run 0.0.0.0:8888
```

Connect to the server from a remote host

```
nc 8.8.8.8 8888
```



# Build

```
cargo build
```
