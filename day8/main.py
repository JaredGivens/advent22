
f = open("input", "r").read()

row_len = f.index("\n") + 1

def get_vis(i, inc):
	v = 0
	h = int(f[i])
	i = inc(i)
	while i > 0:
		v += 1
		if h <= int(f[i]):
			return v

		i = inc(i)

	return v

def is_vis(i, inc):
	h = int(f[i])
	i = inc(i)
	while i > 0:
		if h <= int(f[i]):
			return False
		i = inc(i)

	return True

visable = 0
record = 0
for i in range(len(f)):
	if f[i] == "\n":
		continue

	right = is_vis(i, lambda x: x + 1 if f[x + 1] != "\n" else -1)
	left = is_vis(i, lambda x: x - 1 if f[x - 1] != "\n" else -1)
	down = is_vis(i, lambda x: x + row_len if x + row_len < len(f) else -1)
	up = is_vis(i, lambda x: x - row_len)
	if right or left or down or up:
		visable += 1

	rv = get_vis(i, lambda x: x + 1 if f[x + 1] != "\n" else -1)
	lv = get_vis(i, lambda x: x - 1 if f[x - 1] != "\n" else -1)
	dv = get_vis(i, lambda x: x + row_len if x + row_len < len(f) else -1)
	uv = get_vis(i, lambda x: x - row_len)
	s = rv * lv * dv * uv
	if s > record:
		record = s


print(visable)
print(record)