class PyquilParser():

    SUPPORTED_GATES = [
        'I',
        'X',
        'Y',
        'Z',
        'H',
        'S',
        'RX(%theta)',
        'RY(%theta)',
        'RZ(%theta)',
        'PHASE(%theta)',
        'CNOT',
        'CZ',
    ]

    OPERATION_MAP = {
        'I':'identity',
        'X':'pauli-x',
        'Y':'pauli-y',
        'Z':'pauli-z',
        'H':'hadamard',
        'S':'s',
        'T':'t',
        'S-DAGGER':'s-dagger',
        'T-DAGGER':'t-dagger',
        'RX':'rx-phi',
        'RY':'ry-phi',
        'RZ':'rz-phi',
        'PHASE':'r-phi',

        'CNOT':'ctrl-pauli-x',
        'CZ':'ctrl-pauli-z',
        
        'CONTROLLED-I':'ctrl-identity',
        'CONTROLLED-X':'ctrl-pauli-x',
        'CONTROLLED-Y':'ctrl-pauli-y',
        'CONTROLLED-Z':'ctrl-pauli-z',
        'CONTROLLED-H':'ctrl-hadamard',
        'CONTROLLED-S':'ctrl-s',
        'CONTROLLED-T':'ctrl-t',
        'CONTROLLED-S-DAGGER':'ctrl-s-dagger',
        'CONTROLLED-T-DAGGER':'ctrl-t-dagger',
        'CONTROLLED-RX':'ctrl-rx-phi',
        'CONTROLLED-RY':'ctrl-ry-phi',
        'CONTROLLED-RZ':'ctrl-rz-phi',
        'CONTROLLED-PHASE':'ctrl-r-phi'
    }

    def parse(self, program):
        max_qubit_index = -1

        steps = []
        i = 0
        for instruction in program.instructions:
            instruction_type = type(instruction).__name__

            gate = None
            qubit_index = -1
            if instruction_type == 'Measurement':
                qubit_index = instruction.qubit.index
                gate = { 'name': 'measure-z', 'target':qubit_index }
                
            elif instruction_type == 'Gate':
                parmeter = None
                if instruction.params:
                    parmeter = instruction.params[0]

                gate, qubit_index = self.get_gate(instruction.name, instruction.qubits, parmeter, instruction.modifiers)

            if gate:
                step = { 'index':i, 'gates':[gate] }
                steps.append(step)
                i += 1

            if qubit_index > max_qubit_index:
                max_qubit_index = qubit_index

        return {"steps":steps}, max_qubit_index+1

    def get_gate(self, name, qubits, parameter, modifiers):
        if modifiers:
            modifier = modifiers.pop(0)
            if modifier =='FORKED':
                raise Exception('Forked is not supported')

            if modifier =='DAGGER':
                if parameter != None:
                    return self.get_gate(name, qubits, -parameter, modifiers)
                if name == 'S' or name == 'T':
                    return self.get_gate(name+'-DAGGER', qubits, parameter, modifiers)
                if name == 'S-DAGGER' or name == 'T-DAGGER':
                    return self.get_gate(name[1], qubits, parameter, modifiers)
                return self.get_gate(name, qubits, parameter, modifiers)

            if modifier =='CONTROLLED':
                if name == 'CNOT' or name == 'CZ' or name[:11] =='CONTROLLED-':
                    raise Exception('Multiple controlled gated are not supported yet')
                return self.get_gate('CONTROLLED-'+name, qubits, parameter, modifiers)

            raise Exception(f'Unknown modifier {modifier}')

        q_index = qubits[-1].index
        gate = { 'name': self.OPERATION_MAP[name], 'target':q_index }
        
        if name == 'CNOT' or name == 'CZ' or name[:11] =='CONTROLLED-':
            control = qubits[0].index
            gate['control'] = control
            q_index = max(q_index, control)

        if parameter != None:
            gate['phi'] = parameter

        return gate, q_index


