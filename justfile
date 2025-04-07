set dotenv-load
import 'justfiles/linting.just'



run *args:
    cargo run -- {{args}}


w:
    cargo watch --ignore 'assets/css' -s 'just run'