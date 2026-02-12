import cbor2
from pathlib import Path
import subprocess
import sys

# Diretórios de teste
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")

VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# Lista de OPFs
opfs_valid = {
    "opf_valid_minimal.cbor": {"name": "minimal", "version": 1, "policy": "ok"},
    "opf_valid_restricted.cbor": {"name": "restricted", "version": 1, "policy": "restricted"},
}

opfs_invalid = {
    "opf_invalid_extra_field.cbor": {"name": "invalid_extra", "version": 1, "policy": "ok", "extra_field": "evil"},
    "opf_invalid_policy_conflict.cbor": {"name": "invalid_policy", "version": 1, "policy": "conflict"},
    "opf_invalid_signature.cbor": {"name": "invalid_signature", "version": 1, "policy": "ok", "signature": b"bad"},
}


def generate_opfs():
    """Gera todos os OPFs (válidos e inválidos)"""
    for filename, data in opfs_valid.items():
        path = VALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(data, canonical=True))
        print(f"[VALID] {path} gerado.")

    for filename, data in opfs_invalid.items():
        path = INVALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(data, canonical=True))
        print(f"[INVALID] {path} gerado.")


def run_rust_tests():
    """
    Executa os testes Rust do Verifier e captura resultados individuais
    """
    print("\n🔹 Executando testes Rust...")
    try:
        # Executa todos os testes do crate omniuil_verifier
        result = subprocess.run(
            ["cargo", "test", "--tests", "-p", "omniuil_verifier", "--", "--nocapture"],
            cwd=".",  # raiz do projeto
            capture_output=True,
            text=True,
            check=False
        )

        print(result.stdout)
        print(result.stderr)

        # Resumo por OPF
        summary = {}
        for line in result.stdout.splitlines():
            if line.startswith("test"):
                # linha do tipo: "test valid_minimal_opf_is_accepted ... ok"
                parts = line.split("...")
                if len(parts) == 2:
                    test_name = parts[0].replace("test", "").strip()
                    status = parts[1].strip()
                    summary[test_name] = status

        print("\n📊 Resumo por OPF/teste:")
        for name, status in summary.items():
            print(f"- {name}: {status}")

        # Resultado final
        if all(s == "ok" for s in summary.values()):
            print("\n✅ Todos os testes passaram.")
        else:
            print("\n⚠️ Alguns testes falharam. Verifique os logs acima.")

    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique a instalação e o PATH.")
        sys.exit(1)


if __name__ == "__main__":
    generate_opfs()
    run_rust_tests()
