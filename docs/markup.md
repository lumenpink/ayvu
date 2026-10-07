# Ayvu Dialogue Markup Specification

> **Ayvu** (*Guarani*): Human language, speech, dialogue.
> Declarative dialogue format for compiling conversational text into comic strips.

---

## 1. Syntax Overview

Ayvu scripts are written in plain text, with each line representing a character dialogue, action, or panel boundary.

### Basic Dialogue
```text
Speaker: Speech text
```

### Dialogue with Explicit Emotion
```text
Speaker (emotion): Speech text
```

### Examples
```text
Alice (happy): Hey Bob, did you see the new release?
Bob (thinking): Should I tell her about the bugs?
Bob (laughing): Yes! It works like magic!
Alice (shouting): To the moon!
```

---

## 2. Emotions Catalog

Ayvu maps emotions to character poses and expressions:

| Emotion | Keywords / Aliases | Default Balloon |
| :--- | :--- | :--- |
| `neutral` | `neutro`, `normal`, `default` | Standard speech balloon |
| `happy` | `alegre`, `feliz`, `smile` | Standard speech balloon |
| `laughing` | `rindo`, `gargalhada`, `lol` | Standard speech balloon |
| `sad` | `triste`, `down` | Standard speech balloon |
| `crying` | `chorando`, `pranto` | Standard speech balloon with sweat/tear marks |
| `angry` | `bravo`, `zangado`, `furious` | Sharp / emphasized border |
| `shouting` | `gritando`, `grito`, `shout` | Jagged / spiky shout balloon |
| `thinking` | `pensando`, `pensativo` | Cloud balloon with trailing bubbles |
| `skeptical` | `cético`, `duvidoso`, `sus` | Standard speech balloon |
| `surprised` | `surpreso`, `espantado`, `wow` | Standard speech balloon |

---

## 3. Automatic Emotion & Balloon Heuristics

When no explicit emotion is specified, Ayvu inspects the text using linguistic heuristics:

1. **Thoughts:**
   - Text starting with `oO` or `...` triggers `BalloonType::Thought` and `Emotion::Thinking`.
2. **Shouts:**
   - Text in ALL CAPS with `!` or ending with `!!!` triggers `BalloonType::Shout` and `Emotion::Shouting`.
3. **Emoticons:**
   - `:)`, `:-)`, `\o/` $\rightarrow$ `Emotion::Happy`
   - `:D`, `xD`, `haha`, `kkk` $\rightarrow$ `Emotion::Laughing`
   - `:(`, `:-(` $\rightarrow$ `Emotion::Sad`
   - `:'(` $\rightarrow$ `Emotion::Crying`
   - `>:(`, `x(` $\rightarrow$ `Emotion::Angry`
   - `???` $\rightarrow$ `Emotion::Surprised`

---

## 4. Panel Boundaries

By default, Ayvu packages 1 to 2 consecutive dialogues into each comic panel. You can explicitly split panels using:

```text
Alice (happy): First panel dialogue.
---
Bob (surprised): Second panel dialogue!
```

Or using declarative directives:
```text
[panel title="Arrival"]
Alice: Here we are.
```

---

## 5. Layout & Panel Width Directives

### Panel Flow Layout (`[layout: ...]`)
* `[layout: strip]`: Horizontal classic comic strip (default).
* `[layout: grid]`: 2×2 square comic grid.
* `[layout: vertical]`: Webtoon-style vertical mobile strip.

### Dynamic Panel Widths (`[narrow]`, `[standard]`, `[wide]`)
* `[narrow]` or `[width: narrow]` (210 px): Solo monologues or intimate close-ups.
* `[standard]` or `[width: standard]` (280 px): Classic 2-speaker dialogues.
* `[wide]` or `[width: wide]` (360 px): Group panels (3 speakers) or sprawling panoramic backdrops.

---

## 6. Backdrops Catalog (`[backdrop: ...]`)

Ayvu includes vector-inked backdrops inspired by UFSC campus architecture and classic comic scenes:

