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
Z \approx \trace (\ee^{- \Delta \tau T} \ee^{-\Delta \tau H_\text{I}})^M,
$$
where $M \Delta \tau = \beta$. What we are going to do is to calculate the path integral for each time step.

For the kinetic term, we introduce a matrix $\bold{T}$ such that 
$$
T = \bold{c}^\dagger_\uparrow \bold{T} \bold{c}_\uparrow + \bold{c}^\dagger_\downarrow \bold{T} \bold{c}_\downarrow.
$$
That is, only for nearest neighbors $i, j$, $T_{ij}$ has non-zero value $-t$, and otherwise $T_{ij}=0$.
Therefore, 

So in conclusion, at least theoretically, everything in the TODO



## Updating scheme

## 