f = open("input", "r")
x = 1
class Insc:
	def __init__(self, action, delay):
		self.action = action
		self.delay = delay
	def ready(self):
		ans = self.delay == 0
		self.delay -= 1
		return ans

inscs = []
ans = 0
i = 1
for line in f.read().split('\n'):
	if i == 20 or i == 60 or i == 100 or i == 140 or i == 180 or i == 220:
		ans += i * x

	i += 1

	parts = line.split(' ')

	if parts[0] == "addx":
		inscs.append(Insc(int(parts[1]), 1))

	for insc in reversed(inscs):
		if insc.ready():
			x += insc.action
			inscs.remove(insc)
	


print(ans)