f = open("input", "r")
x = 1
class Insc:
	def __init__(self, action, delay):
		self.action = action
		self.delay = delay
	def ready(self):
		self.delay -= 1
		return self.delay == 0

inscs = []
ans = 0
i = 1
def cycle(i, x):
	if i == 20 or i == 60 or i == 100 or i == 140 or i == 180 or i == 220:
		return i * x
	return 0

for line in f.read().split("\n"):
	ans += cycle(i,x)
	i += 1

	parts = line.split(" ")


	if parts[0] == "addx":
		ans += cycle(i, x)
		i += 1
		x += int(parts[1])
	# 	inscs.append(Insc(int(parts[1]), 2))
	
	# if len(inscs):
			
			

		


	for insc in reversed(inscs):
		if insc.ready():
			x += insc.action
			inscs.remove(insc)
	


print(ans)