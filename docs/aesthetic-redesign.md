# Especificação de Design Estético e Geometria Orgânica (Comic Chat / Jim Woodring)

Esta especificação define a evolução visual e geométrica do gerador de tirinhas **Ayvu**, abandonando formas vetoriais rígidas/artificiais em favor da estética clássica de quadrinhos de jornal consagrada pelo **Microsoft Comic Chat (1996)**, sob a direção de arte do quadrinista **Jim Woodring**.

---

## 1. Problemas da Renderização Anterior ("Dura")

1. **Ausência de Chão e Gravidade:** Painéis eram caixas brancas lisas onde personagens flutuavam no vácuo (`y = 145`), sem ponto de apoio ou perspectiva.
2. **Molduras de UI Web:** Uso de `<rect rx="4" class="panel-bg" />`, parecendo cartões de CSS/Bootstrap em vez de quadros desenhados à mão.
3. **Rótulos Redundantes de Interface:** Nomes e indicações literais como `(neutral)` ou `(happy)` impressos embaixo dos personagens, quebrando a imersão de gibi.
4. **Cruzamento de Balões e Caudas:** Balões com largura fixa de 240px ocupavam todo o painel, fazendo caudas cruzarem os corpos dos balões ou cruzarem-se entre si.
5. **Personagens Clipart:** Figuras compostas por elipses e polígonos com contorno uniforme de 2.5px, sem gestos de corpo, sem hachuras e sem expressividade de gibi.

---

## 2. Princípios de Staging e Moldura Orgânica

### 2.1 Linha de Chão e Sombra de Contato
- Todo painel possui uma **linha de horizonte/piso** no terço inferior (`py + PANEL_HEIGHT - 45.0`) com uma textura sutil de piso ou hachura de nanquim.
- Sob cada personagem é projetada uma **sombra de contato elíptica** suave (`rgba(0,0,0,0.12)`), garantindo que o personagem "pise" no cenário.

### 2.2 Molduras com Traço de Nanquim e Cantos de Prancheta
- Em vez de um retângulo arredondado digital, a borda do painel utiliza linhas de nanquim com pequenos cruzamentos nos cantos (*corner ticks* ou *overshoot* de prancheta de quadrinista):
  - Traços que se estendem 3px além das interseções dos vértices.
  - Variação visual sutil de espessura de nanquim (`stroke-linecap="square"` ou `"round"`).

### 2.3 Remoção de Legendas Artificiais
- O balão de fala apontando para a boca do personagem identifica o falante.
- A pose corporal e a expressão facial transmitem a emoção, sem necessidade de texto explicativo no rodapé.

---

## 3. Algoritmo de Balões Anti-Crossing (Regra Comic Chat)

Inspirado nas diretrizes do paper da Microsoft Research (*Kurlander, Skelly, Salesin, SIGGRAPH 1996*):

1. **Largura Dinâmica:** O balão não é mais estático em 240px. A largura ajusta-se ao comprimento do texto (entre 120px e 170px), permitindo que dois balões convivam confortavelmente sem competir pelo mesmo espaço horizontal.
2. **Posicionamento por Falante:**
   - O primeiro interlocutor é posicionado à esquerda (`px + 15.0`); seu balão fica ancorado no quadrante superior esquerdo.
   - O segundo interlocutor é posicionado à direita (`px + 175.0`); seu balão fica ancorado no quadrante superior/médio direito.
3. **Imunidade a Cruzamento:**
   - A cauda do balão esquerdo desce diretamente para a boca do personagem da esquerda.
   - A cauda do balão direito desce diretamente para a boca do personagem da direita.
   - Nenhuma cauda cruza o balão vizinho e nenhuma cauda intercepta a cauda do outro interlocutor.

---

## 4. Personagens Clássicos em Traço de Nanquim

O elenco padrão é alinhado com os clássicos imortalizados por Jim Woodring no Microsoft Comic Chat:

1. **Tiki (Cachorrinha):**
   - Orelhas caídas orgânicas, focinho expressivo, linguagem corporal canina clássica.
2. **Jordan (O Intelectual):**
   - Sobretudo verde, óculos redondos, cabelo despenteado, gesticulação com as mãos (pensando, apontando, gesticulando no riso/raiva).
3. **Armando (O Sofisticado):**
   - Terno, óculos escuros, sorriso expressivo e posturas dramáticas.

### Tabela de Emoções Suportadas por Personagem
Cada personagem implementa as 9 emoções canônicas do Ayvu com linguagem de corpo e rosto:
- `neutral`: pose relaxada e atenta.
- `happy`: sorriso aberto, postura ereta e braços receptivos.
- `sad`: ombros caídos, olhar para baixo, braços recolhidos.
- `angry`: sobrancelhas franzidas, punhos cerrados ou braços cruzados.
- `surprised`: olhos arregalados, boca em 'O', mãos erguidas em sobressalto.
- `thinking`: mão no queixo, olhar voltado para o alto.
- `shouting`: boca aberta pronunciada, corpo inclinado para frente.
- `laughing`: cabeça jogada levemente para trás, olhos cerrados de rir.
- `whispering`: corpo curvado com mão ao lado da boca em confidência.
