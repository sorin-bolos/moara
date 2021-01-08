from qiskit.providers.models import QasmBackendConfiguration
from qiskit.providers import BaseBackend
from .moara_for_qiskit import simulate
import json

class MoaraBackend(BaseBackend):
    
    NAME = 'moara_for_qiskit'
    VERSION = '0.3'
    MAX_QUBITS_COUNT = 20
    MAX_SHOTS = int(1e6)

    CONFIGURATION = { 'backend_name': NAME,
        'backend_version': VERSION,
        'n_qubits': MAX_QUBITS_COUNT,
        'simulator': True,
        'local': True,
        'conditional': True,
        'memory': False,
        'open_pulse': False,
        'max_shots': MAX_SHOTS,
        'description': 'A simulator',
        'coupling_map': None,
        'basis_gates': ['cx','id', 'x', 'y', 'z', 'h', 's', 'sdg', 't', 'tdg', 'u3', 'u2', 'u', 'sx', 'rx', 'ry', 'rz', 'swap', 'crx'],
        'gates': []
}

    def __init__(self, little_endian=False):
       super().__init__(QasmBackendConfiguration.from_dict(self.CONFIGURATION), None)
       self.little_endian = little_endian

    def run(self, qobj, backend_options=None, noise_model=None, validate=False):
        if not qobj or not qobj.experiments:
            return {}
        experiment = qobj.experiments[0]

        if len(qobj.experiments) > 1:
            raise Exception('Multiple experiments are not supported yet.')

        circuit = { "steps":[] }
        index = 0
        for instruction in experiment.instructions:
            if not instruction or len(instruction.qubits) == 0:
                continue

            gate = None
            if instruction.name == 'id':
                gate = { 'name': 'identity', 'target':instruction.qubits[0] }
            elif instruction.name == 'x':
                gate = { 'name': 'pauli-x', 'target':instruction.qubits[0] }
            elif instruction.name == 'y':
                gate = { 'name': 'pauli-y', 'target':instruction.qubits[0] }
            elif instruction.name == 'z':
                gate = { 'name': 'pauli-z', 'target':instruction.qubits[0] }
            elif instruction.name == 'h':   
                gate = { 'name': 'hadamard', 'target':instruction.qubits[0] }
            elif instruction.name == 'sx':   
                gate = { 'name': 'sqrt_not', 'target':instruction.qubits[0] }
            elif instruction.name == 't':   
                gate = { 'name': 't', 'target':instruction.qubits[0] }
            elif instruction.name == 'tdg':   
                gate = { 'name': 't_dagger', 'target':instruction.qubits[0] }
            elif instruction.name == 's':   
                gate = { 'name': 's', 'target':instruction.qubits[0] }
            elif instruction.name == 'sdg':   
                gate = { 'name': 's_dagger', 'target':instruction.qubits[0] }

            elif instruction.name == 'u3':
                gate = { 'name': 'u', 'target':instruction.qubits[0],'theta': instruction.params[0], 'phi': instruction.params[1], 'lambda': instruction.params[2] }
            elif instruction.name == 'u2':
                gate = { 'name': 'u-phi-theta', 'target':instruction.qubits[0], 'phi': instruction.params[0], 'theta': instruction.params[1] }
            elif instruction.name == 'rx':
                gate = { 'name': 'rx-phi', 'target':instruction.qubits[0], 'phi': instruction.params[0] }
            elif instruction.name == 'ry':
                gate = { 'name': 'ry-phi', 'target':instruction.qubits[0], 'phi': instruction.params[0] }
            elif instruction.name == 'rz':
                gate = { 'name': 'rz-phi', 'target':instruction.qubits[0], 'phi': instruction.params[0] }

            elif instruction.name == 'cx':
                gate = { 'name': 'ctrl-pauli-x', 'control':instruction.qubits[0], 'target':instruction.qubits[1] }
            elif instruction.name == 'crx':
                gate = { 'name': 'ctrl-rx-phi', 'control':instruction.qubits[0], 'target':instruction.qubits[1], 'phi': instruction.params[0] }
            elif instruction.name == 'swap':
                gate = { 'name': 'swap', 'target':instruction.qubits[0], 'target2':instruction.qubits[1] }
            elif instruction.name == 'measure':
                gate = { 'name': 'measure-z', 'target':instruction.qubits[0] }

            if gate:
                step = { 'index':index, 'gates':[gate] }
                circuit['steps'].append(step)
                index += 1
                
        serializedCircuit = json.dumps(circuit)
        
        return simulate(serializedCircuit, qobj.config.shots, experiment.config.n_qubits, self.little_endian)