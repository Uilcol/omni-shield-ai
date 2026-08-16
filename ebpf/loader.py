#!/usr/bin/env python3
import os
import json
from bcc import BPF

# Caminho absoluto do monitor.c
script_dir = os.path.dirname(os.path.realpath(__file__))
monitor_path = os.path.join(script_dir, "omniuil_monitor.c")

# Carrega código C
with open(monitor_path, "r") as f:
    program = f.read()

# Compila BPF
b = BPF(text=program)
b.attach_kprobe(event="sys_execve", fn_name="trace_execve")

# Função de callback para perf event
def print_event(cpu, data, size):
    event = b["events"].event(data)
    output = {
        "pid": event.pid,
        "ppid": event.ppid,
        "uid": event.uid,
        "comm": event.comm.decode('utf-8', 'replace'),
        "filename": event.filename.decode('utf-8', 'replace'),
        "alert": bool(event.alert),
        "severity": event.severity.decode('utf-8', 'replace')
    }
    print(json.dumps(output))

# Associa callback
b["events"].open_perf_buffer(print_event)

print("🚀 OmniUil Runtime iniciado...")
print("Monitorando execve... CTRL+C para sair.")

# Loop principal
while True:
    try:
        b.perf_buffer_poll()
    except KeyboardInterrupt:
        print("Saindo...")
        exit()
