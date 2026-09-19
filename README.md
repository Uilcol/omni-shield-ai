# OmniShield AI

> **Autonomous Application Security Engine**
>
> Rust-based SAST platform focused on static application security analysis, taint/dataflow reasoning, structured security evidence, policy enforcement and an architecture designed to evolve toward AI-native software security.

## Visão geral

O **OmniShield AI** é uma plataforma de **Static Application Security Testing (SAST)** construída em Rust para analisar código-fonte e identificar padrões de risco, fluxos de dados inseguros e potenciais vulnerabilidades.

O projeto combina análise sintática, análise de fluxo, taint analysis, análise interprocedural, Security Graph, validação por SMT/Z3, classificação de risco e políticas de segurança em uma arquitetura modular.

A proposta central é sair de uma detecção baseada apenas em padrões e construir **evidência estruturada do caminho de segurança**, permitindo representar relações como:

```text
Source
   ↓
Variable
   ↓
Call
   ↓
Function / Parameter
   ↓
Return
   ↓
Caller Variable
   ↓
Sink
```

Esse modelo permite que findings sejam associados a um `SecurityPath`, tornando a evidência mais estruturada e adequada para automação, revisão humana e integrações futuras.

---

## Principais objetivos

O OmniShield AI foi projetado para:

- detectar vulnerabilidades de segurança em código-fonte;
- analisar propagação de dados potencialmente contaminados;
- reduzir falsos positivos através de contexto estrutural e validação;
- produzir evidências técnicas para cada finding;
- integrar segurança ao pipeline de desenvolvimento;
- gerar resultados compatíveis com automação e ferramentas de segurança;
- evoluir para segurança de software produzido por humanos e por sistemas de IA;
- fornecer uma arquitetura local, modular e extensível para análise de segurança.

---

# Capacidades atuais

## SAST

O projeto possui uma engine própria de análise estática, com pipeline baseado em Rust.

A arquitetura inclui módulos dedicados para:

- AST;
- CFG;
- Dataflow;
- Taint Analysis;
- análise interprocedural;
- Call Graph;
- Security Graph;
- análise semântica;
- SMT/Z3;
- findings;
- deduplicação;
- políticas;
- relatórios;
- integrações.

---

## AST com Tree-sitter

O OmniShield utiliza **Tree-sitter** para parsing estrutural.

O núcleo atualmente possui grammars configuradas para linguagens como:

- Python;
- JavaScript;
- Rust;
- Go.

A análise Python já utiliza AST real para identificar chamadas e construções relacionadas a segurança.

---

## Detecção de vulnerabilidades

A engine atual possui detecções como:

### Python

- uso de `eval()`;
- possível Command Injection através de `os.system()`;
- possíveis SQL Injections;
- secrets hardcoded;
- uso de MD5;
- fluxos de dados contaminados chegando a sinks.

Exemplo conceitual:

```python
user_input = input()
query = "SELECT * FROM users WHERE id=" + user_input
cursor.execute(query)
```

O objetivo não é apenas encontrar a palavra `input` ou `SELECT`, mas relacionar a entrada controlada com a construção do query e seu sink.

---

# Taint Analysis

O projeto possui mecanismos de **Taint Analysis** para modelar:

```text
Source → Propagation → Sink
```

Entre os conceitos considerados estão:

### Sources

- `input()`;
- `request.get`;
- `request.post`;
- `sys.argv`;
- parâmetros de usuário;
- outras fontes configuráveis.

### Sinks

- `eval()`;
- `exec()`;
- `os.system()`;
- `subprocess.call()`;
- `cursor.execute()`;
- outros sinks definidos pelo mecanismo.

A arquitetura também possui componentes para:

- propagação intraprocedural;
- propagação interprocedural;
- sanitizers;
- source detection;
- sink detection;
- tracking;
- taint state.

---

# Análise interprocedural

O projeto contém uma arquitetura dedicada à análise entre funções.

O objetivo é representar fluxos como:

```text
user_input
    ↓
argument
    ↓
function parameter
    ↓
local variable
    ↓
return
    ↓
caller variable
    ↓
sink
```

