# Especificação Técnica: Desacoplamento de Assets via Diretórios XDG (#23)

## 1. Contexto & Problema
Atualmente, todos os personagens (8 personagens com 8 poses vetoriais SVG e manifestos JSON) e cenários estão embutidos diretamente no binário Rust via macros `include_str!` ou código estático.
Essa abordagem acopla os assets visuais ao executável, impedindo:
1. Que usuárias e usuários adicionem novos personagens ou cenários em tempo de execução sem recompilar o binário.
2. A distribuição modular de pacotes de assets em distribuições Linux (e.g. pacotes `ayvu-characters-ufsc` ou `ayvu-backdrops-retro`).
3. Uma manutenção higiênica e ágil de ilustrações sem reconstruir toda a árvore de compilação Rust.

## 2. Árvore de Decisão
- **Alternativa Descartada (Exigir flags de CLI como `--assets-dir` para tudo):** Descartado por gerar forte atrito no CLI e quebrar a compatibilidade com scripts e chamadas automatizadas existentes.
- **Alternativa Descartada (Remover completamente qualquer fallback e quebrar caso a pasta não exista no disco):** Descartado para garantir robustez: a Natasha deve poder rodar binários de teste ou containers mínimos sem que o motor entre em pânico caso as pastas de sistema ainda não estejam povoadas.
- **Abordagem Escolhida (Hierarquia XDG Base Directory com Discovery Dinâmico):**
  1. Seguir rigorosamente a especificação *XDG Base Directory*:
     - **Camada de Usuária (Mais alta prioridade):** `$XDG_DATA_HOME/ayvu/` (default: `~/.local/share/ayvu/`).
     - **Camadas de Sistema:** Cada entrada de `$XDG_DATA_DIRS` com sufixo `ayvu/` (default: `/usr/local/share/ayvu/` e `/usr/share/ayvu/`).
     - **Camada de Desenvolvimento Local:** `./assets/` relativo ao diretório corrente ou ao executável.
     - **Camada de Segurança (Fallback de Resiliência):** Assets nativos essenciais mantidos como fallback de última instância caso nenhum diretório em disco seja encontrado.
  2. **Estrutura Canônica no Disco:**
     ```
     <data_dir>/
       ├── characters/
       │   └── <character_id>/
       │       ├── manifest.json
       │       ├── neutral.svg
       │       ├── happy.svg
       │       └── ...
       └── backdrops/
           ├── sala_de_aula.svg
           ├── bu_ufsc.svg
           └── ...
     ```
  3. **Sobrescrita Transparente:**
     Se uma usuária colocar um arquivo `assets/characters/arti/manifest.json` ou `backdrops/reitoria.svg` em seu diretório de dados local (`$XDG_DATA_HOME/ayvu/`), o motor priorizará a versão local sobre a versão de sistema.

## 3. Contratos de API
- `ayvu::assets::get_xdg_data_dirs() -> Vec<PathBuf>`: Retorna a lista ordenada por precedência dos caminhos de busca de assets.
- `ayvu::assets::discover_external_characters() -> Vec<CharacterPack>`: Varre dinamicamente os diretórios XDG montando os personagens disponíveis.
- `ayvu::assets::load_backdrop_from_disk(name: &str) -> Option<String>`: Busca arquivos `.svg` nos diretórios de backdrops XDG antes de recorrer ao vetor estático.
