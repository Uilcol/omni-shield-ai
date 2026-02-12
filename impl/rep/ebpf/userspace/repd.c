int main() {
struct decision d = call_verifier();
if (d.allow) load_policy(d.policy);
else lockdown();
}