A base atual já possui estruturas de:

- Call Graph;
- Call Resolver;
- análise interprocedural;
- Taint Engine;
- resolução de chamadas;
- representação de parâmetros;
- representação de retornos.

A evolução dessa camada é uma das etapas centrais do roadmap.

---

# Security Graph

Um dos componentes diferenciais do OmniShield AI é o **Security Graph**.

O grafo representa entidades e relações de segurança como nós e arestas.

## Security Nodes

Entre os tipos representados estão:

- `Source`;
- `Variable`;
- `Function`;
- `Call`;
- `Return`;
- `Sanitizer`;
- `Sink`;
- `Finding`.

## Security Edges

Entre as relações estão:

- `Defines`;
- `Assigns`;
- `Calls`;
- `Returns`;
- `FlowsTo`;
- `Sanitizes`;
- `Reaches`;
- `EvidenceFor`.

Exemplo:

```text
Source
   │
   ▼
Variable
   │
   ▼
Call
   │
   ▼
Function
   │
   ▼
Parameter
   │
   ▼
Return
   │
   ▼
Caller Variable
   │
   ▼
Sink
```

---

# Security Path

Os findings podem carregar uma representação estruturada do caminho de segurança através de `SecurityPath`.

Um caminho contém passos com:

- identificador do nó;
- tipo do nó;
- label;
- arquivo;
- linha.

Isso permite obter:

- origem do fluxo;
- sink atingido;
- arquivos envolvidos;
- intervalo de linhas;
- sequência de evidências;
- representação estruturada para JSON/SARIF e integrações futuras.

Exemplo conceitual:

```json
{
  "source": "user_input",
  "path": [
    "user_input = input()",
    "run(user_input)",
    "value",
    "return query",
    "result",
    "eval(result)"
  ]
}
```

---

# SMT / Z3

O projeto possui integração com **Z3 / SMT**.

A camada SMT fornece a base para:

- construção de constraints;
- modelos simbólicos;
- validação;
- análise de condições;
- verificação de caminhos;
- redução de falsos positivos.

O objetivo arquitetural é combinar:

```text
Detecção determinística
        +
Dataflow / Taint
        +
Security Graph
        +
SMT / Symbolic Reasoning
```

antes de considerar um resultado como um finding de maior confiança.

---

# Findings

Os findings possuem estrutura própria contendo informações como:

- ID;
- título;
- severidade;
- CWE;
- OWASP;
- confidence;
- arquivo;
- linha;
- evidence;
- recommendation;
- `SecurityPath` opcional.

Exemplo conceitual:

```text
ID:           TAINT-001
Severity:     Critical
CWE:          CWE-20
OWASP:        A03:2021 - Injection
Confidence:   0.99
File:         example.py
Line:         12
Evidence:     source -> variable -> sink
```

---

# Deduplicação e priorização

A arquitetura possui componentes específicos para:

- deduplicação de findings;
- priorização;
- supressão;
- classificação;
- scoring de risco.

Existe também um `RiskScoreEngine` que calcula um score agregado com base na severidade dos findings.

---

# Security Policy

O OmniShield possui camada de **policy/security gate**.

A arquitetura contempla:

- threshold de severidade;
- `fail-on`;
- avaliação de findings;
- bloqueio de pipeline;
- configuração de política;
- security gate para CI/CD.

Exemplo conceitual:

```text
Scan
  ↓
Findings
  ↓
Policy Engine
  ↓
Security Gate
  ├── PASS
  └── FAIL
```

Isso permite utilizar o scanner como uma etapa de controle de segurança em pipelines automatizados.

---

# Relatórios e formatos

O projeto possui infraestrutura para diferentes formas de saída.

Entre os componentes presentes estão:

- JSON;
- HTML;
- SARIF;
- tabela;
- relatório executivo;
- GitHub Actions output.

## SARIF

O OmniShield possui geração de **SARIF 2.1.0**, permitindo integração com ferramentas que consomem resultados de análise estática.

---

# Integrações

A arquitetura possui integração com GitHub, incluindo componentes para:

- GitHub API;
- Pull Requests;
- Reviews;
- Check Runs;
- automação de análise;
- criação de PRs.

