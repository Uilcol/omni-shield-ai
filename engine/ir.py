# engine/ir.py
from dataclasses import dataclass
from typing import List, Dict

@dataclass
class IRInstruction:
    opcode: str
    args: List[str]

@dataclass
class IRFunction:
    name: str
    instructions: List[IRInstruction]

@dataclass
class IRModule:
    functions: List[IRFunction]

def sample_ir() -> IRModule:
    """Gera um IR de exemplo minimalista"""
    instr = IRInstruction(opcode="LOAD_CONST", args=["42"])
    func = IRFunction(name="main", instructions=[instr])
    module = IRModule(functions=[func])
    return module
