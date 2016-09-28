# Maintainer: Ticki
pkgname=rem-git
pkgver=r1.0601e2b
pkgrel=1
pkgdesc=" Simple CLI utility for setting reminders."
arch=(i686 x86_64)
url="https://github.com/ticki/rem"
license=('MIT')
depends=(gcc-libs)
makedepends=('git' 'rust' 'cargo')
provides=(rem)
conflicts=(rem)
source=('git://github.com/ticki/rem.git')
md5sums=('SKIP')


pkgver() {
	cd "$srcdir/rem"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
	cd "$srcdir/rem"
	cargo build --release
}

package() {
	cd "$srcdir/rem"
	install -m755 -D target/release/rem "$pkgdir/usr/bin/rem"
}

