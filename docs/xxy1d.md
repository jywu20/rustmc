# Simulation of an 1D XXY spin chain

## Worldlines of up spins and their weights

An XXY spin chain has the following Hamiltonian:
$$
H = J_x \sum_i S_i^x S_{i+1}^x + J_z \sum_i S_i^z S_{i+1}^z,
$$
where all spin operators are spin-1/2 degree of freedoms.
This Hamiltonian is a quite local one, and therefore it can be expected that the Hamiltonian may be decomposed into the sum of a sequence of blocks, in which only nearest blocks do not commute.
The blocks can be easily found: they are $\{H^{(i)} = J_x S_i^x S_{i+1} + J_z S_i^z S_{i+1}^z \}_i$, and we can classify blocks which commute with each other into one group, and obtain
$$
H = H_1 + H_2, \quad H_1 = \sum_{n} H^{(2n+1)}, \quad H_2 = \sum_n H^{(2n+2)}.
$$
Terms in $H_1$ commute with each other, and so does terms in $H_2$, but $H_1$ does not commute with $H_2$.
It is handy to rewrite each block in the form of ladder operators, that is, 
$$
H^{(n)} = \frac{J_x}{2} (S_n^+ S_{n+1}^- + S_n^- S_{n+1}^+) + J_z S_n^z S_{n+1}^z,
$$
where
$$
S^\pm = S^x \pm \ii S^y.
$$
This implies it will be convenient to choose $\{S^z_i\}$ as a basis: the number of the up spins and the down spins are conserved, so we can view up spins as a kind of magnons, and view down spins as empty sites, and the Hamiltonian provides channels for a magnon to travel to a nearest unoccupied site.

By Trotter decomposition, we make the following approximation:
$$
\ee^{-\Delta \tau H} = \ee^{-\Delta \tau H_1} \ee^{-\Delta \tau H_2} + \bigO{\Delta \tau^2},
$$
and thus the partition function is given by the following discrete imaginary time path integral:
$$
Z = \sum_{\sigma_1, \ldots, \sigma_m} \mel{\sigma_1}{\ee^{-\Delta \tau H_1}}{\sigma_{2m}} \cdots \mel{\sigma_3}{\ee^{-\Delta \tau H_1}}{\sigma_2} \mel{\sigma_2}{\ee^{-\Delta \tau H_2}}{\sigma_1},
$$
where $m\Delta \tau = \beta$. The path-integral configuration $\sigma_1, \sigma_2, \ldots, \sigma_{2m}$ is in a 1+1 space, one dimension for the space and one dimension (labeled by the subscript) for the imaginary time.

Due to the conservation of the number of up spins and down spins, each configuration $\sigma_1, \sigma_2, \ldots, \sigma_{2m}$ is composed of worldlines of up spins, where in the direction of imaginary time evolution, interacting channels provided by $H_1$ and $H_2$ are opened successively: a full imaginary time evolution step $\Delta \tau$ actually consists of two steps, the first of which only enables hopping on odd bonds (or no hopping at all), and the second of which only enables hopping on even bonds.
Or, in other words, in the first step the worldline can only goes forward in odd plaquettes while in the second step, the worldline can only goes forward in even plaquettes.
We say even plaquettes in the first step and odd plaquettes in the second step *shaded* plaquettes.

