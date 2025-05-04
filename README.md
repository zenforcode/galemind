## GaleMind - StreamGale Machine Learning Inference Service

| ![Galemind Logo](./assets/galemind.png) | Galemind is an inference server designed to integrate with the StreamGale framework. It enables the deployment and serving of machine learning models with a clean, modular, and developer-friendly architecture. |
|:--:|:--|

---

## 🚀 Features

- **🧠 Model Serving** – Efficient deployment and inference of machine learning models.
- **🧩 Modular Architecture** – Clear separation between engine logic and model definitions.
- **🐳 Containerization Support** – Includes `.devcontainer/` setup for VS Code + Docker development.

---

## ⚙️ Getting Started

### Prerequisites

Make sure you have the following installed:

- Python 3.8+
- Rust
- [Docker](https://www.docker.com/) *(optional, for container-based dev)*
- [`uv`](https://github.com/astral-sh/uv) (fast Python package/dependency manager)
- `make`

---

### 🔧 Installation

1. **Clone the repository:**

```bash
git clone https://github.com/zenforcode/galemind.git
cd galemind
```

2. **Compile the core engine (Rust):**

```bash
cargo build 
```
---

## 🧱 License

This project is licensed under the [MIT License](./LICENSE).

