
class Dir:
	def __init__(self, parent):
		self.parent = parent
		self.children = []
		self.size = 0

	def compute_sizes(self):
		for child in self.children:
			self.size += child.compute_sizes()

		return self.size

	def find(self, min): 
		ans = None
		if self.size > min:
			ans = self

		for child in self.children:
			recur = child.find(min)
			if recur and recur.size > min and recur.size < ans.size:
				ans = recur
		
		return ans


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

root.compute_sizes()
have = 70_000_000 - root.size
print(have)
need = 30_000_000 - have 
print(need)
print(root.find(need).size)
