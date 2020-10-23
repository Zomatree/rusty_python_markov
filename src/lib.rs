extern crate markov;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use markov::Chain;
use pyo3::types::{PyList, PyString};
use petgraph::dot::{Dot, Config};


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
            self.chain.feed_str(elem.downcast::<PyString>().unwrap().to_str().unwrap_or_default());
        }
        Ok(())
    }

    pub fn generate(&self) -> PyResult<String> {
        Ok(self.chain.generate_str())
    }

    pub fn generate_seeded(&self, seed: &str) -> PyResult<String> {
        Ok(self.chain.generate_str_from_token(seed))
    }

    pub fn graph(&self) -> PyResult<String> {
        Ok(format!("{:?}", Dot::with_config(&self.chain.graph(), &[Config::EdgeNoLabel])))
    } 

}


#[pyfunction]
#[text_signature = "(messages: List[str], /)"]
fn gen_text(messages: &PyList) -> PyResult<String> {
    let mut chain = Chain::new();
    for elem in messages.iter() {
        chain.feed_str(elem.downcast::<PyString>().unwrap().to_str().unwrap_or_default());
    }
    Ok(chain.generate_str())
}

#[pymodule]
fn markov(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_text, m)?)?;
    m.add_class::<Markov>()?;
    Ok(())
}

