# DQMC simulation of the Hubbard model

## The Hubbard model and its partition function

The hubbard model is defined by
$$
H = \underbrace{-t \sum_{\pair{i, j}, \sigma} c^\dagger_{i \sigma} c_{j \sigma} + \text{h.c.}}_{T} + \underbrace{U \sum_i (n_{i \uparrow} - \frac{1}{2}) (n_{i \downarrow} - \frac{1}{2})}_{H_\text{I}}.
$$
Here we change the interaction Hamiltonian a little, which is actually just a readjustment of potential zero point, because the total number of particles are conserved.
We are going to simulate the model on a $L \times L$ lattice. Let $N = L^2$.

By Trotter decomposition we have
$$
Z \approx (\ee^{- \Delta \tau T} \ee^{-\Delta \tau H_\text{I}})^M,
$$
where $M \Delta \tau = \beta$.

## Updating scheme



## 