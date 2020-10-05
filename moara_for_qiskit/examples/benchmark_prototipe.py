import numpy as np 
import time
import matplotlib.pyplot as plt

from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit, execute, Aer
from qiskit.providers.models import QasmBackendConfiguration
from qiskit.providers import BaseBackend


def entCX(qc,i):
    for l in range(i-1):
        qc.cx(l,l+1)

def RxRy(qc,i):
    for l in range(i):
        qc.rx(np.pi/3,l)
        qc.ry(np.pi/5,l)

def test(i,j):
    qc=QuantumCircuit(i,i)
    for k in range (j):
        entCX(qc,i)
        RxRy(qc,i)
    qc.measure_all
    return qc

def bench(simulator):
    i=2
    j=2
    depth=[]
    for i in range(2,22):
        duration=0
        while (duration<0.20) and j<101:
            test_qc=test(i,j)
            start_time = time.time()
            execute(test_qc, simulator, shots=1000)
            duration=(time.time() - start_time)
            j=j+1
        depth.append(j-1)
        j=2
    return depth

def scor(depth):
    sc=0
    i=2
    for x in depth:
        sc=sc+x*i
        i=i+1
    return sc

def plot(depth,simulator_name='simulator' ):

    fig = plt.figure()
    ax = fig.add_axes([0,0,1,1])
    ax.set_xscale('linear')
    ax.set_xlim([1.5, 22])
    qub = [i for i in range(2,22)]
    ax.bar(qub,depth)
    ax.set_title(' MaxDepth(nr,Qubits) ['+ simulator_name+']')
    ax.set_ylabel('nr. Qubits')
    ax.set_xlabel('Depth (RxRy_Cx)')
    plt.show()