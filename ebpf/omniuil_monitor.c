#include <uapi/linux/ptrace.h>
#include <linux/sched.h>
#include <linux/types.h>
#include <linux/uidgid.h>
#include <linux/string.h>
#include <linux/bpf.h>
#include <linux/version.h>

struct event {
    u32 pid;
    u32 ppid;
    u32 uid;
    char comm[TASK_COMM_LEN];
    char filename[256];
    bool alert;
    char severity[16];
};

BPF_PERF_OUTPUT(events);

int trace_execve(struct pt_regs *ctx, const char __user *filename,
                 const char __user *const __user *argv,
                 const char __user *const __user *envp) {

    struct event e = {};
    u64 id = bpf_get_current_pid_tgid();
    e.pid = id >> 32;
    e.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;

    bpf_get_current_comm(&e.comm, sizeof(e.comm));

    // PPID (fallback para 0 se não conseguir)
    e.ppid = 0;
#ifdef BPF_CORE_READ
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    e.ppid = BPF_CORE_READ(task, real_parent, tgid);
#endif

    bpf_probe_read_user_str(&e.filename, sizeof(e.filename), filename);

    // regra simples de alerta
    if (e.filename[0] == '/' && e.filename[1] == 't' && e.filename[2] == 'm' && e.filename[3] == 'p') {
        e.alert = true;
        __builtin_memcpy(&e.severity, "HIGH", 5);
    } else {
        e.alert = false;
        __builtin_memcpy(&e.severity, "INFO", 5);
    }

    events.perf_submit(ctx, &e, sizeof(e));
    return 0;
}
