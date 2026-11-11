# Especificação Técnica: Reengenharia de Cenários em Nanquim Rico (#20)

## 1. Contexto & Problema
Os cenários anteriores utilizavam geometria simplista (retângulos básicos e linhas chapadas com aspecto de desenho amador/Paint), destoando radicalmente da qualidade estética do elenco nativo de personagens e do padrão de pranchas clássicas de tirinhas em nanquim.

## 2. Árvore de Decisão
- **Alternativa Descartada (Bitmaps rasterizados / Fotos convertidas em JPEG/PNG):** Descartado por inflar o repositório/binário e quebrar a escalabilidade vetorial SVG do motor.
- **Alternativa Descartada (Manter geometria plana minimalista):** Descartado por falta de densidade visual e profundidade espacial.
- **Abordagem Escolhida (Arquitetura Vetorial de Nanquim com Hachuras):**
  1. Utilizar traços finos de nanquim (`stroke="#111111"`, espessuras `1.0` a `2.0`).
  2. Implementar hachuras paralelas e cruzadas (*cross-hatching*) para volumetria, sombras de edifícios, texturas de tijolos, madeira, folhagens e asfalto.
  3. Composição em perspectiva com linhas de fuga e planos de profundidade (fundo, meio e chão).
  4. Manter total compatibilidade com a linha de piso e sombras de contato dos personagens.

## 3. Catálogo de Cenários (Total: 18)
### Cenários Anteriores Reestilizados:
1. `sala_de_aula`: Lousa com fórmulas matemáticas densas em giz, esquadrias de madeira com hachura e piso tabuado.
2. `cafe_ccs`: Balcão com máquina de expresso detalhada, vitrine de salgados e azulejos em nanquim.
3. `laboratorio`: Bancadas de experimentos, microscópios, vidraria com graduações e armários de reagentes.
4. `hu_fachada`: Arquitetura do Hospital Universitário Polydoro Ernani com esquadrias seriadas e ambulância/estacionamento.
5. `ru`: Bandejão do Restaurante Universitário, esteira de bandejas, panelões e fila em perspectiva.
6. `hall_ccs`: Vãos arquitetônicos amplos, pilastras de concreto aparente com textura de fôrma e murais.
7. `hu_enfermaria`: Leitos com cortinas hospitalares, monitores de sinais vitais e suporte de soro.
8. `bu_ufsc`: Biblioteca Universitária Central, estantes altas cheias de lombadas de livros e mezanino.
9. `praca_cidadania`: Centro do campus, canteiros floridos com hachura, luminárias clássicas e calçamento.
10. `ctc`: Centro Tecnológico, vigas metálicas aparentes, painéis e laboratórios de engenharia.
11. `cfh`: Centro de Filosofia e Ciências Humanas, escadarias icônicas, cartazes em nanquim e pátio.
12. `pantanal`: Vegetação pantaneira do campus, capins altos com traço fino, margem d'água e pontilhismo.
13. `room`: Quarto/estúdio com estante de quadrinhos, mesa de prancheta, luminária articulada e janela.
14. `pastoral`: Colinas com grama em hachuras orgânicas e árvores com copas densas.

### 4 Novos Cenários Temáticos:
15. `bosque_cfh`: Troncos de eucaliptos altos, copas densas com luz filtrada, bancos de concreto com textura e chão de terra/folhas secas.
16. `reitoria`: Fachada do Prédio Central da UFSC, escadaria nobre, mastros de bandeiras, painéis e marquise em perspectiva.
17. `pista_atletismo`: Pista de atletismo do CDS com curvas e raias numeradas, postes de iluminação e arquibancadas em nanquim.
18. `centro_eventos`: Centro de Cultura e Eventos com suas linhas modernistas curvas, grandes vidraças e esplanada da Praça da Cidadania.
