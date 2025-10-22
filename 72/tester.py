DEBUG = 0
CMD = "cargo run --"

import subprocess, os, random
from operator import add, sub, mul, floordiv as quo, mod as rem

bigone, bigtwo = random.randint(2 ** 500, 2 ** 512), random.randint(2 ** 500, 2 ** 512)
hexone, hextwo = hex(bigone), hex(bigtwo)
DEBUG and print("\nhexone =\n", hexone, "\nhextwo = \n", hextwo)

from operator import add, sub, mul, floordiv as quo, mod as rem
ops = {'ADD':add,'SUB':sub,'MUL':mul,'QUO':quo,'REM':rem}
for op in ops:
    result = int(subprocess.check_output(["cargo", "run", hexone, hextwo, op]),16)
    answer = ops[op](bigone,bigtwo)
    if result != answer:
        print("Operator", op, "failed.")
        DEBUG and print("Expected:")
        DEBUG and print(hex(answer))
        DEBUG and print("Received:")
        DEBUG and print(hex(result))
        exit()
    else:
        print(op, "passes.")
