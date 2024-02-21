import ogr_rust as ogr
import sys
import argparse


parser = argparse.ArgumentParser(
)

parser.add_argument('length', type=int)
parser.add_argument('order', type=int)

args = parser.parse_args()


rulers = ogr.enumerate_rulers(args.length)
for r in rulers:
    if r.order() == args.order:
        print(r)