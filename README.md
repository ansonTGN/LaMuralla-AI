# LaMuralla GraphRAG Engine

![Rust](https://img.shields.io/badge/backend-Rust-orange?style=for-the-badge&logo=rust)
![Neo4j](https://img.shields.io/badge/database-Neo4j-008CC1?style=for-the-badge&logo=neo4j&logoColor=white)
![Hybrid RAG](https://img.shields.io/badge/RAG-Hybrid-64748B?style=for-the-badge&logo=Neo4j)
![Docker](https://img.shields.io/badge/deployment-Docker-2496ED?style=for-the-badge&logo=docker)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

**[🇪🇸 Español](#-español) | [🇺🇸 English](#-english) | [🏴󠁥󠁳󠁣󠁴󠁿 Català](#-català)**

---

<a name="es"></a>
## 🇪🇸 Español: LaMuralla GraphRAG

### Descripción
**LaMuralla GraphRAG** es un motor de **Búsqueda y Generación Aumentada (RAG) Híbrido** de alto rendimiento, desarrollado en **Rust** (Axum/Tokio).

El sistema supera los límites de la búsqueda por similitud vectorial pura combinando el poder de los **Embeddings** con la estructura relacional de un **Grafo de Conocimiento (Neo4j)**. Permite la ingesta de documentos (PDF, DOCX, TXT), la extracción automática de conocimiento y ofrece un entorno de chat trazable e interactivo.

### 🚀 Funcionalidades Clave
*   **Recuperación Híbrida (Hybrid Retrieval):** Combina la búsqueda vectorial de fragmentos de documentos con la expansión de vecindario en el grafo de conocimiento, garantizando un contexto más rico y preciso.
*   **Chat Semántico Interactivo:** El asistente IA devuelve respuestas con conceptos clave convertidos en **enlaces navegables** (`[[Concepto]]`), permitiendo la exploración no lineal de la información ("Deep Dive").
*   **Inferencia y Razonamiento del Grafo:** Capacidad para ejecutar un proceso de `CONSOLIDAR GRAFO` que utiliza un LLM (GPT-4o por defecto) para **inferir y guardar nuevas relaciones** lógicas o sinónimos entre entidades existentes.
*   **Backend de Alto Rendimiento:** Construido en Rust para máxima velocidad y seguridad de memoria.
*   **Visualización:** Renderizado dinámico del grafo con Vis.js.

### 🛠️ Pila Tecnológica (Tech Stack)
| Componente | Tecnología | Propósito |
| :--- | :--- | :--- |
| **Backend** | `Rust`, `Axum`, `Tokio` | Servidor web asíncrono y de alto rendimiento. |
| **IA Core** | `Rig-Core` | Cliente de modelos de lenguaje y embeddings. |
| **Modelos** | `OpenAI` (GPT-4o, text-embedding-3-small), Ollama, Groq | Generación de texto, extracción de conocimiento y vectorización. |
| **Base de Datos** | `Neo4j` (Graph DB + Vector Index) | Almacenamiento del Grafo de Conocimiento y los Embeddings. |
| **Parsing** | `lopdf`, `zip`, `xml-rs` | Ingestión de formatos estructurados (PDF, DOCX). |
| **Arquitectura** | `Hexagonal` | Código modular (Domain, Ports, Adapters) para fácil mantenimiento. |

### ⚙️ Configuración y Ejecución

#### Prerrequisitos
*   **Rust** (con `cargo`)
*   Una instancia de **Neo4j** (Local o AuraDB)
*   **API Key** de OpenAI (o configuración de Ollama/Groq)

#### 1. Configuración de Entorno
Crea un archivo `.env` en la raíz (ejemplo basado en el `.env` adjunto):

```env
# Configuración del Servidor
PORT=3000
RUST_LOG=info

# Base de Datos
NEO4J_URI=neo4j+s://d8d2d63f.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=tu_password_segura

# Inteligencia Artificial (OpenAI por defecto)
AI_PROVIDER=openai
OPENAI_API_KEY=sk-proj-...
AI_MODEL=gpt-4o
AI_EMBEDDING_MODEL=text-embedding-3-small
AI_EMBEDDING_DIM=1536
```

#### 2. Ejecutar Localmente
```bash
cargo run --release
```
Accede a la interfaz web en: `http://0.0.0.0:3000` (Credenciales de demo: Preguntar al autor`)

#### 3. Ejecutar con Docker
```bash
# 1. Construir la imagen
docker build -t graph-rag-backend .

# 2. Ejecutar el contenedor, inyectando las variables de entorno
docker run -p 3000:3000 --env-file .env graph-rag-backend
```

---

<a name="en"></a>
## 🇺🇸 English: LaMuralla GraphRAG

### Description
**LaMuralla GraphRAG** is a high-performance **Hybrid Retrieval-Augmented Generation (RAG)** engine, built in **Rust** (Axum/Tokio).

The system goes beyond the limitations of pure vector similarity search by combining the power of **Embeddings** with the relational structure of a **Knowledge Graph (Neo4j)**. It allows for document ingestion (PDF, DOCX, TXT), automatic knowledge extraction, and offers a traceable, interactive chat environment.

### 🚀 Key Features
*   **Hybrid Retrieval:** Combines vector search of document chunks with graph neighborhood expansion, ensuring a richer and more precise context.
*   **Interactive Semantic Chat:** The AI assistant returns responses with key concepts converted into **navigable links** (`[[Concept]]`), enabling non-linear information exploration ("Deep Dive").
*   **Graph Inference and Reasoning:** The ability to run a `CONSOLIDATE GRAPH` process that uses an LLM (GPT-4o by default) to **infer and save new logical relationships** or synonyms between existing entities.
*   **High-Performance Backend:** Built on Rust for maximum speed and memory safety.
*   **Visualization:** Dynamic graph rendering using Vis.js.

### 🛠️ Tech Stack
| Component | Technology | Purpose |
| :--- | :--- | :--- |
| **Backend** | `Rust`, `Axum`, `Tokio` | Asynchronous, high-performance web server. |
| **AI Core** | `Rig-Core` | Language model and embeddings client. |
| **Models** | `OpenAI` (GPT-4o, text-embedding-3-small), Ollama, Groq | Text generation, knowledge extraction, and vectorization. |
| **Database** | `Neo4j` (Graph DB + Vector Index) | Storage for the Knowledge Graph and Embeddings. |
| **Parsing** | `lopdf`, `zip`, `xml-rs` | Ingestion of structured formats (PDF, DOCX). |
| **Architecture** | `Hexagonal` | Modular code (Domain, Ports, Adapters) for easy maintenance. |

### ⚙️ Setup and Running

#### Prerequisites
*   **Rust** (with `cargo`)
*   A **Neo4j Instance** (Local or AuraDB)
*   **OpenAI API Key** (or Ollama/Groq configuration)

#### 1. Environment Configuration
Create a `.env` file in the project root (example based on the attached `.env`):

```env
# Server Config
PORT=3000
RUST_LOG=info

# Database
NEO4J_URI=neo4j+s://d8d2d63f.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=your_secure_password

# Artificial Intelligence (OpenAI default)
AI_PROVIDER=openai
OPENAI_API_KEY=sk-proj-...
AI_MODEL=gpt-4o
AI_EMBEDDING_MODEL=text-embedding-3-small
AI_EMBEDDING_DIM=1536
```

#### 2. Run Locally
```bash
cargo run --release
```
Access the web interface at: `http://0.0.0.0:3000`

#### 3. Run with Docker
```bash
# 1. Build the image
docker build -t graph-rag-backend .

# 2. Run the container, injecting environment variables
docker run -p 3000:3000 --env-file .env graph-rag-backend
```

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català: LaMuralla GraphRAG

### Descripció
**LaMuralla GraphRAG** és un motor de **Cerca i Generació Augmentada (RAG) Híbrid** d'alt rendiment, desenvolupat en **Rust** (Axum/Tokio).

El sistema supera els límits de la cerca per similitud vectorial pura combinant el poder dels **Embeddings** amb l'estructura relacional d'un **Graf de Coneixement (Neo4j)**. Permet la ingesta de documents (PDF, DOCX, TXT), l'extracció automàtica de coneixement i ofereix un entorn de xat traçable i interactiu.

### 🚀 Funcionalitats Clau
*   **Recuperació Híbrida (Hybrid Retrieval):** Combina la cerca vectorial de fragments de documents amb l'expansió del veïnatge al graf de coneixement, garantint un context més ric i precís.
*   **Xat Semàntic Interactiu:** L'assistent IA retorna respostes amb conceptes clau convertits en **enllaços navegables** (`[[Concepte]]`), permetent l'exploració no lineal de la informació ("Deep Dive").
*   **Inferència i Raonament del Graf:** La capacitat d'executar un procés de `CONSOLIDAR GRAFO` que utilitza un LLM (GPT-4o per defecte) per **inferir i guardar noves relacions** lògiques o sinònims entre entitats existents.
*   **Backend d'Alt Rendiment:** Construït en Rust per a màxima velocitat i seguretat de memòria.
*   **Visualització:** Renderitzat dinàmic del graf utilitzant Vis.js.

### 🛠️ Pila Tecnològica (Tech Stack)
| Component | Tecnologia | Propòsit |
| :--- | :--- | :--- |
| **Backend** | `Rust`, `Axum`, `Tokio` | Servidor web asíncron i d'alt rendiment. |
| **IA Core** | `Rig-Core` | Client de models de llenguatge i embeddings. |
| **Models** | `OpenAI` (GPT-4o, text-embedding-3-small), Ollama, Groq | Generació de text, extracció de coneixement i vectorització. |
| **Base de Dades** | `Neo4j` (Graph DB + Vector Index) | Emmagatzematge del Graf de Coneixement i els Embeddings. |
| **Parsing** | `lopdf`, `zip`, `xml-rs` | Ingestió de formats estructurats (PDF, DOCX). |
| **Arquitectura** | `Hexagonal` | Codi modular (Domain, Ports, Adapters) per a fàcil manteniment. |

### ⚙️ Configuració i Execució

#### Requisits previs
*   **Rust** (amb `cargo`)
*   Una instància de **Neo4j** (Local o AuraDB)
*   **API Key** d'OpenAI (o configuració d'Ollama/Groq)

#### 1. Configuració de l'Entorn
Crea un fitxer `.env` a l'arrel del projecte (exemple basat en el `.env` adjunt):

```env
# Configuració del Servidor
PORT=3000
RUST_LOG=info

# Base de Dades
NEO4J_URI=neo4j+s://d8d2d63f.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=la_teva_contrasenya_segura

# Intel·ligència Artificial (OpenAI per defecte)
AI_PROVIDER=openai
OPENAI_API_KEY=sk-proj-...
AI_MODEL=gpt-4o
AI_EMBEDDING_MODEL=text-embedding-3-small
AI_EMBEDDING_DIM=1536
```

#### 2. Executar Localment
```bash
cargo run --release
```
Accedeix a la interfície web a: `http://0.0.0.0:3000`

#### 3. Executar amb Docker
```bash
# 1. Construir la imatge
docker build -t graph-rag-backend .

# 2. Executar el contenidor, injectant les variables d'entorn
docker run -p 3000:3000 --env-file .env graph-rag-backend
```