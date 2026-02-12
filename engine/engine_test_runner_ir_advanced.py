import cbor2
from pathlib import Path
from hashlib import sha256
import subprocess
import sys

# ============================
# Diretórios de teste
# ============================
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# ============================
# IR Interprocedural Avançada
# ============================
class IRNode:
    """Nó da IR: tipo, valor, filhos"""
    def __init__(self, op_type, value=None, children=None):
        self.op_type = op_type  # assign, policy_check, call, branch
        self.value = value      # valor simbólico
        self.children = children or []

    def traverse_paths(self, current_path=None):
        """
        Retorna todos os caminhos possíveis a partir deste nó.
        branch node cria caminhos alternativos (caminhos SMT simbólicos)
        """
        if current_path is None:
            current_path = []

        current_path = current_path + [self]

        if not self.children:
            return [current_path]

        paths = []
        for child in self.children:
            if self.op_type == "branch":
                # Cada valor simbólico do branch gera um caminho distinto
                for val in self.value:
                    branch_node = IRNode("branch", value=val, children=child.children)
                    for p in branch_node.traverse_paths(current_path):
                        paths.append(p)
            else:
                for p in child.traverse_paths(current_path):
                    paths.append(p)
        return paths

# ============================
# OPF
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

# ============================
# Assinatura placeholder
# ============================
def sign_opf(opf: OPF):
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]

# ============================
# Construção da IR
# ============================
def build_ir():
    """
    Função principal 'main':
    - assign version
    - branch policy_check
    - call function foo
    """
    # Variável simbólica version
    assign_version = IRNode("assign", value=1)

    # Branch de política
    branch_policy = IRNode("branch", value=["ok", "restricted", "conflict"],
                           children=[IRNode("policy_check")])

    # Função foo chamada pelo main
    foo_body = IRNode("call", value="foo", children=[IRNode("assign", value=42),
                                                     IRNode("policy_check")])
    call_foo = IRNode("call", value="main", children=[assign_version, branch_policy, foo_body])
    return call_foo

# ============================
# Gerar OPFs a partir da IR
# ============================
def generate_opfs_from_ir():
    root = build_ir()
    paths = root.traverse_paths()
    valid_opfs = []
    invalid_opfs = []

    for idx, path in enumerate(paths, start=1):
        # Encontrar o nó policy_check mais relevante
        policy_nodes = [n for n in path if n.op_type == "policy_check"]
        if not policy_nodes:
            continue
        # Pegar o branch anterior se existir
        branch_node = next((n for n in path if n.op_type == "branch"), None)
        policy = branch_node.value if branch_node else "ok"

        name = f"opf_{policy}_{idx}"
        opf = OPF(name=name, version=1, policy=policy)

        # Determinar válidos e inválidos
        if policy == "conflict":
            invalid_opfs.append(opf)
        else:
            opf.signature = sign_opf(opf)
            valid_opfs.append(opf)

    return valid_opfs, invalid_opfs

# ============================
# Escrever OPFs
# ============================
def write_opfs(opfs, valid=True):
    for opf in opfs:
        dir_path = VALID_DIR if valid else INVALID_DIR
        filename = f"{opf.name}.cbor"
        path = dir_path / filename
        if opf.signature is None and valid:
            opf.signature = sign_opf(opf)
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[{'VALID-FORMAL' if valid else 'INVALID-FORMAL'}] {path} gerado.")

# ============================
# Executar testes Rust
# ============================
def run_rust_tests():
    print("\n🔹 Executando testes Rust...")
    try:
        result = subprocess.run(
            ["cargo", "test", "--tests", "-p", "omniuil_verifier", "--", "--nocapture"],
            cwd=".",
            capture_output=True,
            text=True,
            check=False
        )

        print(result.stdout)
        print(result.stderr)

        summary = {}
        for line in result.stdout.splitlines():
            if line.startswith("test"):
                parts = line.split("...")
                if len(parts) == 2:
                    test_name = parts[0].replace("test", "").strip()
                    status = parts[1].strip()
                    summary[test_name] = status

        print("\n📊 Resumo final por OPF/teste:")
        for name, status in summary.items():
            print(f"- {name}: {status}")

        if all(s == "ok" for s in summary.values()):
            print("\n✅ Todos os testes passaram com OPFs interprocedurais avançados.")
        else:
            print("\n⚠️ Alguns testes falharam. Verifique os logs acima.")

    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique PATH.")
        sys.exit(1)

# ============================
# Pipeline completo Fase 2 Avançada
# ============================
def run_pipeline():
    print("=== Fase 2 Avançada: Engine Formal IR Interprocedural Completa ===")
    valid_opfs, invalid_opfs = generate_opfs_from_ir()
    write_opfs(valid_opfs, valid=True)
    write_opfs(invalid_opfs, valid=False)
    run_rust_tests()

if __name__ == "__main__":
    run_pipeline()
