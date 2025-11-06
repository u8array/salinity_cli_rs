
# salinity_cli_rs

Command-line tool to estimate **Practical Salinity** `SP`, **Absolute Salinity** `SA`, **density** `ρ`, and optional component balances from macro chemical analyses of seawater or reef aquaria.  
Calculations follow TEOS-10 conventions and couple charge/mass balances with density via a simple fixed-point iteration.

## Features

- Compute `SP` and `SA` from elemental/ionic analyses
- TEOS-10 density `ρ(SA,CT,p)` and specific gravity at a reference temperature
- Self-consistent conversion between mg/L and mg/kg via `ρ`
- Optional component tables in mg/L and mg/kg, plus normalization to `SP = 35`
- Chloride estimation from electroneutrality if not measured
- Configurable assumptions: temperature `T`, pressure `p`, alkalinity, borate fraction

## Install

## Quick start

Example with concentrations in mg/L, `T=20 °C`, `p=0 dbar`, and total alkalinity `8 dKH`:

```bash
salinity_cli_rs \
  --na 10780 --mg 1290 --ca 420 --k 400 --sr 8 \
  --br 65 --cl auto --f 1.3 \
  --s 900 --b 4.4 \
  --temp 20 --pressure 0 \
  --alk-dkh 8 \
  --assume-borate true \
  --return-components
```

`--cl auto` triggers Cl⁻ from charge balance. `--return-components` prints mg/L, mg/kg, and SP-35 normalized values.

---

## Background

### Practical vs Absolute Salinity

- **SP**: dimensionless, historically defined from conductivity relative to standard seawater at 15 °C and 1 bar.
- **SA**: g/kg of dissolved material. Primary TEOS-10 salinity variable.  
  For near-standard composition:
  \[
  SA \approx SR = SP\cdot\frac{SR_{\mathrm{REF}}}{35},\qquad SR_{\mathrm{REF}}=35.16504\ \mathrm{g\ kg^{-1}}.
  \]
  Composition anomalies would add \(\delta SA_{\text{composition}}\). This tool assumes \(\delta SA_{\text{composition}}\approx 0\).

### Units and basic conversions

For species \(i\) with input concentration \(c_i\) in mg/L and molar mass \(M_i\) in g/mol:
\[
n_i\ [\mathrm{mol\ L^{-1}}] = \frac{\max(c_i,0)}{1000}\cdot\frac{1}{M_i}.
\]

### Alkalinity split (approximate)

Total alkalinity in dKH:
\[
A_{\mathrm{meq/L}} = \mathrm{dKH}\cdot 0.357.
\]
Split into species by fixed fractions:
\[
(A_{\mathrm{HCO_3^-}}, A_{\mathrm{CO_3^{2-}}}, A_{\mathrm{OH^-}}) = (0.89,\,0.10,\,0.01)\cdot A_{\mathrm{meq/L}},
\]
with stoichiometry
\[
n_{\mathrm{HCO_3^-}}=A_{\mathrm{HCO_3^-}},\quad
n_{\mathrm{CO_3^{2-}}}=\frac{A_{\mathrm{CO_3^{2-}}}}{2},\quad
n_{\mathrm{OH^-}}=A_{\mathrm{OH^-}}.
\]
Mass equivalent for reporting uses a configurable \(\mathrm{mg/meq}\) (default 50.043 mg/meq as CaCO₃).  
Note: exact speciation is pH and DIC dependent; fixed fractions are a robust aquarium approximation.

### Boron partition

Total B split by borate fraction \(\alpha_B\in[0,1]\):
\[
n_{\mathrm{B(OH)_4^-}}=\alpha_B\,n_B,\qquad
n_{\mathrm{B(OH)_3}}=(1-\alpha_B)\,n_B.
\]
This affects charge and mass because species have different molar masses.

### Electroneutrality and Cl⁻ estimation

If Cl⁻ is not provided, solve from
\[
\sum_i z_i n_i = 0.
\]
Positive: \(\mathrm{Na^+}, \mathrm{Mg^{2+}}, \mathrm{Ca^{2+}}, \mathrm{K^+}, \mathrm{Sr^{2+}}\).  
Negative: \(2\,\mathrm{SO_4^{2-}}, \mathrm{Br^-}, \mathrm{F^-}, \mathrm{B(OH)_4^-}, \mathrm{HCO_3^-}, \mathrm{CO_3^{2-}}, \mathrm{OH^-}\).  
Assign the residual negative charge to \(\mathrm{Cl^-}\) and clamp at zero if needed.

### Reference mass per kg

A reference composition in mmol/kg is converted to g/kg using molar masses. Two corrections apply:

