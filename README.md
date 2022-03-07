# gRPC Rust <-> Node

I recently wanted to rewrite parts of an existing Node.js application in Rust. A complete rewrite would take a lot of time, so I was browsing through ways to rewrite some parts in Rust.

While it seems _possible_ to call Rust from Node by compiling Rust into a Node js module using FFI, it felt a bit messy.

Instead I opted to look into gRPC, which I've been wanting to look into anyway. In this example, I have written a small function in both Node.js and Rust. It can be called using gRPC from either Node.js or Rust.

```
+--------------------+     +--------------------+
| Client (Rust/Node) | --> | Server (Rust/Node) |
+--------------------+     +--------------------+

res = mean(4, 5)       --> function mean(a, b)
```

As you can see, the client just wants to know the mean value of two numbers, and apparently needs the full power of another language to do that. As you can see, a totally reasonable example!

## Parts

 * `proto/rustnodegrpc.proto`: Protobuf file with the gRPC service and messages. This describes the shared API between the server and client, and is used by both of the implementations.
 * `src/client.rs`: The client, written in Rust
 * `src/server.rs`: The server, written in Rust
 * `node/client.js`: The client, written in Node.js
 * `node/server.js`: The server, written in Node.js

## Running the example

You can run the examples in four different ways:
 * Node -> Node
 * Rust -> Rust
 * Node -> Rust
 * Rust -> Node

You can, however, only run a single server at the same time, since both Node and Rust listens on the same port.

First we prepare the server. It needs to be running before the client starts.

To run the Node.js server, use:
```sh
cd node/
npm i
node server.js
```

To run the Rust server, use:
```sh
cargo run --bin server
```

### Client

Then we build and run the client in a new shell.

```sh
cargo run --bin client
```

You should see the client printing out the correct mean (4.5) of the numbers 4 and 5. Success!

To run the Node client, use:

```sh
cd node/
npm i
node client.js
```
