#!/usr/bin/env python3
"""
make-themes.py -- assemble complete KorrinOS Xcursor themes.

Builds every theme under os/assets/cursors/ from the two artist-made assets
plus the synthesised designs, so the "Mouse Designs" picker has a real,
bundled, offline set to offer.

THEME LAYOUT (the Xcursor icon-theme spec)
------------------------------------------
    <theme>/index.theme        Directories=cursors, Type=Xcursor
    <theme>/cursors/default    the theme's primary cursor
    <theme>/cursors/left_ptr   ...
    <theme>/cursors/<role>     one file per X11 cursor name

`gsettings set org.gnome.desktop.interface cursor-theme <name>` resolves
against a directory of this name under a XDG icon directory, which is why the
installed directory name matters more than anything inside the theme.

ROLE MAPPING
------------
Per the product decision this implements:

  * the animated Walter White Heisenberg cursor is `default`, i.e. the cursor
    used everywhere the theme does not specify something more specific;
  * the Witch Crystal Staff cursor is the POINTER (`pointer`/`left_ptr`), the
    cursor drawn under the click, and it is static -- it is five sizes of one
    image, not animation frames.

Every other X11 cursor name (text, wait, resize, drag-and-drop, ...) also falls
back to Walter, because the source artwork provides only those two images and
inventing the rest from them would look worse than reusing the base cursor.
"""

import argparse
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

# The tools here use dashes in their filenames (convert-sweezy.py) which are
# not importable module names, so load them by path rather than renaming files
# and breaking how the OS build invokes them.
import importlib.util


def _load(filename, modname):
    spec = importlib.util.spec_from_file_location(
        modname, os.path.join(HERE, filename))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


conv = _load('convert-sweezy.py', 'convert_sweezy')
synth = _load('synth-cursors.py', 'synth_cursors')
import xcursor as xc

DEFAULT_THEME = 'korrinos-walter'

# role -> source name. Any X11 cursor name absent here falls back to the
# theme's `default` source.
ART_ROLES = {
    'default': 'walter',
    'pointer': 'witch',
    'left_ptr': 'witch',
    'arrow': 'witch',
    'top_left_arrow': 'witch',
}

THEMES = {
    'korrinos-walter': dict(
        title='Walter White (animated default, Witch Staff pointer)',
        artist=True,
        roles=ART_ROLES,
    ),
    'korrinos-walter-only': dict(
        title='Walter White everywhere (animated)',
        artist=True,
        roles={},
    ),
    'korrinos-witch-only': dict(
        title='Witch Crystal Staff everywhere (static)',
        artist=True,
        roles={},
        fallback='witch',
    ),
}

DEFAULT_FALLBACK = 'walter'


def load_sources(sources, sizes):
    """Parse each source asset once and build its full size ladder.

    Returns {name: {size: [(image, hotx, hoty)]}} plus a small info dict for
    reporting, so the build output can state what actually happened.
    """
    from PIL import Image

    out = {}
    info = {}
    for name, path in sources.items():
        if path.lower().endswith('.ani'):
            frames, meta = conv.parse_ani(path)
        elif path.lower().endswith('.cur'):
            frames, meta = conv.parse_cur(path)
        else:
            raise ValueError("unsupported source: %s" % path)
        # Sizes absent from the source are derived from the nearest larger
        # native image, with the hotspot rescaled in step. Native sizes are
        # used verbatim, so nothing is invented at 16px.
        ladder = conv.build_size_ladder(frames, sizes,
                                        down_filter=Image.LANCZOS,
                                        up_filter=Image.LANCZOS)
        out[name] = ladder
        info[name] = dict(
            frames=len(frames),
            animated=len(frames) > 1,
            delay_ms=frames[0]['delay_ms'],
            native=sorted({s for f in frames for s in f['images']}),
            title=meta.get('title'),
        )
    return out, info