| Backdrop Directive | Description |
| :--- | :--- |
| `[backdrop: praca_cidadania]` | Praça da Cidadania da UFSC em frente à Reitoria, palmeiras imperiais, postes esféricos e bancos de praça. |
| `[backdrop: ctc]` | Centro Tecnológico (CTC/UFSC), pilares brutalistas de concreto aparente, esteiras aéreas de cabos e rack de servidores. |
| `[backdrop: cfh]` | Bosquinho do CFH, frondosa copa de árvores nativas, mural estudantil com cartazes e mesa rústica de piquenique. |
| `[backdrop: pantanal]` | Bacia do Pantanal e Córrego da UFSC, morros de Floripa com torre de antena, ciclovia e capivara descansando na grama. |
| `[backdrop: hu_fachada]` | Fachada histórica do Hospital Universitário (HU/UFSC) com portão e palmeiras. |
| `[backdrop: ru]` | Restaurante Universitário (Bandejão do RU) com balcão térmico e mesas de inox. |
| `[backdrop: hall_ccs]` | Hall monumental do Centro de Ciências da Saúde (CCS/UFSC) com pórtico e jardineiras. |
| `[backdrop: bu_ufsc]` | Biblioteca Universitária (BU/UFSC) com estantes de livros, sinal de silêncio e baias de estudo. |
| `[backdrop: sala_de_aula]` | Sala de aula acadêmica com quadro-negro, janela para o campus e giz. |
| `[backdrop: cafe_ccs]` | Cafeteria do Centro de Ciências da Saúde com balcão e xícaras. |
| `[backdrop: laboratorio]` | Laboratório de análises com vidraria, microscópios e bancadas. |
| `[backdrop: hu_enfermaria]` | Enfermaria e leito clínico do HU com suporte de soro. |
| `[backdrop: pastoral]` | Cenário clássico Pastoral do Microsoft Comic Chat (1996). |
| `[backdrop: room]` | Quarto/sala clássico de tirinhas. |

---

## 7. Props Catalog (`[prop: ...]`)

Objetos de cena em nanquim autêntico com fixação inteligente no chão do quadro ou nas mãos do orador via `@ Speaker`:

| Prop Directive | Graphic Description |
| :--- | :--- |
| `[prop: bandeja]` | Bandeja de inox estampada do RU com divisórias de comida, talheres e sobremesa. |
| `[prop: mochila]` | Mochila universitária acolchoada com bolso frontal, zíper e garrafa de água lateral. |
| `[prop: celular]` | Smartphone com notificações de mensagens luminosas na tela e câmera frontal. |
| `[prop: guarda_chuva]` | Guarda-chuva clássico com cabo de madeira recurvado em J e capa dobrada. |
| `[prop: caderno]` | Caderno espiral com pauta, espirais aramadas e caneta esferográfica presa. |
| `[prop: cafe]` | Xícara de café fumegante sobre pires de porcelana. |
| `[prop: livro]` | Livro acadêmico aberto com marcador de fita vermelha e linhas de texto. |
| `[prop: laptop]` | Notebook aberto em perspectiva isométrica com código na tela e trackpad. |
| `[prop: erlenmeyer]` | Frasco de vidro Erlenmeyer com líquido químico azul reagente e graduação. |
| `[prop: giz]` | Bastão de giz escolar chanfrado com partículas de pó em suspensão. |
| `[prop: estetoscopio]` | Estetoscópio médico com hastes biauriculares e diafragma metálico. |
| `[prop: microfone]` | Microfone de conferência / púlpito com haste flexível e luz vermelha on-air. |

### Sintaxe de Fixação
* `[prop: cafe]` $\rightarrow$ Pousado no chão ou mesa do painel.
* `[prop: cafe @ Dan]` $\rightarrow$ Empunhado diretamente na mão/lado de Dan.

---

## 8. Onomatopoeias & Emotion Stickers

### Onomatopoeias de Impacto
* `[POW!]`, `[BAM!]`, `[WHAM!]`, `[CRASH!]`, `[KABOOM!]` $\rightarrow$ Estrela explosiva pontiaguda com tipografia de gibi.
* `[ZAP!]`, `[BANG!]`, `[BOOM!]`, `[SMACK!]` $\rightarrow$ Distorção angular e balão de choque.

### Stickers Expressivos
* `[heart]` $\rightarrow$ Coração flutuante de afeto.
* `[sweat]` $\rightarrow$ Gota de suor cômica estilo mangá/quadrinhos.
* `[idea]` $\rightarrow$ Lâmpada acesa de insight sobre a cabeça do personagem.
* `[question]` / `[exclamation]` $\rightarrow$ Pontuação flutuante estilizada.

