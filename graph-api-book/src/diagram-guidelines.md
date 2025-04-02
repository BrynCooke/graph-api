# SVG Diagram Design Guidelines

This document outlines the design principles and styling conventions for SVG diagrams used within the Graph API book.
Adhering to these guidelines ensures visual consistency and clarity.

## General Principles

* **Clarity:** Diagrams should be easy to understand at a glance. Avoid unnecessary complexity.
* **Consistency:** Use the defined styles, colours, and fonts consistently across all diagrams.
* **Accessibility:** Ensure sufficient colour contrast and consider users with colour blindness. Text should be legible.
* **Simplicity:** Favour clean lines and minimal decoration.

## Styling (Referencing `theme/custom.css`)

The primary source of styling is `theme/custom.css`. It uses CSS variables for theming (light/dark mode). Diagrams
should utilize these variables and classes where possible.

### Colour Palette (`:root` variables)

* **Background:** `var(--graph-bg)`
* **Text:**
    * Default: `var(--text-color)`
    * Light (on dark elements): `var(--text-color-light)`
    * Muted: `var(--text-color-muted)`
* **Nodes:**
    * Fill: `var(--node-fill)`
    * Stroke: `var(--node-stroke)`
    * Visited Fill: `var(--node-fill-visited)`
    * Visited Stroke: `var(--node-stroke-visited)`
* **Edges:**
    * Stroke: `var(--edge-stroke)`
    * Visited Stroke: `var(--edge-stroke-visited)`
* **Accents:**
    * Primary (Blue): `var(--accent-primary)` - Used for active elements, start nodes, pointers.
    * Secondary (Orange): `var(--accent-secondary)` - Used for path elements, end nodes, highlights.
    * Highlight Stroke: `var(--accent-highlight-stroke)` (Yellow)
* **Indexes:**
    * Background: `var(--index-bg)`
    * Border: `var(--index-border)`
    * Pointer Stroke: `var(--index-pointer-stroke)` (Uses primary accent)
* **Highlights:**
    * Fill: `var(--highlight-fill)` (Semi-transparent orange)
    * Stroke: `var(--highlight-stroke)` (Uses secondary accent)

### Fonts & Sizes (`:root` variables)

* **Font Family:** `var(--font-family)` (System font stack)
* **Font Sizes:**
    * Normal: `var(--font-size-normal)` (14px)
    * Small: `var(--font-size-small)` (12px)
    * Large: `var(--font-size-large)` (16px)
* **Font Weights:**
    * Normal: `var(--font-weight-normal)` (400)
    * Bold: `var(--font-weight-bold)` (600)
* **Stroke Widths:**
    * Thin: `var(--stroke-width-thin)` (1px)
    * Normal: `var(--stroke-width-normal)` (1.5px)
    * Thick: `var(--stroke-width-thick)` (2.5px)
    * Extra Thick: `var(--stroke-width-extra-thick)` (4px)
* **Node Geometry:**
    * Radius (for circles): `var(--node-radius)` (20px)
    * Padding (internal): `var(--node-padding)` (10px)
    * Corner Radius (for rects): `var(--node-corner-radius)` (5px)

### SVG Structure and Classes

Diagrams should use semantic class names that correspond to the styles in `custom.css`.

* **Nodes:**
    * Container: `<g class="node">`
    * Shape: `<circle>`, `<ellipse>`, `<rect>` within `.node`
    * Label: `<text class="node-label">` (uses `--text-color-light`)
    * Properties: `<text class="property-text">` (uses `--text-color-muted`, small font)
        * Use `<tspan>` for multi-line properties.
    * States: Add classes like `.active`, `.visited`, `.path`, `.start`, `.end`, `.highlighted` to the
      `<g class="node">` element.
    * Filtered Out: Use `<line class="filtered-out-mark">` (draws an 'X').
