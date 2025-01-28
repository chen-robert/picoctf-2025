# Pachinko

- Namespace: picoctf
- ID: pachinko
- Type: custom
- Category: Web Exploitation
- Points: 300
- Templatable: no
- MaxUsers: 1

## Description

History has failed us, but no matter.

{{url_for("server.tar.gz", "Server source")}}

There are two flags in this challenge. Submit flag one here, submit flag two in [Pachinko Revisited PLACEHOLDER](https://play.picoctf.org)

## Details


## Solution Overview

Run `make`

## Challenge Options

```yaml
cpus: 0.5
memory: 128m
pidslimit: 20
ulimits:
  - nofile=128:128
diskquota: 64m
init: true
```


## Attributes

- author: notdeghost
- organization: OtterSec
- event: picoCTF 2025
