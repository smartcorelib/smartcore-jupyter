# Smartcore with Jupyter

Try latest [Smartcore](https://smartcorelib.org) (development branch) in your browser using Jupyter Notebooks.

## Install

Miniconda3 is needed, download latest version from [here](https://docs.conda.io/en/latest/miniconda.html). This setup is based on [evcxr](https://github.com/google/evcxr/).

Once Miniconda3 is installed: 
```
$ conda create --name smartenv
$ conda activate smartenv
(smartenv) $ conda install -y -c conda-forge nb_conda_kernels
(smartenv) $ cargo install --version 0.18.0 evcxr_jupyter --force
(smartenv) $ evcxr_jupyter --install
```

You can find tutorials at:
* [link](https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/)
* [link](https://datacrayon.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)

## Usage

Run `jupyter notebook` while the Conda environment is active:
```
(smartenv) $ jupyter notebook
```

Open notebooks examples in your browser.

Feel free to contribute your notebook.