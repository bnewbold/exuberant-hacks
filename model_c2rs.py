#!/usr/bin/env python

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
        #print(nums)
        assert(len(nums) == 6)
        vertices.append(nums[:3])
        normals.append(nums[3:])
    infile.close()

    outfile.write("""
// This file auto-generated from %s.c using model_c2rs.py
// Don't edit by hand!

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32),
}
implement_vertex!(Vertex, position, normal);

pub const %s_vertices: [Vertex; %d] = [
""" % (fname, fname, len(vertices)))
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
