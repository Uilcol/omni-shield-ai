import cbor2
import sys
from pprint import pprint

with open(sys.argv[1], "rb") as f:
    pprint(cbor2.load(f))
