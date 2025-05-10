pkgname="wretch"
pkgver="1.0.0"
pkgrel=1
pkgdesc="A simple Fetch CLI program Built with Rust"
arch=("x86_64")
source=("src/main.rs")
sha512sums=("SKIP")
license=("GPL3")
package() {
    cargo build --release
    mkdir -p "${pkgdir}/usr/bin"
    cp target/release/wretch "${pkgdir}/usr/bin/"
    chmod +x "${pkgdir}/usr/bin/wretch"

}