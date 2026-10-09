#!/bin/bash
# KorrinOS File Vault - LUKS container MANAGEMENT ONLY.
#
# NOTE: this script creates plain unencrypted container images and can
# open/close/mount them once the user has run cryptsetup by hand. It does
# NOT perform encryption itself, and must never claim that it does.

set -e

VAULT_DIR="$HOME/.tinker/vaults"
CONFIG_FILE="$VAULT_DIR/config.conf"
mkdir -p "$VAULT_DIR"

init() {
    [ ! -f "$CONFIG_FILE" ] && cat > "$CONFIG_FILE" << 'EOF'
# File Vault Configuration
DEFAULT_SIZE_MB=512
DEFAULT_FS=ext4
MOUNT_POINT="$HOME/vault-mount"
EOF
}

# Check dependencies
check_deps() {
    for tool in cryptsetup; do
        command -v $tool &>/dev/null || { echo "Missing: $tool (sudo apt install cryptsetup)"; return 1; }
    done
    command -v losetup &>/dev/null || { echo "Missing: losetup (util-linux)"; return 1; }
}

# Create vault
create() {
    local name=${1:-secure}
    local size=${2:-$(grep DEFAULT_SIZE_MB "$CONFIG_FILE" | cut -d= -f2)}
    size=${size:-512}
    
    check_deps || return 1
    [ -f "$VAULT_DIR/$name.img" ] && { echo "Vault '$name' already exists"; return 1; }
    
    # This creates a PLAIN, UNENCRYPTED sparse image. It does not perform
    # LUKS formatting, so nothing written here is encrypted. The previous
    # version printed "Creating Encrypted Vault" and then "Vault created",
    # which is a false security claim: a user who believed it and stored
    # secrets in the image would have them in the clear.
    echo "=== Creating vault CONTAINER: $name (${size}MB) ==="
    echo "  WARNING: this container is NOT yet encrypted."
    echo ""
    echo "Creating sparse image file..."
    truncate -s "${size}M" "$VAULT_DIR/$name.img"
    chmod 600 "$VAULT_DIR/$name.img"
    echo "   Created $VAULT_DIR/$name.img (mode 600)"
    echo ""
    echo "To actually encrypt it, run these yourself:"
    echo ""
    echo "  sudo losetup --find --show $VAULT_DIR/$name.img"
    echo "  sudo cryptsetup luksFormat <loopdev>"
    echo "  sudo cryptsetup open <loopdev> $name"
    echo "  sudo mkfs.${FS:-ext4} /dev/mapper/$name"
    echo "  sudo mount /dev/mapper/$name $MOUNT_POINT"
    echo ""
    echo "  Container created but UNENCRYPTED. Do not store secrets in it"
    echo "  until you have completed the LUKS format step above."
}

# Try to open/mount (requires user to complete system setup)
open() {
    local name=${1:-secure}
    echo "=== Opening Vault: $name ==="
    echo ""
    echo "This requires the loop device and LUKS passphrase:"
    echo ""
    echo "  sudo cryptsetup open $VAULT_DIR/$name.img $name"
    echo "  sudo mount /dev/mapper/$name $MOUNT_POINT"
    echo ""
    echo "  (Interactive. Run the sudo command above to complete.)"
}

# Close/unmount
close() {
    local name=${1:-secure}
    echo "Closing vault: $name..."
    sudo umount "$MOUNT_POINT" 2>/dev/null || echo "  (mount point not mounted)"
    sudo cryptsetup close "$name" 2>/dev/null || echo "  (already closed)"
    echo "  Vault closed"
}

# List vaults
list() {
    echo "=== File Vaults ==="
    echo ""
    local count=0
    for img in "$VAULT_DIR"/*.img; do
        [ -e "$img" ] || continue
        local size=$(du -h "$img" 2>/dev/null | awk '{print $1}')
        local logical=$(ls -lh "$img" | awk '{print $5}')
        echo "  $(basename "$img" .img): $size used ($logical logical)"
        count=$((count+1))
    done
    [ $count -eq 0 ] && echo "  No vaults created"
}

# Delete vault
delete() {
    local name=${1:-secure}
    echo "Deleting vault: $name..."
    close "$name"
    rm -f "$VAULT_DIR/$name.img"
    echo "  Deleted (irrecoverable!)"
}

show_help() {
    echo "Usage: tinker-filevault [command]"
    echo ""
    echo "Commands:"
    echo "  create [name] [mb]  Create encrypted vault container"
    echo "  open [name]         Open/mount vault"
    echo "  close [name]        Close/unmount vault"
    echo "  list                List vaults"
    echo "  delete [name]       Delete vault"
    echo "  help                Show this help"
}

init

case "$1" in
    create) create "$2" "$3" ;;
    open|mount) open "$2" ;;
    close|unmount) close "$2" ;;
    list) list ;;
    delete) delete "$2" ;;
    *) show_help ;;
esac