This is exactly shown in the discrete path integral: a full time evolution step is $\mel{\sigma_3}{\ee^{-\Delta \tau H_1}}{\sigma_2} \mel{\sigma_2}{\ee^{-\Delta \tau H_2}}{\sigma_1}$, with one step evolving according to $H_1$ and a following step evolving according to $H_2$.
The fact that terms in $H_1$ commute with each other and so does terms in $H_2$ means the weight $\mel{\sigma_3}{\ee^{-\Delta \tau H_1}}{\sigma_2}$ or $\mel{\sigma_2}{\ee^{-\Delta \tau H_2}}{\sigma_1}$ can be computed directly by multiplication of $\mel{\sigma_{i, \tau+1} \sigma_{i+1, \tau+1}}{\ee^{-\Delta \tau H^{(i)}}}{\sigma_{i, \tau} \sigma_{i+1, \tau}}$.
This is the real reason we decompose $H$ into $H_1$ and $H_2$: the worldline picture of path integral configuration is a simple result of symmetry and has nothing to do with the fact that $H^{(n)} commutes with $H^{(n+2)}$, and decomposing the total Hamiltonian into $H_1$ and $H_2$ is just a way to easily calculate the weight of each configuration.

So in conclusion: the configuration of imaginary path integral of XXY model consists of worldlines of up spins in which each full imaginary time evolution step consists of a step with hopping on odd bonds and a step with hopping on even bonds, and to calculate the weight of one configuration all you need to do is to calculate $\mel{\sigma_{i, \tau+1} \sigma_{i+1, \tau+1}}{\ee^{-\Delta \tau H^{(i)}}}{\sigma_{i, \tau} \sigma_{i+1, \tau}}$ and multiply everything together.

Since every block $H^{(n)}$ shares the same form, all we need to do is to determine the matrix elements under $S^z$ basis of 
$$
H_\text{two site} = \frac{J_x}{2} (S_1^+ S_2^- + S_1^- S_2^+) + J_z S_1^z S_2^z.
$$
The eigenvectors are just the singlet and three triplet states, and the eigenvalues are $J_z/4$ for $\ket{1, 1}$ and $\ket{1, -1}$, $-J_z/4 - J_x/2$ for $\ket{0, 0}$ and $-J_z/4+J_x/2$ for $\ket{1, 0}$.
So we have
$$
\begin{aligned}
    \mel{\uparrow \downarrow}{\ee^{-\Delta \tau H}}{\uparrow \downarrow} &= \frac{1}{2} \mel{0, 0}{\ee^{-\Delta \tau H}}{0, 0} + \frac{1}{2} \mel{1, 1}{\ee^{-\Delta \tau H}}{1, 1} \\
    &= \frac{1}{2} \ee^{- \Delta \tau (- J_z / 4 - J_x / 2)} + \frac{1}{2} \ee^{- \Delta \tau (- J_z / 4 + J_x / 2)} \\
    &= \ee^{\Delta \tau J_z / 4} \cosh(\Delta \tau J_x / 2),
\end{aligned}
$$
and so does the other matrix elements. It is worth noting that the spin flip matrix elements are negative, but luckily no sign problem will arise because of this. 
We are working on a 1D spin chain, which is a non-frustrated system, and we know for a non-frustrated lattice we can divide the lattice into two sublattices in which if one site is in sublattice $A$, then its nearest sites must belong to sublattice $B$ to allow an up-down-up-down configuration when in an antiferromagnetism phase.
Therefore, we can add a minus sign to $S^x$s and $S^y$s in $A$ and find the transformation is a canonical transformation, and the only difference it makes to the Hamiltonian is that $J_x \to -J_x$, so now all factors of the weight is positive.

## Observables

There is no general way to detect observables in the worldline formalism described above because correlation functions are required for measuring observables in a general way, and in order to detect a correlation function we need to inserting terms like $B_i S^z_i$ into the Hamiltonian, which destroy the conservation of up spins and therefore destroy the continuous and non-branching worldlines of up spins.

### Energy


## The updating scheme

Since the configurations are well-defined, all we need to do to transform the problem into a classical Monte Carlo problem is to find a method to locally update the configuration.

It can be easily observed that the deformation of a worldline is highly constrained.
Among ways of deformation that preserve the number of up spins, only moving a part of a world line lying on a bond of a shaded plaquette (that is, without flipping) across the shaded plaquette is possible.
Other ways of deformation either end up in worldlines going through shaded plaquette, or require a global change of the worldline.
To 
