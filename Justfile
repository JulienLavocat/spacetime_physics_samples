module := "fps-demo"

server:
    spacetime publish -p server {{module}} -y -c

client:
    cargo run -p client

client-release:
    cargo run -p client --release

sc: server client
