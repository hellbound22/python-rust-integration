from . import _native

ffi = _native.ffi
# This will problably raise some problems in the future
buffer = _native.ffi.new('uint8_t[]', 10000) 

def parse(s):
    data = list(s)

    return _do_task(s, len(s), buffer, len(buffer))

def _do_task(s, l, o_s, o_l):
    ret = _native.lib.parse(s, l, o_s, o_l)

    o_s = _native.ffi.string(buffer, len(buffer))
    s = o_s.decode("utf-8")

    if ret < 0:
        raise Exception(s)
    else:
        return s

