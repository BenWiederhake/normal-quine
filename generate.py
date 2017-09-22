#!/usr/bin/env python3


def generate(content):
    escaped = content.replace('\\', '\\\\').replace('"', '\\"')
    return content.replace('@', escaped)


if __name__ == '__main__':
    from sys import argv
    if len(argv) != 1:
        print('Does\'t take arguments.  Usage: ./generate.py')
        exit(1)
    with open('pre-src/main.rs', 'r') as fp_in:
        with open('src/main.rs', 'w') as fp_out:
            fp_out.write(generate(fp_in.read()))