* **Edges:**
    * Container: `<g class="edge">`
    * Line/Path: `<line>` or `<path>` within `.edge`
    * Label: `<g class="edge-label"><text>...</text></g>` (uses `--text-color-muted`, small font)
    * Use `class="halo"` on text elements for readability against complex backgrounds.
    * States: Add classes like `.active`, `.visited`, `.path` to the `<g class="edge">` element.
    * Direction: Add `class="directed"` to the `<g class="edge">` for arrowheads.
* **Arrowheads (Markers):**
    * Define `<marker>` elements with IDs (e.g., `id="arrowhead"`, `id="arrowhead-active"`).
    * Use `class="marker-base"` and specific classes like `.marker-arrowhead`, `.marker-arrowhead-active`, etc. on the
      `<path>` inside the marker to control the arrowhead's fill color via CSS.
    * **Apply markers using `marker-start` or `marker-end` *attributes* directly on the
      target `<line>`, `<path>`, `<polyline>`, or `<polygon>` elements in the SVG markup (
      e.g., `<line ... marker-end="url(#arrowhead)">`). Do not rely on applying markers via CSS rules, as browser
      support is inconsistent.**
* **Indexes:**
    * Container: `<g class="index-container">`
    * Box: `<rect class="index-box">`
    * Title: `<text class="index-title">`
    * Entry: `<g class="index-entry">`
        * Highlight entry with `class="highlighted"` on the `<g>`.
    * Keys/Values/Tokens: Use classes like `.index-key`, `.index-value`, `.index-token` combined with type `.hash`,
      `.range`, `.fulltext` on `<text>` elements.
    * Dividers: `<line class="index-divider">` with `.hash`, `.range`, or `.fulltext`.
    * Pointers: `<path class="index-pointer" marker-end="url(#arrowhead-pointer)">` (apply marker attribute directly).
* **Query Highlights:**
    * Box: `<rect class="query-highlight-box">`
    * Text: `<text class="query-text">`
* **Scan Pointers (No Index):**
    * Line: `<line class="scan-pointer" marker-end="url(#arrowhead-scan)">` or
      `<path class="scan-pointer" marker-end="url(#arrowhead-scan)">` (apply marker attribute directly).
* **Step Indicators:**
    * Label: `<g class="step-label"><text>Step <tspan class="code">N</tspan>: ...</text></g>`
    * Arrow: `<path class="step-arrow" marker-end="url(#arrowhead-pointer)">` (apply marker attribute directly).

### Layout and Spatial Arrangement

* **SVG Canvas (`viewBox`):** All SVG diagrams **MUST** use a `viewBox` attribute with a fixed width of 600 units. The
  height should be adjusted as needed for the content. Example: `viewBox="0 0 600 350"`. Do not use `width` or `height`
  attributes on the root `<svg>` element; rely on the `viewBox` for scaling.
* **Margins:** Ensure adequate empty space (padding) around the entire diagram content within the SVG canvas. Avoid
  elements touching the edges of the SVG viewbox unless intentional (e.g., for background elements). A margin of at
  least 10-20px is recommended.
* **Spacing:** Maintain consistent visual spacing *between* elements (nodes, edges, labels, index boxes). Use node
  dimensions and padding variables (`--node-padding`, `--node-radius`) as guides for minimum spacing. Avoid
  overcrowding.
* **Alignment & Justification:**
    * Align related elements logically (e.g., center text within nodes, left-align lists of properties, align index
      entries vertically).
    * Use `text-anchor` (middle, start, end) and `dominant-baseline` (central, middle, hanging, text-before-edge)
      attributes correctly for precise text positioning relative to its coordinates.
    * Center diagrams horizontally within their container where possible.
