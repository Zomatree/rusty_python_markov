extern crate markov;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use markov::Chain;
use pyo3::types::{PyList, PyString, PyType};
use petgraph::{dot::{Dot, Config}, graph::Graph};


#[pyclass]
struct Markov {
    chain: Chain<String>
}

#[pymethods]
impl Markov {
    #[new]
    fn new() -> Self {
        Markov {
            chain: Chain::new()
        }
    }

    pub fn train(&mut self, messages: &PyList) -> PyResult<()> {
        for elem in messages.iter() {
            self.train_single(elem.downcast::<PyString>().unwrap()).unwrap();
        }
        Ok(())
    }

    pub fn train_single(&mut self, message: &PyString) -> PyResult<()> {
        self.chain.feed_str(message.downcast::<PyString>().unwrap().to_str().unwrap());
        Ok(())
    }

    pub fn generate(&self) -> PyResult<String> {
        Ok(self.chain.generate_str())
    }

    pub fn generate_seeded(&self, seed: &str) -> PyResult<String> {
        Ok(self.chain.generate_str_from_token(seed))
    }

    pub fn graph(&self) -> PyResult<String> {
        let graph: Graph<Vec<Option<String>>, f64> = self.chain.graph();
        Ok(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])))
    } 

    pub fn save(&self, path: &PyString) -> PyResult<()> {
        self.chain.save(path.downcast::<PyString>().unwrap().to_str().unwrap()).unwrap();
        Ok(())
    }

    #[classmethod]
    pub fn load(_: &PyType, path: &PyString) -> PyResult<Markov> {
        Ok(Markov {chain: Chain::load(path.downcast::<PyString>().unwrap().to_str().unwrap()).unwrap()})
    }
}


#[pyfunction]
#[text_signature = "(messages: List[str], /)"]
fn generate_text(messages: &PyList) -> PyResult<String> {
    let mut chain = Chain::new();
    for elem in messages.iter() {
        chain.feed_str(elem.downcast::<PyString>().unwrap().to_str().unwrap_or_default());
    }
    Ok(chain.generate_str())
}

#[pymodule]
fn markov(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_text, m)?)?;
    m.add_class::<Markov>()?;
    Ok(())
}
