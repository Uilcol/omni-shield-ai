# OmniUil Proof Format (OPF) v1

## 1. Status

**Status:** FINAL — IMUTÁVEL  
**Autoridade:** OmniUil AI  
**Categoria:** Prova Formal de Execução Segura  
**Escopo:** Offline, soberano, auditável

OPF v1 define o formato canônico de uma prova formal que autoriza
ou nega a execução de código, agentes ou pipelines.

Nenhuma execução é permitida sem uma OPF válida.

---

## 2. Princípios Fundamentais

- Zero falsos positivos por definição
- Fail-closed sempre
- Offline por design
- Auditoria independente obrigatória
- Separação estrita entre:
  - geração de prova
  - verificação
  - enforcement

---

## 3. Modelo de Autoridade

OPF **não detecta vulnerabilidades**.

OPF expressa matematicamente que:
> *Não existe caminho de execução inseguro sob as políticas declaradas.*

A autoridade final reside no **verifier**, não no gerador.

---

## 4. Estrutura Lógica da OPF

Uma OPF v1 contém:

1. Metadados imutáveis
2. Hash da IR analisada
3. Conjunto de constraints SMT
4. Resultado SAT
5. Política de execução
6. Binding opcional a hardware
7. Assinatura criptográfica

---

## 5. Semântica de Segurança

- Uma OPF **só é válida se SAT**
- Caminhos inexequíveis **não existem**
- Branches impossíveis **não existem**
- Se qualquer campo falhar validação → execução negada

---

## 6. Versão e Compatibilidade

- OPF v1 é imutável após publicação
- OPF v2+ **não substituem** v1
- Verifiers DEVEM recusar versões desconhecidas

---

## 7. Ameaças Explicitamente Mitigadas

- Tampering de prova
- Replay fora de contexto
- Bypass de runtime
- Confiança em análise heurística
- Dependência de cloud ou SaaS

---

## 8. Fora de Escopo (Deliberado)

- UI
- Dashboards
- Alertas humanos
- ML decidindo segurança

---

## 9. Conclusão

OPF v1 é uma **lei técnica**, não uma sugestão.

Sem OPF válida:
> **Nada executa.**
