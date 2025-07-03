# Kernel Fuzzing Book

Notes about things about Kernel Fuzzing. The template is forked from [nix.dev](https://nix.dev)

You can deploy the book by belowing steps:

First, install nix: [install guide](https://nixos.org/download/), install nginx.

Second, enter the root directory of the respository, and execute the command below:

```bash 
nix-build
```

static html files can be found in directory `result`, you can deploy it in nginx.

## Contributing

Content is written in MyST, a superset of CommonMark. For its syntax, see the [MyST docs](https://myst-parser.readthedocs.io/en/latest/syntax/typography.html#syntax-core).

