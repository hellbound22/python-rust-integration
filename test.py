import task
from task import buffer

s = list("teste".encode("utf-8"))

print(task.test(s, len(s), task.buffer, len(buffer)))

o = task.ffi.string(buffer, len(buffer))
print(o.decode("utf-8") )