O CLI também possui comandos e flags relacionados a:

- `--pr-review`;
- `--push-check`;
- `create-pr`.

Esses recursos permitem evoluir o OmniShield para um fluxo de segurança integrado diretamente ao desenvolvimento.

---

# CLI

O executável principal é:

```text
omnishield
```

Versão atual do CLI no repositório:

```text
0.5.0
```

O core atualmente está na versão:

```text
0.3.0
```

## Scan básico

```bash
cargo run -p omnishield --release -- scan ./meu-projeto
```

## Falhar por severidade

```bash
cargo run -p omnishield --release -- scan ./meu-projeto --fail-on high
```

## Gerar SARIF

```bash
cargo run -p omnishield --release -- \
  scan ./meu-projeto \
  --sarif omnishield.sarif
```

## GitHub PR Review

```bash
cargo run -p omnishield --release -- \
  scan ./meu-projeto \
  --pr-review
```

## Push Check

```bash
cargo run -p omnishield --release -- \
  scan ./meu-projeto \
  --push-check
```

## Semantic Demo

```bash
cargo run -p omnishield --release -- --semantic-demo
```

---

# Estrutura do projeto

O workspace Rust é organizado em múltiplos componentes:

```text
omni-shield-ai/
├── core/
│   └── src/
│       ├── ai/
│       ├── analysis/
│       │   ├── ast/
│       │   ├── cfg/
│       │   ├── dataflow/
│       │   ├── interprocedural/
│       │   └── taint/
│       ├── benchmark/
│       ├── deep/
│       ├── findings/
│       ├── integrations/
│       ├── languages/
│       ├── output/
│       ├── policy/
│       ├── report/
│       ├── security_graph.rs
│       ├── security_graph_builder.rs
│       ├── semantic/
│       ├── smt/
│       └── runtime/
│
├── cli/
├── server/
├── worker/
├── verifier/
├── dashboard/
├── rules/
├── tests/
├── docs/
├── scripts/
├── ebpf/
├── k8s/
├── spec/
├── Dockerfile
├── docker-compose.yml
└── Cargo.toml
```

O workspace Rust declara atualmente os membros:

```text
core
server
cli
verifier
worker
```

---

# Tecnologias

## Linguagem principal

- Rust 2021

## Parsing

- Tree-sitter
- tree-sitter-python
- tree-sitter-javascript
- tree-sitter-rust
- tree-sitter-go

## Segurança / análise

- AST
- CFG
- Dataflow
- Taint Analysis
- Call Graph
- Security Graph
- Interprocedural Analysis
- SMT
- Z3
- CWE
- OWASP
- SARIF

## Infraestrutura

- Cargo workspace
- GitHub Actions
- Docker
- Docker Compose
- Kubernetes
- GitHub API

## Serialização

- Serde
- JSON
- YAML

---

# Configuração

O projeto possui suporte arquitetural para configuração de políticas e regras.

Áreas relevantes:

```text
rules/
core/src/policy/
core/src/policy/config/
core/src/policy/policy_engine/
```

O OmniShield foi projetado para permitir evolução de regras e políticas sem acoplar toda a lógica de segurança à CLI.

---

# Testes

A estratégia de testes cobre diferentes camadas do projeto.

Entre os testes existentes estão:

- Python parser tests;
- runtime executor tests;
- query analysis tests;
- deduplication tests;
- Security Graph tests;
- Security Graph Builder tests;
- workspace tests;
- smoke tests da CLI.

## Executar todos os testes

```bash
CARGO_BUILD_JOBS=1 cargo test --workspace
```

## Build de release

```bash
CARGO_BUILD_JOBS=1 cargo build --workspace --release
```

## Testar um conjunto específico

```bash
CARGO_BUILD_JOBS=1 cargo test -p omniuil_core --test security_graph_test
```

Exemplo:

```bash
CARGO_BUILD_JOBS=1 cargo test -p omniuil_core --test security_graph_builder_test
```

---

# CI/CD

O projeto possui workflow GitHub Actions para:

