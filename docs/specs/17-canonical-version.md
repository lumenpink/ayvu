# Especificação Técnica — Versão Canônica na Flag --version (#17)

- **ID do Ticket:** #17
- **Status:** Aprovado
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / GOST-03 / SemVer 2.0.0

## 1. Contexto & Motivação
Atualmente, o binário `ayvu` expõe através das flags `--version` e `-V` a versão padrão do `Cargo.toml` (`0.1.0-dev.1`), em desacordo com o arquivo de versão canônica `VERSION` do projeto (SSOT normativo).

## 2. Requisitos Técnicos
1. **SSOT do Arquivo `VERSION`:**
   - O arquivo `VERSION` na raiz do repositório é a fonte canônica da versão do projeto.
   - O build script `build.rs` deve capturar `VERSION`, sanitizar quebras de linha e expor a variável de compilação `AYVU_CANONICAL_VERSION`.
2. **Exposição em `src/lib.rs`:**
   - A constante `ayvu::VERSION` e a função `ayvu::version()` devem retornar a versão canônica sanitizada.
3. **Comportamento da CLI:**
   - `ayvu --version` e `ayvu -V` devem exibir a versão canônica formatada conforme SemVer 2.0.0.
4. **Testes Automatizados:**
   - Teste de integração em `tests/cli_test.rs` validando o retorno de `ayvu --version` contra o conteúdo de `VERSION`.
