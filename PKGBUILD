# Maintainer: Jotlet Contributors <https://github.com/example/jotlet>
pkgname=jotlet
pkgver=0.1.0
pkgrel=1
pkgdesc="A lightweight, fast, beautiful, native GNOME Sticky Notes application"
arch=('x86_64')
url="https://github.com/example/jotlet"
license=('GPL-3.0-or-later')
depends=(
    'gtk4'
    'libadwaita'
    'glib2'
    'sqlite'
)
makedepends=(
    'rust'
    'cargo'
    'pkgconf'
)
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "$pkgname-$pkgver" || cd "$srcdir"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')" 2>/dev/null || cargo fetch
}

build() {
    cd "$pkgname-$pkgver" || cd "$srcdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-targets 2>/dev/null || cargo build --release
}

check() {
    cd "$pkgname-$pkgver" || cd "$srcdir"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen 2>/dev/null || cargo test
}

package() {
    cd "$pkgname-$pkgver" || cd "$srcdir"

    # Binary
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

    # Desktop Entry
    install -Dm644 "data/com.example.Jotlet.desktop" \
        "$pkgdir/usr/share/applications/com.example.Jotlet.desktop"

    # Application Icon
    install -Dm644 "data/icons/hicolor/scalable/apps/com.example.Jotlet.svg" \
        "$pkgdir/usr/share/icons/hicolor/scalable/apps/com.example.Jotlet.svg"

    # GSettings Schema
    install -Dm644 "data/com.example.Jotlet.gschema.xml" \
        "$pkgdir/usr/share/glib-2.0/schemas/com.example.Jotlet.gschema.xml"

    # License
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
