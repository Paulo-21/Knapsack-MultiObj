import matplotlib.pyplot as plt
file = open("output.txt")
x = [0]
pls1 = [0]
pls2 = [0]
pls2_opt = [0]
for line in file:
    k = line.strip().split()
    x.append(int(k[0]))
    pls1.append(int(k[1]))
    pls2.append(int(k[2]))
    pls2_opt.append(int(k[3]))

fig, ax = plt.subplots()
ax.plot(x, pls1, 'x-', label='PLS1')
ax.plot(x, pls2, 's-', label='PLS2')
ax.plot(x, pls2_opt, 'o-', label='PLS2 Opt')

ax.set_xlabel("Nb Object")
ax.set_ylabel("Milli secondes")
ax.set_title("Comparaison des temps de résolution")
ax.legend()
plt.show()
