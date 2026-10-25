#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: a PKCS#7-signed module whose signing certificate has no CN in its issuer must report no signer. Non-zero = bug reproduced.
set -u
here=$(dirname "$(realpath "$0")")
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
cd "$tmp"
ln -s "$bld/kmod" modinfo
printf 'payload' > payload
for kind in nocn cn; do
	if [ $kind = nocn ]; then subj="/O=Foo Org/C=PL"; else subj="/O=Foo Org/CN=My CN/C=PL"; fi
	openssl req -x509 -newkey rsa:2048 -nodes -keyout $kind.key -out $kind.pem -subj "$subj" -days 3 2>/dev/null
	openssl smime -sign -binary -noattr -in payload -signer $kind.pem -inkey $kind.key -outform DER -out $kind.der
	python3 "$here/make_signed.py" $kind.ko $kind.der
	echo "issuer $subj -> signer: [$(./modinfo -F signer $kind.ko)]"
done
[ -z "$(./modinfo -F signer nocn.ko)" ] && [ "$(./modinfo -F signer cn.ko)" = "My CN" ]
