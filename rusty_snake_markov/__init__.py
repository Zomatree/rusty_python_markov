from . import markov
from .utils import LRUDict
from typing import Dict, Tuple

Markov = markov.Markov

class Cache:
    def __init__(self, max_size=8):
        self.max_size = max_size
        self.model_cache: Dict[Tuple[int, ...], Markov] = LRUDict(max_size=max_size)
 
    def get_model(self, query: Tuple[int, ...]) -> Markov:
        if query in self.model_cache:
            return self.model_cache[query]
         
        chain = self.model_cache[query] = Markov()
        return chain
