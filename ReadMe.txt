https://doc.rust-lang.org/std/index.html
https://www.rust-lang.org/learn

Środowisko:
rustup.rs -> rustup - instalator

cargo - menedżer kompilacji
rustc - kompilator
rustdoc - generator dokumentacji

Nowy projekt:
cargo new --bin project_name

Budowanie:
cargo build

Budowanie i uruchamianie:
cargo run

Czyszczenie:
cargo clean

Testy:
cargo test







Vec - wektor dynamicznej długości (Vec<u64>)
std::env::args() - argumenty programu



Nie ma wyjątków. Zwrotki z typem Result //TODO


Funkcje:
- jeżeli nazwa arumentu zaczyna się od podkreślnika to kompilator nie będzie zwracał warningów o nieużywanym argumencie

Surowy łańcuch znaków (początek i koniec z taką samą liczbą znaków #): r##" dowolny ciąg znakó "##

Wyrażenie match:
match res
{
	Ok(success) => {...}
	Err(error) => {...}
}

TODO: format!


IDE?