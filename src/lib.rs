extern crate markov;
use markov::Chain;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyList, PyString, PyType};

#[pyclass]
#[text_signature = "(order: Optional[int], /)"]
struct Markov {
    chain: Chain<String>,
}

#[pymethods]
impl Markov {
    #[new]
    fn new(order: Option<&PyInt>) -> PyResult<Self> {
        Ok(Markov {
            chain: match order {
                None => Chain::new(),
                Some(order) => Chain::of_order(order.extract::<usize>()?),
            },
        })
    }

    pub fn train(&mut self, messages: &PyList) -> PyResult<()> {
        for elem in messages.iter() {
            self.train_single(elem.downcast::<PyString>()?)?;
        }
        Ok(())
    }

    pub fn train_single(&mut self, message: &PyString) -> PyResult<()> {
        self.chain.feed_str(message.to_str()?);
        Ok(())
    }

    pub fn generate(&self) -> String {
        self.chain.generate_str()
    }

    pub fn generate_seeded(&self, seed: &str) -> String {
        self.chain.generate_str_from_token(seed)
    }

    pub fn save(&self, path: &PyString) -> PyResult<()> {
        self.chain.save(path.to_str()?)?;
        Ok(())
    }

    #[classmethod]
    pub fn load(_: &PyType, path: &PyString) -> PyResult<Markov> {
        Ok(Markov {
            chain: Chain::load(path.to_str()?)?,
        })
    }
}

#[pymodule]
fn markov(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Markov>()?;
    Ok(())
}
