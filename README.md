# LaMuralla GraphRAG Engine

![Rust](https://img.shields.io/badge/backend-Rust-orange?style=flat-square&logo=rust)
![Neo4j](https://img.shields.io/badge/database-Neo4j-blue?style=flat-square&logo=neo4j)
![Docker](https://img.shields.io/badge/deployment-Docker-2496ED?style=flat-square&logo=docker)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)

**[🇪🇸 Español](#-español) | [🇺🇸 English](#-english) | [🏴󠁥󠁳󠁣󠁴󠁿 Català](#-català)**

---

<a name="es"></a>
## 🇪🇸 Español

### Descripción
**LaMuralla GraphRAG** es un motor de **Búsqueda y Generación Aumentada (RAG) Híbrido** de alto rendimiento desarrollado en **Rust**. A diferencia de los sistemas RAG tradicionales que solo utilizan similitud vectorial, este sistema combina la potencia de los **Embeddings** con la estructura relacional de un **Grafo de Conocimiento (Neo4j)**.

El sistema permite ingestar documentos, extraer automáticamente entidades y relaciones, y ofrece una interfaz de chat interactiva donde los conceptos clave se convierten en **enlaces navegables** ("Deep Dive"), permitiendo una exploración no lineal de la información.

### 🚀 Funcionalidades Clave
*   **Backend de Alto Rendimiento:** Construido con Rust (Axum, Tokio) para máxima velocidad y seguridad de memoria.
*   **Recuperación Híbrida (Hybrid Retrieval):** Combina búsqueda vectorial (Vector Search) con expansión de vecindario en el grafo.
*   **Chat Semántico Interactivo:** El asistente IA devuelve respuestas con conceptos clicables (`[[Concepto]]`) y referencias a fuentes (`Ref: ID`).
*   **Visualización de Grafos:** Renderizado dinámico de nodos y relaciones utilizando Vis.js.
*   **Arquitectura Hexagonal:** Código modular y desacoplado (Domain, Ports, Adapters).

### 🛠️ Tecnologías
*   **Core:** Rust, Axum, Tokio, Serde.
*   **IA:** OpenAI (GPT-4o, text-embedding-3-small), Rig-Core.
*   **Base de Datos:** Neo4j (Graph DB + Vector Index).
*   **Frontend:** HTML5, Bootstrap 5, Vis.js (Server-Side Rendering con Tera).

### ⚙️ Configuración y Ejecución

#### Prerrequisitos
*   Rust (cargo)
*   Una instancia de Neo4j (Local o AuraDB)
*   API Key de OpenAI

#### 1. Configuración de Entorno
Crea un archivo `.env` en la raíz del proyecto:

```env
# Configuración del Servidor
PORT=3000
RUST_LOG=info

# Base de Datos (Ejemplo para Local)
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASS=tu_password

# Inteligencia Artificial
OPENAI_API_KEY=sk-proj-...
```

#### 2. Ejecutar Localmente
```bash
cargo run --release
```
Accede a: `http://localhost:3000`

#### 3. Ejecutar con Docker
```bash
docker build -t graph-rag-backend .
docker run -p 3000:3000 --env-file .env graph-rag-backend
```

---

<a name="en"></a>
## 🇺🇸 English

### Description
**LaMuralla GraphRAG** is a high-performance **Hybrid Retrieval-Augmented Generation (RAG)** engine built in **Rust**. Unlike traditional RAG systems that rely solely on vector similarity, this system leverages the power of **Embeddings** combined with the relational structure of a **Knowledge Graph (Neo4j)**.

The system allows for document ingestion, automatic entity and relationship extraction, and offers an interactive chat interface where key concepts become **navigable links** ("Deep Dive"), enabling non-linear information exploration.

### 🚀 Key Features
*   **High-Performance Backend:** Built with Rust (Axum, Tokio) for maximum speed and memory safety.
*   **Hybrid Retrieval:** Combines Vector Search with graph neighborhood expansion.
*   **Interactive Semantic Chat:** The AI assistant returns responses with clickable concepts (`[[Concept]]`) and source references (`Ref: ID`).
*   **Graph Visualization:** Dynamic rendering of nodes and relationships using Vis.js.
*   **Hexagonal Architecture:** Modular and decoupled code (Domain, Ports, Adapters).

### 🛠️ Tech Stack
*   **Core:** Rust, Axum, Tokio, Serde.
*   **AI:** OpenAI (GPT-4o, text-embedding-3-small), Rig-Core.
*   **Database:** Neo4j (Graph DB + Vector Index).
*   **Frontend:** HTML5, Bootstrap 5, Vis.js (Server-Side Rendering with Tera).

### ⚙️ Setup and Running

#### Prerequisites
*   Rust (cargo)
*   Neo4j Instance (Local or AuraDB)
*   OpenAI API Key

#### 1. Environment Configuration
Create a `.env` file in the project root:

```env
# Server Config
PORT=3000
RUST_LOG=info

# Database (Local example)
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASS=your_password

# AI Credentials
OPENAI_API_KEY=sk-proj-...
```

#### 2. Run Locally
```bash
cargo run --release
```
Access at: `http://localhost:3000`

#### 3. Run with Docker
```bash
docker build -t graph-rag-backend .
docker run -p 3000:3000 --env-file .env graph-rag-backend
```

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### Descripció
**LaMuralla GraphRAG** és un motor de **Cerca i Generació Augmentada (RAG) Híbrid** d'alt rendiment desenvolupat en **Rust**. A diferència dels sistemes RAG tradicionals que només utilitzen similitud vectorial, aquest sistema combina la potència dels **Embeddings** amb l'estructura relacional d'un **Graf de Coneixement (Neo4j)**.

El sistema permet la ingesta de documents, l'extracció automàtica d'entitats i relacions, i ofereix una interfície de xat interactiva on els conceptes clau es converteixen en **enllaços navegables** ("Deep Dive"), permetent una exploració no lineal de la informació.

### 🚀 Funcionalitats Clau
*   **Backend d'Alt Rendiment:** Construït amb Rust (Axum, Tokio) per a màxima velocitat i seguretat de memòria.
*   **Recuperació Híbrida (Hybrid Retrieval):** Combina cerca vectorial (Vector Search) amb l'expansió del veïnatge al graf.
*   **Xat Semàntic Interactiu:** L'assistent IA retorna respostes amb conceptes clicables (`[[Concepte]]`) i referències a fonts (`Ref: ID`).
*   **Visualització de Grafs:** Renderitzat dinàmic de nodes i relacions utilitzant Vis.js.
*   **Arquitectura Hexagonal:** Codi modular i desacoblat (Domain, Ports, Adapters).

### 🛠️ Tecnologies
*   **Core:** Rust, Axum, Tokio, Serde.
*   **IA:** OpenAI (GPT-4o, text-embedding-3-small), Rig-Core.
*   **Base de Dades:** Neo4j (Graph DB + Vector Index).
*   **Frontend:** HTML5, Bootstrap 5, Vis.js (Server-Side Rendering amb Tera).

### ⚙️ Configuració i Execució

#### Requisits previs
*   Rust (cargo)
*   Una instància de Neo4j (Local o AuraDB)
*   API Key d'OpenAI

#### 1. Configuració de l'Entorn
Crea un fitxer `.env` a l'arrel del projecte:

```env
# Configuració del Servidor
PORT=3000
RUST_LOG=info

# Base de Dades (Exemple per a Local)
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASS=el_teu_password

# Intel·ligència Artificial
OPENAI_API_KEY=sk-proj-...
```

#### 2. Executar Localment
```bash
cargo run --release
```
Accedeix a: `http://localhost:3000`

#### 3. Executar amb Docker
```bash
docker build -t graph-rag-backend .
docker run -p 3000:3000 --env-file .env graph-rag-backend
```