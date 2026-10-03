#!/usr/bin/env bash
# setup-gpu-stacks.sh — Installs the GPU stacks so booting + this script = "instant" GPU compute.
# Run AFTER OS boots. Requires sudo.

set -euo pipefail

echo "==> Updating package lists"
apt-get update -qq 2>/dev/null

echo "==> Installing Python GPU stacks"
pip install --quiet --upgrade pip 2>/dev/null
pip install --quiet torch torchvision transformers 2>/dev/null

echo "==> Installing diffusion UI (Stable Diffusion)"
apt-get install -y -qq stable-diffusion-webui 2>/dev/null || echo "Stable Diffusion not in apt; skipping..."

echo "==> Installing RAPIDS data science (if compatible)"
# RAPIDS requires specific CUDA + OS versions; we install the core deps only.
# The heredoc body must be pip requirement specifiers (one per line).
if pip install --quiet -r /dev/stdin 2>/dev/null <<'EOF'
cudf-cu13
cuml-cu13
EOF
then
  echo "  RAPIDS installed."
else
  echo "RAPIDS install skipped (requires proper repos)"
fi

echo "==> Verifying GPU access"
nvidia-smi 2>/dev/null | head -5
python3 -c "import torch; print('PyTorch GPU:', torch.cuda.is_available())" 2>/dev/null

echo "==> GPU setup complete."
