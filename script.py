from pkg1 import read_from_file
from io import StringIO

print("*** this works: ***")
read_from_file(StringIO("micrometer and text > 16 chars long"))

print("\n*** this also does (because < 16 chars): ***")
read_from_file(StringIO("µicrometer"))

print("\n*** this doesn't: ***")
read_from_file(StringIO("µicrometer and text > 16 chars long"))

print("\n*** right below the threshold: ***")
read_from_file(StringIO("µ23456789012345"))

print("\n*** right above the threshold: ***")
read_from_file(StringIO("µ234567890123456"))
