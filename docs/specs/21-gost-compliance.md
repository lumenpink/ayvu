# Especificação Técnica — Adequação Documental e Conformidade GOST (#21)

- **ID do Ticket:** #21
- **Status:** Aprovado
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Perfil Lumen / GOST-20

## 1. Contexto & Motivação
A auditoria normativa estrita do Gosplan (`cccp check --strict`) exige conformidade com a regra GOST-20 (Single Source of Truth), cujo teto para o arquivo `README.md` no perfil do projeto é de no máximo 50 linhas. Atualmente, o arquivo possui 64 linhas.

## 2. Requisitos Técnicos
1. **Condensação do `README.md`:**
   - O arquivo `README.md` não deve ultrapassar 50 linhas de extensão.
   - Informações secundárias e extensas (ex.: detalhes de playground web, exemplos extensos de payload HTTP) devem ser mantidas/apontadas para a pasta `docs/`.
2. **Auditoria Automatizada:**
   - Implementação de teste automatizado verificando que `README.md` possui $\le$ 50 linhas.
3. **Conformidade `cccp check --strict`:**
   - Garantir que a regra GOST-20 seja aprovada (`[PASS]`).
