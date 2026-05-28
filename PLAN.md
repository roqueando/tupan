# Tupan Redesign — Plano de Produto e Engenharia

## 1. Contexto

### Problema
O Tupan atual é um notebook textual tradicional (células Python, execução sequencial, runtime Python persistente). Para engenharia elétrica, isso é insuficiente — o usuário quer **manipular parâmetros visualmente** (sliders, inputs) e **ver resultados instantaneamente** (gráficos, esquemáticos, métricas) sem escrever código.

### O que existe hoje
- Arquitetura funcional em Rust + egui/eframe
- Runtime Python persistente com células executáveis
- Persistência JSON de notebooks
- Gráfico de dependências e scheduler (esboços vazios)
- Módulo `reactive` (iniciado mas não implementado)
- UI com toolbar, células textuais, outputs

### O que será o Tupan
Uma aplicação interativa, visual e técnica para engenharia elétrica — um ambiente de trabalho para **cálculo, simulação e exploração de circuitos de eletrônica de potência**, construído em Rust + egui.

---

## 2. Visão e Proposta de Valor

> *"Um ambiente interativo onde engenheiros eletricistas alteram parâmetros de circuito com sliders e inputs, e veem esquemáticos, gráficos e métricas atualizados em tempo real — sem precisar escrever código."*

### Diferenciais
- **Visual-first**: diferente de notebooks tradicionais (Jupyter, MATLAB Live Script), a experiência primária é manipulação visual, não edição de código.
- **Tempo real**: cada slider move, cada input muda → recálculo automático + re-render de gráficos e esquemáticos.
- **Foco em eletrônica de potência**: conversores CC-CC e inversores CC-CA desde o MVP.
- **Desktop nativo**: sem browser, sem servidor, desempenho máximo com Rust + egui.

---

## 3. Escopo do MVP (Primeira Versão)

### Essencial (P0 — obrigatório para lançar)
| Funcionalidade | Descrição |
|---|---|
| **Conversor CC-CC (Buck/Boost)** | Modelo analítico completo: Vout, ripple, duty cycle, eficiência, perdas |
| **Inversor CC-CA (VSI monofásico)** | Modelo analítico: tensão de saída, THD, PWM senoidal |
| **Parâmetros editáveis por slider + input** | Frequência, duty cycle, L, C, carga, Vin, Vout (target) |
| **Painel de resultados em tempo real** | Métricas calculadas (ripple, eficiência, perdas, THD, etc.) |
| **Gráficos interativos (egui plot)** | Formas de onda (Vout, Iout), ripple, resposta temporal |
| **Esquemático funcional simplificado** | Diagrama SVG-like do conversor com valores anotados |
| **Recálculo automático** | Qualquer mudança em parâmetro → recalcula tudo |

### Importante (P1 — desejável no MVP)
| Funcionalidade | Descrição |
|---|---|
| **Simulação numérica** | Solução de EDOs (Runge-Kutta 4) para formas de onda realistas |
| **Exportação de schematic (SVG)** | Salvar diagrama como SVG |
| **Múltiplos conversores em um "projeto"** | Abas ou painéis para diferentes simulações |
| **Tema escuro/claro** | Seguir padrão egui |

### Futuro (P2 — pós-MVP)
| Funcionalidade | Descrição |
|---|---|
| Retificador CA-CC | Modelo analítico + simulação |
| Conversores trifásicos | VSI trifásico, PWM trifásico |
| Biblioteca de componentes magnéticos | Cálculo de indutor/transformador real |
| Células Python para extensão | Scripts customizados sobre os parâmetros |
| Dependência entre simulações | Parâmetros de um conversor alimentarem outro |
| CAD elétrico completo | Drag-and-drop de componentes (fora do escopo do MVP) |

---

## 4. Arquitetura Proposta

### Princípios
- **Separação clara entre domínio (elétrica), simulação, UI e estado**
- **Estado global imutável gerenciado por um único struct** (AppState)
- **Recálculo reativo**: mudança de parâmetro → dispara evento → pipeline de atualização
- **Cálculo analítico em Rust nativo** (sem Python para o core)
- **Python runtime mantido como opção futura** para scripts de extensão

