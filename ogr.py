"""Script with arguments to """


import argparse
import sys
import ogr_rust as ogr

parser = argparse.ArgumentParser(prog='OGR')
subparsers = parser.add_subparsers()

parser_enum = subparsers.add_parser('enum', help="Enumerate rulers")
parser_enum.add_argument('length', type=int, help="The max length of the Ruler")
parser_enum.add_argument('--subcommand', default='enum', help=argparse.SUPPRESS)
parser_enum.add_argument('-o', '--order', type=int, default=None, help="Restrict the order of our rulers")
parser_enum.add_argument('-g', '--golomb', action='store_true', help="Keep only golomb rulers")
parser_enum.add_argument('-x', '--exact', action='store_true', help="Only print rulers whose length matches `length` exactly")

parser_search = subparsers.add_parser('search', help="Look for an optimal ruler")
parser_search.add_argument('order', help='The order of the ruler to search for')
parser_search.add_argument('--subcommand', default='search', help=argparse.SUPPRESS)

parser_rand = subparsers.add_parser('rand', help="Generate random rulers")
parser_rand.add_argument('length', help='The length of the ruler')
parser_rand.add_argument('--subcommand', default='rand', help=argparse.SUPPRESS)

args = parser.parse_args()

try:
    if args.subcommand is None:
        pass
except:
    parser.print_help()
    exit(1)

if args.subcommand == 'enum':
    print("Executing enum!")
    rulers = ogr.enumerate_rulers(args.length)

    if args.golomb:
        rulers = [r for r in rulers if r.is_golomb_ruler()]

    if args.order is not None:
        rulers = [r for r in rulers if r.order() == args.order]

    if args.exact:
        rulers = [r for r in rulers if r.length() == args.length]

    for r in rulers:
        print(r)

    if args.golomb and args.order:
        print(f"{len(rulers)} golomb rulers with order {args.order} and max length {args.length}")

elif args.subcommand == 'search':
    print("Searching!!")

    # We want to iterate non stop until we've found a ruler
    for i in range(10):
        print(i, ogr.Ruler.from_id(i))


elif args.subcommand == 'rand':

    print("Rand!")


# ---------------------------------------------------------------------------- #
#                                 Library code                                 #
# ---------------------------------------------------------------------------- #
# Functions to get some random rulers
# What's the number to length calculation?
import math
import random

"""Return the id range of rulers with length `length`."""
def range_ruler_length(length: int) -> tuple[int, int]:
    if length == 0: return (0, 0)
    elif length == 1: return (1, 1)
    else: return (2 ** (length - 1), (2 ** length) - 1)

def get_random_length(length: int):
    range = range_ruler_length(length)
    return ogr.Ruler.from_id(random.randint(range[0], range[1]))