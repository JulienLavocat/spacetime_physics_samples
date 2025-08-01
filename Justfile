module := "fps-demo"

server:
    spacetime publish -p server {{module}} -y -c

client:
    cargo run -p client

client-release:
    cargo run -p client --release

bindings:
    spacetime generate --lang rust --out-dir ./client/src/module_bindings --project-path server

sc: server client
