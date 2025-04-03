import 'justfiles/linting.just'

SERVER_PORT := "9999"


run *args:
    cargo run -- {{args}}


w:
    cargo watch --ignore 'assets/css' -s 'just run'