### Diagrama de módulos

```
src/
├── main.rs                      # Entry point, eframe::run_native
│
├── app/
│   ├── mod.rs                   # TupanApp: estado, event loop, handlers
│   └── state.rs                 # AppState: struct único de estado global
│
├── domain/                      # *** Domínio de engenharia elétrica ***
│   ├── mod.rs
│   ├── converters/
│   │   ├── mod.rs
│   │   ├── buck.rs              # Modelo analítico Buck
│   │   ├── boost.rs             # Modelo analítico Boost
│   │   └── common.rs            # Parâmetros compartilhados (duty, freq, etc.)
│   ├── inverter/
│   │   ├── mod.rs
│   │   ├── vsi_single.rs        # VSI monofásico: PWM, THD
│   │   └── pwm.rs               # Geração PWM senoidal
│   ├── components/
│   │   ├── mod.rs
│   │   ├── inductor.rs          # Cálculo de indutor (ripple, núcleo)
│   │   ├── capacitor.rs         # Cálculo de capacitor (ripple, ESR)
│   │   └── load.rs              # Modelos de carga (R, RL, constante)
│   └── metrics/
│       ├── mod.rs
│       ├── ripple.rs            # Cálculo de ripple tensão/corrente
│       ├── efficiency.rs        # Perdas condução/comutação, eficiência
│       └── thd.rs               # THD para inversores
│
├── simulation/                  # *** Simulação numérica ***
│   ├── mod.rs
│   ├── integrator.rs            # Runge-Kutta 4 genérico
│   ├── circuit_odes.rs          # EDOs dos circuitos (Buck, Boost, VSI)
│   └── sim_result.rs            # Estrutura de resultado da simulação
│
├── schematic/                   # *** Representação visual de circuitos ***
│   ├── mod.rs
│   ├── layout.rs                # Posicionamento dos componentes no canvas
│   ├── primitives.rs            # Primitivas de desenho (fonte, indutor, chave, diodo, etc.)
│   └── export_svg.rs            # Exportação para SVG
│
├── ui/
│   ├── mod.rs
│   ├── workspace.rs             # Layout principal (painéis, abas)
│   ├── param_panel.rs           # Painel de parâmetros com sliders + inputs
│   ├── result_panel.rs          # Painel de resultados (métricas)
│   ├── schematic_view.rs        # Render do esquemático (egui paint)
│   ├── plot_panel.rs            # Gráficos interativos (egui_plot)
│   ├── toolbar.rs               # Toolbar superior (reaproveitar existente)
│   └── converter_selector.rs    # Seletor de tipo de conversor
│
├── reactive/                    # *** Sistema reativo (manter, expandir) ***
│   ├── mod.rs
│   ├── graph.rs                 # Grafo de dependências entre parâmetros
│   └── analyzer.rs              # Análise de dependências
│
├── execution/                   # *** Runtime (manter para futuro Python) ***
│   ├── mod.rs
│   ├── scheduler.rs
│   └── state.rs
│
├── notebook/                    # *** Persistência (manter) ***
│   ├── mod.rs
│   ├── ids.rs
│   ├── model.rs
│   └── persistence.rs
│
└── runtime/                     # *** Runtime Python (manter para futuro) ***
    ├── mod.rs
    ├── protocol.rs
    ├── python_process.rs
    └── worker.rs
```

---

## 5. Modelo de Estado da Aplicação

