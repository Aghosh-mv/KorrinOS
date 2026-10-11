#!/bin/bash
#
# make-module-cert.sh - create the KorrinOS module signing certificate.
#
# Why this exists:
#
# The tree ships certs/signing_key.pem (the private key) but historically had
# no matching certificate. CONFIG_MODULE_SIG=y with SYSTEM_TRUSTED_KEYS=""
# means the kernel verifies module signatures against an EMPTY keyring, so no
# module can ever verify. Every load taints with "signature and/or required
# key missing", and the live ISO boot wedges in a modprobe retry loop.
#
# This script derives certs/signing_key.x509.pem from the existing private key
# so that SYSTEM_TRUSTED_KEYS=certs/signing_key.x509.pem actually trusts
# something, and so in-tree modules verify cleanly.
#
# It is idempotent: an existing, still-valid certificate is left alone unless
# --force is passed.
#
set -euo pipefail

cd "$(dirname "$0")/.."

KEY="certs/signing_key.pem"
CERT="certs/signing_key.x509.pem"
CNF="${TMPDIR:-/tmp}/korrinos-module-cert.cnf"
FORCE=0

[ "${1:-}" = "--force" ] && FORCE=1

if [ ! -f "$KEY" ]; then
	echo "error: $KEY is missing; the kernel generates it on first build" >&2
	exit 1
fi

if [ -f "$CERT" ] && [ "$FORCE" -eq 0 ]; then
	if openssl x509 -in "$CERT" -noout -checkend 0 >/dev/null 2>&1; then
		echo "certificate already present and valid: $CERT"
		exit 0
	fi
	echo "certificate present but expired or unreadable; regenerating"
fi

cat > "$CNF" <<'EOF'
[ req ]
default_bits		= 4096
distinguished_name	= req_distinguished_name
prompt			= no
x509_extensions	= v3_req
string_mask		= utf8only

[ req_distinguished_name ]
O			= KorrinOS Project
CN			= KorrinOS Module Signing
emailAddress		= aghoshpratheesh@gmail.com

[ v3_req ]
basicConstraints	= critical,CA:TRUE
keyUsage		= critical,keyCertSign,digitalSignature
subjectKeyIdentifier	= hash
authorityKeyIdentifier	= keyid:always,issuer:always
EOF

openssl req -new -x509 -days 7300 -nodes \
	-key "$KEY" \
	-out "$CERT" \
	-config "$CNF" >/dev/null 2>&1

echo "created $CERT"
openssl x509 -in "$CERT" -noout -subject -dates | sed 's/^/  /'