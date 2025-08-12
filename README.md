# tr-set

Think [transmission-remote](https://github.com/transmission/transmission/blob/main/utils/remote.cc), but reduced to a single function: setting the peer port. When statically linked (`+crt-static`), it can be used in minimal container environments (e.g. with [qdm12/gluetun](https://github.com/qdm12/gluetun), a VPN client with port forwarding support). Transmission RPC is supported via [j0rsa/transmission-rpc](https://github.com/j0rsa/transmission-rpc).