1. Replace elemental B by chosen species masses \( \mathrm{B(OH)_3},\ \mathrm{B(OH)_4^-}\) per \(\alpha_B\).
2. Optionally add a reference alkalinity mass from a chosen `ref_alk_dKH` (default 8.0; 6.2 available for RN compatibility).

Denote the resulting reference total as \(\Sigma^{\mathrm{ref}}_{\mathrm{g/kg}}\).

### Relative salinity and fixed-point iteration

Measured mass per liter:
\[
\Sigma_{\mathrm{g/L}} = \sum_j m_j\ \mathrm{[g/L]}.
\]
Convert to g/kg via density \(ρ\):
\[
\Sigma_{\mathrm{g/kg}} = \frac{\Sigma_{\mathrm{g/L}}}{ρ/1000}.
\]
Define relative salinity from the ratio to reference:
\[
SR_{\text{new}} = SR_{\mathrm{REF}}\cdot \frac{\Sigma_{\mathrm{g/kg}}}{\Sigma^{\mathrm{ref}}_{\mathrm{g/kg}}},\quad
SP_{\text{new}}=35\cdot\frac{SR_{\text{new}}}{SR_{\mathrm{REF}}},\quad
SA_{\text{new}}\approx SR_{\text{new}}.
\]
Because \(ρ\) depends on \(SA\) and \(CT\), and \(SA\) depends on \(SP\), iterate to a fixed point:

1. Initialize \(SP=35\), \(SA = SP\cdot SR_{\mathrm{REF}}/35\).
2. Compute \(CT=\mathrm{CT}(SA,T,p)\), \(ρ=\mathrm{ρ}(SA,CT,p)\) via TEOS-10.
3. Convert all component g/L to g/kg using \(ρ\) and sum to \(\Sigma_{\mathrm{g/kg}}\).
4. Update \(SR\), \(SP\), \(SA\).
5. Stop when \(|SP_{\text{new}}-SP|<\varepsilon\).

Convergence is fast because the density feedback is weak in this range.

### TEOS-10 relations used

- Convert `SP→SA`:
  \[
  SA \approx SP\cdot\frac{SR_{\mathrm{REF}}}{35}.
  \]
- Conservative temperature:
  \[
  CT = \mathrm{CT\_from\_t}(SA, T, p).
  \]
- Density:
  \[
  ρ = \mathrm{ρ}(SA, CT, p).
  \]
- Specific gravity at \(t_\mathrm{ref}\):
  \[
  SG(t_\mathrm{ref}/t_\mathrm{ref})=
  \frac{ρ_{\mathrm{sw}}(SA,t_\mathrm{ref},p_\mathrm{ref})}{ρ_{\mathrm{pw}}(SA{=}0,t_\mathrm{ref},p_\mathrm{ref})}.
  \]

### Component reporting and normalization

For comparability, normalize to `SP = 35`:
\[
\text{norm\_factor}=\frac{35}{SP},\qquad m^\star=m\cdot\text{norm\_factor}.
\]

---

## CLI options (subset)

- `--na --mg --ca --k --sr --br --cl --f --s --b` concentrations in mg/L (`--cl auto` for charge-balance estimate)
- `--temp` °C, `--pressure` dbar
- `--alk-dkh` total alkalinity in dKH
- `--assume-borate true|false`, `--borate-fraction 0..1`
- `--ref-alk-dkh` reference alkalinity for reference mass construction
- `--return-components` print mg/L, mg/kg, SP-35 normalized
- `--sg-at 20|25` specific gravity at 20/20 or 25/25

The exact flag names follow the current binary.

---

## Output example

```text
SP: 34.98
SA: 35.16 g/kg
rho: 1024.6 kg/m^3
SG(20/20): 1.0260
```

With `--return-components`, the tool adds component tables in mg/L, mg/kg, and SP-35 normalization.

## Assumptions and limits

- \(\delta SA_{\text{composition}}\) neglected; acceptable for near-NSW compositions.
- Fixed alkalinity fractions are pH-independent; accuracy drops for unusual pH/CO₂.
- Cl⁻ by electroneutrality is sensitive to input errors.
- Optional default F⁻ used if missing.
- Iteration enforces consistency between volume-based inputs and mass-based reference.

## Validation

Compare `SP` against conductivity-derived `SP` and ensure TEOS-10 `ρ(SA,CT,p)` matches expected densities at 20 °C or 25 °C for typical reef mixes.

## Acknowledgments

- TEOS-10 thermodynamic framework  
- Standard seawater reference \(SR_{\mathrm{REF}}=35.16504\ \mathrm{g/kg}\)

## Build

Requires last stable Rust Toolchain.

```bash
cargo build --release
```
