# Especificação Técnica: Recriação da Personagem Billi em Nanquim (#24)

## 1. Contexto & Problema
A personagem **Billi** foi provisoriamente gerada como uma silhueta negra uniforme com uma estampa circular "B" no peito, funcionando apenas como marcador de posição (*placeholder*) durante a transição do Comic Chat para o elenco nativo.
Com a disponibilização das fotos reais de referência em `~/ref/billi`, torna-se necessário recriar a personagem de forma definitiva em nanquim vetorial autêntico, garantindo expressividade, consistência com o elenco nativo (Arti, Capivara, Jacaré) e reconhecimento visual imediato.

## 2. Árvore de Decisão
- **Alternativa Descartada (Manter silhueta negra com inicial 'B'):** Descartado formalmente por ser uma solução provisória simplista, sem traços fisionômicos, anatomia ou riqueza artística.
- **Alternativa Descartada (Bitmap recortado / Foto rasterizada):** Descartado por inflar o repositório e violar a uniformidade vetorial SVG de alto contraste do motor Ayvu.
- **Abordagem Escolhida (Ilustração Vetorial de Nanquim Baseada em Fotos):**
  1. **Anatomia e Vestimenta (`perfil.jpg`):**
     - Blazer de alfaiataria preto estruturado aberto com lapelas angulares.
     - Blusa preta com decote em U exibindo a tatuagem de mariposa/borboleta com asas abertas no colo.
     - Calça de alfaiataria reta escura e botas de couro pretas de cano curto com salto bloco.
     - Postura ereta e confiante.
  2. **Rosto e Piercings Faciais Identitários (`~/ref/billi`):**
     - Cabelo curto desfiado (pixie ondulado) com franja repicada sobre a testa e mechas na altura das têmporas.
     - Delineador "gatinho" marcante (*wing liner*) em todos os olhos.
     - Conjunto completo de piercings: transversal na sobrancelha esquerda, bridge na ponte nasal entre os olhos, nostril na narina e piercings labiais múltiplos com esferas prateadas.
  3. **8 Poses Canônicas:**
     - `neutral`: Postura em repouso de corpo inteiro com sorriso sutil e mão direita preparada para empunhadura de props.
     - `happy`: Sorriso aberto mostrando dentes, bochechas elevadas e sobrancelhas arqueadas.
     - `laughing`: Risada ampla inspirada na selfie do espelho, cabeça levemente inclinada e boca aberta gargalhando.
     - `shouting`: Boca aberta projetando fala, corpo inclinado para a frente com energia.
     - `thinking`: Mão espalmada no queixo/bochecha (fiel à foto de referência `b87ccca4`), olhar para o alto.
     - `angry`: Sobrancelhas vincadas em V sobre o bridge piercing e lábios cerrados.
     - `sad`: Cantos dos lábios caídos, sobrancelhas arqueadas em tom melancólico e ombros relaxados.
     - `scared`: Olhos arregalados com pupilas puntiformes, boca em 'O' e postura de sobressalto.
  4. **Metadados e Âncoras Anatômicas:**
     - `viewBox`: `0 0 160 440`.
     - `balloon_anchor`: `x: 0.5, y: 0.18`.
     - `hand_anchor`: `x: 0.65, y: 0.48`.
