# Maintainer: shykeiichi <mail@keii.dev>
pkgname=hypofetch
pkgver=0.1.0
pkgrel=1
pkgdesc="A criminally fast and simple system info fetching tool"
arch=("x86_64")
url="https://github.com/shykeiichi/hypofetch"
license=('GPL')
groups=()
depends=()
makedepends=(cargo)
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=($pkgname-$pkgver.tar.gz)
noextract=()
md5sums=() #autofill using updpkgsums

prepare() {
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
	  cp ./hypo ~/.config/
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
