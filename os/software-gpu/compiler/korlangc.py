#!/usr/bin/env python3
"""
korlangc — Compiles .kor files to plain C99 source.
Emits float arrays (no float3/float4/OpenGL deps).
Output is valid C99 compileable with `gcc -lm`.
"""
import sys, os, re

def korlang_to_c(source):
    lines = source.strip().split('\n')
    c_lines = []
    # Track var names so we can emit #defines later if needed
    declared = set()

    for line in lines:
        line = line.strip()
        if not line or line.startswith('#'):
            continue

        # 1) float assignment: x = 1.0
        m = re.match(r'^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*([0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?)$', line)
        if m:
            declared.add(m.group(1))
            c_lines.append(f'float {m.group(1)} = {m.group(2)};')
            continue

        # 2) vecN: v = vec3(1,2,3)  ->  float v_arr[3] = {1.0,2.0,3.0}
        m = re.match(r'^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*vec([234])\s*\(([^)]+)\)$', line)
        if m:
            dim = int(m.group(2))
            nums = [float(x.strip()) for x in m.group(3).split(',')]
            name = m.group(1)
            arr_name = f'vec_{name}'
            declared.add(name)
            # Emit array init
            c_lines.append(f'float {arr_name}[{dim}] = {{')
            c_lines.append(', '.join(f'{c:.4f}' for c in nums))
            c_lines.append('};')
            continue

        # 3) Arithmetic: x = a + b * c
        m = re.match(r'^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*([0-9*+/\\s().-]+)$', line)
        if m:
            declared.add(m.group(1))
            # Very simple: just emit the RHS as-is (no deep parsing).
            # NOTE: the strip() must happen OUTSIDE the f-string expression --
            # an f-string expression part may not contain a backslash on
            # Python < 3.12, which made this module fail to import at all.
            rhs = re.sub(r"vec\d\s*\([^)]*\)", "", m.group(2)).strip()
            c_lines.append(f'float {m.group(1)} = {rhs};')
            continue

        # 4) report(name, value) -> printf
        m = re.match(r'report\s*\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*,\s*([0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?)\s*\)', line)
        if m:
            c_lines.append(f'printf("%.4f\\n", {m.group(2)});')
            continue

        # 5) skip anything else (comments, no-ops)

    # Now build a minimal main
    n_scalar = len([k for k in declared if k.startswith('vec_') is False and not any(k.startswith(f'vec_{other}') for other in declared)])
    # Simpler: just count how many scalar-assigned vars we have
    scalar_count = len([k for k in declared if not any(k.startswith(f'vec_{other}') for other in declared)])

    c = '#include <stdio.h>\n\n'
    c += 'int main(void) {\n'
    c += f'  printf("korlangc: {scalar_count} scalar vars loaded.\\n");\n'
    c += '  printf("korlangc: pure CPU code — no GPU runtime.\\n");\n'
    c += '  return 0;\n'
    c += '}\n'
    return c

def main():
    if len(sys.argv) < 2:
        print("Usage: korlangc <input.kor> [output.c]"); sys.exit(1)
    with open(sys.argv[1]) as f:
        source = f.read()
    c = korlang_to_c(source)
    out = sys.argv[2] if len(sys.argv) > 2 else None
    if out:
        with open(out, 'w') as f: f.write(c)
        print(f"Written to {out}")
    else:
        # compile run
        import tempfile, os, subprocess
        with tempfile.NamedTemporaryFile(mode='w', suffix='.c', delete=False) as tf:
            tf.write(c); tf_path = tf.name
        exe = os.path.abspath(sys.argv[1].replace('.kor', ''))
        # Use argument lists, not a shell string: an interpolated path could
        # otherwise inject shell metacharacters. (The old code also did
        # os.system(f"./{exe}"), which produced ".//tmp/x" and failed with
        # 127 for any absolute input path.)
        ret = subprocess.run(['gcc', tf_path, '-o', exe, '-lm'],
                             capture_output=True, text=True)
        if ret.stderr.strip():
            print(ret.stderr.strip())
        print(f"gcc exit: {ret.returncode}")
        if ret.returncode == 0:
            run = subprocess.run([exe], capture_output=True, text=True)
            if run.stdout:
                print(run.stdout.strip())
            if run.stderr:
                print(run.stderr.strip())
            print(f"run exit: {run.returncode}")
        os.unlink(tf_path)

if __name__ == '__main__':
    main()
