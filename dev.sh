#/bin/bash

trap 'pkill reo' SIGINT

#cargo build --release
#cp ./target/release/reo ./
#chmod +x reo

#./reo --port 8080 --id 1 --storage memory &
#./reo --port 8081 --id 2 --storage memory &
#./reo --port 8082 --id 3 --storage memory &
#./reo --port 8083 --id 4 --storage memory &

#./reo --port 8080 --id 1 --storage sled --db-name db8080 &
#./reo --port 8081 --id 2 --storage sled --db-name db8081 &
#./reo --port 8082 --id 3 --storage sled --db-name db8082 &
#./reo --port 8083 --id 4 --storage sled --db-name db8083 &

cargo run --release -- --port 8080 --id 1 --storage sled --db-name db8080 &
cargo run --release -- --port 8081 --id 2 --storage sled --db-name db8081 &
cargo run --release -- --port 8082 --id 3 --storage sled --db-name db8082 &
cargo run --release -- --port 8083 --id 4 --storage sled --db-name db8083 &

wait
