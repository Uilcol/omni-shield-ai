# engine/test_engine.py
import unittest
from engine.ir import sample_ir
from engine.opf_emitter import hash_ir, generate_opf
from engine.symbolic import symbolic_execute

class TestEngine(unittest.TestCase):
    def test_opf_generation(self):
        ir = sample_ir()
        ir_hash = hash_ir(str(ir))
        path_ok = symbolic_execute(ir)
        self.assertTrue(path_ok, "Symbolic execution failed")
        
        opf = generate_opf(ir_hash)
        # Verificamos campos básicos
        self.assertIn(b"ir_hash", opf)
        self.assertIn(b"has_extra_field", opf)
        self.assertIn(b"policy_conflict", opf)
        self.assertIn(b"signature_valid", opf)

    def test_opf_invalid_extra_field(self):
        ir = sample_ir()
        ir_hash = hash_ir(str(ir))
        opf = generate_opf(ir_hash, extra_field=True)
        self.assertIn(b"has_extra_field", opf)

if __name__ == "__main__":
    unittest.main()
