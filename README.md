# golangci-lint extension for Zed

A [Zed extension](https://zed.dev/docs/extensions) which adds support for [golangci-lint](https://github.com/golangci/golangci-lint).

The extension is based on [golangci-lint-langserver](https://github.com/nametake/golangci-lint-langserver).

## Configuration in Zed

The default configuration which uses all linters in `golangci-lint` is:
```json
"lsp": {
  "golangci-lint": {
    "initialization_options": {
      "command": [
        "golangci-lint",
        "run",
        "--enable-all",
        "--disable",
        "lll",
        "--out-format",
        "json",
        "--issues-exit-code=1"
      ]
    }
  }
}
```

In case you have a `.golangci.yaml` config file present, you can use it as well (make sure to define it per-project in the [project settings](https://zed.dev/docs/configuring-zed#settings-files)):
```json
"lsp": {
  "golangci-lint": {
    "initialization_options": {
      "command": [
        "golangci-lint",
        "run",
        "lll",
        "--config=/path/to/.golangci.yaml",
        "--out-format",
        "json",
        "--issues-exit-code=1"
      ]
    }
  }
}
```