```rust
// src/app/state.rs

pub struct AppState {
    // Projeto ativo
    pub active_converter: ConverterType,
    
    // Parâmetros do conversor ativo
    pub params: ConverterParams,
    
    // Resultados do cálculo analítico (atualizados a cada mudança)
    pub results: ConverterResults,
    
    // Resultados da simulação numérica (atualizados após cálculo analítico)
    pub sim_results: Option<SimulationResults>,
    
    // Estado da UI
    pub theme: Theme,
    pub show_numerical_sim: bool,
    pub status_message: String,
}

pub enum ConverterType {
    Buck,
    Boost,
    VsiSinglePhase,
}

pub struct ConverterParams {
    // Entrada
    pub vin: f64,           // V
    pub vout_target: f64,   // V (target)
    
    // Comutação
    pub frequency: f64,     // Hz
    pub duty_cycle: f64,    // 0..1
    
    // Componentes
    pub inductance: f64,    // H
    pub capacitance: f64,   // F
    pub load_resistance: f64, // Ohm
    
    // Inversor-specific
    pub modulation_index: f64,  // 0..1 (para VSI)
    pub output_frequency: f64,  // Hz (para VSI)
}

pub struct ConverterResults {
    // Tensões e correntes
    pub vout: f64,          // Tensão de saída média
    pub iout: f64,          // Corrente de saída média
    pub iin: f64,           // Corrente de entrada média
    
    // Ripple
    pub vout_ripple: f64,   // Ripple de tensão de saída
    pub il_ripple: f64,     // Ripple de corrente no indutor
    
    // Perdas e eficiência
    pub conduction_losses: f64,
    pub switching_losses: f64,
    pub efficiency: f64,    // 0..1
    
    // Inversor
    pub thd: Option<f64>,
    pub rms_output: Option<f64>,
    pub fundamental_amplitude: Option<f64>,
}
```

---

## 6. Estratégia de Atualização em Tempo Real

### Pipeline de recálculo

```
Usuario mexe slider
       │
       ▼
TupanApp::on_param_changed()
       │
       ▼
1. Atualiza AppState.params
       │
       ▼
2. domain::calcula_analitico(state.params) → state.results
       │
       ▼
3. Se sim_numérica ativa:
   simulation::simular(state.params) → state.sim_results
       │
       ▼
4. ctx.request_repaint()  // egui re-renderiza na próxima frame
       │
       ▼
5. UI lê state.results e state.sim_results para renderizar
   - param_panel mostra sliders com valores atuais
   - result_panel mostra métricas
   - schematic_view mostra diagrama com valores anotados
   - plot_panel mostra gráficos atualizados
```

### Por que isso funciona
- egui já é **immediate mode**: o callback `ui()` é chamado ~60 FPS
- `ctx.request_repaint()` garante que a UI re-renderiza imediatamente após mudança
- Cálculo analítico é **O(1) ~ O(n)** — leva microssegundos, não prejudica a frame
- Simulação numérica (RK4) é mais pesada: pode ser executada em **background thread** se necessário

### Estado reativo
- Não precisamos de um framework reativo complexo
- O `AppState` é a única fonte de verdade
- `on_param_changed` dispara o pipeline completo
- Para evitar loops, sliders e inputs **não disparam recálculo se o valor não mudou de fato**

---

## 7. Estratégia de Simulação e Cálculo

### Cálculo Analítico (síncrono, mesma thread)
Usar equações fechadas de eletrônica de potência:

**Buck Converter (CCM):**
- `Vout = Vin * D`
- `ΔiL = Vin * D * (1-D) / (f * L)`
- `ΔVout = ΔiL / (8 * f * C)`
- Perdas condução: `I² * R_ds(on) * D + I² * R_L + I² * R_D * (1-D)`
- Perdas comutação: `Vin * Iout * (t_rise + t_fall) * f / 2`

**Boost Converter (CCM):**
- `Vout = Vin / (1 - D)`
- `ΔiL = Vin * D / (f * L)`
- `ΔVout = Iout * D / (f * C)`

**VSI Monofásico:**
- `Vout_fundamental = ma * Vin / 2` (PWM senoidal, ma = modulation index)
- `THD` aproximado por tabela ou cálculo de Fourier para PWM bipolar/unipolar

### Simulação Numérica (Runge-Kutta 4)
Para formas de onda realistas (não apenas valores médios):