1. checkout;
2. instalação da toolchain Rust;
3. cache de Cargo;
4. build release;
5. testes do workspace;
6. smoke test da CLI.

Fluxo:

```text
git push
   ↓
GitHub Actions
   ↓
cargo build --workspace --release
   ↓
cargo test --workspace
   ↓
CLI smoke test
```

---

# Arquitetura de segurança

A visão arquitetural do OmniShield pode ser resumida como:

```text
                    OmniShield AI
                         │
              ┌──────────┴──────────┐
              │                     │
           Parsing              Runtime
              │                     │
        AST / Tree-sitter      Scan Pipeline
              │                     │
        ┌─────┴─────┐               │
        │           │               │
      CFG        Dataflow        Findings
        │           │               │
        └─────┬─────┘               │
              │                     │
         Taint Analysis              │
              │                     │
      Interprocedural               │
              │                     │
        Security Graph ─────────────┘
              │
        Security Paths
              │
          SMT / Z3
              │
       False-positive control
              │
         Risk / Policy
              │
     JSON / HTML / SARIF / CI
```

---

# Pontos fortes técnicos

## 1. Engine própria em Rust

O núcleo é implementado em Rust, permitindo construir uma engine de análise de segurança modular, performática e com forte segurança de memória.

## 2. Análise estrutural

O projeto utiliza AST real através de Tree-sitter, evitando depender exclusivamente de regex para as análises estruturais principais.

## 3. Taint + Dataflow

A arquitetura busca rastrear dados desde suas origens até sinks potencialmente perigosos.

## 4. Security Graph

A representação em grafo permite modelar relações de segurança de forma explícita.

## 5. Evidência estruturada

O `SecurityPath` transforma o caminho de uma vulnerabilidade em uma estrutura que pode ser consumida por outros sistemas.

## 6. SMT/Z3

O uso de SMT cria uma base para verificar condições e caminhos além de heurísticas simples.

## 7. CI/CD e Policy

O projeto foi desenhado para não ser apenas um scanner local, mas também uma camada de controle de segurança dentro do pipeline.

## 8. Evolução para AI-native security

A arquitetura contém módulos de AI e uma visão de longo prazo direcionada à segurança de software produzido por humanos e sistemas de IA.

---

# Roadmap

O roadmap é dividido em fases para evoluir a engine sem sacrificar a precisão.

## Fase 1 — Engine Foundation

**Status: implementada**

- workspace Rust;
- CLI;
- runtime;
- findings;
- pipeline básico.

## Fase 2 — AST + Dataflow

**Status: implementada / em evolução**

- Tree-sitter;
- AST;
- CFG;
- dataflow;
- análise estrutural.

## Fase 3 — Taint + Interprocedural

**Status: implementada parcialmente / em evolução**

- taint sources;
- sinks;
- propagação;
- análise interprocedural;
- Call Graph;
- retorno de funções;
- propagação entre argumentos e parâmetros.

## Fase 4 — Security Graph

**Status: implementada**

- Security Nodes;
- Security Edges;
- source-to-sink paths;
- `SecurityPath`;
- integração de paths aos findings.

## Fase 5 — Symbolic / SMT

**Status: implementada parcialmente / em evolução**

- constraints;
- Z3;
- validação simbólica;
- path feasibility;
- redução de falsos positivos.

## Fase 6 — Framework-aware Analysis

**Próxima evolução**

- frameworks web;
- ORM;
- routing;
- authentication;
- authorization;
- framework-specific sources;
- framework-specific sinks;
- sanitizer models.

## Fase 7 — Benchmark

**Roadmap**

Comparação sistemática com engines estabelecidas de SAST usando conjuntos de testes reproduzíveis.

Métricas planejadas:

- precisão;
- recall;
- false positives;
- false negatives;
- tempo de execução;
- custo por análise.

## Fase 8 — Hybrid AI Reasoning

**Roadmap**

Combinar:

```text
Deterministic Analysis
        +
Security Graph
        +
SMT
        +
AI Reasoning
```

com o princípio de que a IA deve complementar a análise verificável, e não substituir a evidência estrutural.

## Fase 9 — AI-generated Code Security

**Direção estratégica**

