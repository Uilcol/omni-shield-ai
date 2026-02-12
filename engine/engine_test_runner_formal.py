import cbor2
from pathlib import Path
from hashlib import sha256
import subprocess
import sys

# Diretórios de teste
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# --- IR simplificada para OPFs ---
class OPF:
    def __init__(self, name, version, policy, extra_field=None, signature=None):
        self.name = name
        self.version = version
        self.policy = policy
        self.extra_field = extra_field
        self.signature = signature

    def to_dict(self):
        d = {
            "name": self.name,
            "version": self.version,
            "policy": self.policy
        }
        if self.extra_field is not None:
            d["extra_field"] = self.extra_field
        if self.signature is not None:
            d["signature"] = self.signature
        return d

# --- Funções auxiliares ---
def sign_opf(opf: OPF):
    """
    Placeholder de assinatura Ed25519 compatível com Verifier.
    Aqui usamos SHA256 truncado como simulação de assinatura.
    """
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]  # 32 bytes simulando assinatura

# --- Gerar OPFs válidos formalmente ---
def generate_valid_opfs():
    opfs = []

    # OPF minimal
    minimal = OPF(name="minimal", version=1, policy="ok")
    minimal.signature = sign_opf(minimal)
    opfs.append(("opf_valid_minimal.cbor", minimal))

    # OPF restricted
    restricted = OPF(name="restricted", version=1, policy="restricted")
    restricted.signature = sign_opf(restricted)
    opfs.append(("opf_valid_restricted.cbor", restricted))

    for filename, opf in opfs:
        path = VALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[VALID-FORMAL] {path} gerado.")

# --- Gerar OPFs inválidos formalmente ---
def generate_invalid_opfs():
    opfs = []

    # Extra field inválido
    extra = OPF(name="invalid_extra", version=1, policy="ok", extra_field="evil")
    extra.signature = sign_opf(extra)
    opfs.append(("opf_invalid_extra_field.cbor", extra))

    # Policy conflict
    conflict = OPF(name="invalid_policy", version=1, policy="conflict")
    conflict.signature = sign_opf(conflict)
    opfs.append(("opf_invalid_policy_conflict.cbor", conflict))

    # Assinatura inválida
    bad_sig = OPF(name="invalid_signature", version=1, policy="ok")
    bad_sig.signature = b"bad"
    opfs.append(("opf_invalid_signature.cbor", bad_sig))

    for filename, opf in opfs:
        path = INVALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[INVALID-FORMAL] {path} gerado.")

# --- Executa testes Rust e retorna resumo ---
def run_rust_tests():
    print("\n🔹 Executando testes Rust...")
    try:
        result = subprocess.run(
            ["cargo", "test", "--tests", "-p", "omniuil_verifier", "--", "--nocapture"],
            cwd=".",  # raiz do projeto
            capture_output=True,
            text=True,
            check=False
        )

        print(result.stdout)
        print(result.stderr)

        # --- Captura resultado por teste ---
        summary = {}
        for line in result.stdout.splitlines():
            if line.startswith("test"):
                parts = line.split("...")
                if len(parts) == 2:
                    test_name = parts[0].replace("test", "").strip()
                    status = parts[1].strip()
                    summary[test_name] = status

        # --- Exibe resumo por OPF ---
        print("\n📊 Resumo por OPF/teste:")
        for name, status in summary.items():
            print(f"- {name}: {status}")

        # Resultado final
        if all(s == "ok" for s in summary.values()):
            print("\n✅ Todos os testes passaram com os OPFs formais gerados.")
        else:
            print("\n⚠️ Alguns testes falharam. Verifique os logs acima.")

    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique a instalação e o PATH.")
        sys.exit(1)

# --- Pipeline completo ---
def run_pipeline():
    print("=== Fase 2: Engine Formal — geração de OPFs e execução de testes ===")
    generate_valid_opfs()
    generate_invalid_opfs()
    run_rust_tests()

if __name__ == "__main__":
    run_pipeline()
