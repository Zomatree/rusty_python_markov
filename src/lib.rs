extern crate markov;
use pyo3::prelude::*;
use markov::Chain;
use pyo3::types::{PyInt, PyList, PyString, PyType};
use petgraph::{dot::{Dot, Config}, graph::Graph};


#[pyclass]
#[text_signature = "(order: Optional[int], /)"]
struct Markov {
    chain: Chain<String>
}

#[pymethods]
impl Markov {
    #[new]
    fn new(order: &PyInt) -> PyResult<Self> {
        Ok(Markov {
            chain: match order.is_none() {
                true => Chain::new(),
                false => {
                    let value: usize = order.extract()?;
                    Chain::of_order(value)
                }
            }
        })
    }

    pub fn train(&mut self, messages: &PyList) -> PyResult<()> {
        for elem in messages.iter() {
            self.train_single(elem.downcast::<PyString>()?)?;
        }
        Ok(())
    }

    pub fn train_single(&mut self, message: &PyString) -> PyResult<()> {
        self.chain.feed_str(message.downcast::<PyString>()?.to_str()?);
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
        self.chain.save(path.downcast::<PyString>()?.to_str()?)?;
        Ok(())
    }

    #[classmethod]
    pub fn load(_: &PyType, path: &PyString) -> PyResult<Markov> {
        Ok(Markov {chain: Chain::load(path.downcast::<PyString>()?.to_str()?)?})
    }
}

#[pymodule]
fn markov(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Markov>()?;
    Ok(())
}
