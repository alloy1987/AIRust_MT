<p align="center">
  <img src="public/app-icon.svg" width="140" alt="Logo de AIRust_MT" />
</p>

<h1 align="center">AIRust_MT</h1>

<p align="center">
  Editor de escritorio Markdown WYSIWYG (lo que ves es lo que obtienes) reconstruido con <b>Rust + Tauri 2</b><br />
  Replica la experiencia de escritura de MarkText con un tamaño menor y un menor consumo de recursos
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-v0.1.3-1f6feb" alt="versión v0.1.3" />
  <img src="https://img.shields.io/badge/license-MIT-31a354" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-Tauri%202-ff6b6b" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Vue-3%20%2B%20TypeScript-42b883" alt="Vue 3 + TypeScript" />
  <img src="https://img.shields.io/badge/AI-vibe_coding-ff6b6b" alt="vibe-coding" />
  <img src="https://img.shields.io/badge/editor-alloy1987-7048e8" alt="alloy1987" />
</p>

> La gran mayoría del código de este proyecto fue escrita por el autor mediante **vibe coding**: usando [DeepSeek V4 Flash](https://www.deepseek.com/) y [Qwen 3.8](https://www.qianwenai.com/) como modelos de codificación y [opencode](https://opencode.ai) como agente de programación con IA, bajo diseño y revisión humanos. Consulte [Método de desarrollo](#método-de-desarrollo) más abajo.

---

## 📖 Índice

- [✨ Características](#características)
- [📦 Instalación](#instalación)
- [🔐 Privacidad](#privacidad)
- [⚙️ Stack tecnológico](#stack-tecnológico)
- [⌨️ Atajos de teclado](#atajos-de-teclado)
- [🧱 Estructura del proyecto](#estructura-del-proyecto)
- [🛠️ Compilación](#compilación)
- [🤝 Proyectos de referencia y agradecimientos](#proyectos-de-referencia-y-agradecimientos)
- [🤖 Método de desarrollo](#método-de-desarrollo)
- [🧑‍💻 Palabras del autor](#palabras-del-autor)
- [📜 Licencia](#licencia)

## ✨ Características

- **Edición WYSIWYG**: basada en el núcleo de editor de MarkText `@muyajs/core`, renderiza al instante mientras escribes, sin necesidad de vista previa en paneles divididos
- **Ricos elementos de bloque**: encabezados, listas, tablas, bloques de código (resaltado de sintaxis), fórmulas matemáticas (KaTeX), diagramas (Mermaid / flowchart / PlantUML / Vega), bloques HTML, Front Matter, etc.
- **Múltiples pestañas**: edita varios documentos a la vez, con aviso de cambios sin guardar
- **Barra lateral de archivos**: abre una carpeta como espacio de trabajo, con árbol de archivos para explorar, crear, renombrar y eliminar
- **Edición de texto plano**: además de Markdown, abre y edita archivos de texto plano comunes como texto sin formato (sin análisis): datos y configuración (.json / .yaml / .yml / .xml / .toml / .ini / .csv / .env), documentos y páginas web (.txt / .html / .htm / .css / .rtf) y código fuente (.py / .js / .ts / .java / .c / .cpp / .go / .rs); la extensión del archivo actual se muestra en la barra de estado
- **Detección inteligente de texto**: la posibilidad de abrir un archivo la decide su contenido, no su extensión: los archivos de texto con extensiones desconocidas se abren sin problema, mientras que los archivos binarios se detectan y se rechazan con una advertencia
- **Panel de esquema**: navegación rápida por niveles de encabezado
- **Buscar / reemplazar**: compatible con expresiones regulares, distinción de mayúsculas y coincidencia de palabras completas
- **Soporte de imágenes**: pega o arrastra imágenes para guardarlas automáticamente en el directorio del documento e insertarlas
- **Observación de archivos**: aviso automático cuando un documento se modifica externamente en el disco
- **Detección de codificación**: detección y conversión automática de codificaciones de archivos no UTF-8 basada en `encoding_rs` + `chardetng`
- **Manejo de archivos grandes**: los archivos muy grandes se abren en modo de vista previa de solo lectura para evitar bloqueos
- **12 temas de piel**: Blanco brillante, Negro oscuro, Índigo, Verde esmeralda, Naranja atardecer, Azul mar profundo, Rosa, Oro amanecer, Menta, Azul cielo, Rosa melocotón, Lavanda
- **7 idiomas de interfaz**: 中文, English, 日本語, Русский, 한국어, Español, Français
- **Zoom de la interfaz**: Ctrl + rueda / menú de zoom, adaptado a pantallas de alto DPI
- **Instancia única**: al iniciar de nuevo, enfoca la ventana existente y abre el archivo
- **Instalador NSIS para Windows**: asistente de instalación multilingüe, admite abrir archivos arrastrando y soltando

## 📦 Instalación

### Windows (instalador .exe)

- Antes de instalar, asegúrate de que el sistema tenga instalado el **Microsoft Edge WebView2 Runtime** (web2view, abreviado WebView2);
- Durante la instalación, el instalador detectará automáticamente si WebView2 está instalado:
  - Si ya está instalado, la instalación continuará directamente;
  - Si no lo está, el instalador mostrará un aviso y descargará e instalará WebView2 automáticamente desde internet;
- También puedes descargar e instalar WebView2 manualmente con antelación. Página oficial de descarga: <https://developer.microsoft.com/microsoft-edge/webview2/>

### macOS y Linux

> Los instaladores para macOS y Linux aún no se han publicado, pero puedes compilar la aplicación desde el código fuente. Consulta la sección [Compilación](#compilación) a continuación.

## 🔐 Privacidad

> La aplicación funciona en general **de forma local y sin conexión**, salvo las siguientes funciones de diagramas, que requieren conexión a internet al renderizar:
>
> - **Diagramas PlantUML**: el código fuente del diagrama se envía al servidor público `plantuml.com`, que devuelve la imagen renderizada; el contenido del diagrama sale de tu equipo;
> - **Diagramas de secuencia (sequence)**: al renderizar, las fuentes se cargan desde Google Fonts mediante webfontloader.
>
> Las demás funciones (edición, guardado, imágenes, detección de codificación, etc.) no realizan ninguna solicitud de red. Si el contenido del documento es confidencial, evita usar los dos tipos de diagramas anteriores.

## ⚙️ Stack tecnológico

| Capa                | Tecnología                                                                                                                                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Shell de escritorio | [Tauri 2](https://tauri.app) + Rust (`tauri-plugin-dialog` / `tauri-plugin-opener` / `tauri-plugin-single-instance`, observación de archivos `notify`, detección de codificación `encoding_rs` + `chardetng`) |
| Frontend            | Vue 3 + TypeScript + Vite + Pinia                                                                                                                                                                             |
| Núcleo del editor   | [@muyajs/core](https://github.com/marktext/marktext) (muya de MarkText, renderizado con DOM virtual snabbdom)                                                                                                 |
| Iconos de archivo   | @marktext/file-icons (de MarkText)                                                                                                                                                                            |

## ⌨️ Atajos de teclado

| Atajo                           | Función                |
| ------------------------------- | ---------------------- |
| `Ctrl + N`                      | Nuevo documento        |
| `Ctrl + O`                      | Abrir archivo          |
| `Ctrl + Shift + O`              | Abrir carpeta          |
| `Ctrl + S`                      | Guardar                |
| `Ctrl + Shift + S`              | Guardar como           |
| `Ctrl + F`                      | Buscar / reemplazar    |
| `Ctrl + Alt + F`                | Alternar barra lateral |
| `Ctrl + A`                      | Seleccionar todo       |
| `Ctrl + Z`                      | Deshacer               |
| `Ctrl + Shift + Z` / `Ctrl + Y` | Rehacer                |
| `Ctrl + 0`                      | Zoom al 100%           |
| `Ctrl + rueda`                  | Zoom de la interfaz    |

> En macOS se usa `Cmd` en lugar de `Ctrl`.

## 🧱 Estructura del proyecto

```
AIRust_MT/
├── src/                  # Frontend (Vue 3 + TypeScript + Pinia)
│   ├── components/       #    Componentes de interfaz (barra lateral, barra de pestañas, búsqueda, diálogos, etc.)
│   ├── stores/           #    Gestión de estado (editor, tema, zoom)
│   ├── editor/           #    Capa de adaptación del núcleo muya
│   └── api.ts            #    Wrappers de comandos de Tauri
├── src-tauri/            # Shell de escritorio (Rust + Tauri 2)
│   ├── src/              #    Comandos, observación de archivos, detección de codificación, manejo de archivos grandes, menús
│   └── nsis/             #    Scripts de gancho del instalador NSIS
├── editor/               # Núcleo y paquete de iconos provenientes de MarkText
│   ├── muya/             #    Núcleo WYSIWYG @muyajs/core
│   └── file-icons/       #    Iconos de archivo @marktext/file-icons
└── public/               # Recursos estáticos
```

## 🛠️ Compilación

Requisitos previos: [Rust](https://www.rust-lang.org/), [Node.js](https://nodejs.org/) ≥ 20, [pnpm](https://pnpmjs.com/) y las [dependencias del sistema de Tauri 2](https://tauri.app/start/prerequisites/).

Requisitos adicionales por plataforma:

- **Windows**: [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (normalmente ya instalado);
- **Linux**: instala los paquetes del sistema enumerados en los [requisitos oficiales de Tauri](https://tauri.app/start/prerequisites/), por ejemplo `webkit2gtk-4.1`, `libappindicator3`, `librsvg2-dev`;
- **macOS**: [Xcode Command Line Tools](https://developer.apple.com/xcode/) (`xcode-select --install`).

```bash
# Instalar dependencias
pnpm install

# Modo desarrollo (recarga en caliente)
pnpm tauri dev

# Compilar el instalador de lanzamiento
pnpm tauri build
```

Resultados de compilación por plataforma:

- Windows: `src-tauri/target/release/bundle/nsis/*.exe`
- macOS: `src-tauri/target/release/bundle/macos/*.app` y `dmg/*.dmg`
- Linux: `src-tauri/target/release/bundle/deb/*.deb`, `rpm/*.rpm`, `appimage/*.AppImage`

## 🤝 Proyectos de referencia y agradecimientos

Durante el desarrollo, este proyecto se basó en los siguientes proyectos de código abierto. Gracias a sus autores y colaboradores. El texto completo de las licencias correspondientes se incluye en la sección «Avisos de terceros» del archivo [LICENSE](LICENSE).

### [MarkText](https://github.com/marktext/marktext) (licencia MIT)

Este proyecto es una versión reescrita de MarkText; las siguientes partes provienen directamente de MarkText:

| Parte de referencia                        | Ubicación en este proyecto                    | Descripción                                                                                 |
| ------------------------------------------ | --------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Núcleo del editor muya                     | `editor/muya/` (`@muyajs/core`)               | Capacidades de edición esenciales: edición WYSIWYG, renderizado de bloques, atajos, formato |
| Iconos de archivo                          | `editor/file-icons/` (`@marktext/file-icons`) | Iconos del árbol de archivos de la barra lateral                                            |
| Forma del producto y diseño de interacción | Global                                        | Filosofía de escritura WYSIWYG, alcance de funciones y formas de interacción                |

### [Markpad](https://github.com/sftwrdotdev/Markpad) (licencia BSD 3-Clause)

Muchos detalles técnicos del lado de escritorio (Tauri 2) se basan en Markpad, principalmente:

| Parte de referencia                   | Descripción                                                                                                             |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Arquitectura de aplicación Tauri 2    | Organización de comandos del lado Rust y comunicación frontend-backend                                                  |
| Menús nativos y despacho de eventos   | Definición de elementos de menú y reenvío de eventos al frontend (ver `src-tauri/src/menu.rs`)                          |
| Prácticas de empaquetado e instalador | Configuración del instalador NSIS, scripts de gancho de instalación (`src-tauri/nsis/`) y otras prácticas de ingeniería |

### Agradecimiento especial

Quiero dar un **agradecimiento especial** a Mozilla y a la Rust Foundation: ¡gracias a sus esfuerzos tenemos un excelente lenguaje de programación, Rust!
En la ola de la programación asistida por IA, Rust se ha convertido en un lenguaje de desarrollo con gran potencial, principalmente por sus cuatro ventajas clave como lenguaje de sistemas de bajo nivel:

1. **Eficiencia de ejecución extrema y control de bajo nivel**
   Como lenguaje de programación de sistemas cercano al hardware, Rust prescinde del mecanismo de recolección de basura (GC) de los lenguajes tradicionales y ofrece abstracciones de coste cero. Esto le permite alcanzar un rendimiento de ejecución y un control de memoria comparables a C/C++ manteniendo la eficiencia del desarrollo, encajando perfectamente con las demandas de la era de la IA en cuanto a computación de alto rendimiento y alta concurrencia.

2. **Rigurosa seguridad de memoria y garantía de fiabilidad**
   Rust es conocido por su sintaxis rigurosa y sus singulares mecanismos de propiedad (Ownership) y comprobación de préstamos (Borrow Checker). Puede interceptar con precisión en tiempo de compilación peligros de seguridad de memoria como punteros nulos y carreras de datos. Esta característica de «seguridad en tiempo de compilación» proporciona una sólida red de seguridad de calidad para el código generado por IA, reduciendo enormemente el riesgo de fallos en tiempo de ejecución.

3. **Un sistema de tipos fuerte como «restricción semántica» para la IA**
   Rust posee un sistema de tipos fuerte, altamente estandarizado y riguroso. En el contexto de la programación con IA, este sistema de tipos no es solo una especificación del código, sino también un «navegador» para la IA. Las definiciones de tipo claras ayudan a la IA a comprender con mayor precisión la lógica de negocio y el flujo de datos, reduciendo eficazmente el código inválido producido por las «alucinaciones» o fallos lógicos de la IA, de modo que el código generado por IA es inherentemente más robusto.

4. **El compilador oficial como «estricto inspector de calidad»**
   Rust ofrece oficialmente una cadena de herramientas de compilador madura y extremadamente estricta. En los flujos de trabajo de desarrollo asistido por IA, la IA se encarga de generar rápidamente borradores de código, mientras que el compilador de Rust actúa como la primera y estricta puerta de control de calidad. Que el código generado por IA pase la compilación significa que se han eliminado la mayoría de los errores fatales de seguridad de memoria y concordancia de tipos. Este modelo complementario de «la IA produce, el compilador inspecciona» mejora enormemente la calidad de entrega del código de nivel industrial.
   **En resumen, en la era de la programación asistida por IA, elegir Rust como lenguaje de desarrollo tiene un altísimo valor estratégico. También espero ver a más personas capaces unirse a las filas de la programación en Rust y hacer que el ecosistema de Rust sea cada vez más rico.**

## 🤖 Método de desarrollo

Este proyecto es una práctica de **programación asistida por IA (vibe coding)**:

- **Modelo de codificación**: [DeepSeek V4 Flash](https://www.deepseek.com/) / [Qwen 3.8](https://www.qianwenai.com/)
- **Modelo de imágenes**: [Qwen 3.8](https://www.qianwenai.com/)
- **Agentes de programación**: [opencode](https://opencode.ai) (CLI interactivo de programación con IA), [Qwencode](https://www.qianwenai.com)
- **Rol humano**: definición de requisitos, decisiones de arquitectura, revisión de código y pruebas de aceptación

El proyecto toma MarkText como modelo, sustituyendo su shell de Electron por Rust + Tauri 2, mientras que el núcleo del editor sigue utilizando y adaptando el muya de MarkText.

## 🧑‍💻 Palabras del autor

**Ingeniero multidisciplinar**: llevo mucho tiempo en el sector financiero. No soy licenciado en informática, pero me apasiona la programación. Aprendí Python por mi cuenta y he desarrollado herramientas prácticas; tengo el nivel 5kyu de Python en Codewars.

**Practicante del vibe coding**: la IA ha permitido a la gente común cruzar el umbral de la programación. Creo que, en la era de la IA, el límite de la IA es el límite de tu imaginación, y todos pueden perseguir sus sueños con la IA.

**Novato en el código abierto**: este es mi primer proyecto en GitHub, así que inevitablemente habrá imperfecciones y carencias.

**Con ganas de intercambiar ideas**: serán muy bienvenidas las sugerencias de mejora en Issues o los PR. ¡Ayúdame a crecer!

**E-mail：**  20360505@qq.com

## 📜 Licencia

El proyecto en su conjunto se publica bajo la **[licencia MIT](LICENSE)**.

Dado que este proyecto deriva de / se basa en MarkText (MIT) y Markpad (BSD 3-Clause), para cumplir los requisitos de conformidad de ambas licencias:

- `editor/muya/` y `editor/file-icons/` conservan el aviso de derechos de autor MIT de MarkText;
- el archivo [LICENSE](LICENSE) incluye una sección «Avisos de terceros (Third-Party Notices)» que recoge íntegramente los avisos de derechos de autor originales y los textos completos de las licencias de MarkText y Markpad;
- los menús «Ayuda → Acerca de» y «Ayuda → Licencia» de la aplicación también muestran la información de atribución anterior.

---

<p align="center"><i>AIRust_MT — reinventa un clásico con IA y Rust para que escribir Markdown sea ligero.</i></p>
