# Especificação Técnica — Elenco Nativo e Descontinuação do Comic Chat (#22)

- **ID do Ticket:** #22
- **Status:** Aprovado
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Vetorial

## 1. Contexto & Árvore de Decisão
O Ayvu nasceu experimentalmente portando regras e assets do Microsoft Comic Chat (1996). Contudo, essa base legada gerou uma contradição de engenharia e dívida técnica insustentável:
1. **Incompatibilidade Estética:** O motor adotou traço em nanquim vetorial de alto contraste, enquanto o Comic Chat utilizava bitmaps cortados em 256 cores da década de 90.
2. **Poluição e Sobrecarga de Binário:** O arquivo `src/assets.rs` acumulava ~1,95 MB de código Rust com 31 personagens antigos embutidos.
3. **Complexidade Algorítmica Artificial:** Fórmulas de escala legadas (`unitHeight / 1.9`, proporções VGA) limitavam o posicionamento dinâmico.

**Decisão:** Descontinuar integralmente os 31 personagens legados e estabelecer o elenco nativo de 6 personagens do universo Ayvu.

## 2. Requisitos Técnicos

### 2.1 Elenco Nativo
O elenco oficial passa a ser composto exclusivamente por 6 personagens:
1. **Arti** (`arti`): Acadêmico misterioso de sobretudo preto, cabelos longos e óculos escuros (masculino, nanquim).
2. **Raio** (`raio`): Silhueta masculina preenchida em branco com contorno preto e estampa circular "R" no peito.
3. **Sombra** (`sombra`): Silhueta feminina preenchida em preto nanquim com estampa circular "S" no peito.
4. **Billi** (`billi`): Base feminina Sombra com estampa circular "B" no peito.
5. **Aly** (`aly`): Base feminina Sombra com estampa circular "A" no peito.
6. **Lu** (`lu`): Base feminina Sombra com estampa circular "L" no peito.

### 2.2 Estrutura Modular de Assets
Cada personagem possui diretório dedicado em `assets/characters/<id>/` contendo:
- `manifest.json`: metadados, identificação, descrição e pontos de ancoragem (`balloon_anchor` e `props_anchor`).
- 8 poses vetoriais canônicas em SVG: `neutral.svg`, `happy.svg`, `laughing.svg`, `shouting.svg`, `thinking.svg`, `angry.svg`, `sad.svg`, `scared.svg`.

### 2.3 Desacoplamento do Renderizador
O arquivo `src/renderer/svg.rs` deve:
- Operar com caixas de colisão e proporções vetoriais normalizadas, sem depender de constantes mágicas de 1996.
- Manter e valorizar a geometria analítica de balões orgânicos (curvas de Bézier e prevenção de cruzamento de rabichos).
- Resolver automaticamente qualquer um dos 6 personagens do elenco pelo nome ou slug.

### 2.4 Migração da Suíte de Testes
- Purgar `tests/comic_chat_engine_test.rs` e substituí-lo por `tests/ayvu_engine_test.rs`, assegurando que o novo elenco seja carregado e renderizado com precisão.
