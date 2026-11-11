# Especificação Técnica — Recriação do Backdrop do HU (#29)

- **ID do Ticket:** #29
- **Status:** Em Implementação
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Comic Chat (Jim Woodring) / Fotos de Referência (`~/ref/backdrop/hu`)

## 1. Contexto & Árvore de Decisão
O cenário do Hospital Universitário (`hu_fachada`) possuía uma representação genérica em SVG que não refletia a arquitetura real e marcante do HU da UFSC. O usuário forneceu fotografias de referência reais em `~/ref/backdrop/hu/` (`hu-ufsc.jpg` e `HU2.jpg`).

**Decisão para a Natasha:**
Recriar o backdrop `hu_fachada` (e alias `hu`) no estilo nanquim de alto contraste do Microsoft Comic Chat (Jim Woodring) capturando com precisão a paisagem real:
1. **Placa Monumental em Primeiro Plano:** Grande totem retangular metálico sobre poste vertical com os títulos e logotipo clássico do HU ("HOSPITAL UNIVERSITÁRIO - UNIVERSIDADE FEDERAL DE SANTA CATARINA - EBSERH") desenhados à mão em nanquim.
2. **Torre da Caixa D'água:** Cilindro vertical alto em concreto no lado esquerdo com o emblema no topo.
3. **Complexo dos Três Pavilhões Hospitalares:** Três blocos interligados em perspectiva com empenas verticais texturizadas com hachuras densas, janelas em fita e brises-soleils rítmicos.
4. **Postes Modernistas Duplos:** Postes com luminárias inclinadas em ângulo marcante.
5. **Cercamento e Paisagismo:** Gramado com hachuras de terra/vegetação, cerca de tela aramada em losangos finos e árvores ao fundo.

## 2. Compatibilidade & Assets Externos
O SVG deve ser atualizado tanto no motor embutido (`src/backdrop.rs`) quanto exportado para o diretório de assets desacoplados (`assets/backdrops/hu_fachada.svg`), assegurando carregamento transparente pelo resolvedor XDG.
