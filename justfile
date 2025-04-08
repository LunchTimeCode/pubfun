set dotenv-load
import 'justfiles/linting.just'



run *args:
    cargo run -- {{args}}


w:
    cargo watch --ignore 'assets/css' -s 'just run'

dist:
    cargo dist init
    cargo dist generate

install-tools:
    cargo binstall cargo-dist
    cargo install -f cargo-upgrades
    cargo install --locked cargo-outdated