#!/usr/bin/env python
import array, msgpack, random, sys

argc = len(sys.argv)
max_source_len = 4 

if argc < 5 or argc > 6:
    sys.exit('Usage: script <rows> <cols> <source out> <selected row ind out> [verbose (defaults to False)]')

# initialization
rows = int(sys.argv[1])
cols = int(sys.argv[2])
source_out_path = sys.argv[3]
selected_row_out_path = sys.argv[4]

if argc >= 6:
    is_verbose = bool(sys.argv[5])
else:
    is_verbose = False

# source matrix
source = []

def gen_val():
    return random.uniform(-1.0, 1.0)

for x in range(0, rows * cols):
    source.append([gen_val(), gen_val()])

with open(source_out_path, mode='wb') as f:
    f.write(msgpack.packb([rows, cols, source]))

if is_verbose:
    with open(source_out_path, mode='rb') as f:
        print("First {} values of source: {}".format(max_source_len, msgpack.unpackb(f.read())[2][:max_source_len]))

# selected row
selected_row = random.randint(0, rows - 1)

with open(selected_row_out_path, mode='wb') as f:
    f.write(msgpack.packb(selected_row))

if is_verbose:
    with open(selected_row_out_path, mode='rb') as f:
        print("Selected row index: {}".format(msgpack.unpackb(f.read())))