```rust
// simulation/integrator.rs
pub fn rk4<F>(
    f: F,        // dy/dt = f(t, y)
    y0: &[f64],  // estado inicial
    t_span: (f64, f64),
    dt: f64,
) -> Vec<Vec<f64>>  // [t, y1, y2, ...]
```

- Buck: 2 estados (iL, vC)
- Boost: 2 estados (iL, vC)
- VSI: 2-3 estados (iL, vC, e referência PWM)

**Estratégia de execução:**
1. Se simulação numérica desligada: apenas cálculo analítico
2. Se ligada: roda após cálculo analítico, mesmos parâmetros
3. Se a simulação for pesada (>100k pontos), escalar `dt` dinamicamente
4. Opcional: executar em `std::thread` com `AtomicBool` para cancelamento

---

## 8. Representação dos Esquemáticos

### Abordagem: Diagrama Funcional (não CAD)
- Não é um editor de esquemáticos com drag-and-drop
- É uma **representação visual gerada** do conversor ativo
- O usuário vê o diagrama do circuito com os valores atuais anotados

### Implementação
```rust
// schematic/primitives.rs
pub enum SchematicElement {
    Source { pos: (f32, f32), label: String, value: String },
    Inductor { pos: (f32, f32), label: String, value: String },
    Capacitor { pos: (f32, f32), label: String, value: String },
    Diode { pos: (f32, f32), label: String },
    Switch { pos: (f32, f32), label: String },
    Load { pos: (f32, f32), label: String, value: String },
    Wire { from: (f32, f32), to: (f32, f32) },
    Node { pos: (f32, f32), label: String },
    Ground { pos: (f32, f32) },
}
```

- Usar `egui::Painter` (`Shape::line`, `Shape::circle`, `Shape::text`) para desenho
- Layout pré-definido para cada tipo de conversor (Buck, Boost, VSI)
- Anotar cada componente com seu valor atual (ex: "L = 100 μH")
- Cores para destacar caminho de corrente

### Exportação SVG
- `schematic/export_svg.rs`: converter `Vec<SchematicElement>` para string SVG
- Botão "Export SVG" no toolbar

---

## 9. Estratégia de Gráficos e Visualização

### Usar `egui_plot` (biblioteca oficial do egui)
- Plot linear para formas de onda (iL(t), vC(t), Vout(t))
- Plot de barras para comparação (ex: ripple vs parâmetro)
- Legendas, grid, zoom (egui_plot já suporta)

### Gráficos do MVP
1. **Forma de onda Vout (t)** — simulação numérica ou forma de onda teórica
2. **Forma de onda Iout (t)** — idem
3. **Corrente no indutor (t)** — ripple visível
4. **Tensão no capacitor (t)** — ripple visível
5. **Eficiência vs parâmetro** (futuro)

### Adicionar `egui_plot` ao Cargo.toml
```toml
eframe = { version = "0.34", features = ["default"] }
egui_plot = "0.34"
```

---

## 10. Estratégia de Testes

### Níveis de teste
1. **Testes unitários (domínio)**: cada função de cálculo analítico testada isoladamente
   - `buck::calculate_vout(vin, duty)` → valor esperado
   - `boost::calculate_ripple(vin, duty, f, L, C, R)` → valor esperado
   - `pwm::thd(ma, tipo)` → valor esperado
2. **Testes de simulação**: RK4 integra circuito conhecido → resultado esperado
3. **Testes de estado**: mudar parâmetro → estado reflete mudança
4. **Testes de persistência**: serializar/deserializar AppState

---

## 11. Riscos Técnicos e Mitigações

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|
| egui_plot não ter recursos suficientes para gráficos de engenharia | Média | Médio | egui_plot suporta LinePlot, scatter, barras. Se faltar algo, implementar custom widget com `egui::Painter` |
| Simulação numérica pesada bloqueia UI | Alta | Alto | Executar em thread separada com `Arc<Mutex<SimResult>>`. Usar `ctx.request_repaint()` para polling. |
| Layout do esquemático fica feio ou ilegível | Média | Médio | Iterar com feedback visual. Começar simples (Buck) e refinar. Usar coordenadas relativas ao tamanho do painel. |
| Sliders/inputs com precisão insuficiente para engenharia | Baixa | Médio | Usar `f64` internamente. Sliders com step configurável. Input numérico com validação. |
| Código do domínio (elétrica) misturado com UI | Médio | Alto | **Separação rigorosa**: `domain/` não importa `egui` nem `app`. Apenas structs de dados e funções puras. |
| Cálculos de perdas/comutação muito simplificados | Médio | Baixo | MVP usa modelos simplificados (primeira ordem). Documentar limitações. Expandir depois. |

