# Client Server

A gRPC/TLS/TCP client/server in Go and Rust, TLS/TCP client/Server in Go and Rust and TCP client/server in Go and Rust.

## Instructions

### gRPC

#### Go

1. Install protoc

    $ cd /tmp

    $ wget https://github.com/protocolbuffers/protobuf/releases/download/v22.0/protoc-22.0-linux-x86_64.zip

    $ unzip protoc-22.0-linux-x86_64.zip

    $ sudo cp bin/protoc /usr/local/bin

    $ sudo cp -r include/google /usr/local/include

2. Install gRPC and protobuf tools for Go

    $ go install google.golang.org/protobuf/cmd/protoc-gen-go@latest

    $ go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

3 Generate client protobuf code

    $ cd grpc/go/client

    $ protoc -I ../proto app.proto --go-grpc_out=app --go_out=app

4 Generate server protobuf code

    $ cd grpc/go/server

    $ protoc -I ../proto app.proto --go-grpc_out=app --go_out=app

5 Run the server

    $ cd grpc/go/server

    $ go run main.go

6 Run the client

    open a new terminal

    $ cd grpc/go/client

    $ go run main.go

#### Rust

1 Run the server

    $ cd grpc/rust

    $ cargo run --bin server

2 Run the client

    open a new terminal

    $ cd grpc/rust

    $ cargo run --bin client

### TLS

#### Go

1 Run the server

    $ cd tls/go/server

    $ go run main.go

2 Run the client

    open a new terminal

    $ cd tls/go/client

    $ go run main.go

#### Rust

1 Run the server

    $ cd tls/rust

    $ cargo run --bin server

2 Run the client

    open a new terminal

    $ cd tls/rust

    $ cargo run --bin client

### TCP

#### Go

1 Run the server

    $ cd tcp/go/server

    $ go run main.go

2 Run the client

    open a new terminal

    $ cd tcp/go/client

    $ go run main.go

#### Rust

1 Run the server

    $ cd tcp/rust

    $ cargo run --bin server

2 Run the client

    open a new terminal

    $ cd tcp/rust

    $ cargo run --bin client
