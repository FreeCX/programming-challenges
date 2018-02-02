#!/bin/env/python
from os import listdir
from os.path import splitext

CARGO_CONF = (
    ('name', '"programming-challenges"'),
    ('version', '"0.1.0"'),
    ('authors', '["Alexey Golubev <dr.freecx@gmail.com>"]')
)
CARGO_DEPS = (('lazy_static', '"*"'), ('time', '"*"'))
CARGO_EXTRA = (
    ('sdl2', {
        'version': '"0.29.1"',
        'default-features': 'false',
        'features': ['ttf']
    }),
)


def extra(extra_list):
    result = ''
    for item in extra_list:
        name, extra = item
        result += f'[dependencies.{name}]\n'
        for k, v in extra.items():
            if type(v) is list:
                result += f'{k} = [' + ','.join(map(lambda x: f'"{x}"', v)) + ']'
            else:
                result += f'{k} = {v}\n'
    return result


if __name__ == '__main__':
    binary = lambda f: '[[bin]]\nname = "{0}"\npath = "src/{0}.rs"\n'.format(splitext(f)[0])
    config = lambda d: '\n'.join(map(lambda l: '{} = {}'.format(*l), d))
    binaries = map(lambda f: binary(f), filter(lambda f: splitext(f)[1] == '.rs', listdir('src/')))
    with open('Cargo.toml', 'w') as f:
        f.write('[package]\n{}\n'.format(config(CARGO_CONF)))
        f.write('\n{}\n'.format('\n'.join(binaries)))
        f.write('[dependencies]\n{}'.format(config(CARGO_DEPS)))
        f.write('\n\n{}'.format(extra(CARGO_EXTRA)))
