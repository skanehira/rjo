# rjo
[jpmens/jo](https://github.com/jpmens/jo) ported with Rust.

## Usage
```sh
$ rjo name=gorilla age=10 like=$(rjo -a Go Vim Docker Kubernetes Rust) | jq
{
  "like": [
    "Go",
    "Vim",
    "Docker",
    "Kubernetes",
    "Rust"
  ],
  "name": "gorilla",
  "age": 10
}
```

## Author
skanehira
