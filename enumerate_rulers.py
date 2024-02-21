import ogr_rust as ogr
import sys

rulers = ogr.enumerate_rulers(int(sys.argv[1]))
for r in rulers:
    print(r)