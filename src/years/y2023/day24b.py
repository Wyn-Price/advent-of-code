from z3 import *

stones = []

for line in sys.stdin:
    pos, vel = line.split('@')
    x, y, z = pos.split(', ')
    vx, vy, vz = vel.split(', ')
    x, y, z = int(x),int(y),int(z)
    vx, vy, vz = int(vx),int(vy),int(vz)
    stones.append(((x,y,z),(vx,vy,vz)))

x, y, z, vx, vy, vz = Int('x'),Int('y'),Int('z'),Int('vx'),Int('vy'),Int('vz')

solver = Solver()

# I think we only need to check 3 stones, but may need more?
for i in range(max(3, len(stones))):
    t = Int(f'T{i}')
    p, v = stones[i]
    # rp + t*rv == sp + t*sv
    solver.add(x + t*vx - p[0] - t*v[0] == 0)
    solver.add(y + t*vy - p[1] - t*v[1] == 0)
    solver.add(z + t*vz - p[2] - t*v[2] == 0)

res = solver.check()
M = solver.model()
print(M.eval(x+y+z))