Analisar especificamente riscos introduzidos por código gerado por IA, incluindo:

- unsafe patterns;
- insecure dependencies;
- injection;
- authentication errors;
- authorization errors;
- insecure defaults;
- secret leakage;
- unsafe tool usage.

## Fase 10 — Agent / MCP / A2A Security

**Roadmap**

Evoluir a engine para analisar sistemas baseados em agentes, incluindo:

- tool invocation;
- model-to-tool flows;
- agent permissions;
- MCP servers;
- inter-agent communication;
- security boundaries;
- data exfiltration paths.

## Fase 11 — Advanced Policy Engine

- políticas organizacionais;
- compliance;
- risk thresholds;
- security gates;
- exemptions;
- baselines;
- enforcement.

## Fase 12 — Autofix + Verification

- geração de sugestões;
- patches;
- verificação automática;
- re-análise após correção;
- confirmação de que a vulnerabilidade foi removida.

## Fase 13 — Enterprise Platform

- servidor;
- dashboard;
- workers;
- multi-project scanning;
- observabilidade;
- gestão de políticas;
- integração corporativa.

---

# Filosofia do projeto

O OmniShield AI foi projetado seguindo alguns princípios:

### Evidência antes de opinião

A engine deve construir evidências técnicas antes de gerar conclusões de segurança.

### Determinismo antes de IA

A análise estrutural, dataflow, taint e SMT devem formar a base verificável.

### IA como camada de raciocínio

Modelos de IA devem ampliar análise e explicação sem substituir as estruturas verificáveis.

### Local-first

A arquitetura suporta uma visão local/offline para cenários em que código sensível não pode ser enviado para serviços externos.

### Segurança para software moderno

A engine foi concebida para evoluir além do código tradicional e tratar também software produzido ou modificado por sistemas de IA.

---

# Estado do projeto

O OmniShield AI é um projeto em desenvolvimento ativo.

O repositório público já contém:

- engine SAST em Rust;
- CLI;
- AST/Tree-sitter;
- Dataflow;
- Taint Analysis;
- componentes interprocedurais;
- Security Graph;
- Security Paths;
- Findings estruturados;
- SMT/Z3;
- Policy/Security Gate;
- SARIF;
- integrações GitHub;
- CI GitHub Actions;
- infraestrutura Docker/Kubernetes;
- testes automatizados.

Algumas capacidades presentes no workspace ainda estão em evolução e não devem ser interpretadas como equivalentes completos a produtos maduros de mercado. O roadmap existe justamente para transformar a base atual em uma plataforma de análise de segurança cada vez mais profunda e verificável.

---

# Desenvolvimento local

## Pré-requisitos

- Rust toolchain;
- Cargo;
- ambiente compatível com as dependências do workspace;
- Z3 através da dependência Rust configurada no `core`.

## Build

```bash
cargo build --workspace
```

## Release

```bash
CARGO_BUILD_JOBS=1 cargo build --workspace --release
```

## Testes

```bash
CARGO_BUILD_JOBS=1 cargo test --workspace
```

## CLI

```bash
cargo run -p omnishield --release -- --help
```

---

# Contribuição

O projeto está em evolução arquitetural. Antes de modificar componentes centrais:

1. preserve as abstrações existentes;
2. mantenha testes para novos fluxos;
3. evite duplicar engines que já possuem responsabilidade definida;
4. priorize evidência verificável;
5. execute os testes do workspace antes de enviar alterações.

---

# Segurança

Para reportar vulnerabilidades no próprio OmniShield AI, utilize o processo de segurança disponível no repositório e evite publicar detalhes exploráveis de uma vulnerabilidade não corrigida em uma issue pública.

---

# Licença

A licença deve ser definida explicitamente antes da distribuição pública do projeto como produto. Enquanto isso, consulte os arquivos de licença existentes no repositório antes de reutilizar o código.

---

# Autor

**Uiliam Coelho Conceição Lima**

Projeto principal:

**OmniShield AI — Autonomous Application Security Engine**

Foco:

**Application Security • SAST • Rust • Taint Analysis • Security Graph • SMT/Z3 • AI Code Security**
