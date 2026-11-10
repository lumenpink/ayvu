# Especificação Técnica: Personagens Temáticos da UFSC (#19)

## 1. Contexto & Motivação
Para enriquecer a ambientação e a identidade do gerador de tirinhas `ayvu`, o elenco nativo é expandido com duas figuras lendárias e onipresentes no campus da UFSC (Trindade/Pantanal):
1. **Capivara**: O mamífero símbolo da convivência universitária nos gramados do campus, serena, impassível e reflexiva.
2. **Jacaré**: O jacaré-do-papo-amarelo que habita as margens do córrego da Trindade, carismático, com sorriso marcante e olhar arguto.

## 2. Padrão Visual e Especificação Vetorial
Ambos os personagens devem seguir estritamente o estilo de traço fino de nanquim sobre prancheta limpa, com hachuras orgânicas e traços expressivos para as 8 emoções canônicas.

### Capivara (`capivara`)
- **Proporção**: 180 x 360 (postura assentada/atenta no plano do chão).
- **balloon_anchor**: `{ x: 0.72, y: 0.28 }` (focinho/narina superior).
- **hand_anchor**: `{ x: 0.68, y: 0.62 }` (pata dianteira de apoio/empunhadura).
- **8 Emoções**: `neutral`, `happy`, `laughing`, `shouting`, `thinking`, `angry`, `sad`, `scared`.

### Jacaré (`jacare`)
- **Proporção**: 190 x 380 (postura semi-ereta característica, cauda no solo, papo texturizado).
- **balloon_anchor**: `{ x: 0.75, y: 0.32 }` (abertura das mandíbulas).
- **hand_anchor**: `{ x: 0.64, y: 0.58 }` (garra/pata dianteira).
- **8 Emoções**: `neutral`, `happy`, `laughing`, `shouting`, `thinking`, `angry`, `sad`, `scared`.

## 3. Contrato de Integração
- Empacotamento em `assets/characters/capivara/` e `assets/characters/jacare/` com `manifest.json`.
- Registro formal no `CharacterRegistry` padrão em `src/assets.rs`.
- Disponibilidade total no parser de diálogos via prefixos `Capivara:` e `Jacare:` / `Jacaré:`.
