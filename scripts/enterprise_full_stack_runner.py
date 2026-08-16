import os
import subprocess
import json
from datetime import datetime
from audit.pdf_report import generate_pdf_report


PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
REPORTS_DIR = os.path.join(PROJECT_ROOT, "reports")

os.makedirs(REPORTS_DIR, exist_ok=True)


def run_command(cmd, cwd=None):
    print(f"\n>> Executando: {cmd}")
    result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)

    print(result.stdout)
    if result.returncode != 0:
        print(result.stderr)
        raise Exception(f"Erro ao executar comando: {cmd}")

    return result.stdout


def run_python_engine():
    print("=== Executando Engine Formal (Python SMT) ===")
    return run_command("python engine/formal_engine.py", cwd=PROJECT_ROOT)


def run_rust_tests():
    print("=== Executando Testes Rust (Workspace Root) ===")
    return run_command("cargo test", cwd=PROJECT_ROOT)


def simulate_deploy():
    print("=== Simulando Deploy eBPF/LSM ===")
    deploy_log = {
        "timestamp": datetime.utcnow().isoformat(),
        "status": "simulated",
        "artifacts": [
            "ebpf_program.o",
            "lsm_hooks.ko",
            "userspace_enforcer"
        ]
    }

    path = os.path.join(REPORTS_DIR, "deploy_log.json")
    with open(path, "w") as f:
        json.dump(deploy_log, f, indent=4)

    print(f"Deploy log salvo em {path}")
    return deploy_log


def generate_enterprise_report(engine_output):
    print("=== Gerando Relatório Enterprise ===")

    report = {
        "timestamp": datetime.utcnow().isoformat(),
        "engine_output": engine_output,
        "status": "Enterprise Full Stack Validation Completed"
    }

    path = os.path.join(REPORTS_DIR, "enterprise_report.json")
    with open(path, "w") as f:
        json.dump(report, f, indent=4)

    print(f"Relatório salvo em {path}")


def main():
    print("🚀 OmniUil AI Enterprise Full Stack Runner")

    engine_output = run_python_engine()
    run_rust_tests()
    simulate_deploy()
    generate_enterprise_report(engine_output)
    generate_pdf_report()


    print("\n✅ Pipeline Enterprise executado com sucesso.")


if __name__ == "__main__":
    main()