def build_theme(name, spec, sources, outdir, sizes, info):
    cursors = os.path.join(outdir, name, 'cursors')
    os.makedirs(cursors, exist_ok=True)

    roles = dict(spec.get('roles') or {})
    fallback = spec.get('fallback') or DEFAULT_FALLBACK

    written = 0
    counts = {}
    groups = {}
    for role in sorted(synth.ROLES):
        src = roles.get(role, fallback)
        ladder = sources[src]
        imgs = []
        if info[src]['animated']:
            # An animated source contributes every frame at every size.
            for size in sizes:
                for _fi, (im, hx, hy) in enumerate(ladder[size]):
                    imgs.append(dict(image=im, hotx=hx, hoty=hy,
                                     delay=info[src]['delay_ms'],
                                     nominal=size))
        else:
            for size in sizes:
                im, hx, hy = ladder[size][0]   # single frame
                imgs.append(dict(image=im, hotx=hx, hoty=hy, delay=0,
                                 nominal=size))
        counts[src] = counts.get(src, 0) + 1
        groups.setdefault(xc.content_key(imgs), (imgs, []))[1].append(role)

    # Identical roles share one file via symlink; see xcursor.write_group.
    for _key, (imgs, role_names) in groups.items():
        paths = [os.path.join(cursors, r) for r in role_names]
        xc.write_group(paths, imgs)
        written += len(role_names)

    synth.write_index_theme(
        os.path.join(outdir, name), name, title=spec.get('title', name),
        comment=('' if spec.get('artist') else
                 'Synthesised for KorrinOS by os/assets/cursors/'
                 'synth-cursors.py.'))

    # Record provenance in the theme itself, so a theme installed on a system
    # can be traced back to its source even outside the repo.
    if spec.get('artist'):
        with open(os.path.join(outdir, name, 'SOURCES'), 'w') as f:
            for src in sorted(set(list(roles.values()) + [fallback])):
                m = info[src]
                f.write("%s: %s\n" % (src, m.get('title') or src))
                f.write("  frames=%d animated=%s delay=%sms native_sizes=%s\n"
                        % (m['frames'], m['animated'], m['delay_ms'],
                           m['native']))
            f.write("See os/assets/cursors/NOTICE.md for full attribution.\n")

    return dict(name=name, roles=written, sources=counts)


def write_manifest(outdir, built, info):
    """Write a manifest describing every bundled theme.

    The Settings picker reads this instead of hardcoding theme names, so
    adding a theme is a matter of adding it to THEMES and rebuilding. It also
    records, per theme, which source asset each cursor came from and whether
    that asset is third-party artwork.
    """
    import json

    entries = []
    for name in built:
        if name in THEMES:
            spec = THEMES[name]
            roles = dict(spec.get('roles') or {})
            fb = spec.get('fallback') or DEFAULT_FALLBACK
            entries.append(dict(
                name=name,
                title=spec.get('title', name),
                origin='artist',
                animated=any(info[s]['animated']
                             for s in set(list(roles.values()) + [fb])),
                pointer_source=roles.get('pointer', fb),
                default_source=roles.get('default', fb),
            ))
        else:
            entries.append(dict(
                name=name,
                title=synth.THEMES[name]['title'],
                origin='synthesised',
                animated=False,
                pointer_source='synthesised',
                default_source='synthesised',
            ))

    manifest = dict(
        default=DEFAULT_THEME,
        install_prefix='/usr/share/icons/korrinos-cursors',
        sizes=sizes_default(),
        themes=entries,
    )
    path = os.path.join(outdir, 'manifest.json')
    with open(path, 'w') as f:
        json.dump(manifest, f, indent=2, sort_keys=True)
        f.write('\n')
    print("  manifest: %s (%d themes)" % (path, len(entries)))
    return path


def sizes_default():
    return list(conv.STANDARD_SIZES)


def main():
    ap = argparse.ArgumentParser(
        description="Assemble the bundled KorrinOS Xcursor themes.")
    ap.add_argument('-o', '--outdir', required=True)
    ap.add_argument('--sizes', default=None)
    ap.add_argument('--include-synth', action='store_true',
                    help="also build the synthesised themes")
    args = ap.parse_args()

    from PIL import Image

    sizes = ([int(s) for s in args.sizes.split(',')] if args.sizes
             else list(conv.STANDARD_SIZES))

    src_dir = os.path.join(HERE, 'source')
    sources, info = load_sources({
        'walter': os.path.join(src_dir, 'walter-animated.ani'),
        'witch': os.path.join(src_dir, 'witch-staff.cur'),
    }, sizes)

    for src, m in sorted(info.items()):
        print("  source %-7s frames=%d animated=%s delay=%sms native=%s"
              % (src, m['frames'], m['animated'], m['delay_ms'], m['native']))
        if m['title']:
            print("           title: %s" % m['title'])

    built = []
    for name, spec in sorted(THEMES.items()):
        st = build_theme(name, spec, sources, args.outdir, sizes, info)
        built.append(name)
        print("  theme %-22s %d roles (from %s)"
              % (name, st['roles'],
                 ", ".join("%s:%d" % kv for kv in sorted(st['sources'].items()))))

    if args.include_synth:
        for name, spec in sorted(synth.THEMES.items()):
            st = synth.build_theme(
                name, os.path.join(args.outdir, name),
                fill=spec['fill'], outline=spec['outline'],
                outline_w=spec['outline_w'], sizes=sizes,
                title=spec['title'], scale=spec.get('scale', 1.0))
            built.append(name)
            print("  theme %-22s %d roles (synthesised)"
                  % (name, st['roles']))

    if DEFAULT_THEME not in built:
        print("ERROR: default theme %s was not built" % DEFAULT_THEME,
              file=sys.stderr)
        return 1
    write_manifest(args.outdir, built, info)
    print("  default theme: %s" % DEFAULT_THEME)
    return 0


if __name__ == '__main__':
    sys.exit(main())