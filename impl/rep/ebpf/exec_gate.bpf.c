SEC("lsm/bprm_check_security")
int block_exec(struct linux_binprm *bprm) {
if (!policy.allow_exec)
return -EPERM;
return 0;
}