# Especificação Técnica — Protocolo OpenDesktop XDG e Distribuição Banca (#14)

- **ID do Ticket:** #14
- **Status:** Aprovado
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / OpenDesktop XDG Base Directory / Banca Release Protocol

## 1. Contexto & Motivação
O Ayvu deve aderir rigorosamente ao padrão OpenDesktop (XDG Base Directory) para separação de arquivos de configuração e estado em ambientes Unix/Linux, além de fornecer auto-atualização atômica a partir do repositório de lançamentos da Banca (https://banca.2lp.in/downloads/ayvu/), com suporte a canais, pinagem de versão, checagens não-bloqueantes no startup e rollback automático.

## 2. Requisitos Técnicos

### 2.1 Diretórios e Arquivos XDG
1. **Configuração do Usuário:**
   - `$XDG_CONFIG_HOME/ayvu/config.toml` (fallback `~/.config/ayvu/config.toml`).
2. **Estado e Cache de Verificação:**
   - `$XDG_STATE_HOME/ayvu/update_check.json` (fallback `~/.local/state/ayvu/update_check.json`).

### 2.2 Estrutura do `config.toml`
Campos suportados com valores padrão:
- `channel`: `"stable"` ou `"nightly"` (padrão: `"stable"`)
- `pinned`: booleano (padrão: `false`)
- `pin`: string com a versão desejada (padrão: `""`)
- `auto_check`: booleano (padrão: `true`)
- `check_interval_hours`: inteiro sem sinal (padrão: `24`)
- `auto_update`: booleano (padrão: `false`)

### 2.3 Ordem de Precedência
1. Flag de linha de comando: `--channel`
2. Variável de ambiente: `AYVU_CHANNEL`
3. Arquivo de configuração: `$XDG_CONFIG_HOME/ayvu/config.toml`
4. Padrão: `"stable"`

### 2.4 Subcomandos da CLI
- `ayvu config show` / `ayvu config get`: Exibe a configuração atual resolvida em formato legível ou TOML.
- `ayvu config pin [<version>]`: Trava a versão atual ou a versão indicada.
- `ayvu config unpin`: Remove o travamento de versão.
- `ayvu config set-channel <stable|nightly>`: Altera o canal de distribuição.
- `ayvu config set-auto-check <true|false>`: Habilita ou desabilita checagem periódica.
- `ayvu config set-auto-update <true|false>`: Habilita ou desabilita substituição automática.
- `ayvu check-update`: Consulta a Banca e informa se há versões mais recentes no canal configurado.
- `ayvu update [--force]`: Baixa e instala atomicamente o binário com rollback em caso de falha. Se estiver pinado, recusa sem `--force`.

### 2.5 Atualização Atômica e Rollback
- O download do artefato para a arquitetura corrente é salvo em arquivo temporário no mesmo diretório/sistema de arquivos de destino (`~/.local/bin/`).
- Aplicação de permissões de execução (`0o755`).
- Backup do binário ativo (`~/.local/bin/ayvu.bak`) e substituição atômica (`fs::rename`).
- Em caso de falha durante a validação ou substituição, restauração imediata do backup.

### 2.6 Verificação Não-Bloqueante de Inicialização
- Durante comandos interativos normais (ex.: `render`), se `auto_check = true` e não estiver pinado, verifica se o intervalo `check_interval_hours` expirou.
- A verificação é assíncrona ou com timeout curtíssimo e emite avisos exclusivamente para `stderr` para não poluir o stdout da saída gráfica.
