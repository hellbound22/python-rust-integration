import task

f = open("input.xlsx", "rb")
#f = open("test.py", "rb")

s = f.read()

print(task.parse(s))