---

## 12. Roadmap em Fases

### Fase 0 — Fundação (dias 1-2)
- [x] Refatorar `src/` para nova estrutura de pastas
- [x] Criar `app/state.rs` com `AppState` e `ConverterParams`
- [x] Implementar `domain/converters/buck.rs` (cálculo analítico completo)
- [x] Implementar `domain/converters/boost.rs`
- [x] Implementar `domain/metrics/ripple.rs`, `domain/metrics/efficiency.rs`
- [x] Escrever testes unitários para todos os cálculos
- [x] Adicionar `egui_plot` ao `Cargo.toml`

### Fase 1 — UI Essencial (dias 3-4)
- [x] `ui/workspace.rs`: layout com painéis (parâmetros, resultados, esquemático, gráficos)
- [x] `ui/param_panel.rs`: sliders + inputs para cada parâmetro
- [x] `ui/result_panel.rs`: métricas calculadas em tempo real
- [x] `ui/converter_selector.rs`: dropdown/abas para Buck, Boost, VSI
- [x] Conectar `on_param_changed` → pipeline de recalculo → `ctx.request_repaint()`

### Fase 2 — Esquemático e Gráficos (dias 5-7)
- [x] `schematic/primitives.rs`: implementar elementos básicos
- [x] `schematic/layout.rs`: layout do Buck e Boost
- [x] `ui/schematic_view.rs`: render com `egui::Painter`
- [x] `ui/plot_panel.rs`: gráficos com `egui_plot`
- [ ] `schematic/export_svg.rs`: exportar como SVG

### Fase 3 — Inversor e Simulação Numérica (dias 8-10)
- [x] `domain/inverter/vsi_single.rs`: modelo analítico VSI
- [x] `domain/inverter/pwm.rs`: geração PWM senoidal
- [x] `domain/metrics/thd.rs`: cálculo de THD
- [x] `simulation/integrator.rs`: RK4
- [x] `simulation/circuit_odes.rs`: EDOs para Buck, Boost, VSI
- [x] Checkbox "Simulação Numérica" na UI
- [x] Gráficos de forma de onda da simulação

### Fase 4 — Polimento e Exportação (dias 11-12)
- [x] Testes end-to-end (59 testes, app funcional)
- [x] Ajustes de UX (tooltips, labels, hints em todos os sliders)
- [x] Tema escuro/claro (toggle no toolbar)
- [x] Persistência do projeto completo (Save/Load com JSON via serde)
- [x] Exportação SVG funcional (botão no toolbar, gera .svg do esquemático)
- [x] README atualizado, exemplos

---

## 13. Critérios de Sucesso da Primeira Versão

1. Usuário consegue **selecionar Buck, Boost ou VSI monofásico**
2. Usuário consegue **alterar todos os parâmetros** via slider ou input numérico
3. **Todas as métricas** (Vout, Iout, ripple, eficiência, perdas) são **calculadas e exibidas instantaneamente**
4. **Esquemático funcional** é renderizado com valores anotados
5. **Gráficos** mostram formas de onda (pelo menos Vout e Iout vs tempo)
6. **Recálculo automático**: qualquer mudança de parâmetro → tudo atualiza em < 16ms (60 FPS)
7. Código do domínio **não importa egui** — módulos puramente funcionais
8. Testes unitários passam para todo `domain/`

---

## 14. Próximos Passos Técnicos Imediatos

