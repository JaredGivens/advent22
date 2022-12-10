
class Dir:
	def __init__(self, parent):
		self.parent = parent
		self.children = []
		self.size = 0

	def compute_sizes(self, ans):
		for child in self.children:
			self.size += child.compute_sizes(ans)

		if self.size < 100_000:
			ans.size += self.size

		return self.size


f = open("input", "r")
root = Dir(None)
current = root
for line in f.read().split('\n'):
	if line:
		if line[0] == '$':
			if line[2] == 'c':
				target = line[5:]
				if target == '..':
					current = current.parent
				elif target == '/':
					current = root
				else:
					current.children.append(Dir(current))
					current = current.children[-1]
		elif line[2] != 'r':
			current.size += int(line.split(' ')[0])

ans = Dir(None)
root.compute_sizes(ans)
print(ans.size)