* **Text Placement & Readability:**
    * Position labels close to the element they describe without overlapping other elements or lines.
    * Labels on arrows should be placed in the middle of the arrow if it is a vertical arrow, or on top of the arrow if
      horizontal.
    * For text associated with arrows (like `.step-label` or `.index-pointer`), place the text clearly offset from the
      arrow's path and arrowhead to avoid collision. Consider placing text *above* or *beside* the arrow shaft.
    * **Text Readability (`halo` class):** Use the `halo` class (defined in `custom.css`) on text elements that might
      overlap lines or complex backgrounds to ensure readability. This adds a background stroke matching the diagram
      background.

        * **Apply `halo` to:** Edge labels (`.edge-label text`), index text (`.index-entry text`, `.index-title`),
          query text (`.query-text`), step labels (`.step-label text`), property text (`.property-text`).
        * **Do NOT typically apply `halo` to:** Node labels (`.node-label`) or index text, as these are
          usually positioned over solid node backgrounds where a halo is unnecessary.

      Example usage: `<text class="edge-label halo">...`, `<text class="property-text halo">...`.
* **Rendering Order:** Ensure text elements are rendered *after* potentially overlapping graphical elements (lines,
  shapes) within the SVG structure or their respective groups. This prevents lines from being drawn over text halos.
* **Symmetry and Balance:** Strive for a balanced composition. Arrange elements symmetrically where it makes sense for
  the structure being depicted (e.g., in simple tree layouts). Avoid lopsided or visually jarring arrangements. Use
  grid-like alignments for index entries or lists.
* **Flow:** Arrange elements to guide the reader's eye naturally, typically left-to-right or top-to-bottom, following
  the logical flow of the process being illustrated (e.g., traversal steps).

## Legends and Keys

* **When to Use:** Include a legend if the diagram uses colours, line styles (dashed/solid), icons, or specific shapes
  to convey distinct meanings that aren't immediately obvious from context or labels alone. Examples: indicating
  different node types, edge relationships, or highlighting specific states (beyond the standard `.active`, `.visited`,
  `.path`).
* **Placement:** Position the legend clearly within the SVG canvas, typically in a corner (e.g., bottom-right or
  top-right), ensuring it doesn't obstruct the main diagram content. Use a simple bounding box if needed to separate it
  visually.
* **Content:** Keep legends concise. Show a small visual sample of the style (e.g., a coloured square, a line segment)
  next to a brief text description of its meaning. Use standard text sizes (`--font-size-small` or
  `--font-size-normal`).
* **Consistency:** Use the same styling for legend elements as used in the main diagram.

## Tooling

* **Recommended:** svgbob (via `mdbook-svgbob` preprocessor), Mermaid (via `mdbook-mermaid` preprocessor), or manual SVG
  creation/editing with tools like Inkscape or Figma.
* **Optimization:** Run SVGs through `svgo` or a similar tool to reduce file size before committing.
* **Export Settings (if manual):**
    * Ensure text is exported as text elements (`<text>`), not outlines/paths, unless unavoidable.
    * Embed fonts if using non-standard fonts (prefer standard/system fonts via `var(--font-family)`).
    * Use CSS classes for styling rather than inline `style` attributes where possible.

## Examples

*(Consider adding links to specific markdown files containing good examples of diagrams)*

* [No Index Example](./user_guide/property_graphs/no_index.md)
* [Hash Index Example](./user_guide/property_graphs/hash_index.md)
* *(Add more)*

## Checklist for New Diagrams

*   [ ] Does the diagram clearly convey its intended message?
*   [ ] Does it use the standard colour palette via CSS variables?
*   [ ] Is the font usage consistent (family, size, weight)?
*   [ ] Are shapes, lines, and spacing consistent with other diagrams?
*   [ ] Does the SVG use the required `viewBox="0 0 600 Y"`?
*   [ ] Does it use the defined CSS classes for elements (nodes, edges, indexes, etc.)?
*   [ ] Are states (active, visited, highlighted) applied correctly using classes?
*   [ ] Is text legible and contrast sufficient in both light and dark modes?
*   [ ] Does text that might overlap lines or other elements (edge labels, index text, query text, property text, etc.)
    use the `halo` class?
*   [ ] Is the `halo` class *not* used on node labels unless specifically needed?
*   [ ] Are text elements rendered *after* potentially overlapping shapes/lines in the SVG structure?
*   [ ] Has the SVG been optimized (e.g., using `svgo`)?
*

---
