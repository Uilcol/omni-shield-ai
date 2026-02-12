# OPF v1 — Test Vectors (Definição)

Os test vectors definem o comportamento esperado do verifier.

Eles são:
- imutáveis
- versionados
- independentes de linguagem

Categorias:

## Válidos
- minimal
- restricted

## Inválidos
- campo extra
- conflito de política
- assinatura inválida

Os binários `.cbor` vivem em `impl/opf/test_vectors/`.
