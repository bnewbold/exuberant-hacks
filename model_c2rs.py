#!/usr/bin/env python

"""
This script is intended to be run in the same directory as a bunch of .c files;
it outputs .rs files.
"""

import sys

def main():
    if len(sys.argv) != 2 or not sys.argv[1].endswith(".c"):
        print("I take a single file to convert; must be a .c file")
        sys.exit(-1)

    fname = sys.argv[1][:-2]
    infile = open(fname + ".c", 'r')
    outfile = open(fname + ".rs", 'w')

    vertices = []
    normals = []
    for line in infile.readlines():
        if line.count(",") != 6:
            continue
        nums = line.strip().split(",")[:-1]
        nums = [n.find(".") != -1 and n or n+".0" for n in nums]
        #print(nums)
        assert(len(nums) == 6)
        normals.append(nums[:3])
        vertices.append(nums[3:])
    infile.close()

    outfile.write("""
// This file auto-generated from %s.c using model_c2rs.py
// Don't edit by hand!

use cow_vertex::Vertex;

pub const %s_VERTICES: [Vertex; %d] = [
""" % (fname, fname.upper(), len(vertices)))
    for i in range(len(vertices)):
        v = vertices[i]
        n = normals [i]
        outfile.write("    Vertex { position: (%s, %s, %s),\n" % (v[0], v[1], v[2]));
        outfile.write("             normal: (%s, %s, %s), },\n" % (n[0], n[1], n[2]));

    outfile.write( "];\n")
    outfile.close()
    print("Done!")

if __name__ == "__main__":
    main()
