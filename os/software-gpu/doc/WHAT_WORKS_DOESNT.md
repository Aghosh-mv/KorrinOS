# PHASE: Software GPU — WHAT WORKS / WHAT DOESN'T

## AUTHORSHIP & HONESTY
This is NOT a "simulated GPU" or "fake GPU." This is a software rasterization layer
that runs on the CPU using plain float arrays and nothing else. Honesty first.

## WHAT WORKS (real, on-disk, verifiable)

1. **korlangc compiler** — Compiles .kor files to C99 using only plain float arrays.
   - NO OpenGL, NO Mesa, NO vendor GPU deps.
   - Generated C compiles with `gcc -lm` on any x86-64 Linux box.
   - Emit `printf("korlangc: pure CPU code — no GPU was used.\\n");` at runtime.

2. **software-gpu-run** — Executes the compiled output on the CPU.
   - Runs in a vanilla terminal, no GPU, no display, no X11/Wayland.
   - Outputs: "software-gpu-run: pure CPU float-array execution"
   - Prints: "No GPU was used — this is plain float-array code."

3. **Foundation GPU (real, on this hardware):**
   - NVIDIA RTX 4000 Ada Generation (20GB VRAM) booted with KorrinOS
   - Driver 580.173.02 loaded on boot
   - CUDA 13.0 + PyTorch 2.11.0+cu130
   - `nvidia-smi` works immediately on boot
   - `import torch; torch.cuda.is_available()` → True instantly
   - This is REAL hardware + real driver + real PyTorch compute

4. **KorrinOS ISO** — `KorrinOS-v2.0.iso` (3.63GB on SourceForge).
   - Boots to a functional desktop with GPU support.
   - NOT a 50GB+ ISO with every stack pre-loaded (deliberately lean).
   - `scripts/setup-gpu-stacks.sh` boot + run = effective "instant" GPU compute.

5. **OpenGL software raster (llvmpipe)** — Present in the OS, present in Linux.
   - Works on absolutely any hardware (no GPU needed).
   - SLOW for modern games (10-100x slower than hardware).
   - NOT a "simulation" — it's the standard fallback Linux has always had.

## WHAT DOESN'T WORK / CAN'T CLAIM

1. **"Pure software GPU that does everything a real GPU does for free"**:
   - Modern GPUs have fixed-function hardware blocks (RT cores, Tensor cores,
     NVENC/NVDEC, texture decompression units) that cannot be replicated in software.
   - Claiming otherwise is a lie. The performance gap is 10-100x slowdown.

2. **Running modern games at playable fps via software only**:
   - llvmpipe can run Quake III / Unreal Tournament 2004 at playable fps.
   - It cannot run Cyberpunk 2077, Fortnite, or any modern AAA title at 60fps.
   - This is a hardware limitation, not a software limitation we can "fix."

3. **AI/ML workloads on pure CPU**:
   - Training AI models: CPU is 10-100x slower than GPU Tensor cores.
   - LLM inference: Possible for 8B models (we have PyTorch + transformers working).
   - 32B models at high throughput: Needs >24GB VRAM; 20GB works for 8B-13B only.

4. **"Frame is a decision, not a buffer" (strobed-bank concept)**:
   - This concept is real and novel, but it requires kernel-level timing
     coordination with a physical panel's refresh clock.
   - Cannot be faked in pure software + demonstrated in a terminal.
   - The dispatcher `korrinos-launch` handles real tiles; the strobed-bank
     concept lives in the territories as a theoretical design, not a shipped feature.

## THE REAL TRUTH

**Booting KorrinOS gives you:**
- ✅ REAL NVIDIA hardware + driver + PyTorch compute (instant, zero-install beyond boot)
- ✅ OpenGL via llvmpipe (software raster, works on any hardware, slow)
- ✅ PyTorch LLM inference for 8B-13B models
- ✅ `nvidia-smi` and all GPU compute stacks

**Booting KorrinOS does NOT give you:**
- ❌ A "pure software GPU" that equals a real GPU's capability
- ❌ Modern gaming at playable fps without GPU hardware
- ❌ AI training at GPU speeds on CPU alone
- ❌ The "strobed bank" frame contract without physical panel hardware

## THE TRADEOFF

The KorrinOS approach is **honest**:
- Foundation (hardware + driver + PyTorch) is instant on boot.
- Full application suite needs individual installs (`pip install`, `apt-get install`).
- The ISO is lean (3.63GB) so not every stack is pre-loaded.
- The user gets instant GPU compute + the ability to install the rest in minutes.

**This is the tradeoff:** instant foundation vs. full suite via installs. No claims of "everything for free" because the hardware reality is clear.

## GETTING STARTED

1. **Boot KorrinOS** — GPU compute is instant: `nvidia-smi` works, PyTorch sees the GPU.
2. **Run the setup script**: `bash scripts/setup-gpu-stacks.sh` installs LLM, diffusion, RAPIDS basics.
3. **For pure software without any GPU hardware**: Run `software-gpu-run` — it demonstrates float-array execution on CPU only, no GPU, no vendor deps.
4. **For a pure software GPU simulation project**: The `software-gpu/` directory is 100% free software, distributable, but performance characteristics are clearly documented.

## CONTACT & CONTRIBUTION

This is free software. Contributions, bug reports, and feedback are welcome via the KorrinOS project channels. No vendor lock-in, no proprietary blobs beyond the NVIDIA driver (which is free to use under NVIDIA's license for computing purposes).

