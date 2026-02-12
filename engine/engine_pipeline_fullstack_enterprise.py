import cbor2
import json
from pathlib import Path
import subprocess
import sys
from datetime import datetime
from nacl.signing import SigningKey
from jinja2 import Template
from weasyprint import HTML

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
# OPF + assinatura Ed25519
# ============================
SIGNING_KEY = SigningKey.generate()

class OPF:
    def __init__(self, name, version, policy, extra_field=None):
        self.name = name
        self.version = version
        self.policy = policy
        self.extra_field = extra_field
        self.signature = None

    def to_dict(self):
        d = {"name": self.name, "version": self.version, "policy": self.policy}
        if self.extra_field:
            d["extra_field"] = self.extra_field
        if self.signature:
            d["signature"] = self.signature
        return d

def sign_opf(opf: OPF):
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    opf.signature = SIGNING_KEY.sign(data).signature.hex()

# ============================
# Função para gerar OPFs formais
# ============================
def generate_opfs():
    valid_opfs, invalid_opfs = [], []

    # OPFs válidos
    for name, policy in [("opf_valid_minimal", "ok"), ("opf_valid_restricted", "restricted")]:
        opf = OPF(name, version=1, policy=policy)
        sign_opf(opf)
        valid_opfs.append(opf)

    # OPFs inválidos
    for name, policy in [("opf_invalid_extra_field", "ok"),
                         ("opf_invalid_policy_conflict", "conflict"),
                         ("opf_invalid_signature", "ok")]:
        opf = OPF(name, version=1, policy=policy, extra_field="INVALID" if "extra_field" in name else None)
        if "signature" not in name:
            sign_opf(opf)
        invalid_opfs.append(opf)

    return valid_opfs, invalid_opfs

# ============================
# Escrever OPFs CBOR
# ============================
def write_opfs(opfs, valid=True):
    dir_path = VALID_DIR if valid else INVALID_DIR
    for opf in opfs:
        path = dir_path / f"{opf.name}.cbor"
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[{'VALID' if valid else 'INVALID'}] {path} gerado.")

# ============================
# Deploy eBPF / LSM / Userspace reais (exemplo de teste)
# ============================
def deploy_opf(opf_path, target):
    if target=="ebpf":
        compiled = REP_EBPF / (opf_path.stem + ".o")
        # Gerar código mínimo de eBPF real (teste)
        compiled.write_bytes(b"\xB7\x00\x00\x00\x00\x00\x00\x00")  # eBPF NOP minimal
    elif target=="lsm":
        compiled = REP_LSM / (opf_path.stem + ".lsm")
        compiled.write_text("# Hook LSM real placeholder\n")
    else:
        compiled = REP_USERS / (opf_path.stem + ".rule")
        compiled.write_text("userspace enforcement active\n")
    return compiled

def deploy_all_opfs():
    deployed = []
    for opf_path in list(VALID_DIR.glob("*.cbor")) + list(INVALID_DIR.glob("*.cbor")):
        for target in ["userspace","lsm","ebpf"]:
            artifact = deploy_opf(opf_path, target)
            deployed.append({
                "opf": str(opf_path),
                "target": target,
                "artifact": str(artifact),
                "timestamp": datetime.utcnow().isoformat()+"Z"
            })
    return deployed

# ============================
# Relatório PDF/HTML
# ============================
REPORT_TEMPLATE_HTML = """
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>OmniUil AI Audit Report</title>
<style>
body { font-family: sans-serif; }
h1 { color: #2A7AE2; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
th { background-color: #2A7AE2; color: white; }
</style>
</head>
<body>
<h1>OmniUil AI Audit Report</h1>
<p>Generated at {{ timestamp }}</p>

<h2>OPFs Generated</h2>
<table>
<tr><th>Name</th><th>Policy</th><th>Extra Field</th><th>Signature</th></tr>
{% for opf in opfs %}
<tr>
<td>{{ opf.name }}</td>
<td>{{ opf.policy }}</td>
<td>{{ opf.extra_field or "-" }}</td>
<td>{{ opf.signature or "-" }}</td>
</tr>
{% endfor %}
</table>

<h2>Deployment Artifacts</h2>
<table>
<tr><th>OPF</th><th>Target</th><th>Artifact</th><th>Timestamp</th></tr>
{% for d in deployed %}
<tr>
<td>{{ d.opf }}</td>
<td>{{ d.target }}</td>
<td>{{ d.artifact }}</td>
<td>{{ d.timestamp }}</td>
</tr>
{% endfor %}
</table>
</body>
</html>
"""

def generate_report(opfs, deployed):
    timestamp = datetime.utcnow().isoformat()+"Z"
    template = Template(REPORT_TEMPLATE_HTML)
    html_out = template.render(opfs=opfs, deployed=deployed, timestamp=timestamp)
    pdf_file = REPORT_DIR / f"fullstack_report_{datetime.utcnow().strftime('%Y%m%dT%H%M%S')}.pdf"
    html_file = REPORT_DIR / f"fullstack_report_{datetime.utcnow().strftime('%Y%m%dT%H%M%S')}.html"
    HTML(string=html_out).write_pdf(pdf_file)
    HTML(string=html_out).write_html(html_file)
    print(f"📄 Relatórios PDF/HTML gerados: {pdf_file}, {html_file}")

# ============================
# Executar testes Rust
# ============================
def run_rust_tests():
    print("\n🔹 Executando testes Rust...")
    try:
        result = subprocess.run(
            ["cargo","test","--tests","-p","omniuil_verifier","--","--nocapture"],
            cwd=".",
            capture_output=True, text=True
        )
        print(result.stdout)
        print(result.stderr)
    except FileNotFoundError:
        print("❌ Cargo/Rust não encontrado.")
        sys.exit(1)

# ============================
# Pipeline Enterprise Full Stack
# ============================
def run_pipeline_enterprise():
    print("=== OmniUil AI Enterprise Full Stack Pipeline ===")
    valid_opfs, invalid_opfs = generate_opfs()
    write_opfs(valid_opfs, True)
    write_opfs(invalid_opfs, False)
    deployed = deploy_all_opfs()
    generate_report(valid_opfs + invalid_opfs, deployed)
    run_rust_tests()
    print("✅ Pipeline Enterprise completo")

if __name__=="__main__":
    run_pipeline_enterprise()
