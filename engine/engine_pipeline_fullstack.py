import cbor2
import json
from pathlib import Path
import subprocess
import sys
from z3 import Solver, Int, Or, sat
from nacl.signing import SigningKey
from datetime import datetime

# ============================
# Diretórios
# ============================
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
REP_EBPF = Path("rep/ebpf")
REP_LSM = Path("rep/lsm")
REP_USERS = Path("rep/userspace")
REPORT_DIR = Path("reports")
for d in [VALID_DIR, INVALID_DIR, REP_EBPF, REP_LSM, REP_USERS, REPORT_DIR]:
    d.mkdir(parents=True, exist_ok=True)

# ============================
# IR Enterprise Full Stack
# ============================
class IRNode:
    def __init__(self, op_type, value=None, children=None):
        self.op_type = op_type
        self.value = value
        self.children = children or []

    def traverse_paths(self):
        if self.op_type == "loop":
            paths = []
            for val in self.value:
                for child in self.children:
                    for p in child.traverse_paths():
                        paths.append([IRNode("assign", value=val)] + p)
            return paths
        elif self.op_type == "branch":
            paths = []
            for val in self.value:
                for child in self.children:
                    for p in child.traverse_paths():
                        paths.append([IRNode("branch", value=val)] + p)
            return paths
        elif not self.children:
            return [[self]]
        else:
            paths = [[]]
            for child in self.children:
                new_paths = []
                for p in paths:
                    for sp in child.traverse_paths():
                        new_paths.append(p + sp)
                paths = new_paths
            return [[self] + p for p in paths]

# ============================
# OPF Enterprise
# ============================
class OPF:
    def __init__(self, name, version, policy, extra_field=None, signature=None):
        self.name = name
        self.version = version
        self.policy = policy
        self.extra_field = extra_field
        self.signature = signature

    def to_dict(self):
        d = {"name": self.name, "version": self.version, "policy": self.policy}
        if self.extra_field is not None:
            d["extra_field"] = self.extra_field
        if self.signature is not None:
            d["signature"] = self.signature
        return d

SIGNING_KEY = SigningKey.generate()

def sign_opf(opf: OPF):
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return SIGNING_KEY.sign(data).signature

# ============================
# Construção IR Full Stack
# ============================
def build_ir_fullstack():
    bar_body = IRNode("loop", value=range(1, 3), children=[IRNode("policy_check")])
    foo_body = IRNode("call", value="foo", children=[
        IRNode("assign", value=42),
        IRNode("branch", value=["ok", "restricted"], children=[bar_body])
    ])
    main_body = IRNode("call", value="main", children=[
        IRNode("assign", value=1),
        IRNode("branch", value=["ok", "restricted", "conflict"], children=[IRNode("policy_check")]),
        foo_body
    ])
    return main_body

# ============================
# SMT Solver
# ============================
def smt_paths():
    solver = Solver()
    version = Int("version")
    policy = Int("policy")  # 0=ok,1=restricted,2=conflict
    solver.add(Or(policy == 0, policy == 1, policy == 2))
    solver.add(version > 0)
    paths = []
    while solver.check() == sat:
        model = solver.model()
        pol_val = model[policy].as_long()
        path = {'policy': 'ok' if pol_val==0 else 'restricted' if pol_val==1 else 'conflict'}
        paths.append(path)
        solver.add(policy != pol_val)
    return paths

# ============================
# Gerar OPFs e provas formais
# ============================
def generate_opfs_and_proofs():
    paths = smt_paths()
    valid_opfs, invalid_opfs, proofs = [], [], []
    for idx, path in enumerate(paths, start=1):
        policy = path['policy']
        name = f"opf_{policy}_{idx}"
        opf = OPF(name=name, version=1, policy=policy)
        opf.signature = sign_opf(opf) if policy != "conflict" else None
        if policy == "conflict":
            invalid_opfs.append(opf)
        else:
            valid_opfs.append(opf)
        proofs.append({
            "opf_name": name,
            "policy": policy,
            "path_id": idx,
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "valid": policy != "conflict",
            "signature": opf.signature.hex() if opf.signature else None
        })
    return valid_opfs, invalid_opfs, proofs

# ============================
# Escrever OPFs
# ============================
def write_opfs(opfs, valid=True):
    dir_path = VALID_DIR if valid else INVALID_DIR
    for opf in opfs:
        path = dir_path / f"{opf.name}.cbor"
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[{'VALID' if valid else 'INVALID'}] {path} gerado.")

# ============================
# Deploy para eBPF / LSM / Userspace
# ============================
def deploy_opf(opf_path, target):
    print(f"📦 Deploying {opf_path} -> {target}")
    if target=="ebpf":
        compiled = REP_EBPF / (opf_path.stem + ".o")
        compiled.write_bytes(b"ebpf_bytecode_placeholder")
    elif target=="lsm":
        compiled = REP_LSM / (opf_path.stem + ".lsm")
        compiled.write_text("# lsm hook placeholder")
    else:
        compiled = REP_USERS / (opf_path.stem + ".rule")
        compiled.write_text("userspace enforcement placeholder")
    return compiled

def deploy_all_opfs():
    deployed=[]
    for opf_path in list(VALID_DIR.glob("*.cbor"))+list(INVALID_DIR.glob("*.cbor")):
        for target in ["userspace","lsm","ebpf"]:
            artifact = deploy_opf(opf_path,target)
            deployed.append({"opf": str(opf_path),"target":target,"artifact":str(artifact),"timestamp":datetime.utcnow().isoformat()+"Z"})
    return deployed

# ============================
# Relatórios auditáveis
# ============================
def write_report(data, prefix="fullstack"):
    report_file = REPORT_DIR / f"{prefix}_{datetime.utcnow().strftime('%Y%m%dT%H%M%S')}.json"
    with open(report_file,"w") as f:
        json.dump(data,f,indent=4)
    print(f"📄 Relatório gerado: {report_file}")

# ============================
# Testes Rust
# ============================
def run_rust_tests():
    print("\n🔹 Executando testes Rust...")
    try:
        result = subprocess.run(
            ["cargo","test","--tests","-p","omniuil_verifier","--","--nocapture"],
            cwd=".",
            capture_output=True,text=True,check=False
        )
        print(result.stdout)
        print(result.stderr)
    except FileNotFoundError:
        print("❌ Cargo/Rust não encontrado.")
        sys.exit(1)

# ============================
# Pipeline Full Stack
# ============================
def run_fullstack_pipeline():
    print("=== Fase 5 Enterprise Full Stack ===")
    valid_opfs, invalid_opfs, proofs = generate_opfs_and_proofs()
    write_opfs(valid_opfs,True)
    write_opfs(invalid_opfs,False)
    write_report(proofs,"opf_proofs")
    deployed = deploy_all_opfs()
    write_report(deployed,"opf_deploy")
    run_rust_tests()
    print("✅ Pipeline Full Stack completo")

if __name__=="__main__":
    run_fullstack_pipeline()
