#!/bin/env/python3
from pathlib import Path


CARGO_CONF = (
    ('name', '"programming-challenges"'),
    ('version', '"0.1.0"'),
    ('authors', '["Alexey Golubev <dr.freecx@gmail.com>"]')
)
CARGO_DEPS = (('lazy_static', '"*"'), ('time', '"*"'))
CARGO_EXTRA = (
    ('sdl2', {
        'version': '"*"',
        'default-features': 'false',
        'features': ['ttf']
    }),
)


def extra(extra_list):
    r = ''
    fx = lambda x: f'"{x}"'
    for item in extra_list:
        name, extra = item
        r += f'[dependencies.{name}]\n'
        for k, v in extra.items():
            r += f'{k} = [' + ','.join(map(fx, v)) + ']' if isinstance(k, list) else f'{k} = {v}\n'
    return r


if __name__ == '__main__':
    binary = lambda f: f'[[bin]]\nname = "{f.stem}"\npath = "{f}"\n'
    config = lambda d: '\n'.join(map(lambda l: '{} = {}'.format(*l), d))
    split = lambda f: f.suffix == '.rs'
    binaries = sorted(map(binary, filter(split, Path('./src').iterdir())))
    with open('Cargo.toml', 'w') as f:
        f.write('[package]\n{}\n'.format(config(CARGO_CONF)))
        f.write('\n{}\n'.format('\n'.join(binaries)))
        f.write('[dependencies]\n{}'.format(config(CARGO_DEPS)))
        f.write('\n\n{}'.format(extra(CARGO_EXTRA)))
