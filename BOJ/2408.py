expressions = "".join(open(0).readlines()[1:]).replace("\n", "").replace("/", "//")
output = eval(expressions)
print(output)
