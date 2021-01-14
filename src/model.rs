pub trait Model {
    type SimulationParameters;
    fn run<MP, SP, T, S, F: Fn(&Self) -> T, G: Fn(Vec<T>) -> S>(
        &mut self,
        model_param: &MP,
        sim_param: &SP,
        diagnose: F,
        binning: G,
    ) -> Vec<S>;
}