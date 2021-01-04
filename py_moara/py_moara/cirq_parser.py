import numpy as np

class CirqParser():

    SUPPORTED_GATES = [
        'MeasurementGate',
        'IdentityGate',
        'X',
        'Y',
        'Z',
        'rx',
        'ry',
        'rz',
        'XPowGate',
        'YPowGate',
        'ZPowGate',
        'S',
        'T',
        'H',
        'CNOT',
        'CZPowGate'
    ]

    def parse(self, circuit):
        qubit_map = {q: i for i, q in enumerate(sorted(circuit.all_qubits()))}

        steps = []
        for i, moment in enumerate(circuit.moments):
            if not moment or not moment.operations:
                continue

            gates = []
            for operation in moment.operations:
                gate = None
                if type(operation).__name__ == 'GateOperation':
                    gate = self._parse_cirq_gate_operation(operation, qubit_map)
                elif type(operation).__name__ == 'ControlledOperation':
                    gate = self._parse_cirq_controlled_gate_operation(operation, qubit_map)
                
                if gate == None:
                    raise 'Could not parse cirq gate type'

                gates.append(gate)

            steps.append({'index':i, 'gates':gates})
        
        return {"steps":steps}, len(qubit_map)

    def _parse_cirq_gate_operation(self, gateOperation, qubit_map):
        gate_name = type(gateOperation.gate).__name__
        if gate_name == 'MeasurementGate':
            return { 'name': 'measure-z', 'target':qubit_map[gateOperation.qubits[0]] }
        elif gate_name == 'IdentityGate':
            return { 'name': 'identity', 'target':qubit_map[gateOperation.qubits[0]] }
        elif gate_name == '_PauliX':
            return { 'name': 'pauli-x', 'target':qubit_map[gateOperation.qubits[0]] }
        elif gate_name == '_PauliY':
            return { 'name': 'pauli-z', 'target':qubit_map[gateOperation.qubits[0]] }
        elif gate_name == '_PauliZ':
            return { 'name': 'pauli-z', 'target':qubit_map[gateOperation.qubits[0]] }
        elif gate_name == 'XPowGate':
            return { 'name': 'rx-phi', 'target':qubit_map[gateOperation.qubits[0]], 'phi':np.pi*gateOperation.gate.exponent }
        elif gate_name == 'YPowGate':
            return { 'name': 'ry-phi', 'target':qubit_map[gateOperation.qubits[0]], 'phi':np.pi*gateOperation.gate.exponent }
        elif gate_name == 'ZPowGate':
            if gateOperation.gate.exponent == 0.5:
                return { 'name': 's', 'target':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == -0.5:
                return { 'name': 's_dagger', 'target':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == 0.25:
                return { 'name': 't', 'target':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == -0.25:
                return { 'name': 't_dagger', 'target':qubit_map[gateOperation.qubits[0]] }
            return { 'name': 'rz-phi', 'target':qubit_map[gateOperation.qubits[0]], 'phi':np.pi*gateOperation.gate.exponent }
        elif gate_name == 'HPowGate':
            if gateOperation.gate.exponent != 1:
                raise 'Exponent of H is not supported yet'
            return { 'name': 'hadamard', 'target':qubit_map[gateOperation.qubits[0]] }
        elif gate_name == 'CXPowGate':
            if gateOperation.gate.exponent == 1:
                return { 'name': 'ctrl-pauli-x', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]] }
            return { 'name': 'ctrl-rx-phi', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]], 'phi':np.pi*gateOperation.gate.exponent }
        elif gate_name == 'CZPowGate':
            if gateOperation.gate.exponent == 1:
                return { 'name': 'ctrl-pauli-z', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == 0.5:
                return { 'name': 'ctrl-s', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == -0.5:
                return { 'name': 'ctrl-s-dagger', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == 0.25:
                return { 'name': 'ctrl-t', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]] }
            if gateOperation.gate.exponent == -0.25:
                return { 'name': 'ctrl-t-dagger', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]] }
            return { 'name': 'ctrl-rz-phi', 'target':qubit_map[gateOperation.qubits[1]], 'control':qubit_map[gateOperation.qubits[0]], 'phi':np.pi*gateOperation.gate.exponent }
        elif gate_name == 'SwapPowGate':
            if gateOperation.gate.exponent != 1:
                raise 'Exponent of SWAP is not supported yet'
            return { 'name': 'swap', 'target':qubit_map[gateOperation.qubits[0]], 'target2':qubit_map[gateOperation.qubits[1]] }
        
        else:
            raise Exception(f'Gate `{gate_name}` is not suppoerted yet')

    def _parse_cirq_controlled_gate_operation(self, gateOperation, qubit_map):
        inner_operation = self._parse_cirq_gate_operation(gateOperation.sub_operation, qubit_map)

        if type(inner_operation) is list:
            raise 'Controlled measurement is not supported'
                
        if len(gateOperation.controls) > 1 or inner_operation['name'].startswith('ctrl-'):
            raise 'Multi-controlled gates are not supported yet'

        if len(gateOperation.controls) == 1:
            inner_operation['control'] = qubit_map[gateOperation.controls[0]]
            inner_operation['name'] = 'ctrl-'+inner_operation['name']

        return inner_operation