### Passo 1 — Estrutura de pastas e Cargo.toml
```bash
# Criar diretórios
mkdir -p src/domain/converters src/domain/components src/domain/metrics
mkdir -p src/domain/inverter
mkdir -p src/simulation
mkdir -p src/schematic
mkdir -p src/ui

# Adicionar ao Cargo.toml
# egui_plot = "0.34"
```

### Passo 2 — Criar `app/state.rs`
Migrar o estado do `TupanApp` para um `AppState` separado. Manter o `TupanApp` como wrapper que gerencia o event loop.

### Passo 3 — Implementar modelo Buck
```rust
// src/domain/converters/buck.rs
pub fn calculate_buck(params: &ConverterParams) -> ConverterResults
```
Função pura, sem dependências de UI.

### Passo 4 — Conectar UI
Criar `param_panel.rs` com sliders para cada campo de `ConverterParams`. Cada `Slider::new()` chama `on_param_changed`.

### Passo 5 — Esquemático
Implementar `schematic/layout.rs` com posições fixas para Buck.
Renderizar com `ui.painter().line(...)` e `ui.painter().text(...)`.

---

## 15. Suposições Explícitas

1. **Regime permanente CCM**: os modelos analíticos assumem condução contínua (CCM) no MVP. DCM será adicionado depois.
2. **Componentes ideais**: sem perdas no núcleo magnético, sem resistência de PCB, sem parasitics (primeira ordem apenas).
3. **PWM senoidal bipolar**: para o VSI, assumimos PWM bipolar simples. Unipolar será adicionado depois.
4. **Carga resistiva**: no MVP, a carga é puramente resistiva. Carga RL/constante será adicionada depois.
5. **Sem threading complexo**: cálculo analítico é síncrono na thread da UI. Simulação numérica pode ir para thread separada se necessário.
6. **egui_plot é suficiente**: para o MVP, gráficos 2D com `egui_plot::Plot` atendem. Se precisar de mais, implementar widget custom.
7. **Layout de painéis fixo**: sidebar esquerda (parâmetros), centro (esquemático + gráficos), direita (resultados). Responsivo dentro do possível.

---

## 16. Plano de Execução — Etapas Curtas

### Etapa 1 — "Calc Engine Only"
- Criar estrutura de diretórios
- Implementar `domain/converters/buck.rs` com testes
- Implementar `domain/metrics/ripple.rs` e `domain/metrics/efficiency.rs`
- Implementar `ConverterParams` e `ConverterResults`
- **Verificação**: `cargo test` passa

### Etapa 2 — "Buck na Tela"
- Criar `app/state.rs` com `AppState`
- Refatorar `TupanApp` para usar `AppState`
- Implementar `ui/param_panel.rs` (sliders para Buck)
- Implementar `ui/result_panel.rs`
- Conectar pipeline recalculo
- **Verificação**: abrir app, mexer sliders, ver resultados mudarem

### Etapa 3 — "Buck Visual"
- Implementar `schematic/layout.rs` e `schematic/primitives.rs`
- Implementar `ui/schematic_view.rs`
- Implementar `ui/plot_panel.rs` com Vout(t) e iL(t) teóricos
- **Verificação**: mexer sliders → esquemático e gráficos atualizam

### Etapa 4 — "Boost e VSI"
- Implementar `domain/converters/boost.rs`
- Implementar `domain/inverter/vsi_single.rs`
- Adicionar `ui/converter_selector.rs` (abas)
- **Verificação**: alternar entre Buck, Boost, VSI — cada um com seus parâmetros e visualização

### Etapa 5 — "Simulação Numérica"
- Implementar `simulation/integrator.rs` (RK4)
- Implementar EDOs para Buck, Boost, VSI
- Adicionar checkbox na UI para ativar
- Conectar gráficos da simulação
- **Verificação**: ativar simulação → ver formas de onda realistas nos gráficos

### Etapa 6 — "Exportação e Polimento"
- Implementar `schematic/export_svg.rs`
- Persistência do estado completo
- Testes, ajustes finos, README
- **Verificação**: exportar SVG, salvar/carregar projeto, tudo funcionando
