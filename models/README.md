# MNIST Digit Classifier Model and cli

This project provides a command-line interface (CLI) to train a Convolutional Neural Network (CNN) on the MNIST dataset using PyTorch and Typer.

## 🧠 Model Overview

- The model is a simple CNN defined in `model.py` (you'll need to implement or provide it).
- Trained on the MNIST dataset (handwritten digit classification).
- Saves the trained model to `digit_detector.pth`.

## 📦 Requirements

Install the required dependencies using [`uv`](https://github.com/astral-sh/uv):

```bash
uv venv
source .venv/bin/activate
uv pip install -e .
```
## Execute the training script 
```bash
cli --help
```

## Package Structure
```
src/
├── gm_models/train/ # Training CLI
├── gm_models/models # Machine learning model definitions and related code
├── pyproject.yaml # Specify dependencies
├── uv.lock # Package lock
└── README.md # Project documentation
```
### Training

```
$ train --help
                                                                                                                                                                            
 Usage: train [OPTIONS]                                                                                                                                                      
                                                                                                                                                                             
 Train a CNN model on the MNIST dataset.                                                                                                                                     
                                                                                                                                                                             
                                                                                                                                                                             
╭─ Options ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ --epochs                    INTEGER  [default: 5]                                                                                                                         │
│ --batch-size                INTEGER  [default: 64]                                                                                                                        │
│ --lr                        FLOAT    [default: 0.001]                                                                                                                     │
│ --install-completion                 Install completion for the current shell.                                                                                            │
│ --show-completion                    Show completion for the current shell, to copy it or customize the installation.                                                     │
│ --help                               Show this message and exit.                                                                                                          │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯ 
```
