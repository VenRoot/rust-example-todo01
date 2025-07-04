pkgname=ven-todo-rs
pkgver=0.1.0
pkgrel=1
pkgdesc="A CLI todo application with YAML storage"
arch=("x86_64")
url="https://github.com/VenRoot/rust-example-todo01"
license=("GPL")
depends=()
makedepends=("rust" "cargo")
source=("$pkgname-$pkgver.tar.gz::https://github.com/venroot/rust-example-todo01/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Replace after creating a GitHub release

build() {
  cd "rust-example-todo01-$pkgver"  # Use repository name instead of pkgname
  cargo build --release --locked
}

package() {
  cd "rust-example-todo01-$pkgver"  # Use repository name instead of pkgname
  install -Dm755 "target/release/ven-todo" -t "$pkgdir/usr/bin/"  # Use binary name from Cargo.toml
  install -Dm644 LICENSE -t "$pkgdir/usr/share/licenses/$pkgname/"
  install -Dm644 README.md -t "$pkgdir/usr/share/doc/$pkgname/"
}