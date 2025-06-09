# cfproxy-rs

> Socks5 proxy server based on Cloudflare Workers/Pages.  
> Written in Rust! 🦀


## Build

```bash
git clone https://github.com/achyuki/cfproxy-rs.git
cd cfproxy-rs
cargo build --release
```

## Deploy

Before using it, you need to deploy this project on Cloudflare workers or pages.

1. [Fork](https://github.com/achyuki/cfproxy-rs/fork) this repository
2. Deploy on Cloudflare pages
3. Configure `TOKEN`​ in environment variable

## Usage

```bash
# Use a configuration file:
cfproxy-rs --config config.json
# Or via command-line arguments:
cfproxy-rs --cfhost <domain> --token <token> --loglevel debug
```

For more information, see `cfproxy-rs --help`​.

## Config

Here is a complete configuration example.

```json
// config_full.json
// Don't save with comments, it will not be parsed!!
{
  "cfhost": "xxxx.pages.dev", // Cloudflare workers/pages domain [Required]
  "cfip": "104.16.0.0", // Cloudflare IP, generally no changes are required, or via https://github.com/XIU2/CloudflareSpeedTest
  "token": "", // Authentication token
  "host": "127.0.0.1", // SOCKS5 bind address
  "port": 4514, // SOCKS5 bind port
  "user": "", // SOCKS5 username
  "passwd": "", // SOCKS5 password
  "log": "", // Log saving path
  "loglevel": "info" // Log level (error/warn/info/debug/trace)
}

```

# Tips

* Due to [issues with Cloudflare](https://developers.cloudflare.com/workers/runtime-apis/tcp-sockets/#considerations), outbound TCP sockets to Cloudflare IP ranges are temporarily blocked.
* Due to Cloudflare does not support it, UDP proxy is not supported yet.
* 中国大陆用户请使用 Pages 部署.

## TODO

* [ ] ~~Support UDP proxy/DNS query~~
* [ ] Calling proxychains-ng/graftcp

## Credit

* [CF-Worker-Socks](https://github.com/ialihastam/CF-Worker-Socks)

## License

```
MIT License

Copyright (c) 2025 YukiChan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
