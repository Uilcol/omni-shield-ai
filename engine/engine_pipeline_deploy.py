import cbor2
from pathlib import Path
import subprocess
import json
from datetime import datetime

OPF_DIR_VALID = Path("tests/valid")
OPF_DIR_INVALID = Path("tests/invalid")
REP_EBPF = Path("rep/ebpf")
REP_LSM = Path("rep/lsm")
REP_USERS = Path("rep/userspace")
REPORT_DIR = Path("reports")
REPORT_DIR.mkdir(parents=True, exist_ok=True)

# ============================
# Função de deploy para eBPF/LSM/Userspace
# ============================
def deploy_opf(opf_path, target="userspace"):
    """
    Converte OPF em regra de enforcement.
    target: ebpf, lsm, userspace
    """
    print(f"📦 Deploying {opf_path} to {target}")
    # Aqui, em produção, você geraria:
    # - eBPF C bytecode
    # - LSM hooks via Rust/C
    # - Userspace enforcement scripts
    # Exemplo placeholder:
    if target == "ebpf":
        # Gerar .o compilado
        compiled_path = REP_EBPF / (opf_path.stem + ".o")
        with open(compiled_path, "wb") as f:
            f.write(b"ebpf_bytecode_placeholder")
    elif target == "lsm":
        compiled_path = REP_LSM / (opf_path.stem + ".lsm")
        with open(compiled_path, "w") as f:
            f.write("# lsm hook placeholder")
    else:
        compiled_path = REP_USERS / (opf_path.stem + ".rule")
        with open(compiled_path, "w") as f:
            f.write("userspace enforcement placeholder")

    return compiled_path

# ============================
# Pipeline de deploy
# ============================
def deploy_all_opfs():
    deployed = []
    for opf_path in list(OPF_DIR_VALID.glob("*.cbor")) + list(OPF_DIR_INVALID.glob("*.cbor")):
        for target in ["userspace", "lsm", "ebpf"]:
            compiled = deploy_opf(opf_path, target)
            deployed.append({
                "opf": str(opf_path),
                "target": target,
                "artifact": str(compiled),
                "timestamp": datetime.utcnow().isoformat() + "Z"
            })
    return deployed

# ============================
# Gerar relatório Enterprise
# ============================
def write_deploy_report(deployed):
    report_file = REPORT_DIR / f"opf_deploy_report_{datetime.utcnow().strftime('%Y%m%dT%H%M%S')}.json"
    with open(report_file, "w") as f:
        json.dump(deployed, f, indent=4)
    print(f"📄 Relatório de deploy gerado: {report_file}")

# ============================
# Executar testes integrados Rust
# ============================
def run_rust_tests():
    print("\n🔹 Executando testes Rust de enforcement...")
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
    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique PATH.")
        exit(1)

# ============================
# Pipeline completo Fase 4
# ============================
def run_pipeline_fase4():
    print("=== Fase 4 Enterprise: Deployment Kernel/Userspace + Auditoria ===")
    deployed = deploy_all_opfs()
    write_deploy_report(deployed)
    run_rust_tests()
    print("✅ Pipeline Fase 4 completo")

if __name__ == "__main__":
    run_pipeline_fase4()
