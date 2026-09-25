# Maintainer: Suresh Soundararajan <sureshsoundararajan18@gmail.com>
pkgname=jotlet
pkgver=0.1.0
pkgrel=1
pkgdesc="A lightweight, fast, beautiful, native GNOME Sticky Notes application"
arch=('x86_64')
url="https://github.com/sureshsoudararajan/Jotlet"
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
    'git'
)
source=("$pkgname::git+$url.git")
sha256sums=('SKIP')

pkgver() {
    cd "$pkgname"
    git describe --long --tags --always 2>/dev/null | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g' || echo "0.1.0"
}

build() {
    cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo build --release
}

check() {
    cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo test
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
