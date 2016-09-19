#!/bin/env/python3
from os import listdir
from os.path import splitext

CARGO_CONF = (
    ('name', '"programming-challenges"'),
    ('version', '"0.1.0"'),
    ('authors', '["Alexey Golubev <dr.freecx@gmail.com>"]')
)
CARGO_DEPS = (('lazy_static', '"*"'), )

if __name__ == '__main__':
    binary = lambda f: '[[bin]]\nname = "{}"\n'.format(splitext(f)[0])
    config = lambda d: '\n'.join(map(lambda l: '{} = {}'.format(*l), d))
    binaries = map(lambda f: binary(f), filter(lambda f: splitext(f)[1] == '.rs', listdir('src/')))
    with open('Cargo.toml', 'w') as f:
        f.write('[package]\n{}\n'.format(config(CARGO_CONF)))
        f.write('\n{}\n'.format('\n'.join(binaries)))
        f.write('[dependencies]\n{}'.format(config(CARGO_DEPS)))
