import numpy as np
import matplotlib.pyplot as plt

from read_file import *
from indicators import *

numInstance=0
n=100
p=2

w=np.zeros(n,dtype=int)
v=np.zeros((n,p),dtype=int)
filename = "Data/"+str(n)+"_items/2KP"+str(n)+"-TA-"+str(numInstance)+".dat"

W=readFile(filename,w,v)

#Lecture des point non-dominées

filename = "Data/"+str(n)+"_items/2KP"+str(n)+"-TA-"+str(numInstance)+".eff"
YN=readPoints(filename,p)

#plt.grid()
#plt.scatter(YN[:,0],YN[:,1],color='blue')

YND=[]  #YND est la liste des solutions non-dominées (approximation)

#Génération de m solutions aléatoires :
m=100

YND=[]
for i in range(m):
	xStart=np.zeros(n,dtype=int)
	arr = np.arange(n)
	np.random.shuffle(arr)
	wTotal=0
	vStart=np.zeros(p,dtype=int)
	for i in range(n):
		if wTotal+w[arr[i]]<=W:
			xStart[arr[i]]=1
			wTotal=wTotal+w[arr[i]]
			for j in range(p):
				vStart[j]=vStart[j]+v[arr[i],j]

	#miseAJour(YND,[xStart,vStart])
print(YND)
#for sol in YND:
#	plt.scatter(sol[1][0],sol[1][1],color='red')

plt.show()

#Calcule de la proportion

#print("Proportion = ",proportion(YN,YND))

#Calcule de la distance DM

#print("DM =",DM(YN,YND,p))
